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

use gui::Point;
use save::{Access, Location};
use save::plane::{PlaneGrid, PlaneObj};
use save::util::{ACCESS_KEY, Tomlable, pop_array, to_table};
use super::PuzzleState;

// ========================================================================= //

const PIPES_KEY: &str = "pipes";
const STAGE_KEY: &str = "stage";

const FIRST_STAGE: i32 = 1;
const LAST_STAGE: i32 = 4;

#[cfg_attr(rustfmt, rustfmt_skip)]
const FIRST_STAGE_WALLS: &[(i32, i32)] = &[
    (0, 0), (2, 0), (4, 0), (6, 0), (8, 0), (10, 0), (12, 0),
    (0, 1), (2, 1), (4, 1), (6, 1), (10, 1),
    (6, 2), (8, 2), (11, 2),
    (1, 3), (2, 3), (4, 3), (6, 3), (8, 3), (10, 3), (12, 3),
    (2, 4), (3, 4), (4, 4), (6, 4), (8, 4), (11, 4), (12, 4),
    (0, 5), (4, 5), (8, 5), (9, 5), (10, 5),
    (2, 6), (4, 6), (5, 6), (7, 6), (12, 6),
    (1, 7), (4, 7), (6, 7), (7, 7), (9, 7), (11, 7),
    (1, 8), (6, 8), (9, 8),
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const SOLVED_PIPES: &[&[(i32, i32)]] = &[
    &[(8, 2), (8, 1), (7, 1), (6, 1), (5, 1), (4, 1), (3, 1), (2, 1), (1, 1),
      (1, 2), (1, 3)],
    &[(8, 2), (9, 2), (9, 3)],
    &[(8, 2), (7, 2), (6, 2), (6, 3), (6, 4)],
    &[(8, 2), (8, 3), (8, 4), (8, 5), (8, 6), (7, 6), (6, 6), (5, 6), (4, 6)],
    &[(2, 3), (1, 3)],
    &[(2, 3), (2, 2), (3, 2), (4, 2), (5, 2), (5, 3), (6, 3), (7, 3), (8, 3),
      (9, 3)],
    &[(2, 3), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4)],
    &[(2, 3), (3, 3), (4, 3), (4, 4), (4, 5), (4, 6)],
    &[(11, 4), (12, 4), (12, 3), (12, 2), (12, 1), (11, 1), (10, 1), (9, 1),
      (9, 0), (8, 0), (7, 0), (6, 0), (5, 0), (4, 0), (3, 0), (2, 0), (1, 0),
      (0, 0), (0, 1), (0, 2), (0, 3), (1, 3)],
    &[(11, 4), (11, 3), (10, 3), (9, 3)],
    &[(11, 4), (10, 4), (10, 5), (9, 5), (8, 5), (7, 5), (7, 4), (6, 4)],
    &[(11, 4), (11, 5), (11, 6), (10, 6), (10, 7), (9, 7), (8, 7), (7, 7),
      (6, 7), (5, 7), (4, 7), (4, 6)],
    &[(1, 5), (1, 4), (1, 3)],
    &[(1, 5), (1, 6), (1, 7), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (6, 8),
      (7, 8), (8, 8), (9, 8), (9, 7), (9, 6), (9, 5), (9, 4), (9, 3)],
    &[(1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (6, 4)],
    &[(1, 5), (0, 5), (0, 6), (0, 7), (1, 7), (2, 7), (3, 7), (3, 6), (4, 6)],
];

// ========================================================================= //

pub struct DayState {
    access: Access,
    grid: PlaneGrid,
    stage: i32,
}

impl DayState {
    fn initial_grid() -> PlaneGrid {
        let mut grid = PlaneGrid::new(13, 9);
        for &(col, row) in FIRST_STAGE_WALLS {
            grid.place_object(col, row, PlaneObj::Wall);
        }
        grid.place_object(9, 3, PlaneObj::BlueNode);
        grid.place_object(1, 5, PlaneObj::RedNode);
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
            2 => {
                self.grid.remove_object(6, 2);
                self.grid.remove_object(4, 3);
                self.grid.remove_object(10, 3);
                self.grid.remove_object(3, 4);
                self.grid.place_object(6, 4, PlaneObj::BlueNode);
                self.grid.remove_object(8, 4);
                self.grid.place_object(11, 4, PlaneObj::RedNode);
            }
            3 => {
                self.grid.remove_object(0, 1);
                self.grid.remove_object(2, 1);
                self.grid.remove_object(4, 1);
                self.grid.remove_object(6, 1);
                self.grid.remove_object(10, 1);
                self.grid.place_object(8, 2, PlaneObj::RedNode);
                self.grid.place_object(1, 3, PlaneObj::BlueNode);
                self.grid.place_object(6, 3, PlaneObj::Cross);
                self.grid.place_object(8, 3, PlaneObj::Cross);
                self.grid.remove_object(5, 6);
                self.grid.remove_object(7, 6);
                self.grid.remove_object(1, 8);
            }
            4 => {
                self.grid.remove_object(0, 0);
                self.grid.remove_object(2, 0);
                self.grid.remove_object(4, 0);
                self.grid.remove_object(6, 0);
                self.grid.remove_object(8, 0);
                self.grid.place_object(2, 3, PlaneObj::RedNode);
                self.grid.remove_object(12, 3);
                self.grid.remove_object(2, 4);
                self.grid.place_object(4, 4, PlaneObj::Cross);
                self.grid.remove_object(12, 4);
                self.grid.remove_object(0, 5);
                self.grid.place_object(4, 5, PlaneObj::Cross);
                self.grid.place_object(8, 5, PlaneObj::Cross);
                self.grid.place_object(9, 5, PlaneObj::Cross);
                self.grid.remove_object(10, 5);
                self.grid.place_object(4, 6, PlaneObj::BlueNode);
                self.grid.place_object(1, 7, PlaneObj::Cross);
                self.grid.remove_object(4, 7);
                self.grid.remove_object(6, 7);
                self.grid.remove_object(7, 7);
                self.grid.place_object(9, 7, PlaneObj::Cross);
                self.grid.remove_object(6, 8);
                self.grid.remove_object(9, 8);
            }
            _ => panic!("bad stage: {}", self.stage),
        }
    }
}

