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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Access {
    Unvisited,
    Unsolved,
    Solved,
    Replay,
}

impl Access {
    pub fn from_toml(value: Option<&toml::Value>) -> Access {
        if let Some(string) = value.and_then(toml::Value::as_str) {
            match string {
                "unvisited" => return Access::Unvisited,
                "unsolved" => return Access::Unsolved,
                "solved" => return Access::Solved,
                "replay" => return Access::Replay,
                _ => {}
            }
        }
        Default::default()
    }

    pub fn to_toml(self) -> toml::Value {
        let string = match self {
            Access::Unvisited => "unvisited",
            Access::Unsolved => "unsolved",
            Access::Solved => "solved",
            Access::Replay => "replay",
        };
        toml::Value::String(string.to_string())
    }

    pub fn visit(&mut self) {
        if *self < Access::Unsolved {
            *self = Access::Unsolved;
        }
    }
}

impl Default for Access {
    fn default() -> Access { Access::Unvisited }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::Access;

    #[test]
    fn toml_round_trip() {
        let all = &[Access::Unvisited,
                    Access::Unsolved,
                    Access::Solved,
                    Access::Replay];
        for original in all {
            let result = Access::from_toml(Some(&original.to_toml()));
            assert_eq!(result, *original);
        }
    }
}

// ========================================================================= //
