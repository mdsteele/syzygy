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

use num_integer::mod_floor;
use toml;

use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array, to_i32};

// ========================================================================= //

const COLUMNS_KEY: &str = "columns";

const WORDS: [&str; 6] = ["JAIL", "BUST", "SCANT", "LIONS", "LOAF", "TURF"];
const LINKAGES: [&[usize]; 6] = [&[0, 3], &[0, 1, 2], &[0, 2, 4], &[1, 3, 5],
                                 &[3, 4, 5], &[2, 5]];

const INITIAL_COLUMNS: [i32; 6] = [0, 0, 0, 0, 0, 0];
const SOLVED_COLUMNS: [i32; 6] = [2, 3, 1, 4, 0, 1];

// ========================================================================= //

pub struct WhatchaState {
    access: Access,
    columns: [i32; 6],
}

impl WhatchaState {
    pub fn from_toml(mut table: toml::value::Table) -> WhatchaState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let mut columns = INITIAL_COLUMNS;
        for (index, value) in pop_array(&mut table, COLUMNS_KEY)
            .into_iter()
            .enumerate() {
            if index >= WORDS.len() {
                break;
            }
            let value = to_i32(value);
            if value >= 0 && (value as usize) < WORDS[index].chars().count() {
                columns[index] = value;
            }
        }
        WhatchaState {
            access: access,
            columns: columns,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.columns = SOLVED_COLUMNS;
    }

    pub fn num_columns(&self) -> usize { self.columns.len() }

    pub fn column_offset(&self, col: usize) -> i32 {
        if col < 4 { -2 } else { -1 }
    }

    pub fn column_linkages(&self, col: usize) -> &[usize] { LINKAGES[col] }

    pub fn column_position(&self, col: usize) -> i32 { self.columns[col] }

    pub fn column_letters(&self, col: usize) -> Vec<char> {
        let position = self.column_position(col) as usize;
        let mut letters1: Vec<char> = WORDS[col].chars().collect();
        debug_assert!(position < letters1.len());
        let mut letters2 = letters1.split_off(position);
        letters2.append(&mut letters1);
        letters2
    }

    pub fn column_word_len(&self, col: usize) -> usize {
        WORDS[col].chars().count()
    }

    pub fn rotate_column(&mut self, col: usize, by: i32) {
        for &other in LINKAGES[col] {
            self.columns[other] = mod_floor(self.columns[other] - by,
                                            self.column_word_len(other) as
                                            i32);
        }
        if self.columns == SOLVED_COLUMNS {
            self.access = Access::Solved;
        }
    }
}

impl PuzzleState for WhatchaState {
    fn location(&self) -> Location { Location::WhatchaColumn }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.columns != INITIAL_COLUMNS }

    fn reset(&mut self) { self.columns = INITIAL_COLUMNS; }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && self.columns != INITIAL_COLUMNS {
            table.insert(COLUMNS_KEY.to_string(),
                         toml::Value::Array(self.columns
                                                .iter()
                                                .map(|&val| {
                                                    toml::Value::Integer(val as
                                                                         i64)
                                                })
                                                .collect()));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //
