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

use rand;
use std::collections::HashSet;
use toml;

use save::util::to_i8;

// ========================================================================= //

#[derive(Clone)]
pub struct Shape(pub [i8; 9]);

impl Shape {
    pub fn symbol(&self) -> Option<i8> {
        self.tiles().next().map(|(_, symbol)| symbol)
    }

    pub fn tiles(&self) -> TilesIter {
        let &Shape(ref values) = self;
        TilesIter {
            values: values,
            width: 3,
            index: 0,
        }
    }
}

// ========================================================================= //

pub struct Grid {
    width: usize,
    height: usize,
    values: Vec<i8>,
}

impl Grid {
    pub fn from_toml(width: usize, height: usize, array: toml::value::Array)
                     -> Grid {
        let mut values: Vec<i8> = array.into_iter().map(to_i8).collect();
        values.resize(width * height, 0);
        Grid {
            width: width,
            height: height,
            values: values,
        }
    }

    pub fn to_toml(&self) -> toml::Value {
        toml::Value::Array(self.values
                               .iter()
                               .map(|&val| toml::Value::Integer(val as i64))
                               .collect())
    }

    pub fn num_cols(&self) -> i32 { self.width as i32 }

    pub fn num_rows(&self) -> i32 { self.height as i32 }

    pub fn tiles(&self) -> TilesIter {
        TilesIter {
            values: &self.values,
            width: self.width,
            index: 0,
        }
    }

    pub fn num_distinct_symbols(&self) -> usize {
        let mut symbols = HashSet::new();
        for (_, symbol) in self.tiles() {
            symbols.insert(symbol);
        }
        symbols.len()
    }

    pub fn symbol_at(&self, col: i32, row: i32) -> Option<i8> {
        if (col >= 0 && col < self.num_cols()) &&
           (row >= 0 && row < self.num_rows()) {
            let value = self.values[row as usize * self.width + col as usize];
            if value == 0 { None } else { Some(value.abs()) }
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        for value in self.values.iter_mut() {
            *value = 0;
        }
    }

    pub fn try_place_shape(&mut self, shape: &Shape, col: i32, row: i32)
                           -> bool {
        let mut symbols: Vec<(i8, usize)> = Vec::new();
        for ((shape_col, shape_row), symbol) in shape.tiles() {
            let col = col + shape_col;
            let row = row + shape_row;
            if (col < 0 || col >= self.num_cols()) ||
               (row < 0 || row >= self.num_rows()) {
                return false;
            }
            let index = row as usize * self.width + col as usize;
            if self.values[index] != 0 {
                return false;
            }
            symbols.push((symbol, index));
        }
        for &(symbol, index) in &symbols {
            self.values[index] = symbol;
        }
        true
    }

    pub fn can_remove_symbol(&self, symbol: i8) -> bool {
        debug_assert!(symbol > 0);
        for &value in self.values.iter() {
            if value == symbol {
                return false;
            }
        }
        true
    }

    pub fn remove_symbol(&mut self, symbol: i8) {
        debug_assert!(symbol > 0);
        for value in self.values.iter_mut() {
            if value.abs() == symbol {
                *value = 0;
            }
        }
    }

    pub fn decay_symbol(&mut self, symbol: i8, num: usize) {
        let mut indices: Vec<usize> = Vec::new();
        for (index, &value) in self.values.iter().enumerate() {
            if value == symbol {
                indices.push(index);
            }
        }
        let sample = rand::sample(&mut rand::thread_rng(), indices, num);
        for index in sample {
            self.values[index] = -self.values[index];
        }
    }
}

// ========================================================================= //

pub struct TilesIter<'a> {
    values: &'a [i8],
    width: usize,
    index: usize,
}

impl<'a> Iterator for TilesIter<'a> {
    type Item = ((i32, i32), i8);

    fn next(&mut self) -> Option<((i32, i32), i8)> {
        while self.index < self.values.len() {
            let value = self.values[self.index];
            if value != 0 {
                let col = (self.index % self.width) as i32;
                let row = (self.index / self.width) as i32;
                self.index += 1;
                return Some(((col, row), value));
            }
            self.index += 1;
        }
        None
    }
}

// ========================================================================= //
