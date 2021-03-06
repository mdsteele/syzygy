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

use std::collections::HashSet;
use toml;

use super::PuzzleState;
use crate::save::util::{to_table, Tomlable, ACCESS_KEY};
use crate::save::{Access, Location};

// ========================================================================= //

const TOGGLED_KEY: &str = "toggled";

const SOLVED_TOGGLED: &[i32] = &[0, 3, 4, 9, 10, 13, 15];

// ========================================================================= //

pub struct AtticState {
    access: Access,
    toggled: HashSet<i32>,
}

impl AtticState {
    pub fn new() -> AtticState {
        AtticState::from_toml(toml::Value::Boolean(false))
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.toggled = SOLVED_TOGGLED.iter().cloned().collect();
    }

    pub fn is_lit(&self, pos: (i32, i32)) -> bool {
        match pos {
            (1, 0) => self.is_toggled((1, 1)) ^ self.is_toggled((2, 1)),
            (2, 0) => {
                self.is_toggled((1, 1))
                    ^ self.is_toggled((2, 1))
                    ^ self.is_toggled((3, 1))
            }
            (3, 0) => true,
            (4, 0) => self.is_toggled((3, 1)) ^ self.is_toggled((4, 1)),
            (0, 1) => self.is_toggled((1, 2)),
            (1, 1) => self.is_toggled((1, 1)) ^ self.is_toggled((2, 2)),
            (2, 1) => {
                self.is_toggled((2, 1))
                    ^ self.is_toggled((3, 1))
                    ^ self.is_toggled((1, 2))
                    ^ self.is_toggled((3, 2))
            }
            (3, 1) => {
                self.is_toggled((3, 1))
                    ^ self.is_toggled((4, 1))
                    ^ self.is_toggled((2, 2))
            }
            (4, 1) => {
                self.is_toggled((3, 1))
                    ^ self.is_toggled((4, 1))
                    ^ self.is_toggled((3, 2))
                    ^ self.is_toggled((4, 2))
            }
            (5, 1) => self.is_toggled((4, 1)) ^ self.is_toggled((4, 2)),
            (0, 2) => self.is_toggled((1, 2)),
            (1, 2) => {
                self.is_toggled((1, 1))
                    ^ self.is_toggled((1, 2))
                    ^ self.is_toggled((1, 3))
                    ^ self.is_toggled((2, 3))
            }
            (2, 2) => {
                self.is_toggled((1, 1))
                    ^ self.is_toggled((2, 1))
                    ^ self.is_toggled((3, 1))
                    ^ self.is_toggled((1, 2))
                    ^ self.is_toggled((2, 2))
                    ^ self.is_toggled((2, 3))
            }
            (3, 2) => {
                self.is_toggled((2, 1))
                    ^ self.is_toggled((4, 1))
                    ^ self.is_toggled((3, 2))
                    ^ self.is_toggled((2, 3))
                    ^ self.is_toggled((3, 3))
                    ^ self.is_toggled((4, 3))
            }
            (4, 2) => !(self.is_toggled((3, 1)) ^ self.is_toggled((4, 2))),
            (5, 2) => self.is_toggled((4, 1)) ^ self.is_toggled((4, 3)),
            (0, 3) => !self.is_toggled((1, 4)),
            (1, 3) => self.is_toggled((1, 3)) ^ self.is_toggled((2, 4)),
            (2, 3) => {
                !(self.is_toggled((3, 2))
                    ^ self.is_toggled((2, 3))
                    ^ self.is_toggled((1, 4))
                    ^ self.is_toggled((2, 4)))
            }
            (3, 3) => {
                !(self.is_toggled((4, 2))
                    ^ self.is_toggled((3, 3))
                    ^ self.is_toggled((4, 3))
                    ^ self.is_toggled((2, 4))
                    ^ self.is_toggled((3, 4)))
            }
            (4, 3) => {
                !(self.is_toggled((3, 2))
                    ^ self.is_toggled((4, 2))
                    ^ self.is_toggled((4, 3)))
            }
            (5, 3) => self.is_toggled((4, 4)),
            (0, 4) => !self.is_toggled((1, 3)),
            (1, 4) => {
                self.is_toggled((1, 3))
                    ^ self.is_toggled((1, 4))
                    ^ self.is_toggled((2, 4))
            }
            (2, 4) => self.is_toggled((2, 3)),
            (3, 4) => {
                self.is_toggled((3, 3))
                    ^ self.is_toggled((4, 3))
                    ^ self.is_toggled((2, 4))
                    ^ self.is_toggled((3, 4))
                    ^ self.is_toggled((4, 4))
            }
            (4, 4) => self.is_toggled((4, 4)),
            (5, 4) => self.is_toggled((4, 3)) ^ self.is_toggled((4, 4)),
            (1, 5) => self.is_toggled((1, 4)) ^ self.is_toggled((2, 4)),
            (2, 5) => self.is_toggled((2, 4)),
            (3, 5) => {
                !(self.is_toggled((2, 4))
                    ^ self.is_toggled((3, 4))
                    ^ self.is_toggled((4, 4)))
            }
            (4, 5) => !(self.is_toggled((3, 4))),
            _ => false,
        }
    }

