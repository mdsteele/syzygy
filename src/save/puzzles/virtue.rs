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
use save::ice::{BlockSlide, Object, ObjectGrid, Symbol, Transform};
use save::util::{ACCESS_KEY, Tomlable, pop_table};
use super::PuzzleState;

// ========================================================================= //

const GRID_KEY: &str = "grid";

// ========================================================================= //

pub struct VirtueState {
    access: Access,
    grid: ObjectGrid,
}

impl VirtueState {
    pub fn from_toml(mut table: toml::value::Table) -> VirtueState {
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let grid = if access == Access::Solved {
            VirtueState::solved_grid()
        } else {
            let grid = pop_table(&mut table, GRID_KEY);
            ObjectGrid::from_toml(grid, &VirtueState::initial_grid())
        };
        VirtueState {
            access: access,
            grid: grid,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.grid = VirtueState::solved_grid();
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
        let red = Symbol::RedTriangle(Direction::East);
        let yellow = Symbol::YellowRhombus(true, true);
        let purple = Symbol::PurpleCheckmark(Transform::identity());
        let mut grid = ObjectGrid::new(8, 7);
        grid.add_object(7, 0, Object::Gap);
        grid.add_object(6, 1, Object::Gap);
        grid.add_object(7, 1, Object::Goal(yellow));
        grid.add_object(5, 2, Object::PushPop(Direction::North));
        grid.add_object(0, 3, Object::Goal(red));
        grid.add_object(1, 3, Object::Gap);
        grid.add_object(2, 3, Object::Reflector(true));
        grid.add_object(3, 3, Object::Gap);
        grid.add_object(4, 3, Object::Rotator);
        grid.add_object(5, 3, Object::Gap);
        grid.add_object(6, 3, Object::Gap);
        grid.add_object(7, 3, Object::Gap);
        grid.add_object(3, 4, Object::PushPop(Direction::South));
        grid.add_object(2, 5, Object::Gap);
        grid.add_object(6, 5, Object::Gap);
        grid.add_object(7, 5, Object::Goal(purple));
        grid.add_object(7, 6, Object::Gap);
        grid
    }

    fn initial_grid() -> ObjectGrid {
        let mut grid = VirtueState::base_grid();
        grid.add_ice_block(2, 0, Symbol::RedTriangle(Direction::South));
        let transform = Transform::identity().flipped_horz().rotated_cw();
        grid.add_ice_block(7, 2, Symbol::PurpleCheckmark(transform));
        grid.add_ice_block(7, 4, Symbol::YellowRhombus(false, false));
        grid
    }

    fn solved_grid() -> ObjectGrid { VirtueState::base_grid().solved() }
}

impl PuzzleState for VirtueState {
    fn location(&self) -> Location { Location::VirtueOrIce }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.grid.is_modified() }

    fn reset(&mut self) { self.grid = VirtueState::initial_grid(); }

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
