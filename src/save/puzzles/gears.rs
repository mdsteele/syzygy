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

use std::cmp::{max, min};
use toml;

use save::{Access, Location};
use save::util::{ACCESS_KEY, Tomlable, to_table};
use super::PuzzleState;

// ========================================================================= //

const UGRENT_ROW_KEY: &str = "ugrent";
const POSITIONS_KEY: &str = "positions";

const NUM_ROWS: i32 = 8;
const MIN_UGRENT_ROW: i32 = -1;
const MAX_UGRENT_ROW: i32 = NUM_ROWS;
const INITIAL_POSITIONS: &[i32] = &[1, 0, 0, 0, 10, 10, 10, 9];
const INITIAL_UGRENT_ROW: i32 = MAX_UGRENT_ROW;

// ========================================================================= //

pub struct GearsState {
    access: Access,
    positions: Vec<i32>,
    ugrent_row: i32,
    is_initial: bool,
}

impl GearsState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.positions[0] = GearsState::max_position_for_row(0);
        self.ugrent_row = MIN_UGRENT_ROW;
        self.is_initial = false;
    }

    pub fn num_rows() -> i32 { NUM_ROWS }

    pub fn min_position_for_row(row: i32) -> i32 {
        assert!(row >= 0 && row < NUM_ROWS);
        if row == 4 { 6 } else { 0 }
    }

    pub fn max_position_for_row(row: i32) -> i32 {
        assert!(row >= 0 && row < NUM_ROWS);
        if row == 3 { 4 } else { 10 }
    }

    pub fn get_position(&self, row: i32) -> i32 {
        assert!(row >= 0 && row < NUM_ROWS);
        self.positions[row as usize]
    }

    pub fn set_position(&mut self, row: i32, pos: i32) {
        assert!(row >= 0 && row < NUM_ROWS);
        assert!(pos >= GearsState::min_position_for_row(row) &&
                    pos <= GearsState::max_position_for_row(row));
        self.positions[row as usize] = pos;
        self.is_initial = self.ugrent_row == INITIAL_UGRENT_ROW &&
            &self.positions as &[i32] == INITIAL_POSITIONS;
    }

    pub fn get_ugrent_row(&self) -> i32 { self.ugrent_row }

    pub fn set_ugrent_row(&mut self, row: i32) {
        assert!(row >= MIN_UGRENT_ROW && row <= MAX_UGRENT_ROW);
        self.ugrent_row = row;
        self.is_initial = self.ugrent_row == INITIAL_UGRENT_ROW &&
            &self.positions as &[i32] == INITIAL_POSITIONS;
        if self.ugrent_row == MIN_UGRENT_ROW {
            self.access = Access::Solved;
        }
    }

    pub fn fall_from(&self, mut row: i32, pos: i32) -> i32 {
        row += 1;
        while row < NUM_ROWS {
            if self.get_position(row) == pos {
                let prev_row_clear = if row == 4 {
                    self.get_position(2) != pos
                } else if row == 5 {
                    self.get_position(3) != pos && self.get_position(4) != pos
                } else {
                    self.get_position(row - 1) != pos
                };
                if prev_row_clear {
                    break;
                }
            }
            row += 1;
        }
        row
    }
}

impl PuzzleState for GearsState {
    fn location() -> Location { Location::ShiftGears }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.is_initial }

    fn reset(&mut self) {
        self.positions = INITIAL_POSITIONS.to_vec();
        self.ugrent_row = INITIAL_UGRENT_ROW;
        self.is_initial = true;
    }
}

