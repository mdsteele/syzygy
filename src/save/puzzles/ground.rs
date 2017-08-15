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

use std::cmp;
use toml;

use save::{Access, Location};
use save::util::{ACCESS_KEY, Tomlable, to_table};
use super::PuzzleState;

// ========================================================================= //

const ELINSA_ROW_KEY: &str = "elinsa";
const POSITIONS_KEY: &str = "positions";

const NUM_ROWS: i32 = 7;
const MAX_POSITION: i32 = 9;
const MIN_ELINSA_ROW: i32 = -1;
const MAX_ELINSA_ROW: i32 = NUM_ROWS;
const INITIAL_POSITIONS: &[i32] = &[9, 9, 9, 9, 9, 9, 8];
const INITIAL_ELINSA_ROW: i32 = MAX_ELINSA_ROW;

// ========================================================================= //

pub struct GroundState {
    access: Access,
    positions: Vec<i32>,
    elinsa_row: i32,
    is_initial: bool,
}

impl GroundState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.positions[0] = MAX_POSITION;
        self.elinsa_row = MIN_ELINSA_ROW;
        self.is_initial = false;
    }

    pub fn num_rows() -> i32 { NUM_ROWS }

    pub fn max_position() -> i32 { MAX_POSITION }

    pub fn get_position(&self, row: i32) -> i32 {
        assert!(row >= 0 && row < NUM_ROWS);
        self.positions[row as usize]
    }

    pub fn set_position(&mut self, row: i32, pos: i32) {
        assert!(row >= 0 && row < NUM_ROWS);
        assert!(pos >= 0 && pos <= MAX_POSITION);
        self.positions[row as usize] = pos;
        self.is_initial = self.elinsa_row == INITIAL_ELINSA_ROW &&
                          &self.positions as &[i32] == INITIAL_POSITIONS;
    }

    pub fn get_elinsa_row(&self) -> i32 { self.elinsa_row }

    pub fn set_elinsa_row(&mut self, row: i32) {
        assert!(row >= MIN_ELINSA_ROW && row <= MAX_ELINSA_ROW);
        self.elinsa_row = row;
        self.is_initial = self.elinsa_row == INITIAL_ELINSA_ROW &&
                          &self.positions as &[i32] == INITIAL_POSITIONS;
        if self.elinsa_row == MIN_ELINSA_ROW {
            self.access = Access::Solved;
        }
    }

    pub fn fall_from(&self, mut row: i32, pos: i32) -> i32 {
        row += 1;
        while row < NUM_ROWS {
            if self.get_position(row) == pos {
                break;
            }
            row += 1;
        }
        row
    }
}

impl PuzzleState for GroundState {
    fn location() -> Location { Location::ShiftingGround }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.is_initial }

    fn reset(&mut self) {
        self.positions = INITIAL_POSITIONS.to_vec();
        self.elinsa_row = INITIAL_ELINSA_ROW;
        self.is_initial = true;
    }
}

impl Tomlable for GroundState {
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
            table.insert(ELINSA_ROW_KEY.to_string(),
                         toml::Value::Integer(self.elinsa_row as i64));
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> GroundState {
        let mut table = to_table(value);
        let mut access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let elinsa_row = if access.is_solved() {
            MIN_ELINSA_ROW
        } else {
            let mut row = table.remove(ELINSA_ROW_KEY)
                               .map(i32::from_toml)
                               .unwrap_or(INITIAL_ELINSA_ROW);
            if row < MIN_ELINSA_ROW || row > MAX_ELINSA_ROW {
                row = INITIAL_ELINSA_ROW;
            }
            if row == MIN_ELINSA_ROW {
                access = Access::Solved;
            }
            row
        };
        let mut positions = Vec::<i32>::pop_from_table(&mut table,
                                                       POSITIONS_KEY);
        if positions.len() != INITIAL_POSITIONS.len() {
            positions = INITIAL_POSITIONS.to_vec();
        } else {
            for position in positions.iter_mut() {
                *position = cmp::min(cmp::max(0, *position), MAX_POSITION);
            }
        }
        let is_initial = &positions as &[i32] == INITIAL_POSITIONS &&
                         elinsa_row == INITIAL_ELINSA_ROW;
        GroundState {
            access: access,
            positions: positions,
            elinsa_row: elinsa_row,
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
    use super::{ELINSA_ROW_KEY, GroundState, INITIAL_ELINSA_ROW,
                INITIAL_POSITIONS, MIN_ELINSA_ROW, POSITIONS_KEY};

    #[test]
    fn toml_round_trip() {
        let mut state = GroundState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.set_position(0, 1);
        state.set_position(1, 1);
        state.set_position(2, 2);
        state.set_position(3, 2);
        state.set_position(4, 3);
        state.set_position(5, 3);
        state.set_position(6, 4);
        state.set_elinsa_row(6);

        let state = GroundState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.get_elinsa_row(), 6);
        assert_eq!(state.positions, vec![1, 1, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn from_empty_toml() {
        let state = GroundState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.get_elinsa_row(), INITIAL_ELINSA_ROW);
        assert_eq!(&state.positions as &[i32], INITIAL_POSITIONS);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = GroundState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.elinsa_row, MIN_ELINSA_ROW);
    }

    #[test]
    fn from_elinsa_already_at_top_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        table.insert(ELINSA_ROW_KEY.to_string(),
                     toml::Value::Integer(MIN_ELINSA_ROW as i64));
        let state = GroundState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.elinsa_row, MIN_ELINSA_ROW);
    }

    #[test]
    fn from_invalid_elinsa_row_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ELINSA_ROW_KEY.to_string(), toml::Value::Integer(77));
        let state = GroundState::from_toml(toml::Value::Table(table));
        assert_eq!(state.elinsa_row, INITIAL_ELINSA_ROW);
        assert!(!state.is_solved());

        let mut table = toml::value::Table::new();
        table.insert(ELINSA_ROW_KEY.to_string(), toml::Value::Integer(-77));
        let state = GroundState::from_toml(toml::Value::Table(table));
        assert_eq!(state.elinsa_row, INITIAL_ELINSA_ROW);
        assert!(!state.is_solved());
    }

    #[test]
    fn from_invalid_positions_toml() {
        let mut table = toml::value::Table::new();
        table.insert(POSITIONS_KEY.to_string(),
                     toml::Value::Array(vec![1, 2, -3, 4, 55, 66, 77]
                         .into_iter()
                         .map(toml::Value::Integer)
                         .collect()));
        let state = GroundState::from_toml(toml::Value::Table(table));
        assert_eq!(state.positions, vec![1, 2, 0, 4, 9, 9, 9]);

        let mut table = toml::value::Table::new();
        table.insert(POSITIONS_KEY.to_string(),
                     toml::Value::Array(vec![1, 2, 3, 4, 5, 6]
                         .into_iter()
                         .map(toml::Value::Integer)
                         .collect()));
        let state = GroundState::from_toml(toml::Value::Table(table));
        assert_eq!(&state.positions as &[i32], INITIAL_POSITIONS);
    }
}

// ========================================================================= //
