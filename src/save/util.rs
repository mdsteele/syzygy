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

use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::{i32, i8, u32, u8, usize};
use toml;

use crate::gui::Point;

// ========================================================================= //

pub const ACCESS_KEY: &str = "access";

pub fn pop_array(
    table: &mut toml::value::Table,
    key: &str,
) -> toml::value::Array {
    if let Some(value) = table.remove(key) {
        to_array(value)
    } else {
        toml::value::Array::new()
    }
}

pub fn pop_table(
    table: &mut toml::value::Table,
    key: &str,
) -> toml::value::Table {
    if let Some(value) = table.remove(key) {
        to_table(value)
    } else {
        toml::value::Table::new()
    }
}

/// Coerces a `toml` value to an array.  If the value is not already an array,
/// returns the empty array.
pub fn to_array(value: toml::Value) -> toml::value::Array {
    match value {
        toml::Value::Array(array) => array,
        _ => toml::value::Array::new(),
    }
}

/// Coerces a `toml` value to a table.  If the value is not already a table,
/// returns the empty table.
pub fn to_table(value: toml::Value) -> toml::value::Table {
    match value {
        toml::Value::Table(table) => table,
        _ => toml::value::Table::new(),
    }
}

// ========================================================================= //

pub trait Tomlable {
    fn to_toml(&self) -> toml::Value;

    fn from_toml(value: toml::Value) -> Self
    where
        Self: Sized;

    fn pop_from_table(table: &mut toml::value::Table, key: &str) -> Self
    where
        Self: Sized,
    {
        let value = table.remove(key).unwrap_or(toml::Value::Boolean(false));
        Self::from_toml(value)
    }
}

impl Tomlable for bool {
    fn to_toml(&self) -> toml::Value {
        toml::Value::Boolean(*self)
    }

    fn from_toml(value: toml::Value) -> bool {
        match value {
            toml::Value::Boolean(boolean) => boolean,
            _ => false,
        }
    }
}

impl Tomlable for i8 {
    fn to_toml(&self) -> toml::Value {
        toml::Value::Integer(*self as i64)
    }

    fn from_toml(value: toml::Value) -> i8 {
        match value {
            toml::Value::Integer(integer) => {
                if integer > (i8::MAX as i64) {
                    i8::MAX
                } else if integer < (i8::MIN as i64) {
                    i8::MIN
                } else {
                    integer as i8
                }
            }
            _ => 0,
        }
    }
}

impl Tomlable for i32 {
    fn to_toml(&self) -> toml::Value {
        toml::Value::Integer(*self as i64)
    }

    fn from_toml(value: toml::Value) -> i32 {
        match value {
            toml::Value::Integer(integer) => {
                if integer > (i32::MAX as i64) {
                    i32::MAX
                } else if integer < (i32::MIN as i64) {
                    i32::MIN
                } else {
                    integer as i32
                }
            }
            _ => 0,
        }
    }
}

impl Tomlable for u8 {
    fn to_toml(&self) -> toml::Value {
        toml::Value::Integer(*self as i64)
    }

    fn from_toml(value: toml::Value) -> u8 {
        match value {
            toml::Value::Integer(integer) => {
                if integer > (u8::MAX as i64) {
                    u8::MAX
                } else if integer < (u8::MIN as i64) {
                    u8::MIN
                } else {
                    integer as u8
                }
            }
            _ => 0,
        }
    }
}

impl Tomlable for u32 {
    fn to_toml(&self) -> toml::Value {
        toml::Value::Integer(*self as i64)
    }

    fn from_toml(value: toml::Value) -> u32 {
        match value {
            toml::Value::Integer(integer) => {
                if integer > (u32::MAX as i64) {
                    u32::MAX
                } else if integer < (u32::MIN as i64) {
                    u32::MIN
                } else {
                    integer as u32
                }
            }
            _ => 0,
        }
    }
}

impl<T: Hash + Ord + Tomlable> Tomlable for HashSet<T> {
    fn to_toml(&self) -> toml::Value {
        let mut vector: Vec<&T> = self.iter().collect();
        vector.sort();
        toml::Value::Array(vector.into_iter().map(Tomlable::to_toml).collect())
    }

    fn from_toml(value: toml::Value) -> HashSet<T> {
        to_array(value).into_iter().map(Tomlable::from_toml).collect()
    }
}

impl Tomlable for Point {
    fn to_toml(&self) -> toml::Value {
        vec![self.x(), self.y()].to_toml()
    }

    fn from_toml(value: toml::Value) -> Point {
        let array = Vec::<i32>::from_toml(value);
        if array.len() != 2 {
            return Point::new(0, 0);
        }
        Point::new(array[0], array[1])
    }
}

