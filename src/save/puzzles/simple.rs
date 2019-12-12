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

use crate::gui::Point;
use crate::save::{Access, Location};
use crate::save::plane::{PlaneGrid, PlaneObj};
use crate::save::util::{ACCESS_KEY, Tomlable, pop_array, to_table};
use super::PuzzleState;

// ========================================================================= //

const PIPES_KEY: &str = "pipes";
const STAGE_KEY: &str = "stage";

const FIRST_STAGE: i32 = 2;
const LAST_STAGE: i32 = 5;

#[cfg_attr(rustfmt, rustfmt_skip)]
const SOLVED_PIPES: &[&[(i32, i32)]] = &[
    &[(4, 3), (5, 3), (6, 3)],
    &[(4, 3), (4, 4), (3, 4), (2, 4)],
    &[(4, 3), (3, 3), (3, 4), (3, 5), (4, 5), (5, 5)],
    &[(4, 3), (4, 2), (4, 1), (5, 1), (6, 1), (7, 1), (7, 2), (8, 2), (8, 3),
      (8, 4), (8, 5), (7, 5)],
    &[(6, 3), (6, 2), (5, 2), (4, 2), (3, 2), (2, 2), (2, 3), (2, 4)],
    &[(6, 3), (6, 4), (5, 4), (5, 5)],
    &[(6, 3), (7, 3), (7, 4), (7, 5)],
    &[(2, 4), (1, 4), (1, 5), (1, 6), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7),
      (5, 6), (5, 5)],
    &[(2, 4), (2, 5), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6), (7, 5)],
    &[(5, 5), (6, 5), (7, 5)],
];

// ========================================================================= //

pub struct SimpleState {
    access: Access,
    grid: PlaneGrid,
    stage: i32,
}

impl SimpleState {
    fn initial_grid() -> PlaneGrid {
        let mut grid = PlaneGrid::new(10, 9);
        for row in 0..9 {
            for col in 0..10 {
                grid.place_object(col, row, PlaneObj::Wall);
            }
        }
        grid.remove_object(3, 1);
        grid.remove_object(4, 1);
        grid.remove_object(5, 1);
        grid.remove_object(3, 2);
        grid.remove_object(5, 2);
        grid.remove_object(2, 3);
        grid.remove_object(3, 3);
        grid.remove_object(5, 3);
        grid.place_object(2, 4, PlaneObj::PurpleNode);
        grid.remove_object(4, 4);
        grid.remove_object(5, 4);
        grid.place_object(5, 5, PlaneObj::PurpleNode);
        grid.remove_object(2, 6);
        grid.remove_object(4, 6);
        grid
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid.remove_all_pipes();
        while self.stage < LAST_STAGE {
            self.advance_stage();
        }
        for pipe in SOLVED_PIPES {
            let mut p1 = Point::new(pipe[0].0, pipe[0].1);
            for i in 1..pipe.len() {
                let p2 = Point::new(pipe[i].0, pipe[i].1);
                self.grid.toggle_pipe(p1, p2);
                p1 = p2;
            }
        }
        debug_assert!(self.grid.all_nodes_are_connected());
    }

    pub fn grid(&self) -> &PlaneGrid { &self.grid }

    pub fn grid_mut(&mut self) -> &mut PlaneGrid { &mut self.grid }

    pub fn advance_stage_if_done(&mut self) -> bool {
        if !self.grid.all_nodes_are_connected() {
            return false;
        }
        if self.stage == LAST_STAGE {
            self.access = Access::Solved;
        } else {
            self.advance_stage();
        }
        true
    }

