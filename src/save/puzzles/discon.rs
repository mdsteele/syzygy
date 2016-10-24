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

use std::default::Default;
use toml;

use gui::Point;
use save::Access;
use super::super::util::ACCESS_KEY;

// ========================================================================= //

pub struct DisconState {
    access: Access,
    grid: DeviceGrid,
}

impl DisconState {
    pub fn from_toml(table: toml::Table) -> DisconState {
        let mut state: DisconState = Default::default();
        state.access = Access::from_toml(table.get(ACCESS_KEY));
        // TODO parse grid
        state
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        // TODO serialize grid
        toml::Value::Table(table)
    }

    pub fn access(&self) -> Access { self.access }

    pub fn is_visited(&self) -> bool { self.access.is_visited() }

    pub fn visit(&mut self) { self.access.visit(); }

    pub fn is_solved(&self) -> bool { self.access == Access::Solved }

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

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum LaserColor {
    Red,
    Green,
    Blue,
}

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn degrees(self) -> i32 {
        match self {
            Direction::East => 0,
            Direction::South => 90,
            Direction::West => 180,
            Direction::North => -90,
        }
    }

    pub fn is_vertical(self) -> bool {
        match self {
            Direction::East | Direction::West => false,
            Direction::South | Direction::North => true,
        }
    }

    pub fn is_parallel_to(self, other: Direction) -> bool {
        self.is_vertical() == other.is_vertical()
    }

    pub fn delta(self) -> Point {
        match self {
            Direction::East => Point::new(1, 0),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
            Direction::North => Point::new(0, -1),
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
        }
    }

    pub fn rotated_cw(self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
}

// ========================================================================= //

#[derive(Clone, Copy)]
pub enum Device {
    Wall,
    Channel,
    Emitter(LaserColor),
    Detector(LaserColor),
    Mirror,
}

impl Device {
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
    use gui::Point;
    use super::Direction;

    const ALL_DIRECTIONS: &'static [Direction] = &[Direction::East,
                                                   Direction::South,
                                                   Direction::West,
                                                   Direction::North];

    #[test]
    fn opposites() {
        for original in ALL_DIRECTIONS {
            let opposite = original.opposite();
            assert!(original.is_parallel_to(opposite));
            assert!(opposite.is_parallel_to(*original));
            assert_eq!(opposite.opposite(), *original);
            assert_eq!(original.delta() + opposite.delta(), Point::new(0, 0));
        }
    }

    #[test]
    fn rotated() {
        for original in ALL_DIRECTIONS {
            let rotated = original.rotated_cw();
            assert!(!original.is_parallel_to(rotated));
            assert!(!rotated.is_parallel_to(*original));
            assert_eq!(rotated.rotated_cw(), original.opposite());
        }
    }
}

// ========================================================================= //
