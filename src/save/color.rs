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

use crate::save::util::Tomlable;

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrimaryColor {
    Red,
    Green,
    Blue,
}

impl Tomlable for PrimaryColor {
    fn from_toml(value: toml::Value) -> PrimaryColor {
        if let Some(string) = value.as_str() {
            match string {
                "red" => return PrimaryColor::Red,
                "green" => return PrimaryColor::Green,
                "blue" => return PrimaryColor::Blue,
                _ => {}
            }
        }
        Default::default()
    }

    fn to_toml(&self) -> toml::Value {
        let string = match *self {
            PrimaryColor::Red => "red",
            PrimaryColor::Green => "green",
            PrimaryColor::Blue => "blue",
        };
        toml::Value::String(string.to_string())
    }
}

impl Default for PrimaryColor {
    fn default() -> PrimaryColor {
        PrimaryColor::Red
    }
}

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MixedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl MixedColor {
    pub fn from_rgb(red: bool, green: bool, blue: bool) -> MixedColor {
        match (red, green, blue) {
            (false, false, false) => MixedColor::Black,
            (true, false, false) => MixedColor::Red,
            (false, true, false) => MixedColor::Green,
            (true, true, false) => MixedColor::Yellow,
            (false, false, true) => MixedColor::Blue,
            (true, false, true) => MixedColor::Magenta,
            (false, true, true) => MixedColor::Cyan,
            (true, true, true) => MixedColor::White,
        }
    }

    pub fn has_red(self) -> bool {
        match self {
            MixedColor::Black => false,
            MixedColor::Red => true,
            MixedColor::Green => false,
            MixedColor::Yellow => true,
            MixedColor::Blue => false,
            MixedColor::Magenta => true,
            MixedColor::Cyan => false,
            MixedColor::White => true,
        }
    }

    pub fn has_green(self) -> bool {
        match self {
            MixedColor::Black => false,
            MixedColor::Red => false,
            MixedColor::Green => true,
            MixedColor::Yellow => true,
            MixedColor::Blue => false,
            MixedColor::Magenta => false,
            MixedColor::Cyan => true,
            MixedColor::White => true,
        }
    }

    pub fn has_blue(self) -> bool {
        match self {
            MixedColor::Black => false,
            MixedColor::Red => false,
            MixedColor::Green => false,
            MixedColor::Yellow => false,
            MixedColor::Blue => true,
            MixedColor::Magenta => true,
            MixedColor::Cyan => true,
            MixedColor::White => true,
        }
    }

    pub fn with_red(self) -> MixedColor {
        MixedColor::from_rgb(true, self.has_green(), self.has_blue())
    }

    pub fn with_green(&mut self) -> MixedColor {
        MixedColor::from_rgb(self.has_red(), true, self.has_blue())
    }

    pub fn with_blue(&mut self) -> MixedColor {
        MixedColor::from_rgb(self.has_red(), self.has_green(), true)
    }

    #[cfg(test)]
    pub fn all() -> Vec<MixedColor> {
        vec![
            MixedColor::Black,
            MixedColor::Red,
            MixedColor::Green,
            MixedColor::Yellow,
            MixedColor::Blue,
            MixedColor::Magenta,
            MixedColor::Cyan,
            MixedColor::White,
        ]
    }
}

impl Default for MixedColor {
    fn default() -> MixedColor {
        MixedColor::Red
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::{MixedColor, PrimaryColor};
    use crate::save::util::Tomlable;

    #[test]
    fn primary_color_toml_round_trip() {
        let all =
            &[PrimaryColor::Red, PrimaryColor::Green, PrimaryColor::Blue];
        for &original in all {
            let result = PrimaryColor::from_toml(original.to_toml());
            assert_eq!(result, original);
        }
    }

    #[test]
    fn mixed_color_rgb() {
        for &red in &[false, true] {
            for &green in &[false, true] {
                for &blue in &[false, true] {
                    let color = MixedColor::from_rgb(red, green, blue);
                    assert_eq!(color.has_red(), red);
                    assert_eq!(color.has_green(), green);
                    assert_eq!(color.has_blue(), blue);
                }
            }
        }
    }
}

// ========================================================================= //
