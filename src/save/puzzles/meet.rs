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

use gui::Point;
use save::{Access, Direction, Location};
use save::ice::{BlockSlide, Object, ObjectGrid};
use save::util::{ACCESS_KEY, pop_table};
use super::PuzzleState;

// ========================================================================= //

const GRID_KEY: &str = "grid";

// ========================================================================= //

pub struct MeetState {
    access: Access,
    grid: ObjectGrid,
}

impl MeetState {
    pub fn from_toml(mut table: toml::value::Table) -> MeetState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let grid = if access == Access::Solved {
            MeetState::solved_grid()
        } else {
            let grid = pop_table(&mut table, GRID_KEY);
            ObjectGrid::from_toml(grid, &MeetState::initial_grid())
        };
        MeetState {
            access: access,
            grid: grid,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid = MeetState::solved_grid();
    }

    pub fn grid(&self) -> &ObjectGrid { &self.grid }

    pub fn grid_mut(&mut self) -> &mut ObjectGrid { &mut self.grid }

    pub fn slide_ice_block(&mut self, coords: Point, slide_dir: Direction)
                           -> Option<BlockSlide> {
        let slide = self.grid.slide_ice_block(coords, slide_dir);
        if self.grid.all_blocks_on_goals() {
            self.access = Access::Solved;
        }
        slide
    }

    fn base_grid() -> ObjectGrid {
        let mut grid = ObjectGrid::new(7, 7);
        grid.add_object(0, 0, Object::Wall);
        grid.add_object(1, 0, Object::Wall);
        grid.add_object(2, 0, Object::Wall);
        grid.add_object(0, 1, Object::Goal(Direction::North, 0));
        grid.add_object(1, 1, Object::Wall);
        grid.add_object(4, 1, Object::Wall);
        grid.add_object(5, 1, Object::Goal(Direction::East, 2));
        grid.add_object(0, 3, Object::Wall);
        grid.add_object(1, 3, Object::Wall);
        grid.add_object(2, 3, Object::Wall);
        grid.add_object(4, 3, Object::Goal(Direction::East, 1));
        grid.add_object(6, 3, Object::Wall);
        grid.add_object(2, 4, Object::PushPop(Direction::South));
        grid.add_object(1, 5, Object::Wall);
        grid.add_object(4, 5, Object::Wall);
        grid.add_object(5, 5, Object::Wall);
        grid.add_object(0, 6, Object::Wall);
        grid
    }

    fn initial_grid() -> ObjectGrid {
        let mut grid = MeetState::base_grid();
        grid.add_ice_block(4, 0, Direction::East, 2);
        grid.add_ice_block(0, 2, Direction::East, 1);
        grid.add_ice_block(0, 4, Direction::North, 0);
        grid
    }

    fn solved_grid() -> ObjectGrid { MeetState::base_grid().solved() }
}

impl PuzzleState for MeetState {
    fn location(&self) -> Location { Location::IceToMeetYou }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.grid.is_modified() }

    fn reset(&mut self) { self.grid = MeetState::initial_grid(); }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if self.grid.is_modified() && !self.is_solved() {
            table.insert(GRID_KEY.to_string(), self.grid.to_toml());
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //
