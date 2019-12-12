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

use std::collections::VecDeque;
use toml;

use crate::save::{Access, Direction, Location};
use crate::save::util::{ACCESS_KEY, Tomlable, rotate_deque, to_table};
use super::PuzzleState;

// ========================================================================= //

const NUM_COLS: i32 = 9;
const NUM_ROWS: i32 = 7;

const GRID_KEY: &str = "grid";

#[cfg_attr(rustfmt, rustfmt_skip)]
const INITIAL_GRID: &[i8] = &[
    2,   1,  2,  1,  2, -1,  2,  0, 1,
    -1, -1,  1,  2,  0, -1, -1,  2, 0,
    0,   1,  2,  1, -1,  0,  1,  1, 2,
    2,   0, -1, -1,  2, -1, -1, -1, 1,
    2,   1,  0,  2, -1,  1,  2,  2, 0,
    0,  -1, -1, -1,  1,  0, -1,  0, 1,
    1,   0,  2,  2,  0,  1, -1,  2, 2,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const SOLVED_GRID: &[i8] = &[
    0,   0,  0,  1,  1, -1,  2,  2, 2,
    -1, -1,  0,  1,  1, -1, -1,  2, 2,
    2,   2,  2,  0, -1,  0,  1,  1, 1,
    2,   2, -1, -1,  0, -1, -1, -1, 1,
    2,   2,  2,  0, -1,  0,  1,  1, 1,
    1,  -1, -1, -1,  2,  2, -1,  0, 0,
    1,   1,  1,  2,  2,  2, -1,  0, 0,
];

// ========================================================================= //

pub struct WreckedState {
    access: Access,
    grid: Vec<i8>,
    is_initial: bool,
}

impl WreckedState {
    pub fn new() -> WreckedState {
        WreckedState::from_toml(toml::Value::Boolean(false))
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid = SOLVED_GRID.to_vec();
        self.is_initial = false;
    }

    pub fn tile_at(&self, col: i32, row: i32) -> Option<usize> {
        if col < 0 || col >= NUM_COLS || row < 0 || row >= NUM_ROWS {
            None
        } else {
            let index = (row * NUM_COLS + col) as usize;
            let value = self.grid[index];
            if value < 0 {
                None
            } else {
                Some(value as usize)
            }
        }
    }

    pub fn shift_tiles(&mut self, dir: Direction, rank: i32, by: i32) {
        match dir {
            Direction::East | Direction::West => {
                if rank >= 0 && rank < NUM_ROWS {
                    let mut tiles = VecDeque::new();
                    for col in 0..NUM_COLS {
                        let index = (rank * NUM_COLS + col) as usize;
                        let value = self.grid[index];
                        if value >= 0 {
                            tiles.push_back(value);
                        }
                    }
                    rotate_deque(&mut tiles, dir.delta().x() * by);
                    for col in 0..NUM_COLS {
                        let index = (rank * NUM_COLS + col) as usize;
                        if self.grid[index] >= 0 {
                            self.grid[index] = tiles.pop_front().unwrap();
                        }
                    }
                }
            }
            Direction::South | Direction::North => {
                if rank >= 0 && rank < NUM_COLS {
                    let mut tiles = VecDeque::new();
                    for row in 0..NUM_ROWS {
                        let index = (row * NUM_COLS + rank) as usize;
                        let value = self.grid[index];
                        if value >= 0 {
                            tiles.push_back(value);
                        }
                    }
                    rotate_deque(&mut tiles, dir.delta().y() * by);
                    for row in 0..NUM_ROWS {
                        let index = (row * NUM_COLS + rank) as usize;
                        if self.grid[index] >= 0 {
                            self.grid[index] = tiles.pop_front().unwrap();
                        }
                    }
                }
            }
        }
        self.is_initial = &self.grid as &[i8] == INITIAL_GRID;
        if &self.grid as &[i8] == SOLVED_GRID {
            self.access = Access::Solved;
        }
    }
}

impl PuzzleState for WreckedState {
    fn location() -> Location { Location::WreckedAngle }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.is_initial }

    fn reset(&mut self) {
        self.grid = INITIAL_GRID.to_vec();
        self.is_initial = true;
    }
}

