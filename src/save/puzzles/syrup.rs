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

use std::collections::HashSet;
use toml;

use super::PuzzleState;
use crate::save::util::{to_table, Tomlable, ACCESS_KEY};
use crate::save::{Access, Location, PrimaryColor};

// ========================================================================= //

const RED_TOGGLED_KEY: &str = "red";
const GREEN_TOGGLED_KEY: &str = "green";
const BLUE_TOGGLED_KEY: &str = "blue";
const NEXT_COLOR_KEY: &str = "next";

#[cfg_attr(rustfmt, rustfmt_skip)]
const INITIAL_RED_GRID: &[bool] = &[
           true,  true,  true,
    true,  false, true,  true,  true,
    false, true,  true,  true,  true,
    false, true,  false, true,  true,
           false, true,  true,
];
#[cfg_attr(rustfmt, rustfmt_skip)]
const INITIAL_GREEN_GRID: &[bool] = &[
           true,  false, true,
    false, false, true,  true,  true,
    false, false, true,  false, true,
    true,  false, false, true,  true,
           true,  true,  true,
];
#[cfg_attr(rustfmt, rustfmt_skip)]
const INITIAL_BLUE_GRID: &[bool] = &[
           true,  false, true,
    true,  false, false, false, false,
    true,  true,  true,  true,  true,
    true,  true,  false, true,  false,
           true,  true,  false,
];

const SOLVED_RED_TOGGLED: &[i32] = &[3, 8, 13, 14];
const SOLVED_GREEN_TOGGLED: &[i32] = &[0, 4, 9, 10];
const SOLVED_BLUE_TOGGLED: &[i32] = &[5, 7, 11, 16];

// ========================================================================= //

pub struct SyrupState {
    access: Access,
    next_color: PrimaryColor,
    red_toggled: HashSet<i32>,
    green_toggled: HashSet<i32>,
    blue_toggled: HashSet<i32>,
    red_grid: Vec<bool>,
    green_grid: Vec<bool>,
    blue_grid: Vec<bool>,
}

impl SyrupState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.next_color = Default::default();
        self.red_toggled = SOLVED_RED_TOGGLED.iter().cloned().collect();
        self.green_toggled = SOLVED_GREEN_TOGGLED.iter().cloned().collect();
        self.blue_toggled = SOLVED_BLUE_TOGGLED.iter().cloned().collect();
        self.rebuild_grids();
    }

    pub fn next_color(&self) -> PrimaryColor {
        self.next_color
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
        self.check_if_solved();
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
            PrimaryColor::Red => rebuild_grid(
                &mut self.red_grid,
                &self.red_toggled,
                INITIAL_RED_GRID,
            ),
            PrimaryColor::Green => rebuild_grid(
                &mut self.green_grid,
                &self.green_toggled,
                INITIAL_GREEN_GRID,
            ),
            PrimaryColor::Blue => rebuild_grid(
                &mut self.blue_grid,
                &self.blue_toggled,
                INITIAL_BLUE_GRID,
            ),
        }
    }

    fn rebuild_grids(&mut self) {
        rebuild_grid(&mut self.red_grid, &self.red_toggled, INITIAL_RED_GRID);
        rebuild_grid(
            &mut self.green_grid,
            &self.green_toggled,
            INITIAL_GREEN_GRID,
        );
        rebuild_grid(
            &mut self.blue_grid,
            &self.blue_toggled,
            INITIAL_BLUE_GRID,
        );
    }

    fn check_if_solved(&mut self) {
        if self.red_grid.iter().all(|&r| r)
            && self.green_grid.iter().all(|&g| g)
            && self.blue_grid.iter().all(|&b| b)
        {
            self.access = Access::Solved;
        }
    }
}

impl PuzzleState for SyrupState {
    fn location() -> Location {
        Location::LightSyrup
    }

    fn access(&self) -> Access {
        self.access
    }

    fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    fn can_reset(&self) -> bool {
        self.next_color != Default::default()
            || !self.red_toggled.is_empty()
            || !self.green_toggled.is_empty()
            || !self.blue_toggled.is_empty()
    }

    fn reset(&mut self) {
        self.next_color = Default::default();
        self.red_toggled.clear();
        self.green_toggled.clear();
        self.blue_toggled.clear();
        self.rebuild_grids();
    }
}

