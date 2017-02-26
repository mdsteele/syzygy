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
use std::cmp::min;
use toml;

use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array, to_i32, to_u32};

// ========================================================================= //

const GRID_KEY: &'static str = "grid";
const STAGE_KEY: &'static str = "stage";

const NUM_COLS: i32 = 6;
const NUM_ROWS: i32 = 4;
const GRID_SIZE: usize = (NUM_COLS * NUM_ROWS) as usize;
const NUM_SYMBOLS: i32 = 6;

#[cfg_attr(rustfmt, rustfmt_skip)]
const STAGES: &'static [(i8, [i8; 9], &'static [(i8, usize)])] = &[
    (0, [0, 1, 0, 0, 1, 1, 0, 1, 0], &[]),
    (0, [0, 0, 0, 2, 2, 2, 0, 0, 2], &[(1, 1)]),
    (0, [0, 0, 0, 3, 3, 0, 3, 3, 0], &[(1, 2), (2, 1)]),
    (0, [0, 0, 0, 5, 5, 5, 5, 0, 0], &[(1, 1), (2, 1), (3, 2)]),
    (1, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(5, 1)]),
    (0, [0, 6, 6, 0, 6, 0, 0, 6, 0], &[(2, 1), (3, 2), (5, 1)]),
    (3, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(5, 1), (6, 2)]),
    (0, [0, 4, 0, 4, 4, 0, 4, 0, 0], &[(2, 1), (6, 1)]),
    (2, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(4, 2), (5, 1)]),
    (5, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(4, 1), (6, 1)]),
    (6, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[]),
    (0, [0, 3, 0, 3, 3, 3, 0, 0, 0], &[]),
    (0, [0, 0, 0, 0, 2, 2, 2, 2, 0], &[(3, 2)]),
    (0, [0, 1, 1, 0, 1, 0, 0, 1, 0], &[(3, 1), (2, 1)]),
    (0, [5, 5, 0, 0, 5, 0, 0, 5, 0], &[(1, 1), (2, 2), (4, 1)]),
    (4, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(2, 1), (5, 3)]),
    (2, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(1, 1)]),
    (0, [0, 0, 0, 6, 6, 6, 6, 0, 0], &[(1, 1), (3, 1)]),
    (3, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(6, 1), (5, 1)]),
    (5, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(1, 1), (6, 2)]),
    (1, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[(6, 1)]),
    (6, [0, 0, 0, 0, 0, 0, 0, 0, 0], &[]),
];

// ========================================================================= //

pub struct LaneState {
    access: Access,
    grid: Vec<i8>,
    stage: usize,
}

impl LaneState {
    pub fn from_toml(mut table: toml::value::Table) -> LaneState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let stage = if access.is_solved() {
            STAGES.len()
        } else {
            min(table.remove(STAGE_KEY).map(to_u32).unwrap_or(0) as usize,
                STAGES.len() - 1)
        };
        let mut grid: Vec<i8> = pop_array(&mut table, GRID_KEY)
            .into_iter()
            .map(to_i32)
            .filter(|&val| -NUM_SYMBOLS <= val && val <= NUM_SYMBOLS)
            .map(|val| val as i8)
            .collect();
        grid.resize(GRID_SIZE, 0);
        LaneState {
            access: access,
            grid: grid,
            stage: stage,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid = LaneState::empty_grid();
        self.stage = STAGES.len();
    }

    pub fn num_cols(&self) -> i32 { NUM_COLS }

    pub fn num_rows(&self) -> i32 { NUM_ROWS }

    pub fn total_num_stages(&self) -> usize { STAGES.len() }

    pub fn current_stage(&self) -> usize { self.stage }

    pub fn grid(&self) -> &Vec<i8> { &self.grid }

    pub fn symbol_at(&self, col: i32, row: i32) -> Option<i8> {
        if col >= 0 && col < NUM_COLS && row >= 0 && row < NUM_ROWS {
            let value = self.grid[(row * NUM_COLS + col) as usize];
            if value == 0 { None } else { Some(value.abs()) }
        } else {
            None
        }
    }

