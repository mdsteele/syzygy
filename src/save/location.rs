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
use toml;

// ========================================================================= //

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Location {
    Map,
    ALightInTheAttic,
}

impl Location {
    pub fn key(self) -> &'static str {
        match self {
            Location::Map => "map",
            Location::ALightInTheAttic => "a_light_in_the_attic",
        }
    }

    pub fn from_toml(value: Option<&toml::Value>) -> Location {
        if let Some(string) = value.and_then(toml::Value::as_str) {
            match string {
                "map" => return Location::Map,
                "a_light_in_the_attic" => return Location::ALightInTheAttic,
                _ => {}
            }
        }
        Default::default()
    }

    pub fn to_toml(self) -> toml::Value {
        toml::Value::String(self.key().to_string())
    }
}

impl Default for Location {
    fn default() -> Location { Location::Map }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::Location;

    #[test]
    fn toml_round_trip() {
        let locations = &[Location::Map, Location::ALightInTheAttic];
        for original in locations {
            let result = Location::from_toml(&original.to_toml());
            assert_eq!(result, *original);
        }
    }
}

// ========================================================================= //
