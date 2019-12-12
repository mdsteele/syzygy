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

use crate::gui::Point;
use crate::save::util::Tomlable;

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn degrees(self) -> i32 {
        match self {
            Direction::East => 0,
            Direction::South => 90,
            Direction::West => 180,
            Direction::North => -90,
        }
    }

    pub fn is_vertical(self) -> bool {
        match self {
            Direction::East | Direction::West => false,
            Direction::South | Direction::North => true,
        }
    }

    pub fn is_parallel_to(self, other: Direction) -> bool {
        self.is_vertical() == other.is_vertical()
    }

    pub fn delta(self) -> Point {
        match self {
            Direction::East => Point::new(1, 0),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
            Direction::North => Point::new(0, -1),
        }
    }

    pub fn from_delta(delta: Point) -> Direction {
        if delta.x().abs() < delta.y().abs() {
            if delta.y() < 0 {
                Direction::North
            } else {
                Direction::South
            }
        } else {
            if delta.x() < 0 {
                Direction::West
            } else {
                Direction::East
            }
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
        }
    }

    pub fn rotated_cw(self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    pub fn rotated_ccw(self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        }
    }

    pub fn rotated_ccw_by(self, by: i32) -> Direction {
        match mod_floor(by, 4) {
            1 => self.rotated_ccw(),
            2 => self.opposite(),
            3 => self.rotated_cw(),
            _ => self,
        }
    }

    #[cfg(test)]
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ]
    }
}

impl Tomlable for Direction {
    fn to_toml(&self) -> toml::Value {
        let string = match *self {
            Direction::East => "E",
            Direction::South => "S",
            Direction::West => "W",
            Direction::North => "N",
        };
        toml::Value::String(string.to_string())
    }

    fn from_toml(value: toml::Value) -> Direction {
        if let Some(string) = value.as_str() {
            match string {
                "E" => return Direction::East,
                "S" => return Direction::South,
                "W" => return Direction::West,
                "N" => return Direction::North,
                _ => {}
            }
        }
        Direction::East
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use crate::gui::Point;
    use crate::save::util::Tomlable;
    use super::Direction;

    #[test]
    fn toml_round_trip() {
        for original in Direction::all() {
            let result = Direction::from_toml(original.to_toml());
            assert_eq!(result, original);
        }
    }

    #[test]
    fn delta_round_trip() {
        for original in Direction::all() {
            let result = Direction::from_delta(original.delta());
            assert_eq!(result, original);
        }
    }

    #[test]
    fn opposites() {
        for original in Direction::all() {
            let opposite = original.opposite();
            assert!(original.is_parallel_to(opposite));
            assert!(opposite.is_parallel_to(original));
            assert_eq!(opposite.opposite(), original);
            assert_eq!(original.delta() + opposite.delta(), Point::new(0, 0));
        }
    }

    #[test]
    fn rotated() {
        for original in Direction::all() {
            let rotated = original.rotated_cw();
            assert!(!original.is_parallel_to(rotated));
            assert!(!rotated.is_parallel_to(original));
            assert_eq!(rotated.rotated_cw(), original.opposite());
            assert_eq!(rotated.rotated_ccw(), original);
        }
    }

    #[test]
    fn rotated_by() {
        for original in Direction::all() {
            assert_eq!(original.rotated_ccw_by(0), original);
            assert_eq!(original.rotated_ccw_by(1), original.rotated_ccw());
            assert_eq!(original.rotated_ccw_by(-1), original.rotated_cw());
            assert_eq!(original.rotated_ccw_by(2), original.opposite());
            assert_eq!(original.rotated_ccw_by(-2), original.opposite());
            assert_eq!(original.rotated_ccw_by(3), original.rotated_cw());
            assert_eq!(original.rotated_ccw_by(-3), original.rotated_ccw());
            assert_eq!(original.rotated_ccw_by(4), original);
            assert_eq!(original.rotated_ccw_by(-4), original);
            assert_eq!(original.rotated_ccw_by(5), original.rotated_ccw());
            assert_eq!(original.rotated_ccw_by(-5), original.rotated_cw());
        }
    }
}

// ========================================================================= //