impl PuzzleState for DayState {
    fn location() -> Location { Location::PlaneAsDay }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.grid.pipes().is_empty() }

    fn reset(&mut self) { self.grid.remove_all_pipes(); }

    fn replay(&mut self) {
        self.grid = DayState::initial_grid();
        self.stage = FIRST_STAGE;
        self.access = Access::BeginReplay;
    }
}

impl Tomlable for DayState {
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

    fn from_toml(value: toml::Value) -> DayState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let mut state = DayState {
            access: access,
            grid: DayState::initial_grid(),
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

    use gui::Point;
    use save::Access;
    use save::util::{ACCESS_KEY, Tomlable};
    use super::{DayState, FIRST_STAGE, LAST_STAGE, STAGE_KEY};

    #[test]
    fn toml_round_trip() {
        let mut state = DayState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.stage = FIRST_STAGE + 1;
        assert!(state.grid_mut()
                     .toggle_pipe(Point::new(3, 0), Point::new(3, 1)));

        let state = DayState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.stage, FIRST_STAGE + 1);
        assert_eq!(state.grid.pipes(),
                   &vec![vec![Point::new(3, 0), Point::new(3, 1)]]);
    }

    #[test]
    fn from_empty_toml() {
        let state = DayState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.stage, FIRST_STAGE);
        assert!(state.grid.pipes().is_empty());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = DayState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.stage, LAST_STAGE);
        assert!(state.grid.all_nodes_are_connected());
    }

    #[test]
    fn from_invalid_stage_toml() {
        let mut table = toml::value::Table::new();
        table.insert(STAGE_KEY.to_string(),
                     toml::Value::Integer((FIRST_STAGE - 1) as i64));
        let state = DayState::from_toml(toml::Value::Table(table));
        assert_eq!(state.stage, FIRST_STAGE);

        let mut table = toml::value::Table::new();
        table.insert(STAGE_KEY.to_string(),
                     toml::Value::Integer((LAST_STAGE + 1) as i64));
        let state = DayState::from_toml(toml::Value::Table(table));
        assert_eq!(state.stage, FIRST_STAGE);
    }

}

// ========================================================================= //
