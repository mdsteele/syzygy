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

use std::{i32, i8, u32};
use std::collections::VecDeque;
use toml;

// ========================================================================= //

pub const ACCESS_KEY: &str = "access";

pub fn pop_array(table: &mut toml::value::Table, key: &str)
                 -> toml::value::Array {
    if let Some(value) = table.remove(key) {
        to_array(value)
    } else {
        toml::value::Array::new()
    }
}

pub fn pop_i32(table: &mut toml::value::Table, key: &str) -> i32 {
    if let Some(value) = table.remove(key) {
        to_i32(value)
    } else {
        0
    }
}

pub fn pop_table(table: &mut toml::value::Table, key: &str)
                 -> toml::value::Table {
    if let Some(value) = table.remove(key) {
        to_table(value)
    } else {
        toml::value::Table::new()
    }
}

/// Removes and returns a value from a table.  If the key isn't in the table,
/// returns a value of false (the closest thing `toml` has to null).
pub fn pop_value(table: &mut toml::value::Table, key: &str) -> toml::Value {
    if let Some(value) = table.remove(key) {
        value
    } else {
        toml::Value::Boolean(false)
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

pub fn to_bool(value: toml::Value) -> bool {
    match value {
        toml::Value::Boolean(boolean) => boolean,
        _ => false,
    }
}

pub fn to_i32(value: toml::Value) -> i32 {
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

pub fn to_i8(value: toml::Value) -> i8 {
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

/// Coerces a `toml` value to a string.  If the value is not already a string,
/// returns the empty string.
pub fn to_string(value: toml::Value) -> String {
    match value {
        toml::Value::String(string) => string,
        _ => String::new(),
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

pub fn to_u32(value: toml::Value) -> u32 {
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

    use super::rotate_deque;

    #[test]
    fn deque_rotation() {
        let mut deque: VecDeque<i32> = [1, 2, 3, 4, 5]
            .iter()
            .cloned()
            .collect();
        rotate_deque(&mut deque, 0);
        assert_eq!(deque.iter().cloned().collect::<Vec<i32>>(),
                   vec![1, 2, 3, 4, 5]);
        rotate_deque(&mut deque, 1);
        assert_eq!(deque.iter().cloned().collect::<Vec<i32>>(),
                   vec![5, 1, 2, 3, 4]);
        rotate_deque(&mut deque, -2);
        assert_eq!(deque.iter().cloned().collect::<Vec<i32>>(),
                   vec![2, 3, 4, 5, 1]);
        rotate_deque(&mut deque, 8);
        assert_eq!(deque.iter().cloned().collect::<Vec<i32>>(),
                   vec![4, 5, 1, 2, 3]);
        rotate_deque(&mut deque, -14);
        assert_eq!(deque.iter().cloned().collect::<Vec<i32>>(),
                   vec![3, 4, 5, 1, 2]);
        rotate_deque(&mut deque, 15);
        assert_eq!(deque.iter().cloned().collect::<Vec<i32>>(),
                   vec![3, 4, 5, 1, 2]);
    }
}

// ========================================================================= //
