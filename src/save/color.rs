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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrimaryColor {
    Red,
    Green,
    Blue,
}

impl PrimaryColor {
    pub fn from_toml(value: Option<&toml::Value>) -> PrimaryColor {
        if let Some(string) = value.and_then(toml::Value::as_str) {
            match string {
                "red" => return PrimaryColor::Red,
                "green" => return PrimaryColor::Green,
                "blue" => return PrimaryColor::Blue,
                _ => {}
            }
        }
        Default::default()
    }

    pub fn to_toml(self) -> toml::Value {
        let string = match self {
            PrimaryColor::Red => "red",
            PrimaryColor::Green => "green",
            PrimaryColor::Blue => "blue",
        };
        toml::Value::String(string.to_string())
    }
}

impl Default for PrimaryColor {
    fn default() -> PrimaryColor { PrimaryColor::Red }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::PrimaryColor;

    #[test]
    fn toml_round_trip() {
        let all =
            &[PrimaryColor::Red, PrimaryColor::Green, PrimaryColor::Blue];
        for original in all {
            let result = PrimaryColor::from_toml(Some(&original.to_toml()));
            assert_eq!(result, *original);
        }
    }
}

// ========================================================================= //