    pub fn next_shape(&self) -> Option<&'static [i8; 9]> {
        if self.stage < STAGES.len() && STAGES[self.stage].0 == 0 {
            Some(&STAGES[self.stage].1)
        } else {
            None
        }
    }

    pub fn next_remove(&self) -> Option<i8> {
        if self.stage < STAGES.len() && STAGES[self.stage].0 != 0 {
            Some(STAGES[self.stage].0)
        } else {
            None
        }
    }

    pub fn try_place_shape(&mut self, col: i32, row: i32) -> Option<i8> {
        if let Some(shape) = self.next_shape() {
            let mut symbols: Vec<(i8, usize)> = Vec::new();
            for (index, &symbol) in shape.iter().enumerate() {
                if symbol != 0 {
                    let col = col + (index as i32 % 3);
                    let row = row + (index as i32 / 3);
                    if (col < 0 || col >= NUM_COLS) ||
                       (row < 0 || row >= NUM_ROWS) {
                        return None;
                    }
                    let index = (row * NUM_COLS + col) as usize;
                    if self.grid[index] != 0 {
                        return None;
                    }
                    symbols.push((symbol, index));
                }
            }
            debug_assert!(!symbols.is_empty());
            for &(symbol, index) in &symbols {
                self.grid[index] = symbol;
            }
            self.advance();
            Some(symbols[0].0)
        } else {
            None
        }
    }

    pub fn can_remove_symbol(&self, symbol: i8) -> bool {
        assert!(symbol > 0 && symbol as i32 <= NUM_SYMBOLS);
        for &value in self.grid.iter() {
            if value == symbol {
                return false;
            }
        }
        true
    }

    pub fn remove_symbol(&mut self, symbol: i8) {
        assert!(symbol > 0 && symbol as i32 <= NUM_SYMBOLS);
        let mut failure = false;
        for value in self.grid.iter_mut() {
            if *value == symbol {
                failure = true;
                break;
            } else if *value == -symbol {
                *value = 0;
            }
        }
        if failure {
            self.reset();
        } else if self.next_remove() == Some(symbol) {
            self.advance();
        }
    }

    fn advance(&mut self) {
        debug_assert!(self.stage < STAGES.len());
        for &(symbol, num) in STAGES[self.stage as usize].2 {
            let mut indices: Vec<usize> = Vec::new();
            for (index, &value) in self.grid.iter().enumerate() {
                if value == symbol {
                    indices.push(index);
                }
            }
            let sample = rand::sample(&mut rand::thread_rng(), indices, num);
            debug_assert_eq!(sample.len(), num);
            for index in sample {
                self.grid[index] = -self.grid[index];
            }
        }
        self.stage += 1;
        if self.stage == STAGES.len() {
            self.access = Access::Solved;
        }
    }

    fn empty_grid() -> Vec<i8> { vec![0; GRID_SIZE] }
}

impl Default for LaneState {
    fn default() -> LaneState {
        LaneState {
            access: Default::default(),
            grid: LaneState::empty_grid(),
            stage: 0,
        }
    }
}

impl PuzzleState for LaneState {
    fn location(&self) -> Location { Location::MemoryLane }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.stage > 0 }

    fn reset(&mut self) {
        self.grid = LaneState::empty_grid();
        self.stage = 0;
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.access.is_solved() {
            table.insert(STAGE_KEY.to_string(),
                         toml::Value::Integer(self.stage as i64));
            let grid = self.grid
                           .iter()
                           .map(|&val| toml::Value::Integer(val as i64))
                           .collect();
            table.insert(GRID_KEY.to_string(), toml::Value::Array(grid));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use super::{NUM_SYMBOLS, STAGES};

    #[test]
    fn stages_are_well_formed() {
        let mut symbols_on_board = BTreeSet::new();
        let mut num_symbols_in_use = vec![0; NUM_SYMBOLS as usize + 1];
        for (stage, &(free, shape, decay)) in STAGES.iter().enumerate() {
            if free != 0 {
                assert!(free > 0 && free as i32 <= NUM_SYMBOLS);
                assert!(symbols_on_board.contains(&free),
                        "Stage {} frees {}, but it's not on the board.",
                        stage,
                        free);
                assert_eq!(num_symbols_in_use[free as usize], 0);
                for &symbol in &shape {
                    assert_eq!(symbol, 0);
                }
                symbols_on_board.remove(&free);
            } else {
                let mut num_symbols = 0;
                for &symbol in &shape {
                    assert!(symbol >= 0 && symbol as i32 <= NUM_SYMBOLS);
                    if symbol > 0 {
                        num_symbols += 1;
                        symbols_on_board.insert(symbol);
                        num_symbols_in_use[symbol as usize] += 1;
                    }
                }
                assert_eq!(num_symbols, 4);
            }
            for &(symbol, num) in decay {
                assert!(symbol > 0 && symbol as i32 <= NUM_SYMBOLS);
                assert!(symbols_on_board.contains(&symbol),
                        "Stage {} decays {}, but it's not on the board.",
                        stage,
                        symbol);
                assert!(num_symbols_in_use[symbol as usize] >= num,
                        "Stage {} decays {} by {}, but only {} are in use.",
                        stage,
                        symbol,
                        num,
                        num_symbols_in_use[symbol as usize]);
                num_symbols_in_use[symbol as usize] -= num;
            }
        }
        assert!(symbols_on_board.is_empty(),
                "At the end of the puzzle, {:?} are still on the board.",
                symbols_on_board);
        for count in num_symbols_in_use {
            assert_eq!(count, 0);
        }
    }
}

// ========================================================================= //
