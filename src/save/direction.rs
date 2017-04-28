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

use toml;

use gui::Point;

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn from_toml(value: Option<&toml::Value>) -> Direction {
        if let Some(string) = value.and_then(toml::Value::as_str) {
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

    pub fn to_toml(self) -> toml::Value {
        let string = match self {
            Direction::East => "E",
            Direction::South => "S",
            Direction::West => "W",
            Direction::North => "N",
        };
        toml::Value::String(string.to_string())
    }

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
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use gui::Point;
    use super::Direction;

    const ALL_DIRECTIONS: &[Direction] = &[Direction::East,
                                           Direction::South,
                                           Direction::West,
                                           Direction::North];

    #[test]
    fn toml_round_trip() {
        for &original in ALL_DIRECTIONS {
            let result = Direction::from_toml(Some(&original.to_toml()));
            assert_eq!(result, original);
        }
    }

    #[test]
    fn delta_round_trip() {
        for &original in ALL_DIRECTIONS {
            let result = Direction::from_delta(original.delta());
            assert_eq!(result, original);
        }
    }

    #[test]
    fn opposites() {
        for original in ALL_DIRECTIONS {
            let opposite = original.opposite();
            assert!(original.is_parallel_to(opposite));
            assert!(opposite.is_parallel_to(*original));
            assert_eq!(opposite.opposite(), *original);
            assert_eq!(original.delta() + opposite.delta(), Point::new(0, 0));
        }
    }

    #[test]
    fn rotated() {
        for original in ALL_DIRECTIONS {
            let rotated = original.rotated_cw();
            assert!(!original.is_parallel_to(rotated));
            assert!(!rotated.is_parallel_to(*original));
            assert_eq!(rotated.rotated_cw(), original.opposite());
            assert_eq!(rotated.rotated_ccw(), *original);
        }
    }
}

// ========================================================================= //
