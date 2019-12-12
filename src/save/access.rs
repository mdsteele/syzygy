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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Access {
    Unvisited,
    Unsolved,
    Solved,
    BeginReplay,
    Replaying,
}

impl Access {
    pub fn has_been_solved(&self) -> bool {
        *self >= Access::Solved
    }

    pub fn is_solved(&self) -> bool {
        *self == Access::Solved
    }

    pub fn has_been_visited(&self) -> bool {
        *self != Access::Unvisited
    }

    pub fn is_visited(&self) -> bool {
        *self != Access::Unvisited && *self != Access::BeginReplay
    }

    pub fn visit(&mut self) {
        if *self == Access::Unvisited {
            *self = Access::Unsolved;
        }
    }

    pub fn revisit(&mut self) {
        if *self == Access::BeginReplay {
            *self = Access::Replaying;
        }
    }
}

impl Default for Access {
    fn default() -> Access {
        Access::Unvisited
    }
}

impl Tomlable for Access {
    fn to_toml(&self) -> toml::Value {
        let string = match *self {
            Access::Unvisited => "unvisited",
            Access::Unsolved => "unsolved",
            Access::Solved => "solved",
            Access::BeginReplay => "begin_replay",
            Access::Replaying => "replaying",
        };
        toml::Value::String(string.to_string())
    }

    fn from_toml(value: toml::Value) -> Access {
        if let Some(string) = value.as_str() {
            match string {
                "unvisited" => return Access::Unvisited,
                "unsolved" => return Access::Unsolved,
                "solved" => return Access::Solved,
                "begin_replay" => return Access::BeginReplay,
                "replaying" => return Access::Replaying,
                _ => {}
            }
        }
        Default::default()
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::save::util::Tomlable;

    #[test]
    fn toml_round_trip() {
        let all = &[
            Access::Unvisited,
            Access::Unsolved,
            Access::Solved,
            Access::BeginReplay,
            Access::Replaying,
        ];
        for &original in all.iter() {
            let result = Access::from_toml(original.to_toml());
            assert_eq!(result, original);
        }
    }
}

// ========================================================================= //
