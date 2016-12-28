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

use std::collections::BTreeSet;
use toml;

use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array};

// ========================================================================= //

const TOGGLED_KEY: &'static str = "toggled";

const NUM_COLS: i32 = 4;
const NUM_ROWS: i32 = 3;

const INITIAL_GRID: &'static [bool] = &[false, true, false, true, true,
                                        false, true, false, true, true, true,
                                        false, false, false, false, true,
                                        false, true, true, false, true,
                                        false, true, true, false, true, true,
                                        false, true, false];

const LETTERS: &'static [char] = &['T', 'A', 'S', 'H', 'L', 'E', 'T'];

const SOLVED_TOGGLED_1: &'static [i32] = &[2, 7, 0, 10, 8, 4, 9];
const SOLVED_TOGGLED_2: &'static [i32] = &[9, 7, 0, 10, 8, 4, 2];

// ========================================================================= //

#[derive(Default)]
pub struct TreadState {
    access: Access,
    toggled: Vec<i32>,
    grid: Vec<bool>,
}

impl TreadState {
    pub fn from_toml(mut table: toml::Table) -> TreadState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let toggled = if access == Access::Solved {
            SOLVED_TOGGLED_1.iter().cloned().collect()
        } else {
            let set: BTreeSet<i32> = pop_array(&mut table, TOGGLED_KEY)
                                         .iter()
                                         .filter_map(toml::Value::as_integer)
                                         .filter(|&idx| 0 <= idx && idx < 12)
                                         .map(|idx| idx as i32)
                                         .collect();
            set.into_iter().take(LETTERS.len()).collect()
        };
        let mut state = TreadState {
            access: access,
            toggled: toggled,
            grid: Vec::new(),
        };
        state.rebuild_grid();
        state
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if self.access != Access::Solved {
            let toggled = self.toggled
                              .iter()
                              .map(|&idx| toml::Value::Integer(idx as i64))
                              .collect();
            table.insert(TOGGLED_KEY.to_string(), toml::Value::Array(toggled));
        }
        toml::Value::Table(table)
    }

    pub fn reset(&mut self) {
        self.toggled.clear();
        self.rebuild_grid();
    }

    pub fn replay(&mut self) {
        self.access = Access::Replay;
        self.reset();
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.toggled = SOLVED_TOGGLED_1.iter().cloned().collect();
        self.rebuild_grid();
    }

    pub fn is_lit(&self, (col, row): (i32, i32)) -> bool {
        if col >= 0 && col <= NUM_COLS + 1 && row >= 0 && row <= NUM_ROWS + 1 {
            self.grid[(row * (NUM_COLS + 2) + col) as usize]
        } else {
            false
        }
    }

    pub fn next_label(&self) -> Option<char> {
        let num_toggled = self.toggled.len();
        if num_toggled < LETTERS.len() {
            Some(LETTERS[num_toggled])
        } else {
            None
        }
    }

    pub fn toggled_label(&self, (col, row): (i32, i32)) -> Option<char> {
        if col >= 1 && col <= NUM_COLS && row >= 1 && row <= NUM_ROWS {
            let index = (row - 1) * NUM_COLS + (col - 1);
            for (char_index, &grid_index) in self.toggled.iter().enumerate() {
                if index == grid_index {
                    return Some(LETTERS[char_index]);
                }
            }
        }
        None
    }

    pub fn push_toggle(&mut self, pos: (i32, i32)) -> bool {
        let (col, row) = pos;
        if self.toggled.len() < LETTERS.len() &&
           (col >= 1 && col <= NUM_COLS) &&
           (row >= 1 && row <= NUM_ROWS) {
            let index = (row - 1) * NUM_COLS + (col - 1);
            if !self.toggled.contains(&index) {
                self.toggled.push(index);
                self.rebuild_grid();
                let toggled = &self.toggled as &[i32];
                if toggled == SOLVED_TOGGLED_1 || toggled == SOLVED_TOGGLED_2 {
                    self.access = Access::Solved;
                }
                return true;
            }
        }
        false
    }

    pub fn pop_toggle(&mut self) {
        self.toggled.pop();
        self.rebuild_grid();
    }

    fn rebuild_grid(&mut self) {
        self.grid = INITIAL_GRID.iter().cloned().collect();
        debug_assert_eq!(self.grid.len() as i32,
                         (NUM_ROWS + 2) * (NUM_COLS + 2));
        debug_assert!(self.toggled.len() <= LETTERS.len());
        for (char_index, &entry) in self.toggled.iter().enumerate() {
            let row = 1 + (entry / NUM_COLS);
            let col = 1 + (entry % NUM_COLS);
            let shape = match LETTERS[char_index] {
                'A' => vec![(0, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (1, 1)],
                'E' => {
                    vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (-1, 1),
                         (0, 1), (1, 1)]
                }
                'H' => {
                    vec![(-1, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1),
                         (1, 1)]
                }
                'L' => vec![(0, -1), (0, 0), (0, 1), (1, 1)],
                'S' => vec![(0, -1), (1, -1), (0, 0), (-1, 1), (0, 1)],
                'T' => vec![(-1, -1), (0, -1), (1, -1), (0, 0), (0, 1)],
                _ => vec![],
            };
            for (dx, dy) in shape {
                let index = ((row + dy) * (NUM_COLS + 2) + col + dx) as usize;
                self.grid[index] = !self.grid[index];
            }
        }
    }
}

impl PuzzleState for TreadState {
    fn location(&self) -> Location { Location::TreadLightly }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.toggled.is_empty() }
}

// ========================================================================= //
