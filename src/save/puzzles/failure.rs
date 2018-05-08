// +--------------------------------------------------------------------------+
// | Copyright 2016 Matthew D. Steele <mdsteele@alum.mit.edu>                 |
// |                                                                          |
// | This file is part of System Syzygy.                                      |
// |                                                                          |
// | System Syzygy is free software: you can redistribute it and/or modify it |
// | under the terms of the GNU General Public License as published by the    |
// | Free Software Foundation, either version 3 of the License, or (at your   |
// | option) any later version.                                               |
// |                                                                          |
// | System Syzygy is distributed in the hope that it will be useful, but     |
// | WITHOUT ANY WARRANTY; without even the implied warranty of               |
// | MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU        |
// | General Public License for details.                                      |
// |                                                                          |
// | You should have received a copy of the GNU General Public License along  |
// | with System Syzygy.  If not, see <http://www.gnu.org/licenses/>.         |
// +--------------------------------------------------------------------------+

use toml;

use save::{Access, Location};
use save::pyramid::{Board, Coords, Team};
use save::util::{ACCESS_KEY, Tomlable, to_table};
use super::PuzzleState;

// ========================================================================= //

const BOARD_KEY: &str = "board";
const MID_SCENE_DONE_KEY: &str = "mid_done";

// ========================================================================= //

pub struct FailureState {
    access: Access,
    mid_scene_done: bool,
    board: Board,
    committed_board: Board,
}

impl FailureState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.mid_scene_done = true;
        for coords in Coords::all() {
            if self.board.piece_at(coords).is_none() {
                let team = if self.board.srb_supply() > 0 {
                    Team::SRB
                } else {
                    Team::You
                };
                self.board.set_piece_at(coords, team);
            }
        }
        debug_assert_eq!(self.board.you_supply(), 0);
        debug_assert_eq!(self.board.srb_supply(), 0);
    }

    pub fn mid_scene_is_done(&self) -> bool { self.mid_scene_done }

    pub fn set_mid_scene_is_done(&mut self, done: bool) {
        self.mid_scene_done = done;
    }

    pub fn board(&self) -> &Board { &self.board }

    pub fn board_mut(&mut self) -> &mut Board { &mut self.board }

    pub fn roll_back_board(&mut self) {
        self.board = self.committed_board.clone();
    }

    pub fn commit_board(&mut self) {
        self.committed_board = self.board.clone();
    }

    pub fn clear_committed_board(&mut self) {
        self.committed_board = Board::new();
    }
}

impl PuzzleState for FailureState {
    fn location() -> Location { Location::SystemFailure }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.board.is_empty() }

    fn reset(&mut self) {
        self.board = Board::new();
        self.committed_board = Board::new();
    }
}

impl Tomlable for FailureState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if self.mid_scene_done {
            table.insert(MID_SCENE_DONE_KEY.to_string(),
                         toml::Value::Boolean(self.mid_scene_done));
            table
                .insert(BOARD_KEY.to_string(), self.committed_board.to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> FailureState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let board = Board::pop_from_table(&mut table, BOARD_KEY);
        let mut state = FailureState {
            access: access,
            mid_scene_done: bool::pop_from_table(&mut table,
                                                 MID_SCENE_DONE_KEY),
            board: board.clone(),
            committed_board: board,
        };
        if access.is_solved() {
            state.solve();
        }
        state
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use save::Access;
    use save::pyramid::{Coords, Team};
    use save::util::{ACCESS_KEY, Tomlable};
    use super::FailureState;

    #[test]
    fn toml_round_trip() {
        let mut state = FailureState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.mid_scene_done = true;
        state.board.set_piece_at(Coords::new(0, 1), Team::You);
        state.board.set_piece_at(Coords::new(0, 5), Team::SRB);
        state.commit_board();

        let state = FailureState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert!(state.mid_scene_done);
        assert_eq!(state.board().piece_at(Coords::new(0, 1)), Some(Team::You));
        assert_eq!(state.board().piece_at(Coords::new(0, 5)), Some(Team::SRB));
    }

    #[test]
    fn from_empty_toml() {
        let state = FailureState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert!(!state.mid_scene_is_done());
        assert!(state.board().is_empty());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = FailureState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert!(state.mid_scene_is_done());
        assert!(!state.board().is_empty());
        assert_eq!(state.board().you_supply(), 0);
        assert_eq!(state.board().srb_supply(), 0);
        assert_eq!(state.board().piece_at(Coords::new(7, 0)), Some(Team::You));
    }
}

// ========================================================================= //