impl Tomlable for GearsState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_initial && !self.is_solved() {
            let positions = self.positions
                .iter()
                .map(|&idx| toml::Value::Integer(idx as i64))
                .collect();
            table.insert(POSITIONS_KEY.to_string(),
                         toml::Value::Array(positions));
            table.insert(UGRENT_ROW_KEY.to_string(),
                         toml::Value::Integer(self.ugrent_row as i64));
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> GearsState {
        let mut table = to_table(value);
        let mut access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let ugrent_row = if access.is_solved() {
            MIN_UGRENT_ROW
        } else {
            let mut row = table
                .remove(UGRENT_ROW_KEY)
                .map(i32::from_toml)
                .unwrap_or(INITIAL_UGRENT_ROW);
            if row < MIN_UGRENT_ROW || row > MAX_UGRENT_ROW {
                row = INITIAL_UGRENT_ROW;
            }
            if row == MIN_UGRENT_ROW {
                access = Access::Solved;
            }
            row
        };
        let mut positions = Vec::<i32>::pop_from_table(&mut table,
                                                       POSITIONS_KEY);
        if positions.len() != INITIAL_POSITIONS.len() {
            positions = INITIAL_POSITIONS.to_vec();
        } else {
            for (row, position) in positions.iter_mut().enumerate() {
                *position =
                    min(max(GearsState::min_position_for_row(row as i32),
                            *position),
                        GearsState::max_position_for_row(row as i32));
            }
        }
        let is_initial = &positions as &[i32] == INITIAL_POSITIONS &&
            ugrent_row == INITIAL_UGRENT_ROW;
        GearsState {
            access: access,
            positions: positions,
            ugrent_row: ugrent_row,
            is_initial: is_initial,
        }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use save::{Access, PuzzleState};
    use save::util::{ACCESS_KEY, Tomlable};
    use super::{GearsState, INITIAL_POSITIONS, INITIAL_UGRENT_ROW,
                MIN_UGRENT_ROW, POSITIONS_KEY, UGRENT_ROW_KEY};

    #[test]
    fn toml_round_trip() {
        let mut state = GearsState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.set_position(0, 3);
        state.set_position(1, 3);
        state.set_position(2, 4);
        state.set_position(3, 4);
        state.set_position(4, 6);
        state.set_position(5, 6);
        state.set_position(6, 7);
        state.set_position(7, 7);
        state.set_ugrent_row(3);

        let state = GearsState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.get_ugrent_row(), 3);
        assert_eq!(state.positions, vec![3, 3, 4, 4, 6, 6, 7, 7]);
    }

    #[test]
    fn from_empty_toml() {
        let state = GearsState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.get_ugrent_row(), INITIAL_UGRENT_ROW);
        assert_eq!(&state.positions as &[i32], INITIAL_POSITIONS);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = GearsState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.ugrent_row, MIN_UGRENT_ROW);
    }

    #[test]
    fn from_ugrent_already_at_top_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        table.insert(UGRENT_ROW_KEY.to_string(),
                     toml::Value::Integer(MIN_UGRENT_ROW as i64));
        let state = GearsState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.ugrent_row, MIN_UGRENT_ROW);
    }

    #[test]
    fn from_invalid_ugrent_row_toml() {
        let mut table = toml::value::Table::new();
        table.insert(UGRENT_ROW_KEY.to_string(), toml::Value::Integer(77));
        let state = GearsState::from_toml(toml::Value::Table(table));
        assert_eq!(state.ugrent_row, INITIAL_UGRENT_ROW);
        assert!(!state.is_solved());

        let mut table = toml::value::Table::new();
        table.insert(UGRENT_ROW_KEY.to_string(), toml::Value::Integer(-77));
        let state = GearsState::from_toml(toml::Value::Table(table));
        assert_eq!(state.ugrent_row, INITIAL_UGRENT_ROW);
        assert!(!state.is_solved());
    }

    #[test]
    fn from_invalid_positions_toml() {
        let mut table = toml::value::Table::new();
        table.insert(POSITIONS_KEY.to_string(),
                     toml::Value::Array(vec![1, 2, -3, 44, -5, 66, 77, 88]
                                            .into_iter()
                                            .map(toml::Value::Integer)
                                            .collect()));
        let state = GearsState::from_toml(toml::Value::Table(table));
        assert_eq!(state.positions, vec![1, 2, 0, 4, 6, 10, 10, 10]);

        let mut table = toml::value::Table::new();
        table.insert(POSITIONS_KEY.to_string(),
                     toml::Value::Array(vec![1, 2, 3, 4, 5, 6, 7]
                                            .into_iter()
                                            .map(toml::Value::Integer)
                                            .collect()));
        let state = GearsState::from_toml(toml::Value::Table(table));
        assert_eq!(&state.positions as &[i32], INITIAL_POSITIONS);
    }

    #[test]
    fn fall_from_platform() {
        let mut state = GearsState::from_toml(toml::Value::Boolean(false));
        state.positions = vec![1, 0, 0, 0, 10, 10, 1, 9];
        assert_eq!(state.fall_from(0, 1), 6);
    }

    #[test]
    fn fall_from_stacked_platforms_1() {
        let mut state = GearsState::from_toml(toml::Value::Boolean(false));
        state.positions = vec![0, 1, 1, 2, 10, 10, 1, 1];
        assert_eq!(state.fall_from(1, 1), 6);
    }

    #[test]
    fn fall_from_stacked_platforms_2() {
        let mut state = GearsState::from_toml(toml::Value::Boolean(false));
        state.positions = vec![0, 1, 1, 1, 10, 1, 2, 1];
        assert_eq!(state.fall_from(1, 1), 7);
    }

    #[test]
    fn fall_from_stacked_platforms_3() {
        let mut state = GearsState::from_toml(toml::Value::Boolean(false));
        state.positions = vec![9, 10, 10, 0, 10, 10, 9, 10];
        assert_eq!(state.fall_from(1, 10), 7);
    }
}

// ========================================================================= //
