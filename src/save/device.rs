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
use toml;

use crate::save::{Direction, MixedColor};
use crate::save::util::{Tomlable, to_table};

// ========================================================================= //

const COORDS_KEY: &str = "coords";
const DEVICE_KEY: &str = "device";
const DIRECTION_KEY: &str = "direction";

// ========================================================================= //

#[derive(Clone)]
pub struct DeviceGrid {
    num_cols: i32,
    num_rows: i32,
    grid: Vec<Option<(Device, Direction)>>,
    is_modified: bool,
}

impl DeviceGrid {
    pub fn new(num_cols: usize, num_rows: usize) -> DeviceGrid {
        DeviceGrid {
            num_cols: num_cols as i32,
            num_rows: num_rows as i32,
            grid: vec![None; num_cols * num_rows],
            is_modified: false,
        }
    }

    pub fn from_toml(array: toml::value::Array, default: &DeviceGrid)
                     -> DeviceGrid {
        let mut grid = default.clone();
        grid.is_modified = true;
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
            let coords = Vec::<i32>::pop_from_table(&mut table, COORDS_KEY);
            if coords.len() != 2 {
                return default.clone();
            }
            let col = coords[0];
            let row = coords[1];
            if (col < 0 || col >= grid.num_cols) ||
                (row < 0 || row >= grid.num_rows)
            {
                return default.clone();
            }
            let index = (row * grid.num_cols + col) as usize;
            if grid.grid[index].is_some() {
                return default.clone();
            }
            let device = Device::pop_from_table(&mut table, DEVICE_KEY);
            let dir = Direction::pop_from_table(&mut table, DIRECTION_KEY);
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
        let mut array = toml::value::Array::new();
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let index = (row * self.num_cols + col) as usize;
                if let Some((device, dir)) = self.grid[index] {
                    if device.is_moveable() {
                        let mut table = toml::value::Table::new();
                        let mut coords = toml::value::Array::new();
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

    pub fn is_modified(&self) -> bool { self.is_modified }

    pub fn set_is_modified(&mut self, is_modified: bool) {
        self.is_modified = is_modified;
    }

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

    pub fn clear_all_movable_objects(&mut self) {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let index = (row * self.num_cols + col) as usize;
                if let Some((device, _)) = self.grid[index] {
                    if device.is_moveable() {
                        self.grid[index] = None;
                    }
                }
            }
        }
    }

    pub fn rotate(&mut self, col: i32, row: i32) {
        if col >= 0 && col < self.num_cols && row >= 0 && row < self.num_rows {
            let index = (row * self.num_cols + col) as usize;
            if let Some((device, ref mut dir)) = self.grid[index] {
                if device.is_moveable() {
                    *dir = dir.rotated_cw();
                    self.is_modified = true;
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
                    self.is_modified = true;
                }
            }
        }
    }

    pub fn move_to(&mut self, from_col: i32, from_row: i32, to_col: i32,
                   to_row: i32)
                   -> bool {
        if (from_col >= 0 && from_col < self.num_cols) &&
            (from_row >= 0 && from_row < self.num_rows) &&
            (to_col >= 0 && to_col < self.num_cols) &&
            (to_row >= 0 && to_row < self.num_rows) &&
            (from_col != to_col || from_row != to_row)
        {
            let from = (from_row * self.num_cols + from_col) as usize;
            let to = (to_row * self.num_cols + to_col) as usize;
            if let Some((dev1, dir1)) = self.grid[from] {
                if dev1.is_moveable() {
                    match self.grid[to] {
                        Some((dev2, dir2)) => {
                            if dev2.is_moveable() {
                                self.grid[from] = Some((dev2, dir2));
                                self.grid[to] = Some((dev1, dir1));
                                self.is_modified = true;
                                return true;
                            }
                        }
                        None => {
                            self.grid[from] = None;
                            self.grid[to] = Some((dev1, dir1));
                            self.is_modified = true;
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Device {
    Wall,
    Channel,
    CrossChannel,
    Emitter(MixedColor),
    Detector(MixedColor),
    Mirror,
    Splitter,
    Mixer,
}

impl Device {
    pub fn is_moveable(self) -> bool {
        match self {
            Device::Mirror | Device::Splitter | Device::Mixer => true,
            _ => false,
        }
    }

    #[cfg(test)]
    pub fn all() -> Vec<Device> {
        let mut devices = vec![
            Device::Wall,
            Device::Channel,
            Device::CrossChannel,
            Device::Mirror,
            Device::Splitter,
            Device::Mixer,
        ];
        for color in MixedColor::all() {
            devices.push(Device::Emitter(color));
            devices.push(Device::Detector(color));
        }
        devices
    }
}

impl Tomlable for Device {
    fn to_toml(&self) -> toml::Value {
        let string = match *self {
            Device::Wall => "O",
            Device::Channel => "=",
            Device::CrossChannel => "+",
            Device::Emitter(MixedColor::Black) => "Ek",
            Device::Emitter(MixedColor::Red) => "Er",
            Device::Emitter(MixedColor::Green) => "Eg",
            Device::Emitter(MixedColor::Yellow) => "Ey",
            Device::Emitter(MixedColor::Blue) => "Eb",
            Device::Emitter(MixedColor::Magenta) => "Em",
            Device::Emitter(MixedColor::Cyan) => "Ec",
            Device::Emitter(MixedColor::White) => "Ew",
            Device::Detector(MixedColor::Black) => "Dk",
            Device::Detector(MixedColor::Red) => "Dr",
            Device::Detector(MixedColor::Green) => "Dg",
            Device::Detector(MixedColor::Yellow) => "Dy",
            Device::Detector(MixedColor::Blue) => "Db",
            Device::Detector(MixedColor::Magenta) => "Dm",
            Device::Detector(MixedColor::Cyan) => "Dc",
            Device::Detector(MixedColor::White) => "Dw",
            Device::Mirror => "/",
            Device::Splitter => "T",
            Device::Mixer => "M",
        };
        toml::Value::String(string.to_string())
    }

    fn from_toml(value: toml::Value) -> Device {
        if let Some(string) = value.as_str() {
            match string {
                "O" => return Device::Wall,
                "=" => return Device::Channel,
                "+" => return Device::CrossChannel,
                "Ek" => return Device::Emitter(MixedColor::Black),
                "Er" => return Device::Emitter(MixedColor::Red),
                "Eg" => return Device::Emitter(MixedColor::Green),
                "Ey" => return Device::Emitter(MixedColor::Yellow),
                "Eb" => return Device::Emitter(MixedColor::Blue),
                "Em" => return Device::Emitter(MixedColor::Magenta),
                "Ec" => return Device::Emitter(MixedColor::Cyan),
                "Ew" => return Device::Emitter(MixedColor::White),
                "Dk" => return Device::Detector(MixedColor::Black),
                "Dr" => return Device::Detector(MixedColor::Red),
                "Dg" => return Device::Detector(MixedColor::Green),
                "Dy" => return Device::Detector(MixedColor::Yellow),
                "Db" => return Device::Detector(MixedColor::Blue),
                "Dm" => return Device::Detector(MixedColor::Magenta),
                "Dc" => return Device::Detector(MixedColor::Cyan),
                "Dw" => return Device::Detector(MixedColor::White),
                "/" => return Device::Mirror,
                "T" => return Device::Splitter,
                "M" => return Device::Mixer,
                _ => {}
            }
        }
        Device::Wall
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use crate::save::Direction;
    use crate::save::util::{Tomlable, to_array};
    use super::{Device, DeviceGrid};

    #[test]
    fn device_toml_round_trip() {
        for original in Device::all() {
            let result = Device::from_toml(original.to_toml());
            assert_eq!(result, original);
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
