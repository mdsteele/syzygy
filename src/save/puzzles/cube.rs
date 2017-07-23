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

use toml;

use save::{Access, Direction, Location};
use save::util::{ACCESS_KEY, Tomlable, pop_array};
use super::PuzzleState;

// ========================================================================= //

const GRID_KEY: &str = "grid";

const NUM_COLS: i32 = 4;
const NUM_ROWS: i32 = 4;

const INITIAL_GRID: &[i32] = &[23, 16, 18, 12, 15, 9, 17, 22, 21, 3, 8, 6,
                               19, 1, 4, 11];
const SOLVED_GRID: &[i32] = &[4, 4, 3, 3, 4, 2, 2, 3, 1, 2, 2, 0, 1, 1, 0, 0];

// ========================================================================= //

pub struct CubeState {
    access: Access,
    grid: Vec<i32>,
    is_initial: bool,
}

impl CubeState {
    pub fn from_toml(mut table: toml::value::Table) -> CubeState {
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let grid = if access == Access::Solved {
            SOLVED_GRID.to_vec()
        } else {
            let mut grid: Vec<i32> = pop_array(&mut table, GRID_KEY)
                .iter()
                .filter_map(toml::Value::as_integer)
                .filter(|&idx| 0 <= idx && idx < 24)
                .map(|idx| idx as i32)
                .collect();
            if grid.len() != (NUM_COLS * NUM_ROWS) as usize {
                grid = INITIAL_GRID.to_vec();
            }
            grid
        };
        let is_initial = &grid as &[i32] == INITIAL_GRID;
        CubeState {
            access: access,
            grid: grid,
            is_initial: is_initial,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid = SOLVED_GRID.to_vec();
        self.is_initial = false;
    }

    /// Returns the front, right, and bottom faces for the cube at the given
    /// position.  Each face is a number from 0 to 5 inclusive.
    pub fn faces_at(&self, col: i32, row: i32) -> (usize, usize, usize) {
        assert!(0 <= col && col < NUM_COLS);
        assert!(0 <= row && row < NUM_ROWS);
        orientation_faces(self.grid[(NUM_COLS * row + col) as usize])
    }

    pub fn rotate_cubes(&mut self, dir: Direction, rank: i32, by: i32) {
        if dir.is_vertical() {
            assert!(rank >= 0 && rank < NUM_COLS);
            for row in 0..NUM_ROWS {
                let index = (row * NUM_COLS + rank) as usize;
                self.grid[index] = rotate_vert(self.grid[index],
                                               dir.delta().y() * by);
            }
        } else {
            assert!(rank >= 0 && rank < NUM_ROWS);
            for col in 0..NUM_COLS {
                let index = (rank * NUM_COLS + col) as usize;
                self.grid[index] = rotate_horz(self.grid[index],
                                               dir.delta().x() * by);
            }
        }
        self.is_initial = &self.grid as &[i32] == INITIAL_GRID;
        if self.grid.iter().zip(SOLVED_GRID.iter()).all(fronts_match) {
            self.access = Access::Solved;
        }
    }
}

impl PuzzleState for CubeState {
    fn location(&self) -> Location { Location::CubeTangle }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.is_initial }

    fn reset(&mut self) {
        self.grid = INITIAL_GRID.to_vec();
        self.is_initial = true;
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_initial && !self.is_solved() {
            let grid = self.grid
                           .iter()
                           .map(|&idx| toml::Value::Integer(idx as i64))
                           .collect();
            table.insert(GRID_KEY.to_string(), toml::Value::Array(grid));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //

/// Determines whether two cube orientations have the same front face.
fn fronts_match((&ori1, &ori2): (&i32, &i32)) -> bool {
    orientation_faces(ori1).0 == orientation_faces(ori2).0
}

/// Returns the front, right, and bottom faces for the given orientation.
fn orientation_faces(orientation: i32) -> (usize, usize, usize) {
    match orientation {
        // Red on right:
        0 => (1, 0, 2),
        1 => (3, 0, 1),
        2 => (4, 0, 3),
        3 => (2, 0, 4),
        // Green on right:
        4 => (0, 1, 3),
        5 => (2, 1, 0),
        6 => (5, 1, 2),
        7 => (3, 1, 5),
        // Yellow on right:
        8 => (0, 2, 1),
        9 => (4, 2, 0),
        10 => (5, 2, 4),
        11 => (1, 2, 5),
        // Cyan on right:
        12 => (1, 5, 3),
        13 => (2, 5, 1),
        14 => (4, 5, 2),
        15 => (3, 5, 4),
        // Magenta on right:
        16 => (3, 4, 0),
        17 => (5, 4, 3),
        18 => (2, 4, 5),
        19 => (0, 4, 2),
        // Blue on right:
        20 => (1, 3, 0),
        21 => (5, 3, 1),
        22 => (4, 3, 5),
        23 => (0, 3, 4),
        _ => panic!("bad orientation: {}", orientation),
    }
}

fn rotate_right(orientation: i32) -> i32 {
    match orientation {
        // Yellow on bottom:
        0 => 6,
        6 => 14,
        14 => 19,
        19 => 0,
        // Green on bottom:
        1 => 21,
        21 => 13,
        13 => 8,
        8 => 1,
        // Blue on bottom:
        2 => 17,
        17 => 12,
        12 => 4,
        4 => 2,
        // Magenta on bottom:
        3 => 10,
        10 => 15,
        15 => 23,
        23 => 3,
        // Red on bottom:
        5 => 9,
        9 => 16,
        16 => 20,
        20 => 5,
        // Cyan on bottom:
        7 => 22,
        22 => 18,
        18 => 11,
        11 => 7,
        _ => panic!("bad orientation: {}", orientation),
    }
}

fn rotate_horz(mut orientation: i32, by: i32) -> i32 {
    for _ in 0..(by & 0x3) {
        orientation = rotate_right(orientation);
    }
    orientation
}

fn rotate_vert(orientation: i32, by: i32) -> i32 {
    (orientation / 4) * 4 + ((orientation + by) & 0x3)
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use save::{Access, PuzzleState};
    use save::util::{ACCESS_KEY, Tomlable, to_table};
    use super::{CubeState, INITIAL_GRID, NUM_COLS, NUM_ROWS, SOLVED_GRID,
                rotate_vert};

    #[test]
    fn grid_sizes() {
        assert_eq!(NUM_COLS * NUM_ROWS, INITIAL_GRID.len() as i32);
        assert_eq!(NUM_COLS * NUM_ROWS, SOLVED_GRID.len() as i32);
    }

    #[test]
    fn vertical_rotation() {
        assert_eq!(1, rotate_vert(0, 1));
        assert_eq!(2, rotate_vert(1, 1));
        assert_eq!(3, rotate_vert(2, 1));
        assert_eq!(0, rotate_vert(3, 1));
        assert_eq!(5, rotate_vert(4, 1));
    }

    #[test]
    fn toml_round_trip() {
        let mut state = CubeState::from_toml(toml::value::Table::new());
        state.access = Access::Replaying;
        state.grid = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
                          15];
        state.is_initial = false;

        let state = CubeState::from_toml(to_table(state.to_toml()));
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.grid,
                   vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        assert!(!state.is_initial);
    }

    #[test]
    fn from_empty_toml() {
        let state = CubeState::from_toml(toml::value::Table::new());
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.grid, INITIAL_GRID.to_vec());
        assert!(state.is_initial);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = CubeState::from_toml(table);
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.grid, SOLVED_GRID.to_vec());
        assert!(!state.is_initial);
    }
}

// ========================================================================= //
