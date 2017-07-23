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
use save::util::{ACCESS_KEY, Tomlable};
use super::PuzzleState;

// ========================================================================= //

const BOARD_KEY: &str = "board";
const MID_SCENE_DONE_KEY: &str = "mid_done";

// ========================================================================= //

pub struct FailureState {
    access: Access,
    mid_scene_done: bool,
    board: Board,
}

impl FailureState {
    pub fn from_toml(mut table: toml::value::Table) -> FailureState {
        FailureState {
            access: Access::pop_from_table(&mut table, ACCESS_KEY),
            mid_scene_done: table.get(MID_SCENE_DONE_KEY)
                                 .and_then(toml::Value::as_bool)
                                 .unwrap_or(false),
            board: Board::pop_from_table(&mut table, BOARD_KEY),
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
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

    pub fn board(&self) -> &Board { &self.board }

    pub fn board_mut(&mut self) -> &mut Board { &mut self.board }
}

impl PuzzleState for FailureState {
    fn location(&self) -> Location { Location::SystemFailure }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.board.is_empty() }

    fn reset(&mut self) { self.board = Board::new(); }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if self.mid_scene_done {
            table.insert(MID_SCENE_DONE_KEY.to_string(),
                         toml::Value::Boolean(self.mid_scene_done));
            table.insert(BOARD_KEY.to_string(), self.board.to_toml());
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //
