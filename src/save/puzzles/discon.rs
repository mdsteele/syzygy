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

use std::collections::HashMap;
use std::default::Default;
use toml;

use save::{Access, Direction};
use super::super::util::{ACCESS_KEY, pop_array, to_i32, to_table};

// ========================================================================= //

const GRID_KEY: &'static str = "grid";

const COORDS_KEY: &'static str = "coords";
const DEVICE_KEY: &'static str = "device";
const DIRECTION_KEY: &'static str = "direction";

// ========================================================================= //

pub struct DisconState {
    access: Access,
    grid: DeviceGrid,
}

impl DisconState {
    pub fn from_toml(mut table: toml::Table) -> DisconState {
        let mut state: DisconState = Default::default();
        state.access = Access::from_toml(table.get(ACCESS_KEY));
        let grid = pop_array(&mut table, GRID_KEY);
        state.grid = DeviceGrid::from_toml(grid, &state.grid);
        state
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        table.insert(GRID_KEY.to_string(), self.grid.to_toml());
        toml::Value::Table(table)
    }

    pub fn access(&self) -> Access { self.access }

    pub fn is_visited(&self) -> bool { self.access.is_visited() }

    pub fn visit(&mut self) { self.access.visit(); }

    pub fn is_solved(&self) -> bool { self.access == Access::Solved }

    pub fn mark_solved(&mut self) { self.access = Access::Solved; }

    pub fn grid(&self) -> &DeviceGrid { &self.grid }

    pub fn grid_mut(&mut self) -> &mut DeviceGrid { &mut self.grid }
}

