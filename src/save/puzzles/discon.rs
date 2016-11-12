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

use save::{Access, Device, DeviceGrid, Direction, LaserColor};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array};

// ========================================================================= //

const GRID_KEY: &'static str = "grid";

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
        if self.grid.is_modified() {
            table.insert(GRID_KEY.to_string(), self.grid.to_toml());
        }
        toml::Value::Table(table)
    }

    pub fn visit(&mut self) { self.access.visit(); }

    pub fn mark_solved(&mut self) { self.access = Access::Solved; }

    pub fn reset(&mut self) { self.grid = DisconState::default_grid(); }

    pub fn replay(&mut self) {
        self.access = Access::Replay;
        self.grid = DisconState::default_grid();
    }

    pub fn grid(&self) -> &DeviceGrid { &self.grid }

    pub fn grid_mut(&mut self) -> &mut DeviceGrid { &mut self.grid }

    fn default_grid() -> DeviceGrid {
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
        grid
    }
}

impl Default for DisconState {
    fn default() -> DisconState {
        DisconState {
            access: Default::default(),
            grid: DisconState::default_grid(),
        }
    }
}

impl PuzzleState for DisconState {
    fn access(&self) -> Access { self.access }

    fn can_reset(&self) -> bool { self.grid.is_modified() }
}

// ========================================================================= //
