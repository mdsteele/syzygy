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
use std::collections::{HashMap, HashSet};
use toml;

use save::Direction;
use save::util::Tomlable;

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
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width: width,
            height: height,
            values: vec![0; width * height],
        }
    }

    pub fn from_toml(width: usize, height: usize, array: toml::value::Array)
                     -> Grid {
        let mut values = Vec::<i8>::from_toml(toml::Value::Array(array));
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

    pub fn shift_tiles(&mut self, direction: Direction)
                       -> HashMap<(i32, i32), (i32, i32)> {
        let mut shifts = HashMap::new();
        if direction.is_vertical() {
            let mut col_values = Vec::with_capacity(self.width);
            for col in 0..self.width {
                let mut values = Vec::new();
                for row in 0..self.height {
                    let value = &mut self.values[row * self.width + col];
                    if *value != 0 {
                        values.push((*value, row));
                        *value = 0;
                    }
                }
                col_values.push(values);
            }
            for col in 0..self.width {
                let values = &col_values[col];
                debug_assert!(values.len() <= self.height);
                let mut row = if direction == Direction::North {
                    0
                } else {
                    debug_assert_eq!(direction, Direction::South);
                    self.height - values.len()
                };
                for &(value, old_row) in values.iter() {
                    self.values[row * self.width + col] = value;
                    if row != old_row {
                        shifts.insert((col as i32, row as i32),
                                      (col as i32, old_row as i32));
                    }
                    row += 1;
                }
            }
        } else {
            let mut row_values = Vec::with_capacity(self.height);
            for row in 0..self.height {
                let mut values = Vec::new();
                for col in 0..self.width {
                    let value = &mut self.values[row * self.width + col];
                    if *value != 0 {
                        values.push((*value, col));
                        *value = 0;
                    }
                }
                row_values.push(values);
            }
            for row in 0..self.height {
                let values = &row_values[row];
                debug_assert!(values.len() <= self.width);
                let mut col = if direction == Direction::West {
                    0
                } else {
                    debug_assert_eq!(direction, Direction::East);
                    self.width - values.len()
                };
                for &(value, old_col) in values.iter() {
                    self.values[row * self.width + col] = value;
                    if col != old_col {
                        shifts.insert((col as i32, row as i32),
                                      (old_col as i32, row as i32));
                    }
                    col += 1;
                }
            }
        }
        shifts
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