    fn advance_stage(&mut self) {
        debug_assert!(self.stage >= FIRST_STAGE);
        debug_assert!(self.stage < LAST_STAGE);
        self.stage += 1;
        match self.stage {
            3 => {
                self.grid.place_object(6, 3, PlaneObj::PurpleNode);
                self.grid.remove_object(6, 4);
                self.grid.remove_object(2, 5);
                self.grid.remove_object(4, 5);
                self.grid.remove_object(3, 6);
                self.grid.remove_object(6, 6);
            }
            4 => {
                self.grid.remove_object(1, 1);
                self.grid.remove_object(1, 2);
                self.grid.remove_object(2, 2);
                self.grid.remove_object(6, 2);
                self.grid.remove_object(1, 3);
                self.grid.place_object(4, 3, PlaneObj::PurpleNode);
                self.grid.remove_object(1, 4);
                self.grid.remove_object(6, 5);
                self.grid.remove_object(1, 6);
            }
            5 => {
                self.grid.remove_object(6, 1);
                self.grid.remove_object(7, 1);
                self.grid.place_object(4, 2, PlaneObj::Cross);
                self.grid.remove_object(7, 2);
                self.grid.remove_object(8, 2);
                self.grid.remove_object(7, 3);
                self.grid.remove_object(8, 3);
                self.grid.place_object(3, 4, PlaneObj::Cross);
                self.grid.remove_object(7, 4);
                self.grid.remove_object(8, 4);
                self.grid.remove_object(1, 5);
                self.grid.remove_object(3, 5);
                self.grid.place_object(7, 5, PlaneObj::PurpleNode);
                self.grid.remove_object(8, 5);
                self.grid.place_object(5, 6, PlaneObj::Cross);
                self.grid.remove_object(7, 6);
                self.grid.remove_object(8, 6);
                self.grid.remove_object(1, 7);
                self.grid.remove_object(2, 7);
                self.grid.remove_object(3, 7);
                self.grid.remove_object(4, 7);
                self.grid.remove_object(5, 7);
                self.grid.remove_object(6, 7);
                self.grid.remove_object(8, 7);
            }
            _ => panic!("bad stage: {}", self.stage),
        }
    }
}

impl PuzzleState for SimpleState {
    fn location() -> Location { Location::PlaneAndSimple }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.grid.pipes().is_empty() }

    fn reset(&mut self) { self.grid.remove_all_pipes(); }

    fn replay(&mut self) {
        self.grid = SimpleState::initial_grid();
        self.stage = FIRST_STAGE;
        self.access = Access::BeginReplay;
    }
}

impl Tomlable for SimpleState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(STAGE_KEY.to_string(),
                         toml::Value::Integer(self.stage as i64));
            table.insert(PIPES_KEY.to_string(), self.grid.pipes_to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> SimpleState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let mut state = SimpleState {
            access: access,
            grid: SimpleState::initial_grid(),
            stage: FIRST_STAGE,
        };
        if access.is_solved() {
            state.solve();
        } else {
            let mut stage = i32::pop_from_table(&mut table, STAGE_KEY);
            if stage < FIRST_STAGE || stage > LAST_STAGE {
                stage = FIRST_STAGE;
            }
            while state.stage < stage {
                state.advance_stage();
            }
            state.grid.set_pipes_from_toml(pop_array(&mut table, PIPES_KEY));
        }
        state
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use crate::gui::Point;
    use crate::save::Access;
    use crate::save::util::{ACCESS_KEY, Tomlable};
    use super::{FIRST_STAGE, LAST_STAGE, STAGE_KEY, SimpleState};

    #[test]
    fn toml_round_trip() {
        let mut state = SimpleState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.stage = FIRST_STAGE + 1;
        assert!(state
                    .grid_mut()
                    .toggle_pipe(Point::new(3, 1), Point::new(4, 1)));

        let state = SimpleState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.stage, FIRST_STAGE + 1);
        assert_eq!(state.grid.pipes(),
                   &vec![vec![Point::new(3, 1), Point::new(4, 1)]]);
    }

    #[test]
    fn from_empty_toml() {
        let state = SimpleState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.stage, FIRST_STAGE);
        assert!(state.grid.pipes().is_empty());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = SimpleState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.stage, LAST_STAGE);
        assert!(state.grid.all_nodes_are_connected());
    }

    #[test]
    fn from_invalid_stage_toml() {
        let mut table = toml::value::Table::new();
        table.insert(STAGE_KEY.to_string(),
                     toml::Value::Integer((FIRST_STAGE - 1) as i64));
        let state = SimpleState::from_toml(toml::Value::Table(table));
        assert_eq!(state.stage, FIRST_STAGE);

        let mut table = toml::value::Table::new();
        table.insert(STAGE_KEY.to_string(),
                     toml::Value::Integer((LAST_STAGE + 1) as i64));
        let state = SimpleState::from_toml(toml::Value::Table(table));
        assert_eq!(state.stage, FIRST_STAGE);
    }

}

// ========================================================================= //
