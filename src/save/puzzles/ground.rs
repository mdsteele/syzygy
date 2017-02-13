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
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array, to_i32};

// ========================================================================= //

const ELINSA_ROW_KEY: &'static str = "elinsa";
const POSITIONS_KEY: &'static str = "positions";

const NUM_ROWS: i32 = 7;
const MAX_POSITION: i32 = 9;
const MIN_ELINSA_ROW: i32 = -1;
const MAX_ELINSA_ROW: i32 = NUM_ROWS;
const INITIAL_POSITIONS: &'static [i32] = &[9, 9, 9, 9, 9, 9, 8];
const INITIAL_ELINSA_ROW: i32 = MAX_ELINSA_ROW;

// ========================================================================= //

pub struct GroundState {
    access: Access,
    positions: Vec<i32>,
    elinsa_row: i32,
    is_initial: bool,
}

impl GroundState {
    pub fn from_toml(mut table: toml::value::Table) -> GroundState {
        let mut positions: Vec<i32> = pop_array(&mut table, POSITIONS_KEY)
                                          .iter()
                                          .filter_map(toml::Value::as_integer)
                                          .filter(|&pos| {
                                              0 <= pos &&
                                              pos <= MAX_POSITION as i64
                                          })
                                          .map(|idx| idx as i32)
                                          .collect();
        if positions.len() != INITIAL_POSITIONS.len() {
            positions = INITIAL_POSITIONS.to_vec();
        }
        let mut elinsa_row = table.remove(ELINSA_ROW_KEY)
                                  .map(to_i32)
                                  .unwrap_or(INITIAL_ELINSA_ROW);
        if elinsa_row < -1 || elinsa_row > MAX_ELINSA_ROW {
            elinsa_row = INITIAL_ELINSA_ROW;
        }
        let is_initial = &positions as &[i32] == INITIAL_POSITIONS &&
                         elinsa_row == INITIAL_ELINSA_ROW;
        GroundState {
            access: Access::from_toml(table.get(ACCESS_KEY)),
            positions: positions,
            elinsa_row: elinsa_row,
            is_initial: is_initial,
        }
    }

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

impl Default for GroundState {
    fn default() -> GroundState {
        GroundState {
            access: Default::default(),
            positions: INITIAL_POSITIONS.to_vec(),
            elinsa_row: INITIAL_ELINSA_ROW,
            is_initial: true,
        }
    }
}

impl PuzzleState for GroundState {
    fn location(&self) -> Location { Location::ShiftingGround }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.is_initial }

    fn reset(&mut self) {
        self.positions = INITIAL_POSITIONS.to_vec();
        self.elinsa_row = INITIAL_ELINSA_ROW;
        self.is_initial = true;
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_initial {
            let positions = self.positions
                                .iter()
                                .map(|&idx| toml::Value::Integer(idx as i64))
                                .collect();
            table.insert(POSITIONS_KEY.to_string(),
                         toml::Value::Array(positions));
        }
        table.insert(ELINSA_ROW_KEY.to_string(),
                     toml::Value::Integer(self.elinsa_row as i64));
        toml::Value::Table(table)
    }
}

// ========================================================================= //
