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

use save::{Access, Direction, Location, MixedColor};
use save::device::{Device, DeviceGrid};
use save::util::{ACCESS_KEY, Tomlable, pop_array, to_table};
use super::PuzzleState;

// ========================================================================= //

const GRID_KEY: &str = "grid";

// ========================================================================= //

pub struct DisconState {
    access: Access,
    grid: DeviceGrid,
}

impl DisconState {
    pub fn mark_solved(&mut self) { self.access = Access::Solved; }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid = DisconState::solved_grid();
    }

    pub fn grid(&self) -> &DeviceGrid { &self.grid }

    pub fn grid_mut(&mut self) -> &mut DeviceGrid { &mut self.grid }

    fn base_grid() -> DeviceGrid {
        let mut grid = DeviceGrid::new(9, 5);
        grid.set(0, 0, Device::Emitter(MixedColor::Red), Direction::East);
        grid.set(3, 0, Device::Wall, Direction::East);
        grid.set(4, 0, Device::Wall, Direction::East);
        grid.set(5, 0, Device::Wall, Direction::East);
        grid.set(8, 0, Device::Detector(MixedColor::Blue), Direction::West);
        grid.set(0, 1, Device::Wall, Direction::East);
        grid.set(1, 1, Device::Wall, Direction::East);
        grid.set(8, 1, Device::Wall, Direction::East);
        grid.set(0, 2, Device::Emitter(MixedColor::Green), Direction::East);
        grid.set(4, 2, Device::Wall, Direction::East);
        grid.set(5, 2, Device::Wall, Direction::East);
        grid.set(8, 2, Device::Detector(MixedColor::Green), Direction::West);
        grid.set(0, 3, Device::Wall, Direction::East);
        grid.set(7, 3, Device::Wall, Direction::East);
        grid.set(8, 3, Device::Wall, Direction::East);
        grid.set(0, 4, Device::Emitter(MixedColor::Blue), Direction::East);
        grid.set(4, 4, Device::Channel, Direction::East);
        grid.set(8, 4, Device::Detector(MixedColor::Red), Direction::West);
        grid
    }

    fn initial_grid() -> DeviceGrid {
        let mut grid = DisconState::base_grid();
        grid.set(1, 0, Device::Mirror, Direction::East);
        grid.set(6, 0, Device::Mirror, Direction::South);
        grid.set(7, 0, Device::Mirror, Direction::East);
        grid.set(2, 1, Device::Mirror, Direction::East);
        grid.set(4, 1, Device::Mirror, Direction::South);
        grid.set(7, 2, Device::Mirror, Direction::East);
        grid.set(2, 3, Device::Mirror, Direction::East);
        grid.set(5, 3, Device::Mirror, Direction::East);
        grid.set(3, 4, Device::Mirror, Direction::East);
        grid.set(6, 4, Device::Mirror, Direction::South);
        grid
    }

    fn solved_grid() -> DeviceGrid {
        let mut grid = DisconState::base_grid();
        grid.set(2, 0, Device::Mirror, Direction::South);
        grid.set(6, 0, Device::Mirror, Direction::East);
        grid.set(3, 1, Device::Mirror, Direction::East);
        grid.set(7, 1, Device::Mirror, Direction::South);
        grid.set(3, 2, Device::Mirror, Direction::East);
        grid.set(7, 2, Device::Mirror, Direction::South);
        grid.set(1, 3, Device::Mirror, Direction::East);
        grid.set(6, 3, Device::Mirror, Direction::East);
        grid.set(1, 4, Device::Mirror, Direction::East);
        grid.set(2, 4, Device::Mirror, Direction::South);
        grid.set_is_modified(true);
        grid
    }
}

impl PuzzleState for DisconState {
    fn location() -> Location { Location::Disconnected }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.grid.is_modified() }

    fn reset(&mut self) { self.grid = DisconState::initial_grid(); }
}

impl Tomlable for DisconState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if self.grid.is_modified() && !self.is_solved() {
            table.insert(GRID_KEY.to_string(), self.grid.to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> DisconState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let grid = if access == Access::Solved {
            DisconState::solved_grid()
        } else {
            let grid = pop_array(&mut table, GRID_KEY);
            DeviceGrid::from_toml(grid, &DisconState::initial_grid())
        };
        DisconState {
            access: access,
            grid: grid,
        }
    }
}

// ========================================================================= //