impl Tomlable for String {
    fn to_toml(&self) -> toml::Value {
        toml::Value::String(self.clone())
    }

    fn from_toml(value: toml::Value) -> String {
        match value {
            toml::Value::String(string) => string,
            _ => String::new(),
        }
    }
}

impl<T: Tomlable> Tomlable for Vec<T> {
    fn to_toml(&self) -> toml::Value {
        toml::Value::Array(self.iter().map(Tomlable::to_toml).collect())
    }

    fn from_toml(value: toml::Value) -> Vec<T> {
        to_array(value).into_iter().map(Tomlable::from_toml).collect()
    }
}

// ========================================================================= //

pub fn rotate_deque<T>(deque: &mut VecDeque<T>, by: i32) {
    let len = deque.len();
    if by > 0 {
        let by = (by as usize) % len;
        if by > 0 {
            let mut rest = deque.split_off(len - by);
            rest.append(deque);
            *deque = rest;
        }
    } else if by < 0 {
        let by = ((-by) as usize) % len;
        if by > 0 {
            let mut rest = deque.split_off(by);
            rest.append(deque);
            *deque = rest;
        }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use toml;

    use super::{rotate_deque, Tomlable};

    #[test]
    fn deque_rotation() {
        let mut deque: VecDeque<i32> =
            [1, 2, 3, 4, 5].iter().cloned().collect();
        rotate_deque(&mut deque, 0);
        assert_eq!(
            deque.iter().cloned().collect::<Vec<i32>>(),
            vec![1, 2, 3, 4, 5]
        );
        rotate_deque(&mut deque, 1);
        assert_eq!(
            deque.iter().cloned().collect::<Vec<i32>>(),
            vec![5, 1, 2, 3, 4]
        );
        rotate_deque(&mut deque, -2);
        assert_eq!(
            deque.iter().cloned().collect::<Vec<i32>>(),
            vec![2, 3, 4, 5, 1]
        );
        rotate_deque(&mut deque, 8);
        assert_eq!(
            deque.iter().cloned().collect::<Vec<i32>>(),
            vec![4, 5, 1, 2, 3]
        );
        rotate_deque(&mut deque, -14);
        assert_eq!(
            deque.iter().cloned().collect::<Vec<i32>>(),
            vec![3, 4, 5, 1, 2]
        );
        rotate_deque(&mut deque, 15);
        assert_eq!(
            deque.iter().cloned().collect::<Vec<i32>>(),
            vec![3, 4, 5, 1, 2]
        );
    }

    #[test]
    fn i8_to_toml() {
        assert_eq!(i8::from_toml(toml::Value::Boolean(false)), 0);
        assert_eq!(i8::from_toml(toml::Value::Boolean(true)), 0);
        assert_eq!(i8::from_toml(toml::Value::Integer(1)), 1);
        assert_eq!(i8::from_toml(toml::Value::Integer(127)), 127);
        assert_eq!(i8::from_toml(toml::Value::Integer(128)), 127);
        assert_eq!(i8::from_toml(toml::Value::Integer(-128)), -128);
        assert_eq!(i8::from_toml(toml::Value::Integer(-129)), -128);
    }

    #[test]
    fn i32_to_toml() {
        assert_eq!(i32::from_toml(toml::Value::Boolean(false)), 0);
        assert_eq!(i32::from_toml(toml::Value::Boolean(true)), 0);
        assert_eq!(i32::from_toml(toml::Value::Integer(-17)), -17);
        assert_eq!(
            i32::from_toml(toml::Value::Integer(2147483647)),
            2147483647
        );
        assert_eq!(
            i32::from_toml(toml::Value::Integer(2147483648)),
            2147483647
        );
        assert_eq!(
            i32::from_toml(toml::Value::Integer(-2147483648)),
            -2147483648
        );
        assert_eq!(
            i32::from_toml(toml::Value::Integer(-2147483649)),
            -2147483648
        );
    }

    #[test]
    fn u32_to_toml() {
        assert_eq!(u32::from_toml(toml::Value::Boolean(false)), 0);
        assert_eq!(u32::from_toml(toml::Value::Boolean(true)), 0);
        assert_eq!(u32::from_toml(toml::Value::Integer(-1)), 0);
        assert_eq!(u32::from_toml(toml::Value::Integer(1)), 1);
        assert_eq!(
            u32::from_toml(toml::Value::Integer(2147483648)),
            2147483648
        );
        assert_eq!(
            u32::from_toml(toml::Value::Integer(4294967295)),
            4294967295
        );
        assert_eq!(
            u32::from_toml(toml::Value::Integer(4294967296)),
            4294967295
        );
        assert_eq!(u32::from_toml(toml::Value::Integer(-2147483648)), 0);
    }
}

// ========================================================================= //
