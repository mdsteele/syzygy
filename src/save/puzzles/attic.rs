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

use std::collections::BTreeSet;
use toml;

use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array};

// ========================================================================= //

const TOGGLED_KEY: &'static str = "toggled";

const SOLVED_TOGGLED: &'static [i32] = &[0, 3, 4, 9, 10, 13, 15];

// ========================================================================= //

#[derive(Default)]
pub struct AtticState {
    access: Access,
    toggled: BTreeSet<i32>,
}

impl AtticState {
    pub fn from_toml(mut table: toml::value::Table) -> AtticState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let toggled = if access == Access::Solved {
            SOLVED_TOGGLED.iter().cloned().collect()
        } else {
            pop_array(&mut table, TOGGLED_KEY)
                .iter()
                .filter_map(toml::Value::as_integer)
                .filter(|&idx| 0 <= idx && idx < 16)
                .map(|idx| idx as i32)
                .collect()
        };
        AtticState {
            access: access,
            toggled: toggled,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.toggled = SOLVED_TOGGLED.iter().cloned().collect();
    }

    pub fn is_lit(&self, pos: (i32, i32)) -> bool {
        match pos {
            (1, 0) => self.is_toggled((1, 1)) ^ self.is_toggled((2, 1)),
            (2, 0) => {
                (self.is_toggled((1, 1)) ^ self.is_toggled((2, 1)) ^
                 self.is_toggled((3, 1)))
            }
            (3, 0) => true,
            (4, 0) => self.is_toggled((3, 1)) ^ self.is_toggled((4, 1)),
            (0, 1) => self.is_toggled((1, 2)),
            (1, 1) => self.is_toggled((1, 1)) ^ self.is_toggled((2, 2)),
            (2, 1) => {
                self.is_toggled((2, 1)) ^ self.is_toggled((3, 1)) ^
                self.is_toggled((1, 2)) ^
                self.is_toggled((3, 2))
            }
            (3, 1) => {
                self.is_toggled((3, 1)) ^ self.is_toggled((4, 1)) ^
                self.is_toggled((2, 2))
            }
            (4, 1) => {
                self.is_toggled((3, 1)) ^ self.is_toggled((4, 1)) ^
                self.is_toggled((3, 2)) ^
                self.is_toggled((4, 2))
            }
            (5, 1) => self.is_toggled((4, 1)) ^ self.is_toggled((4, 2)),
            (0, 2) => self.is_toggled((1, 2)),
            (1, 2) => {
                self.is_toggled((1, 1)) ^ self.is_toggled((1, 2)) ^
                self.is_toggled((1, 3)) ^
                self.is_toggled((2, 3))
            }
            (2, 2) => {
                self.is_toggled((1, 1)) ^ self.is_toggled((2, 1)) ^
                self.is_toggled((3, 1)) ^
                self.is_toggled((1, 2)) ^
                self.is_toggled((2, 2)) ^
                self.is_toggled((2, 3))
            }
            (3, 2) => {
                self.is_toggled((2, 1)) ^ self.is_toggled((4, 1)) ^
                self.is_toggled((3, 2)) ^
                self.is_toggled((2, 3)) ^
                self.is_toggled((3, 3)) ^
                self.is_toggled((4, 3))
            }
            (4, 2) => !(self.is_toggled((3, 1)) ^ self.is_toggled((4, 2))),
            (5, 2) => self.is_toggled((4, 1)) ^ self.is_toggled((4, 3)),
            (0, 3) => !self.is_toggled((1, 4)),
            (1, 3) => self.is_toggled((1, 3)) ^ self.is_toggled((2, 4)),
            (2, 3) => {
                !(self.is_toggled((3, 2)) ^ self.is_toggled((2, 3)) ^
                  self.is_toggled((1, 4)) ^
                  self.is_toggled((2, 4)))
            }
            (3, 3) => {
                !(self.is_toggled((4, 2)) ^ self.is_toggled((3, 3)) ^
                  self.is_toggled((4, 3)) ^
                  self.is_toggled((2, 4)) ^
                  self.is_toggled((3, 4)))
            }
            (4, 3) => {
                !(self.is_toggled((3, 2)) ^ self.is_toggled((4, 2)) ^
                  self.is_toggled((4, 3)))
            }
            (5, 3) => self.is_toggled((4, 4)),
            (0, 4) => !self.is_toggled((1, 3)),
            (1, 4) => {
                self.is_toggled((1, 3)) ^ self.is_toggled((1, 4)) ^
                self.is_toggled((2, 4))
            }
            (2, 4) => self.is_toggled((2, 3)),
            (3, 4) => {
                self.is_toggled((3, 3)) ^ self.is_toggled((4, 3)) ^
                self.is_toggled((2, 4)) ^
                self.is_toggled((3, 4)) ^
                self.is_toggled((4, 4))
            }
            (4, 4) => self.is_toggled((4, 4)),
            (5, 4) => self.is_toggled((4, 3)) ^ self.is_toggled((4, 4)),
            (1, 5) => self.is_toggled((1, 4)) ^ self.is_toggled((2, 4)),
            (2, 5) => self.is_toggled((2, 4)),
            (3, 5) => {
                !(self.is_toggled((2, 4)) ^ self.is_toggled((3, 4)) ^
                  self.is_toggled((4, 4)))
            }
            (4, 5) => !(self.is_toggled((3, 4))),
            _ => false,
        }
    }

    pub fn is_toggled(&self, pos: (i32, i32)) -> bool {
        let (col, row) = pos;
        col >= 1 && col <= 4 && row >= 1 && row <= 4 &&
        self.toggled.contains(&((row - 1) * 4 + (col - 1)))
    }

    pub fn toggle(&mut self, pos: (i32, i32)) {
        let (col, row) = pos;
        if col >= 1 && col <= 4 && row >= 1 && row <= 4 {
            let index = (row - 1) * 4 + (col - 1);
            if self.toggled.contains(&index) {
                self.toggled.remove(&index);
            } else {
                self.toggled.insert(index);
            }
            let actual: Vec<i32> = self.toggled.iter().cloned().collect();
            if &actual as &[i32] == SOLVED_TOGGLED {
                self.access = Access::Solved;
            }
        }
    }
}

impl PuzzleState for AtticState {
    fn location(&self) -> Location { Location::ALightInTheAttic }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.toggled.is_empty() }

    fn reset(&mut self) { self.toggled.clear(); }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && !self.toggled.is_empty() {
            let toggled = self.toggled
                              .iter()
                              .map(|&idx| toml::Value::Integer(idx as i64))
                              .collect();
            table.insert(TOGGLED_KEY.to_string(), toml::Value::Array(toggled));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //
