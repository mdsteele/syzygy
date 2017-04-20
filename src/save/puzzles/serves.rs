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

use std::cmp::min;
use toml;

use save::{Access, Location};
use save::memory::{Grid, Shape};
use save::util::{ACCESS_KEY, pop_array, to_u32};
use super::PuzzleState;

// ========================================================================= //

const GRID_KEY: &'static str = "grid";
const NUM_PLACED_KEY: &'static str = "placed";

const NUM_COLS: usize = 7;
const NUM_ROWS: usize = 4;
const NUM_SYMBOLS: i32 = 6;

#[cfg_attr(rustfmt, rustfmt_skip)]
const SHAPES: &'static [(Shape, &'static [(i8, usize)])] = &[
    (Shape([0, 2, 0, 2, 2, 0, 0, 2, 0]), &[]),
    (Shape([0, 6, 6, 0, 6, 0, 0, 6, 6]), &[(2, 1)]),
    (Shape([0, 5, 0, 5, 5, 0, 0, 5, 5]), &[(2, 1), (6, 2)]),
    (Shape([0, 1, 1, 1, 1, 0, 1, 0, 0]), &[(2, 1), (5, 1), (6, 1)]),
    (Shape([3, 0, 0, 3, 0, 0, 3, 3, 3]), &[(1, 1), (2, 1), (5, 1), (6, 1)]),
    (Shape([4, 4, 0, 0, 4, 0, 4, 4, 0]), &[(1, 2), (3, 1), (5, 1)]),
    (Shape([2, 2, 0, 0, 2, 0, 0, 2, 0]), &[(3, 2), (4, 1), (6, 1)]),
    (Shape([1, 1, 1, 0, 1, 0, 0, 1, 0]), &[(2, 1), (3, 1), (5, 1)]),
    (Shape([0, 6, 6, 0, 6, 6, 0, 6, 0]), &[(1, 1), (4, 1)]),
    (Shape([0, 5, 5, 0, 5, 0, 5, 5, 0]), &[(2, 1), (6, 2)]),
    (Shape([0, 3, 0, 3, 3, 3, 0, 3, 0]), &[(1, 1), (5, 3)]),
    (Shape([4, 4, 0, 4, 4, 4, 0, 4, 4]), &[(3, 3), (5, 2)]),
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const REMOVALS: &'static [&'static [(i8, usize)]] = &[
    &[(1, 2), (5, 1)],
    &[(3, 1), (4, 1)],
    &[(2, 1), (4, 1)],
    &[(2, 1), (4, 1)],
    &[(1, 1)],
    &[(6, 3)],
    &[(1, 1)],
    &[(1, 1)],
    &[(3, 1)],
    &[(3, 1), (4, 4)],
    &[(4, 3)],
    &[],
];

// ========================================================================= //

pub struct ServesState {
    access: Access,
    grid: Grid,
    num_placed: usize,
    num_removed: usize,
}

impl ServesState {
    pub fn from_toml(mut table: toml::value::Table) -> ServesState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let (grid, num_placed, num_removed) = if access.is_solved() {
            (Grid::new(NUM_COLS, NUM_ROWS), SHAPES.len(), REMOVALS.len())
        } else {
            let num_placed =
                min(table.remove(NUM_PLACED_KEY).map(to_u32).unwrap_or(0) as
                    usize,
                    SHAPES.len());
            let grid = Grid::from_toml(NUM_COLS,
                                       NUM_ROWS,
                                       pop_array(&mut table, GRID_KEY));
            let distinct = grid.num_distinct_symbols();
            if distinct <= num_placed {
                (grid, num_placed, num_placed - distinct)
            } else {
                (Grid::new(NUM_COLS, NUM_ROWS), 0, 0)
            }
        };
        ServesState {
            access: access,
            grid: grid,
            num_placed: num_placed,
            num_removed: num_removed,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid.clear();
        self.num_placed = SHAPES.len();
        self.num_removed = REMOVALS.len();
    }

