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

use std::{i32, u32};
use toml;

// ========================================================================= //

pub const ACCESS_KEY: &'static str = "access";

pub fn pop_array(table: &mut toml::Table, key: &str) -> toml::Array {
    if let Some(value) = table.remove(key) {
        to_array(value)
    } else {
        toml::Array::new()
    }
}

pub fn pop_table(table: &mut toml::Table, key: &str) -> toml::Table {
    if let Some(value) = table.remove(key) {
        to_table(value)
    } else {
        toml::Table::new()
    }
}

pub fn to_array(value: toml::Value) -> toml::Array {
    match value {
        toml::Value::Array(array) => array,
        _ => toml::Array::new(),
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

pub fn to_string(value: toml::Value) -> String {
    match value {
        toml::Value::String(string) => string,
        _ => String::new(),
    }
}

pub fn to_table(value: toml::Value) -> toml::Table {
    match value {
        toml::Value::Table(table) => table,
        _ => toml::Table::new(),
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
