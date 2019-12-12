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

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use toml;

use super::PuzzleState;
use crate::save::util::{rotate_deque, to_table, Tomlable, ACCESS_KEY};
use crate::save::{Access, Location};

// ========================================================================= //

const GRID_KEY: &str = "grid";

const NUM_COLS: i32 = 5;
const NUM_ROWS: i32 = 5;

#[cfg_attr(rustfmt, rustfmt_skip)]
const INITIAL_GRID: &[((i32, i32), [u8; 4])] = &[
    ((0, 0), [0, 1, 2, 3]),
    ((4, 0), [2, 2, 2, 3]),
    ((1, 1), [0, 1, 0, 2]),
    ((2, 2), [0, 0, 1, 3]),
    ((3, 3), [4, 4, 3, 2]),
    ((0, 4), [1, 3, 2, 4]),
    ((4, 4), [1, 2, 1, 3]),
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const SOLVED_GRID: &[((i32, i32), [u8; 4])] = &[
    ((1, 0), [0, 2, 0, 1]),
    ((2, 1), [2, 2, 2, 3]),
    ((4, 1), [3, 2, 4, 1]),
    ((0, 2), [3, 0, 1, 2]),
    ((1, 3), [4, 3, 2, 4]),
    ((3, 3), [0, 1, 3, 0]),
    ((2, 4), [3, 1, 2, 1]),
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const GOALS: [u8; 20] = [
    0, 3, 3, 0, 3, // Right side
    2, 1, 1, 3, 0, // Bottom side
    2, 2, 1, 2, 0, // Left side
    2, 1, 3, 0, 1, // Top side
];

// ========================================================================= //

pub struct PovState {
    access: Access,
    grid: HashMap<(i32, i32), [u8; 4]>,
    colors: [u8; 20],
    is_initial: bool,
}

impl PovState {
    fn is_valid_grid(grid: &HashMap<(i32, i32), [u8; 4]>) -> bool {
        if grid.len() != INITIAL_GRID.len() {
            return false;
        }
        let mut tiles: HashSet<[u8; 4]> =
            INITIAL_GRID.iter().map(|&(_, tile)| tile).collect();
        for &tile in grid.values() {
            let mut found = false;
            for by in 0..4 {
                if tiles.remove(&rotate(tile, by)) {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        tiles.is_empty()
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid = SOLVED_GRID.iter().cloned().collect();
        self.regenerate();
        debug_assert_eq!(self.colors, GOALS);
    }

    pub fn col_top_goal(&self, col: i32) -> u8 {
        debug_assert!(col >= 0 && col < NUM_COLS);
        GOALS[(15 + col) as usize]
    }

    pub fn col_bottom_goal(&self, col: i32) -> u8 {
        debug_assert!(col >= 0 && col < NUM_COLS);
        GOALS[(9 - col) as usize]
    }

    pub fn row_left_goal(&self, row: i32) -> u8 {
        debug_assert!(row >= 0 && row < NUM_ROWS);
        GOALS[(14 - row) as usize]
    }

    pub fn row_right_goal(&self, row: i32) -> u8 {
        debug_assert!(row >= 0 && row < NUM_ROWS);
        GOALS[row as usize]
    }

    pub fn col_top_color(&self, col: i32) -> u8 {
        debug_assert!(col >= 0 && col < NUM_COLS);
        self.colors[(15 + col) as usize]
    }

    pub fn col_bottom_color(&self, col: i32) -> u8 {
        debug_assert!(col >= 0 && col < NUM_COLS);
        self.colors[(9 - col) as usize]
    }

    pub fn row_left_color(&self, row: i32) -> u8 {
        debug_assert!(row >= 0 && row < NUM_ROWS);
        self.colors[(14 - row) as usize]
    }

    pub fn row_right_color(&self, row: i32) -> u8 {
        debug_assert!(row >= 0 && row < NUM_ROWS);
        self.colors[row as usize]
    }

    pub fn tiles(&self) -> &HashMap<(i32, i32), [u8; 4]> {
        &self.grid
    }

    pub fn tile_at(&self, coords: (i32, i32)) -> Option<[u8; 4]> {
        self.grid.get(&coords).cloned()
    }

    pub fn move_tile(&mut self, from: (i32, i32), to: (i32, i32)) -> bool {
        if (from.0 < 0 || from.0 >= NUM_COLS)
            || (from.1 < 0 || from.1 >= NUM_ROWS)
            || (to.0 < 0 || to.0 >= NUM_COLS)
            || (to.1 < 0 || to.1 >= NUM_ROWS)
        {
            return false;
        }
        if let Some(tile1) = self.grid.remove(&from) {
            if let Some(tile2) = self.grid.remove(&to) {
                self.grid.insert(from, tile2);
            }
            self.grid.insert(to, tile1);
            self.regenerate();
            true
        } else {
            false
        }
    }

    pub fn rotate_tile(&mut self, coords: (i32, i32), by: i32) -> bool {
        if (coords.0 < 0 || coords.0 >= NUM_COLS)
            || (coords.1 < 0 || coords.1 >= NUM_ROWS)
        {
            return false;
        }
        if let Some(tile) = self.grid.get_mut(&coords) {
            *tile = rotate(*tile, by);
        } else {
            return false;
        }
        self.regenerate();
        true
    }

    fn regenerate(&mut self) {
        self.is_initial = self.grid == INITIAL_GRID.iter().cloned().collect();
        // Right side:
        for row in 0..5 {
            let mut color = 255;
            for step in 0..5 {
                let col = 4 - step;
                if let Some(tile) = self.grid.get(&(col, row)) {
                    color = tile[0];
                    break;
                }
            }
            self.colors[row as usize] = color;
        }
        // Bottom side:
        for col in 0..5 {
            let mut color = 255;
            for step in 0..5 {
                let row = 4 - step;
                if let Some(tile) = self.grid.get(&(col, row)) {
                    color = tile[1];
                    break;
                }
            }
            self.colors[(9 - col) as usize] = color;
        }
        // Left side:
        for row in 0..5 {
            let mut color = 255;
            for col in 0..5 {
                if let Some(tile) = self.grid.get(&(col, row)) {
                    color = tile[2];
                    break;
                }
            }
            self.colors[(14 - row) as usize] = color;
        }
        // Top side:
        for col in 0..5 {
            let mut color = 255;
            for row in 0..5 {
                if let Some(tile) = self.grid.get(&(col, row)) {
                    color = tile[3];
                    break;
                }
            }
            self.colors[(15 + col) as usize] = color;
        }
        if self.colors == GOALS {
            self.access = Access::Solved;
        }
    }
}

impl PuzzleState for PovState {
    fn location() -> Location {
        Location::PointOfView
    }

    fn access(&self) -> Access {
        self.access
    }

    fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    fn can_reset(&self) -> bool {
        !self.is_initial
    }

    fn reset(&mut self) {
        self.grid = INITIAL_GRID.iter().cloned().collect();
        self.regenerate();
    }
}

impl Tomlable for PovState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_initial && !self.is_solved() {
            let mut grid = toml::value::Array::new();
            for (&(col, row), tile) in self.grid.iter() {
                let mut entry = toml::value::Array::new();
                entry.push(toml::Value::Integer(col as i64));
                entry.push(toml::Value::Integer(row as i64));
                for &color in tile.iter() {
                    entry.push(toml::Value::Integer(color as i64));
                }
                grid.push(toml::Value::Array(entry));
            }
            table.insert(GRID_KEY.to_string(), toml::Value::Array(grid));
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> PovState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let grid = if access.is_solved() {
            SOLVED_GRID.iter().cloned().collect()
        } else {
            let mut grid = HashMap::new();
            for entry in Vec::<Vec<i32>>::pop_from_table(&mut table, GRID_KEY)
                .into_iter()
            {
                if entry.len() != 6 {
                    continue;
                }
                let coords = (entry[0], entry[1]);
                let mut tile = [0u8; 4];
                for index in 0..4 {
                    tile[index] = max(0, min(4, entry[2 + index])) as u8;
                }
                grid.insert(coords, tile);
            }
            if PovState::is_valid_grid(&grid) {
                grid
            } else {
                INITIAL_GRID.iter().cloned().collect()
            }
        };
        let mut state =
            PovState { access, grid, colors: [255; 20], is_initial: true };
        state.regenerate();
        state
    }
}

fn rotate(mut tile: [u8; 4], by: i32) -> [u8; 4] {
    let mut deque: VecDeque<u8> = tile.iter().cloned().collect();
    rotate_deque(&mut deque, by);
    for (index, value) in deque.into_iter().enumerate() {
        tile[index] = value;
    }
    tile
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use super::{PovState, GOALS, INITIAL_GRID};
    use crate::save::util::{Tomlable, ACCESS_KEY};
    use crate::save::Access;

    #[test]
    fn toml_round_trip() {
        let mut state = PovState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        assert!(state.tile_at((0, 0)).is_some());
        state.move_tile((0, 0), (2, 0));
        assert!(state.tile_at((3, 3)).is_some());
        state.rotate_tile((3, 3), 1);
        let grid = state.grid.clone();
        let colors = state.colors.clone();
        assert!(!state.is_initial);

        let state = PovState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.grid, grid);
        assert_eq!(state.colors, colors);
        assert!(!state.is_initial);
    }

    #[test]
    fn from_empty_toml() {
        let state = PovState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.grid, INITIAL_GRID.iter().cloned().collect());
        assert!(state.is_initial);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = PovState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.colors, GOALS);
        assert!(!state.is_initial);
    }
}

// ========================================================================= //
