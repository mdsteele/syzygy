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

use save::util::to_i32;

// ========================================================================= //

struct Column {
    letters: Vec<char>,
    linkages: Vec<(usize, i32)>,
    offset: i32,
    current_position: i32,
    solved_position: i32,
}

// ========================================================================= //

pub struct Columns {
    columns: Vec<Column>,
}

impl Columns {
    pub fn from_toml(spec: &[(&str, i32, i32, &[(usize, i32)])],
                     array: toml::value::Array)
                     -> Columns {
        let mut columns: Vec<Column> =
            spec.iter()
                .map(|&(word, offset, solved, linkages)| {
                Column {
                    letters: word.chars().collect(),
                    linkages: linkages.iter().cloned().collect(),
                    offset: offset,
                    current_position: 0,
                    solved_position: solved,
                }
            })
                .collect();
        for (index, value) in array.into_iter().enumerate() {
            if index >= columns.len() {
                break;
            }
            let column = &mut columns[index];
            let value = to_i32(value);
            if value >= 0 && (value as usize) < column.letters.len() {
                column.current_position = value;
            }
        }
        Columns { columns: columns }
    }

    pub fn to_toml(&self) -> toml::Value {
        toml::Value::Array(self.columns
                               .iter()
                               .map(|column| column.current_position as i64)
                               .map(toml::Value::Integer)
                               .collect())
    }

    pub fn num_columns(&self) -> usize { self.columns.len() }

    pub fn column_linkages(&self, col: usize) -> &[(usize, i32)] {
        &self.columns[col].linkages
    }

    pub fn set_linkages(&mut self, col: usize, linkages: Vec<(usize, i32)>) {
        debug_assert!(col < self.columns.len());
        debug_assert!(linkages.iter().all(|&(i, _)| i < self.columns.len()));
        self.columns[col].linkages = linkages;
    }

    pub fn column_offset(&self, col: usize) -> i32 {
        debug_assert!(col < self.columns.len());
        self.columns[col].offset
    }

    pub fn column_position(&self, col: usize) -> i32 {
        debug_assert!(col < self.columns.len());
        self.columns[col].current_position
    }

    pub fn column_letters(&self, col: usize) -> Vec<char> {
        let position = self.column_position(col) as usize;
        let mut letters1 = self.columns[col].letters.clone();
        debug_assert!(position < letters1.len());
        let mut letters2 = letters1.split_off(position);
        letters2.append(&mut letters1);
        letters2
    }

    pub fn column_word_len(&self, col: usize) -> usize {
        debug_assert!(col < self.columns.len());
        self.columns[col].letters.len()
    }

    pub fn rotate_column(&mut self, col: usize, by: i32) {
        debug_assert!(col < self.columns.len());
        let linkages = self.columns[col].linkages.clone();
        for (other, factor) in linkages.into_iter() {
            let column = &mut self.columns[other];
            column.current_position = mod_floor(column.current_position -
                                                by * factor,
                                                column.letters.len() as i32);
        }
    }

    pub fn is_solved(&self) -> bool {
        for column in self.columns.iter() {
            if column.current_position != column.solved_position {
                return false;
            }
        }
        true
    }

    pub fn solve(&mut self) {
        for column in self.columns.iter_mut() {
            column.current_position = column.solved_position;
        }
    }

    pub fn can_reset(&self) -> bool {
        for column in self.columns.iter() {
            if column.current_position != 0 {
                return true;
            }
        }
        false
    }

    pub fn reset(&mut self) {
        for column in self.columns.iter_mut() {
            column.current_position = 0;
        }
    }
}

// ========================================================================= //