    pub fn is_toggled(&self, pos: (i32, i32)) -> bool {
        let (col, row) = pos;
        col >= 1
            && col <= 4
            && row >= 1
            && row <= 4
            && self.toggled.contains(&((row - 1) * 4 + (col - 1)))
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
            if self.toggled == SOLVED_TOGGLED.iter().cloned().collect() {
                self.access = Access::Solved;
            }
        }
    }
}

impl PuzzleState for AtticState {
    fn location() -> Location {
        Location::ALightInTheAttic
    }

    fn access(&self) -> Access {
        self.access
    }

    fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    fn can_reset(&self) -> bool {
        !self.toggled.is_empty()
    }

    fn reset(&mut self) {
        self.toggled.clear();
    }
}

impl Tomlable for AtticState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && !self.toggled.is_empty() {
            table.insert(TOGGLED_KEY.to_string(), self.toggled.to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> AtticState {
        let mut table = to_table(value);
        let mut access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let toggled = if access == Access::Solved {
            SOLVED_TOGGLED.iter().cloned().collect()
        } else {
            let mut toggled =
                HashSet::<i32>::pop_from_table(&mut table, TOGGLED_KEY);
            toggled.retain(|&idx| 0 <= idx && idx < 16);
            toggled
        };
        if toggled == SOLVED_TOGGLED.iter().cloned().collect() {
            access = Access::Solved;
        }
        AtticState { access, toggled }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use super::{AtticState, SOLVED_TOGGLED, TOGGLED_KEY};
    use crate::save::util::{Tomlable, ACCESS_KEY};
    use crate::save::Access;

    #[test]
    fn toml_round_trip() {
        let mut state = AtticState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.toggled.insert(3);
        state.toggled.insert(1);
        state.toggled.insert(4);

        let state = AtticState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.toggled, vec![1, 3, 4].into_iter().collect());
    }

    #[test]
    fn from_empty_toml() {
        let state = AtticState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert!(state.toggled.is_empty());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = AtticState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.toggled, SOLVED_TOGGLED.iter().cloned().collect());
    }

    #[test]
    fn from_invalid_toggled_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        let toggled = vec![-1, 0, 15, 16];
        table.insert(
            TOGGLED_KEY.to_string(),
            toml::Value::Array(
                toggled.into_iter().map(toml::Value::Integer).collect(),
            ),
        );

        let state = AtticState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Unsolved);
        assert_eq!(state.toggled, vec![0, 15].into_iter().collect());
    }

    #[test]
    fn from_toggled_already_correct_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        let toggled = SOLVED_TOGGLED
            .iter()
            .map(|&t| toml::Value::Integer(t as i64))
            .collect();
        table.insert(TOGGLED_KEY.to_string(), toml::Value::Array(toggled));

        let state = AtticState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.toggled, SOLVED_TOGGLED.iter().cloned().collect());
    }
}

// ========================================================================= //