    pub fn total_num_steps(&self) -> usize { SHAPES.len() + REMOVALS.len() }

    pub fn current_step(&self) -> usize { self.num_placed + self.num_removed }

    pub fn grid(&self) -> &Grid { &self.grid }

    pub fn grid_mut(&mut self) -> &mut Grid { &mut self.grid }

    pub fn next_shape(&self) -> Option<Shape> {
        if self.num_placed < SHAPES.len() {
            Some(SHAPES[self.num_placed].0.clone())
        } else {
            None
        }
    }

    pub fn try_place_shape(&mut self, col: i32, row: i32) -> Option<i8> {
        if let Some(shape) = self.next_shape() {
            if self.grid.try_place_shape(&shape, col, row) {
                for &(symbol, num) in SHAPES[self.num_placed].1 {
                    self.grid.decay_symbol(symbol, num);
                }
                self.num_placed += 1;
                return shape.symbol();
            }
        }
        None
    }

    pub fn can_remove_symbol(&self, symbol: i8) -> bool {
        assert!(symbol > 0 && symbol as i32 <= NUM_SYMBOLS);
        self.grid.can_remove_symbol(symbol)
    }

    pub fn remove_symbol(&mut self, symbol: i8) {
        assert!(symbol > 0 && symbol as i32 <= NUM_SYMBOLS);
        if self.grid.can_remove_symbol(symbol) {
            self.grid.remove_symbol(symbol);
            for &(symbol, num) in REMOVALS[self.num_removed] {
                self.grid.decay_symbol(symbol, num);
            }
            self.num_removed += 1;
            if self.num_removed == REMOVALS.len() {
                self.access = Access::Solved;
            }
        } else {
            self.reset();
        }
    }
}

impl PuzzleState for ServesState {
    fn location(&self) -> Location { Location::IfMemoryServes }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.num_placed > 0 }

    fn reset(&mut self) {
        self.grid.clear();
        self.num_placed = 0;
        self.num_removed = 0;
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.access.is_solved() {
            table.insert(NUM_PLACED_KEY.to_string(),
                         toml::Value::Integer(self.num_placed as i64));
            table.insert(GRID_KEY.to_string(), self.grid.to_toml());
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};
    use std::iter::FromIterator;

    use super::{NUM_SYMBOLS, REMOVALS, SHAPES};

    #[test]
    fn steps_are_well_formed() {
        assert_eq!(SHAPES.len(), REMOVALS.len());
        let mut num_symbols_in_use = HashMap::new();
        let mut num_removed = 0;
        for &(ref shape, decay) in SHAPES.iter() {
            let add_symbol = shape.symbol().unwrap();
            assert!((add_symbol as i32) <= NUM_SYMBOLS);
            assert_eq!(num_symbols_in_use.get(&add_symbol)
                                         .cloned()
                                         .unwrap_or(0),
                       0);
            for &(symbol, _) in decay {
                assert!(symbol != add_symbol,
                        "Can't decay {} while adding it.",
                        symbol);
            }
            num_symbols_in_use.insert(add_symbol, shape.tiles().count());
            let mut decay_queue = VecDeque::from_iter(decay);
            while let Some(&(symbol, num)) = decay_queue.pop_front() {
                assert!((symbol as i32) <= NUM_SYMBOLS);
                let old_count =
                    num_symbols_in_use.get(&symbol).cloned().unwrap_or(0);
                assert!(old_count >= num,
                        "Can't decay {} by {} (only {} are in use).",
                        symbol,
                        num,
                        old_count);
                let new_count = old_count - num;
                num_symbols_in_use.insert(symbol, new_count);
                if new_count == 0 {
                    decay_queue.extend(REMOVALS[num_removed]);
                    num_removed += 1;
                }
            }
        }
        for (symbol, count) in num_symbols_in_use.into_iter() {
            assert_eq!(count, 0, "Symbol {} still in use at the end.", symbol);
        }
        assert_eq!(num_removed, REMOVALS.len());
    }
}

// ========================================================================= //