impl Default for DisconState {
    fn default() -> DisconState {
        let mut grid = DeviceGrid::new(9, 5);
        grid.set(0, 0, Device::Emitter(LaserColor::Red), Direction::East);
        grid.set(1, 0, Device::Mirror, Direction::East);
        grid.set(3, 0, Device::Wall, Direction::East);
        grid.set(4, 0, Device::Wall, Direction::East);
        grid.set(5, 0, Device::Wall, Direction::East);
        grid.set(6, 0, Device::Mirror, Direction::South);
        grid.set(7, 0, Device::Mirror, Direction::East);
        grid.set(8, 0, Device::Detector(LaserColor::Blue), Direction::West);
        grid.set(0, 1, Device::Wall, Direction::East);
        grid.set(1, 1, Device::Wall, Direction::East);
        grid.set(2, 1, Device::Mirror, Direction::East);
        grid.set(3, 1, Device::Mirror, Direction::South);
        grid.set(8, 1, Device::Wall, Direction::East);
        grid.set(0, 2, Device::Emitter(LaserColor::Green), Direction::East);
        grid.set(2, 2, Device::Mirror, Direction::East);
        grid.set(4, 2, Device::Wall, Direction::East);
        grid.set(5, 2, Device::Wall, Direction::East);
        grid.set(7, 2, Device::Mirror, Direction::East);
        grid.set(8, 2, Device::Detector(LaserColor::Green), Direction::West);
        grid.set(0, 3, Device::Wall, Direction::East);
        grid.set(5, 3, Device::Mirror, Direction::East);
        grid.set(7, 3, Device::Wall, Direction::East);
        grid.set(8, 3, Device::Wall, Direction::East);
        grid.set(0, 4, Device::Emitter(LaserColor::Blue), Direction::East);
        grid.set(3, 4, Device::Mirror, Direction::East);
        grid.set(4, 4, Device::Channel, Direction::East);
        grid.set(6, 4, Device::Mirror, Direction::South);
        grid.set(8, 4, Device::Detector(LaserColor::Red), Direction::West);
        DisconState {
            access: Default::default(),
            grid: grid,
        }
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct DeviceGrid {
    num_cols: i32,
    num_rows: i32,
    grid: Vec<Option<(Device, Direction)>>,
}

impl DeviceGrid {
    pub fn new(num_cols: usize, num_rows: usize) -> DeviceGrid {
        DeviceGrid {
            num_cols: num_cols as i32,
            num_rows: num_rows as i32,
            grid: vec![None; num_cols * num_rows],
        }
    }

    pub fn from_toml(array: toml::Array, default: &DeviceGrid) -> DeviceGrid {
        let mut grid = default.clone();
        let mut default_device_counts: HashMap<Device, i32> = HashMap::new();
        for row in 0..grid.num_rows {
            for col in 0..grid.num_cols {
                let index = (row * grid.num_cols + col) as usize;
                if let Some((device, _)) = grid.grid[index] {
                    if device.is_moveable() {
                        *default_device_counts.entry(device).or_insert(0) += 1;
                        grid.grid[index] = None;
                    }
                }
            }
        }
        let mut actual_device_counts: HashMap<Device, i32> = HashMap::new();
        for value in array.into_iter() {
            let mut table = to_table(value);
            let mut coords = pop_array(&mut table, COORDS_KEY);
            if coords.len() != 2 {
                return default.clone();
            }
            let row = to_i32(coords.pop().unwrap());
            let col = to_i32(coords.pop().unwrap());
            if (col < 0 || col >= grid.num_cols) ||
               (row < 0 || row >= grid.num_rows) {
                return default.clone();
            }
            let index = (row * grid.num_cols + col) as usize;
            if grid.grid[index].is_some() {
                return default.clone();
            }
            let device = Device::from_toml(table.get(DEVICE_KEY));
            let dir = Direction::from_toml(table.get(DIRECTION_KEY));
            grid.grid[index] = Some((device, dir));
            *actual_device_counts.entry(device).or_insert(0) += 1;
        }
        if actual_device_counts == default_device_counts {
            grid
        } else {
            default.clone()
        }
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut array = toml::Array::new();
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let index = (row * self.num_cols + col) as usize;
                if let Some((device, dir)) = self.grid[index] {
                    if device.is_moveable() {
                        let mut table = toml::Table::new();
                        let mut coords = toml::Array::new();
                        coords.push(toml::Value::Integer(col as i64));
                        coords.push(toml::Value::Integer(row as i64));
                        table.insert(COORDS_KEY.to_string(),
                                     toml::Value::Array(coords));
                        table.insert(DEVICE_KEY.to_string(), device.to_toml());
                        table.insert(DIRECTION_KEY.to_string(), dir.to_toml());
                        array.push(toml::Value::Table(table));
                    }
                }
            }
        }
        toml::Value::Array(array)
    }

    pub fn size(&self) -> (i32, i32) { (self.num_cols, self.num_rows) }

    pub fn get(&self, col: i32, row: i32) -> Option<(Device, Direction)> {
        if col >= 0 && col < self.num_cols && row >= 0 && row < self.num_rows {
            self.grid[(row * self.num_cols + col) as usize]
        } else {
            Some((Device::Wall, Direction::East))
        }
    }

    pub fn set(&mut self, col: i32, row: i32, dev: Device, dir: Direction) {
        if col >= 0 && col < self.num_cols && row >= 0 && row < self.num_rows {
            self.grid[(row * self.num_cols + col) as usize] = Some((dev, dir));
        }
    }

    pub fn rotate(&mut self, col: i32, row: i32) {
        if col >= 0 && col < self.num_cols && row >= 0 && row < self.num_rows {
            let index = (row * self.num_cols + col) as usize;
            if let Some((device, ref mut dir)) = self.grid[index] {
                if device.is_moveable() {
                    *dir = dir.rotated_cw();
                }
            }
        }
    }

    pub fn unrotate(&mut self, col: i32, row: i32) {
        if col >= 0 && col < self.num_cols && row >= 0 && row < self.num_rows {
            let index = (row * self.num_cols + col) as usize;
            if let Some((device, ref mut dir)) = self.grid[index] {
                if device.is_moveable() {
                    *dir = dir.rotated_ccw();
                }
            }
        }
    }