impl Tomlable for WreckedState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_initial && !self.is_solved() {
            table.insert(GRID_KEY.to_string(), self.grid.to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> WreckedState {
        let mut table = to_table(value);
        let mut access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let grid = if access.is_solved() {
            SOLVED_GRID.to_vec()
        } else {
            let mut grid = Vec::<i8>::pop_from_table(&mut table, GRID_KEY);
            let mut init_sorted = INITIAL_GRID.to_vec();
            init_sorted.sort();
            let mut grid_sorted = grid.clone();
            grid_sorted.sort();
            if grid_sorted != init_sorted {
                grid = INITIAL_GRID.to_vec()
            } else {
                let init_neg: Vec<bool> =
                    INITIAL_GRID.iter().map(|&tile| tile < 0).collect();
                let grid_neg: Vec<bool> =
                    grid.iter().map(|&tile| tile < 0).collect();
                if grid_neg != init_neg {
                    grid = INITIAL_GRID.to_vec();
                }
            }
            if &grid as &[i8] == SOLVED_GRID {
                access = Access::Solved;
            }
            grid
        };
        let is_initial = &grid as &[i8] == INITIAL_GRID;
        WreckedState {
            access: access,
            grid: grid,
            is_initial: is_initial,
        }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use crate::save::{Access, Direction};
    use crate::save::util::{ACCESS_KEY, Tomlable};
    use super::{GRID_KEY, INITIAL_GRID, SOLVED_GRID, WreckedState};

    #[test]
    fn toml_round_trip() {
        let mut state = WreckedState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.shift_tiles(Direction::East, 0, 1);
        state.shift_tiles(Direction::South, 0, 1);
        assert!(!state.is_initial);
        let grid = state.grid.clone();

        let state = WreckedState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.grid, grid);
        assert!(!state.is_initial);
    }

    #[test]
    fn from_empty_toml() {
        let state = WreckedState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(&state.grid as &[i8], INITIAL_GRID);
        assert!(state.is_initial);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = WreckedState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(&state.grid as &[i8], SOLVED_GRID);
        assert!(!state.is_initial);
    }

    #[test]
    fn from_already_correct_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        table.insert(GRID_KEY.to_string(), SOLVED_GRID.to_vec().to_toml());

        let state = WreckedState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(&state.grid as &[i8], SOLVED_GRID);
    }

    #[test]
    fn shift_east() {
        let mut state = WreckedState::from_toml(toml::Value::Boolean(false));
        state.shift_tiles(Direction::East, 0, 1);
        assert_eq!(state.tile_at(0, 0), Some(1));
        assert_eq!(state.tile_at(5, 0), None);
        assert_eq!(state.tile_at(6, 0), Some(2));
    }

    #[test]
    fn shift_west() {
        let mut state = WreckedState::from_toml(toml::Value::Boolean(false));
        state.shift_tiles(Direction::West, 1, 1);
        assert_eq!(state.tile_at(1, 1), None);
        assert_eq!(state.tile_at(4, 1), Some(2));
        assert_eq!(state.tile_at(8, 1), Some(1));
    }

    #[test]
    fn shift_south() {
        let mut state = WreckedState::from_toml(toml::Value::Boolean(false));
        state.shift_tiles(Direction::South, 0, 1);
        assert_eq!(state.tile_at(0, 0), Some(1));
        assert_eq!(state.tile_at(0, 1), None);
        assert_eq!(state.tile_at(0, 2), Some(2));
    }

    #[test]
    fn shift_north() {
        let mut state = WreckedState::from_toml(toml::Value::Boolean(false));
        state.shift_tiles(Direction::North, 8, 1);
        assert_eq!(state.tile_at(8, 1), Some(2));
        assert_eq!(state.tile_at(8, 4), Some(1));
        assert_eq!(state.tile_at(8, 6), Some(1));
    }
}

// ========================================================================= //
