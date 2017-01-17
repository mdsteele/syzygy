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

use save::{Access, PrimaryColor, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array};

// ========================================================================= //

const RED_TOGGLED_KEY: &'static str = "red";
const GREEN_TOGGLED_KEY: &'static str = "green";
const BLUE_TOGGLED_KEY: &'static str = "blue";
const NEXT_COLOR_KEY: &'static str = "next";

const INITIAL_RED_GRID: &'static [bool] = &[true, true, true, true, false,
                                            true, true, true, false, true,
                                            true, true, true, false, true,
                                            false, true, true, false, true,
                                            true];
const INITIAL_GREEN_GRID: &'static [bool] = &[true, false, true, false,
                                              false, true, true, true, false,
                                              false, true, false, true, true,
                                              false, false, true, true, true,
                                              true, true];
const INITIAL_BLUE_GRID: &'static [bool] = &[true, false, true, true, false,
                                             false, false, false, true, true,
                                             true, true, true, true, true,
                                             false, true, false, true, true,
                                             false];

const SOLVED_RED_TOGGLED: &'static [i32] = &[3, 8, 13, 14];
const SOLVED_GREEN_TOGGLED: &'static [i32] = &[0, 4, 9, 10];
const SOLVED_BLUE_TOGGLED: &'static [i32] = &[5, 7, 11, 16];

// ========================================================================= //

#[derive(Default)]
pub struct SyrupState {
    access: Access,
    next_color: PrimaryColor,
    red_toggled: BTreeSet<i32>,
    green_toggled: BTreeSet<i32>,
    blue_toggled: BTreeSet<i32>,
    red_grid: Vec<bool>,
    green_grid: Vec<bool>,
    blue_grid: Vec<bool>,
}

impl SyrupState {
    pub fn from_toml(mut table: toml::Table) -> SyrupState {
        let table_ref = &mut table;
        let mut state = SyrupState {
            access: Access::from_toml(table_ref.get(ACCESS_KEY)),
            next_color: PrimaryColor::from_toml(table_ref.get(NEXT_COLOR_KEY)),
            red_toggled: pop_toggled(table_ref, RED_TOGGLED_KEY),
            green_toggled: pop_toggled(table_ref, GREEN_TOGGLED_KEY),
            blue_toggled: pop_toggled(table_ref, BLUE_TOGGLED_KEY),
            red_grid: Vec::new(),
            green_grid: Vec::new(),
            blue_grid: Vec::new(),
        };
        if state.access.is_solved() {
            state.solve();
        } else {
            state.rebuild_grids();
        }
        state
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.next_color = Default::default();
        self.red_toggled = SOLVED_RED_TOGGLED.iter().cloned().collect();
        self.green_toggled = SOLVED_GREEN_TOGGLED.iter().cloned().collect();
        self.blue_toggled = SOLVED_BLUE_TOGGLED.iter().cloned().collect();
        self.rebuild_grids();
    }

    pub fn light_colors(&self, pos: (i32, i32)) -> (bool, bool, bool) {
        let index = pos_to_index(pos).unwrap() as usize;
        (self.red_grid[index], self.green_grid[index], self.blue_grid[index])
    }

    pub fn toggle(&mut self, pos: (i32, i32)) {
        self.internal_toggle(pos);
        self.next_color = match self.next_color {
            PrimaryColor::Red => PrimaryColor::Green,
            PrimaryColor::Green => PrimaryColor::Blue,
            PrimaryColor::Blue => PrimaryColor::Red,
        };
        if self.red_grid.iter().all(|&r| r) &&
           self.green_grid.iter().all(|&g| g) &&
           self.blue_grid.iter().all(|&b| b) {
            self.access = Access::Solved;
        }
    }

    pub fn untoggle(&mut self, pos: (i32, i32)) {
        self.next_color = match self.next_color {
            PrimaryColor::Red => PrimaryColor::Blue,
            PrimaryColor::Green => PrimaryColor::Red,
            PrimaryColor::Blue => PrimaryColor::Green,
        };
        self.internal_toggle(pos);
    }