    pub fn move_to(&mut self, from_col: i32, from_row: i32, to_col: i32,
                   to_row: i32) {
        if (from_col >= 0 && from_col < self.num_cols) &&
           (from_row >= 0 && from_row < self.num_rows) &&
           (to_col >= 0 && to_col < self.num_cols) &&
           (to_row >= 0 && to_row < self.num_rows) &&
           (from_col != to_col || from_row != to_row) {
            let from = (from_row * self.num_cols + from_col) as usize;
            let to = (to_row * self.num_cols + to_col) as usize;
            if let Some((dev1, dir1)) = self.grid[from] {
                if dev1.is_moveable() {
                    match self.grid[to] {
                        Some((dev2, dir2)) => {
                            if dev2.is_moveable() {
                                self.grid[from] = Some((dev2, dir2));
                                self.grid[to] = Some((dev1, dir1));
                            }
                        }
                        None => {
                            self.grid[from] = None;
                            self.grid[to] = Some((dev1, dir1));
                        }
                    }
                }
            }
        }
    }
}

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaserColor {
    Red,
    Green,
    Blue,
}

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Device {
    Wall,
    Channel,
    Emitter(LaserColor),
    Detector(LaserColor),
    Mirror,
}

impl Device {
    pub fn from_toml(value: Option<&toml::Value>) -> Device {
        if let Some(string) = value.and_then(toml::Value::as_str) {
            match string {
                "O" => return Device::Wall,
                "=" => return Device::Channel,
                "Er" => return Device::Emitter(LaserColor::Red),
                "Eg" => return Device::Emitter(LaserColor::Green),
                "Eb" => return Device::Emitter(LaserColor::Blue),
                "Dr" => return Device::Detector(LaserColor::Red),
                "Dg" => return Device::Detector(LaserColor::Green),
                "Db" => return Device::Detector(LaserColor::Blue),
                "/" => return Device::Mirror,
                _ => {}
            }
        }
        Device::Wall
    }

    pub fn to_toml(self) -> toml::Value {
        let string = match self {
            Device::Wall => "O",
            Device::Channel => "=",
            Device::Emitter(LaserColor::Red) => "Er",
            Device::Emitter(LaserColor::Green) => "Eg",
            Device::Emitter(LaserColor::Blue) => "Eb",
            Device::Detector(LaserColor::Red) => "Dr",
            Device::Detector(LaserColor::Green) => "Dg",
            Device::Detector(LaserColor::Blue) => "Db",
            Device::Mirror => "/",
        };
        toml::Value::String(string.to_string())
    }

    pub fn is_moveable(self) -> bool {
        match self {
            Device::Mirror => true,
            _ => false,
        }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use save::Direction;
    use super::{Device, DeviceGrid, LaserColor};
    use super::super::super::util::to_array;

    #[test]
    fn device_toml_round_trip() {
        let all = &[Device::Wall,
                    Device::Channel,
                    Device::Emitter(LaserColor::Red),
                    Device::Emitter(LaserColor::Green),
                    Device::Emitter(LaserColor::Blue),
                    Device::Detector(LaserColor::Red),
                    Device::Detector(LaserColor::Green),
                    Device::Detector(LaserColor::Blue),
                    Device::Mirror];
        for original in all {
            let result = Device::from_toml(Some(&original.to_toml()));
            assert_eq!(result, *original);
        }
    }

    #[test]
    fn grid_toml_round_trip() {
        let mut default = DeviceGrid::new(2, 3);
        default.set(0, 0, Device::Mirror, Direction::East);
        default.set(1, 0, Device::Mirror, Direction::South);
        default.set(2, 1, Device::Wall, Direction::East);
        let mut grid = default.clone();
        grid.move_to(1, 0, 1, 1);
        let result = DeviceGrid::from_toml(to_array(grid.to_toml()), &default);
        assert_eq!(result.grid, grid.grid);
    }
}

// ========================================================================= //