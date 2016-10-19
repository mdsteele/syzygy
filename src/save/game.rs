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
use std::default::Default;
use super::access::Access;
use super::location::Location;
use toml;

// ========================================================================= //

const ACCESS_KEY: &'static str = "access";
const LOCATION_KEY: &'static str = "location";

// ========================================================================= //

#[derive(Default)]
pub struct Game {
    location: Location,
    prolog: PrologState,
    pub a_light_in_the_attic: AtticState,
}

impl Game {
    pub fn new() -> Game { Default::default() }

    pub fn from_toml(value: toml::Value) -> Game {
        let mut table = to_table(value);
        let prolog = pop_table(&mut table, Location::Prolog.key());
        let attic = pop_table(&mut table, Location::ALightInTheAttic.key());
        Game {
            location: Location::from_toml(table.get(LOCATION_KEY)),
            prolog: PrologState::from_toml(prolog),
            a_light_in_the_attic: AtticState::from_toml(attic),
        }
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(LOCATION_KEY.to_string(), self.location.to_toml());
        table.insert(Location::Prolog.key().to_string(),
                     self.prolog.to_toml());
        table.insert(Location::ALightInTheAttic.key().to_string(),
                     self.a_light_in_the_attic.to_toml());
        toml::Value::Table(table)
    }

    pub fn location(&self) -> Location { self.location }

    pub fn is_unlocked(&self, location: Location) -> bool {
        match location {
            Location::Map => true,
            Location::Prolog => true,
            Location::ALightInTheAttic => self.is_solved(Location::Prolog),
        }
    }

    pub fn is_solved(&self, location: Location) -> bool {
        self.access(location) >= Access::Solved
    }

    pub fn access(&self, location: Location) -> Access {
        match location {
            Location::Map => Access::Solved,
            Location::Prolog => self.prolog.access,
            Location::ALightInTheAttic => self.a_light_in_the_attic.access,
        }
    }
}

// ========================================================================= //

#[derive(Default)]
pub struct PrologState {
    access: Access,
}

impl PrologState {
    fn from_toml(table: toml::Table) -> PrologState {
        PrologState { access: Access::from_toml(table.get(ACCESS_KEY)) }
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        toml::Value::Table(table)
    }
}

// ========================================================================= //

#[derive(Default)]
pub struct AtticState {
    access: Access,
    toggled: BTreeSet<i32>,
}

impl AtticState {
    fn from_toml(mut table: toml::Table) -> AtticState {
        AtticState {
            access: Access::from_toml(table.get(ACCESS_KEY)),
            toggled: pop_array(&mut table, "toggled")
                         .iter()
                         .filter_map(|value| value.as_integer())
                         .map(|idx| idx as i32)
                         .filter(|&idx| 0 <= idx && idx < 16)
                         .collect(),
        }
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        let toggled = self.toggled
                          .iter()
                          .map(|&idx| toml::Value::Integer(idx as i64))
                          .collect();
        table.insert("toggled".to_string(), toml::Value::Array(toggled));
        toml::Value::Table(table)
    }

    pub fn is_visited(&self) -> bool { self.access > Access::Unvisited }

    pub fn visit(&mut self) { self.access.visit(); }

    pub fn is_solved(&self) -> bool { self.access == Access::Solved }

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

    pub fn any_toggled(&self) -> bool { !self.toggled.is_empty() }

    pub fn toggle(&mut self, pos: (i32, i32)) {
        let (col, row) = pos;
        if col >= 1 && col <= 4 && row >= 1 && row <= 4 {
            let index = (row - 1) * 4 + (col - 1);
            if self.toggled.contains(&index) {
                self.toggled.remove(&index);
            } else {
                self.toggled.insert(index);
            }
            let correct: Vec<i32> = vec![0, 3, 4, 9, 10, 13, 15];
            let actual: Vec<i32> = self.toggled.iter().cloned().collect();
            if actual == correct {
                self.access = Access::Solved;
            }
        }
    }

    pub fn reset(&mut self) { self.toggled.clear(); }
}

// ========================================================================= //

fn pop_array(table: &mut toml::Table, key: &str) -> toml::Array {
    if let Some(value) = table.remove(key) {
        to_array(value)
    } else {
        toml::Array::new()
    }
}

fn pop_table(table: &mut toml::Table, key: &str) -> toml::Table {
    if let Some(value) = table.remove(key) {
        to_table(value)
    } else {
        toml::Table::new()
    }
}

fn to_array(value: toml::Value) -> toml::Array {
    match value {
        toml::Value::Array(array) => array,
        _ => toml::Array::new(),
    }
}

fn to_table(value: toml::Value) -> toml::Table {
    match value {
        toml::Value::Table(table) => table,
        _ => toml::Table::new(),
    }
}

// ========================================================================= //
