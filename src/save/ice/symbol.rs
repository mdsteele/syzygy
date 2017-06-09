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

use save::Direction;
use save::ice::Transform;
use save::util::{pop_value, to_bool, to_string, to_table};

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Symbol {
    RedTriangle(Direction),
    GreenSquare,
    BlueCircle,
    YellowRhombus(bool, bool),
    PurpleCheckmark(Transform),
    CyanQ(Transform),
    CyanU(Transform),
    CyanA(Transform),
}

impl Symbol {
    pub fn from_toml(value: toml::Value) -> Symbol {
        let mut table = to_table(value);
        match to_string(pop_value(&mut table, "shape")).as_str() {
            "RT" => {
                let direction = Direction::from_toml(table.get("direction"));
                Symbol::RedTriangle(direction)
            }
            "GS" => Symbol::GreenSquare,
            "BC" => Symbol::BlueCircle,
            "YR" => {
                let vertical = to_bool(pop_value(&mut table, "vertical"));
                let mirrored = to_bool(pop_value(&mut table, "mirrored"));
                Symbol::YellowRhombus(vertical, mirrored)
            }
            "PC" => {
                let transform = Transform::from_toml(table.get("transform"));
                Symbol::PurpleCheckmark(transform)
            }
            "CQ" => {
                let transform = Transform::from_toml(table.get("transform"));
                Symbol::CyanQ(transform)
            }
            "CU" => {
                let transform = Transform::from_toml(table.get("transform"));
                Symbol::CyanU(transform)
            }
            "CA" => {
                let transform = Transform::from_toml(table.get("transform"));
                Symbol::CyanA(transform)
            }
            _ => Symbol::BlueCircle,
        }
    }

    pub fn to_toml(self) -> toml::Value {
        let mut table = toml::value::Table::new();
        let shape = match self {
            Symbol::RedTriangle(direction) => {
                table.insert("direction".to_string(), direction.to_toml());
                "RT"
            }
            Symbol::GreenSquare => "GS",
            Symbol::BlueCircle => "BC",
            Symbol::YellowRhombus(vertical, mirrored) => {
                table.insert("vertical".to_string(),
                             toml::Value::Boolean(vertical));
                table.insert("mirrored".to_string(),
                             toml::Value::Boolean(mirrored));
                "YR"
            }
            Symbol::PurpleCheckmark(transform) => {
                table.insert("transform".to_string(), transform.to_toml());
                "PC"
            }
            Symbol::CyanQ(transform) => {
                table.insert("transform".to_string(), transform.to_toml());
                "CQ"
            }
            Symbol::CyanU(transform) => {
                table.insert("transform".to_string(), transform.to_toml());
                "CU"
            }
            Symbol::CyanA(transform) => {
                table.insert("transform".to_string(), transform.to_toml());
                "CA"
            }
        };
        table.insert("shape".to_string(),
                     toml::Value::String(shape.to_string()));
        toml::Value::Table(table)
    }

    pub fn transformed(self, transform: Transform) -> Symbol {
        match self {
            Symbol::RedTriangle(dir) => {
                Symbol::RedTriangle(transform.apply_to_direction(dir))
            }
            Symbol::GreenSquare => Symbol::GreenSquare,
            Symbol::BlueCircle => Symbol::BlueCircle,
            Symbol::YellowRhombus(vertical, mirrored) => {
                Symbol::YellowRhombus(transform.apply_to_vertical(vertical),
                                      transform.apply_to_mirrored(mirrored))
            }
            Symbol::PurpleCheckmark(trans) => {
                Symbol::PurpleCheckmark(trans.compose(transform))
            }
            Symbol::CyanQ(trans) => Symbol::CyanQ(trans.compose(transform)),
            Symbol::CyanU(trans) => Symbol::CyanU(trans.compose(transform)),
            Symbol::CyanA(trans) => Symbol::CyanA(trans.compose(transform)),
        }
    }

    pub fn sprite_index(self) -> usize {
        match self {
            Symbol::RedTriangle(_) => 0,
            Symbol::GreenSquare => 1,
            Symbol::BlueCircle => 2,
            Symbol::YellowRhombus(_, _) => 3,
            Symbol::PurpleCheckmark(_) => 4,
            Symbol::CyanQ(_) => 5,
            Symbol::CyanU(_) => 6,
            Symbol::CyanA(_) => 7,
        }
    }

    pub fn sprite_degrees(self) -> i32 {
        match self {
            Symbol::RedTriangle(dir) => dir.degrees(),
            Symbol::GreenSquare => 0,
            Symbol::BlueCircle => 0,
            Symbol::YellowRhombus(false, _) => 0,
            Symbol::YellowRhombus(true, _) => 90,
            Symbol::PurpleCheckmark(transform) |
            Symbol::CyanQ(transform) |
            Symbol::CyanU(transform) |
            Symbol::CyanA(transform) => transform.degrees(),
        }
    }

    pub fn sprite_mirrored(self) -> bool {
        match self {
            Symbol::RedTriangle(_) => false,
            Symbol::GreenSquare => false,
            Symbol::BlueCircle => false,
            Symbol::YellowRhombus(_, mirrored) => mirrored,
            Symbol::PurpleCheckmark(transform) |
            Symbol::CyanQ(transform) |
            Symbol::CyanU(transform) |
            Symbol::CyanA(transform) => transform.is_mirrored(),
        }
    }

    #[cfg(test)]
    pub fn all() -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for dir in Direction::all() {
            symbols.push(Symbol::RedTriangle(dir));
        }
        symbols.push(Symbol::GreenSquare);
        symbols.push(Symbol::BlueCircle);
        for &vertical in &[false, true] {
            for &mirrored in &[false, true] {
                symbols.push(Symbol::YellowRhombus(vertical, mirrored));
            }
        }
        for trans in Transform::all() {
            symbols.push(Symbol::PurpleCheckmark(trans));
            symbols.push(Symbol::CyanQ(trans));
            symbols.push(Symbol::CyanU(trans));
            symbols.push(Symbol::CyanA(trans));
        }
        symbols
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use save::Direction;
    use save::ice::Transform;
    use super::Symbol;

    #[test]
    fn symbol_toml_round_trip() {
        for original in Symbol::all() {
            let result = Symbol::from_toml(original.to_toml());
            assert_eq!(result, original);
        }
    }

    #[test]
    fn symbol_rotated() {
        let trans = Transform::identity().rotated_cw();
        assert_eq!(Symbol::RedTriangle(Direction::North).transformed(trans),
                   Symbol::RedTriangle(Direction::East));
        assert_eq!(Symbol::GreenSquare.transformed(trans),
                   Symbol::GreenSquare);
        assert_eq!(Symbol::BlueCircle.transformed(trans), Symbol::BlueCircle);
        assert_eq!(Symbol::YellowRhombus(true, true).transformed(trans),
                   Symbol::YellowRhombus(false, true));
    }

    #[test]
    fn symbol_flipped() {
        let trans = Transform::identity().flipped_vert();
        assert_eq!(Symbol::RedTriangle(Direction::North).transformed(trans),
                   Symbol::RedTriangle(Direction::South));
        assert_eq!(Symbol::GreenSquare.transformed(trans),
                   Symbol::GreenSquare);
        assert_eq!(Symbol::BlueCircle.transformed(trans), Symbol::BlueCircle);
        assert_eq!(Symbol::YellowRhombus(true, false).transformed(trans),
                   Symbol::YellowRhombus(true, true));
    }
}

// ========================================================================= //