    fn internal_toggle(&mut self, pos: (i32, i32)) {
        let index = pos_to_index(pos).unwrap();
        {
            let toggled = match self.next_color {
                PrimaryColor::Red => &mut self.red_toggled,
                PrimaryColor::Green => &mut self.green_toggled,
                PrimaryColor::Blue => &mut self.blue_toggled,
            };
            if toggled.contains(&index) {
                toggled.remove(&index);
            } else {
                toggled.insert(index);
            }
        }
        match self.next_color {
            PrimaryColor::Red => {
                rebuild_grid(&mut self.red_grid,
                             &self.red_toggled,
                             INITIAL_RED_GRID)
            }
            PrimaryColor::Green => {
                rebuild_grid(&mut self.green_grid,
                             &self.green_toggled,
                             INITIAL_GREEN_GRID)
            }
            PrimaryColor::Blue => {
                rebuild_grid(&mut self.blue_grid,
                             &self.blue_toggled,
                             INITIAL_BLUE_GRID)
            }
        }
    }

    fn rebuild_grids(&mut self) {
        rebuild_grid(&mut self.red_grid, &self.red_toggled, INITIAL_RED_GRID);
        rebuild_grid(&mut self.green_grid,
                     &self.green_toggled,
                     INITIAL_GREEN_GRID);
        rebuild_grid(&mut self.blue_grid,
                     &self.blue_toggled,
                     INITIAL_BLUE_GRID);
    }
}

impl PuzzleState for SyrupState {
    fn location(&self) -> Location { Location::LightSyrup }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool {
        self.next_color != Default::default() ||
        !self.red_toggled.is_empty() ||
        !self.green_toggled.is_empty() || !self.blue_toggled.is_empty()
    }

    fn reset(&mut self) {
        self.next_color = Default::default();
        self.red_toggled.clear();
        self.green_toggled.clear();
        self.blue_toggled.clear();
        self.rebuild_grids();
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(NEXT_COLOR_KEY.to_string(),
                         self.next_color.to_toml());
            insert_toggled(&mut table, RED_TOGGLED_KEY, &self.red_toggled);
            insert_toggled(&mut table, GREEN_TOGGLED_KEY, &self.green_toggled);
            insert_toggled(&mut table, BLUE_TOGGLED_KEY, &self.blue_toggled);
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //

fn index_to_pos(index: i32) -> (i32, i32) {
    if index < 3 {
        (index + 1, 0)
    } else if index < 18 {
        ((index + 2) % 5, (index + 2) / 5)
    } else {
        (index - 17, 4)
    }
}

fn pos_to_index((col, row): (i32, i32)) -> Option<i32> {
    if row == 0 {
        if col >= 1 && col <= 3 {
            Some(col - 1)
        } else {
            None
        }
    } else if row == 4 {
        if col >= 1 && col <= 3 {
            Some(col + 17)
        } else {
            None
        }
    } else if row >= 1 && row <= 3 && col >= 0 && col <= 4 {
        Some(5 * row + col - 2)
    } else {
        None
    }
}

fn rebuild_grid(grid: &mut Vec<bool>, toggled: &BTreeSet<i32>,
                initial: &[bool]) {
    *grid = initial.iter().cloned().collect();
    for &index in toggled {
        grid[index as usize] = !grid[index as usize];
        let (col, row) = index_to_pos(index);
        if let Some(index) = pos_to_index((col - 1, row)) {
            grid[index as usize] = !grid[index as usize];
        }
        if let Some(index) = pos_to_index((col + 1, row)) {
            grid[index as usize] = !grid[index as usize];
        }
        if let Some(index) = pos_to_index((col, row - 1)) {
            grid[index as usize] = !grid[index as usize];
        }
        if let Some(index) = pos_to_index((col, row + 1)) {
            grid[index as usize] = !grid[index as usize];
        }
    }
}

fn insert_toggled(table: &mut toml::Table, key: &str, tog: &BTreeSet<i32>) {
    if !tog.is_empty() {
        let vec = tog.iter()
                     .map(|&idx| toml::Value::Integer(idx as i64))
                     .collect();
        table.insert(key.to_string(), toml::Value::Array(vec));
    }
}

fn pop_toggled(mut table: &mut toml::Table, key: &str) -> BTreeSet<i32> {
    pop_array(&mut table, key)
        .iter()
        .filter_map(toml::Value::as_integer)
        .filter(|&idx| 0 <= idx && idx < 21)
        .map(|idx| idx as i32)
        .collect()
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::{index_to_pos, pos_to_index};

    #[test]
    fn index_to_pos_to_index() {
        for index in 0..21 {
            assert_eq!(Some(index), pos_to_index(index_to_pos(index)));
        }
    }
}

// ========================================================================= //
