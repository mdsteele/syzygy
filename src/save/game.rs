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

use std::default::Default;
use super::location::Location;
use toml;

// ========================================================================= //

const LOCATION_KEY: &'static str = "location";

// ========================================================================= //

#[derive(Default)]
pub struct Game {
    location: Location,
}

impl Game {
    pub fn new() -> Game { Default::default() }

    pub fn from_toml(table: &toml::Table) -> Game {
        Game { location: Location::from_toml(table.get(LOCATION_KEY)) }
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(LOCATION_KEY.to_string(), self.location.to_toml());
        toml::Value::Table(table)
    }

    pub fn location(&self) -> Location { self.location }
}

// ========================================================================= //
