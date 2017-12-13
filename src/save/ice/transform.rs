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

use save::Direction;
use save::util::Tomlable;

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Transform {
    rotated_cw: i32,
    mirrored: bool,
}

impl Transform {
    fn new(rotated_cw: i32, mirrored: bool) -> Transform {
        Transform {
            rotated_cw: mod_floor(rotated_cw, 4),
            mirrored: mirrored,
        }
    }

    pub fn identity() -> Transform { Transform::new(0, false) }

    pub fn rotated_cw(self) -> Transform {
        Transform::new(self.rotated_cw + 1, self.mirrored)
    }

    pub fn flipped_horz(mut self) -> Transform {
        if self.rotated_cw % 2 == 0 {
            self.rotated_cw = 2 - self.rotated_cw;
        }
        self.mirrored = !self.mirrored;
        self
    }

    pub fn flipped_vert(self) -> Transform {
        Transform::new(-self.rotated_cw, !self.mirrored)
    }

    pub fn inverse(self) -> Transform {
        if self.mirrored {
            Transform::new(self.rotated_cw, true)
        } else {
            Transform::new(-self.rotated_cw, false)
        }
    }

    pub fn compose(mut self, other: Transform) -> Transform {
        if other.mirrored {
            self.rotated_cw = -self.rotated_cw;
            self.mirrored = !self.mirrored;
        }
        self.rotated_cw = mod_floor(self.rotated_cw + other.rotated_cw, 4);
        self
    }

    pub fn apply_to_direction(self, mut dir: Direction) -> Direction {
        if self.mirrored && dir.is_vertical() {
            dir = dir.opposite();
        }
        dir.rotated_ccw_by(-self.rotated_cw)
    }

    pub fn apply_to_vertical(self, vertical: bool) -> bool {
        vertical ^ (self.rotated_cw % 2 != 0)
    }

    pub fn apply_to_mirrored(self, mirrored: bool) -> bool {
        mirrored ^ self.mirrored
    }

    pub fn degrees(self) -> i32 { self.rotated_cw * 90 }

    pub fn is_mirrored(self) -> bool { self.mirrored }

    #[cfg(test)]
    pub fn all() -> Vec<Transform> {
        vec![
            Transform::new(0, false),
            Transform::new(0, true),
            Transform::new(1, false),
            Transform::new(1, true),
            Transform::new(2, false),
            Transform::new(2, true),
            Transform::new(3, false),
            Transform::new(3, true),
        ]
    }
}

impl Tomlable for Transform {
    fn to_toml(&self) -> toml::Value {
        debug_assert!(self.rotated_cw >= 0);
        debug_assert!(self.rotated_cw < 4);
        let mirrored = if self.mirrored { 'T' } else { 'F' };
        toml::Value::String(format!("{}{}", self.rotated_cw, mirrored))
    }

    fn from_toml(value: toml::Value) -> Transform {
        if let Some(string) = value.as_str() {
            match string {
                "0F" => return Transform::new(0, false),
                "0T" => return Transform::new(0, true),
                "1F" => return Transform::new(1, false),
                "1T" => return Transform::new(1, true),
                "2F" => return Transform::new(2, false),
                "2T" => return Transform::new(2, true),
                "3F" => return Transform::new(3, false),
                "3T" => return Transform::new(3, true),
                _ => {}
            }
        }
        Transform::identity()
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use save::Direction;
    use save::util::Tomlable;
    use super::Transform;

    #[test]
    fn toml_round_trip() {
        for original in Transform::all() {
            let result = Transform::from_toml(original.to_toml());
            assert_eq!(result, original);
        }
    }

    #[test]
    fn apply_identity() {
        assert_eq!(Transform::identity().degrees(), 0);
        assert_eq!(Transform::identity().is_mirrored(), false);
        assert_eq!(Transform::identity().inverse(), Transform::identity());
        for dir in Direction::all() {
            assert_eq!(Transform::identity().apply_to_direction(dir), dir);
        }
    }

    #[test]
    fn compose_with_identity() {
        for transform in Transform::all() {
            assert_eq!(transform.compose(Transform::identity()), transform);
            assert_eq!(Transform::identity().compose(transform), transform);
        }
    }

    #[test]
    fn compose_with_inverse() {
        for transform in Transform::all() {
            let inverse = transform.inverse();
            assert_eq!(transform.compose(inverse),
                       Transform::identity(),
                       "{:?} compose {:?}",
                       transform,
                       inverse);
            assert_eq!(inverse.compose(transform),
                       Transform::identity(),
                       "{:?} compose {:?}",
                       inverse,
                       transform);
        }
    }

    #[test]
    fn flipped_twice() {
        for transform in Transform::all() {
            assert_eq!(transform.flipped_vert().flipped_vert(), transform);
            assert_eq!(transform.flipped_horz().flipped_horz(), transform);
            assert_eq!(transform
                           .flipped_vert()
                           .rotated_cw()
                           .rotated_cw()
                           .flipped_horz(),
                       transform);
        }
    }
}

// ========================================================================= //