impl Tomlable for SyrupState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table
                .insert(NEXT_COLOR_KEY.to_string(), self.next_color.to_toml());
            insert_toggled(&mut table, RED_TOGGLED_KEY, &self.red_toggled);
            insert_toggled(&mut table, GREEN_TOGGLED_KEY, &self.green_toggled);
            insert_toggled(&mut table, BLUE_TOGGLED_KEY, &self.blue_toggled);
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> SyrupState {
        let mut table = to_table(value);
        let table_ref = &mut table;
        let mut state = SyrupState {
            access: Access::pop_from_table(table_ref, ACCESS_KEY),
            next_color: PrimaryColor::pop_from_table(
                table_ref,
                NEXT_COLOR_KEY,
            ),
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
            state.check_if_solved();
        }
        state
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

fn rebuild_grid(
    grid: &mut Vec<bool>,
    toggled: &HashSet<i32>,
    initial: &[bool],
) {
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

fn insert_toggled(
    table: &mut toml::value::Table,
    key: &str,
    toggled: &HashSet<i32>,
) {
    if !toggled.is_empty() {
        table.insert(key.to_string(), toggled.to_toml());
    }
}

fn pop_toggled(mut table: &mut toml::value::Table, key: &str) -> HashSet<i32> {
    let mut toggled = HashSet::<i32>::pop_from_table(&mut table, key);
    toggled.retain(|&idx| 0 <= idx && idx < 21);
    toggled
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use super::{
        index_to_pos, pos_to_index, SyrupState, BLUE_TOGGLED_KEY,
        GREEN_TOGGLED_KEY, INITIAL_BLUE_GRID, INITIAL_GREEN_GRID,
        INITIAL_RED_GRID, RED_TOGGLED_KEY, SOLVED_BLUE_TOGGLED,
        SOLVED_GREEN_TOGGLED, SOLVED_RED_TOGGLED,
    };
    use crate::save::util::{Tomlable, ACCESS_KEY};
    use crate::save::{Access, PrimaryColor};

    #[test]
    fn index_to_pos_to_index() {
        for index in 0..21 {
            assert_eq!(Some(index), pos_to_index(index_to_pos(index)));
        }
    }

    #[test]
    fn toml_round_trip() {
        let mut state = SyrupState::from_toml(toml::Value::Boolean(false));
        state.toggle((1, 1));
        state.toggle((2, 2));
        state.toggle((3, 3));
        state.toggle((3, 2));
        assert_eq!(state.next_color, PrimaryColor::Green);
        let red_grid = state.red_grid.clone();
        let green_grid = state.green_grid.clone();
        let blue_grid = state.blue_grid.clone();

        let state = SyrupState::from_toml(state.to_toml());
        assert_eq!(state.next_color, PrimaryColor::Green);
        assert_eq!(
            state.red_toggled,
            vec![pos_to_index((1, 1)).unwrap(), pos_to_index((3, 2)).unwrap()]
                .into_iter()
                .collect()
        );
        assert_eq!(
            state.green_toggled,
            vec![pos_to_index((2, 2)).unwrap()].into_iter().collect()
        );
        assert_eq!(
            state.blue_toggled,
            vec![pos_to_index((3, 3)).unwrap()].into_iter().collect()
        );
        assert_eq!(state.red_grid, red_grid);
        assert_eq!(state.green_grid, green_grid);
        assert_eq!(state.blue_grid, blue_grid);
    }

    #[test]
    fn from_empty_toml() {
        let state = SyrupState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.next_color, PrimaryColor::Red);
        assert!(state.red_toggled.is_empty());
        assert!(state.green_toggled.is_empty());
        assert!(state.blue_toggled.is_empty());
        assert_eq!(state.red_grid, INITIAL_RED_GRID.to_vec());
        assert_eq!(state.green_grid, INITIAL_GREEN_GRID.to_vec());
        assert_eq!(state.blue_grid, INITIAL_BLUE_GRID.to_vec());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = SyrupState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.next_color, PrimaryColor::Red);
        assert_eq!(
            state.red_toggled,
            SOLVED_RED_TOGGLED.iter().cloned().collect()
        );
        assert_eq!(
            state.green_toggled,
            SOLVED_GREEN_TOGGLED.iter().cloned().collect()
        );
        assert_eq!(
            state.blue_toggled,
            SOLVED_BLUE_TOGGLED.iter().cloned().collect()
        );
        assert!(state.red_grid.iter().all(|&lit| lit));
        assert!(state.green_grid.iter().all(|&lit| lit));
        assert!(state.blue_grid.iter().all(|&lit| lit));
    }

    #[test]
    fn from_invalid_toggled_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        let toggled = toml::Value::Array(
            vec![-1, 0, 20, 21]
                .into_iter()
                .map(toml::Value::Integer)
                .collect(),
        );
        table.insert(RED_TOGGLED_KEY.to_string(), toggled.clone());
        table.insert(GREEN_TOGGLED_KEY.to_string(), toggled.clone());
        table.insert(BLUE_TOGGLED_KEY.to_string(), toggled.clone());

        let state = SyrupState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Unsolved);
        assert_eq!(state.red_toggled, vec![0, 20].into_iter().collect());
        assert_eq!(state.green_toggled, vec![0, 20].into_iter().collect());
        assert_eq!(state.blue_toggled, vec![0, 20].into_iter().collect());
    }

    #[test]
    fn from_toggled_already_correct_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        let red = SOLVED_RED_TOGGLED
            .iter()
            .map(|&t| toml::Value::Integer(t as i64))
            .collect();
        table.insert(RED_TOGGLED_KEY.to_string(), toml::Value::Array(red));
        let green = SOLVED_GREEN_TOGGLED
            .iter()
            .map(|&t| toml::Value::Integer(t as i64))
            .collect();
        table.insert(GREEN_TOGGLED_KEY.to_string(), toml::Value::Array(green));
        let blue = SOLVED_BLUE_TOGGLED
            .iter()
            .map(|&t| toml::Value::Integer(t as i64))
            .collect();
        table.insert(BLUE_TOGGLED_KEY.to_string(), toml::Value::Array(blue));

        let state = SyrupState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert!(state.red_grid.iter().all(|&lit| lit));
        assert!(state.green_grid.iter().all(|&lit| lit));
        assert!(state.blue_grid.iter().all(|&lit| lit));
    }
}

// ========================================================================= //
