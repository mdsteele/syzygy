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
use save::ice::{BlockSlide, Object, ObjectGrid, Symbol};
use save::util::{ACCESS_KEY, Tomlable, pop_table, to_table};
use super::PuzzleState;

// ========================================================================= //

const GRID_KEY: &str = "grid";

// ========================================================================= //

pub struct MeetState {
    access: Access,
    grid: ObjectGrid,
}

impl MeetState {
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
        let red = Symbol::RedTriangle(Direction::North);
        let mut grid = ObjectGrid::new(7, 7);
        grid.add_object(0, 0, Object::Gap);
        grid.add_object(1, 0, Object::Gap);
        grid.add_object(2, 0, Object::Gap);
        grid.add_object(0, 1, Object::Goal(red));
        grid.add_object(1, 1, Object::Gap);
        grid.add_object(4, 1, Object::Gap);
        grid.add_object(5, 1, Object::Goal(Symbol::BlueCircle));
        grid.add_object(0, 3, Object::Gap);
        grid.add_object(1, 3, Object::Gap);
        grid.add_object(2, 3, Object::Gap);
        grid.add_object(4, 3, Object::Goal(Symbol::GreenSquare));
        grid.add_object(6, 3, Object::Gap);
        grid.add_object(2, 4, Object::PushPop(Direction::South));
        grid.add_object(1, 5, Object::Gap);
        grid.add_object(4, 5, Object::Gap);
        grid.add_object(5, 5, Object::Gap);
        grid.add_object(0, 6, Object::Gap);
        grid
    }

    fn initial_grid() -> ObjectGrid {
        let mut grid = MeetState::base_grid();
        grid.add_ice_block(4, 0, Symbol::BlueCircle);
        grid.add_ice_block(0, 2, Symbol::GreenSquare);
        grid.add_ice_block(0, 4, Symbol::RedTriangle(Direction::North));
        grid
    }

    fn solved_grid() -> ObjectGrid { MeetState::base_grid().solved() }
}

impl PuzzleState for MeetState {
    fn location() -> Location { Location::IceToMeetYou }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.grid.is_modified() }

    fn reset(&mut self) { self.grid = MeetState::initial_grid(); }
}

impl Tomlable for MeetState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if self.grid.is_modified() && !self.is_solved() {
            table.insert(GRID_KEY.to_string(), self.grid.to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> MeetState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
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
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use gui::Point;
    use save::{Access, Direction};
    use save::util::{ACCESS_KEY, Tomlable};
    use super::MeetState;

    #[test]
    fn toml_round_trip() {
        let mut state = MeetState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        let slide1 = state.slide_ice_block(Point::new(0, 2), Direction::East);
        assert!(slide1.is_some());
        let coords1 = slide1.unwrap().to_coords();
        let slide2 = state.slide_ice_block(Point::new(0, 4), Direction::South);
        assert!(slide2.is_some());
        let coords2 = slide2.unwrap().to_coords();
        assert!(state.grid().is_modified());
        assert!(state.grid().ice_blocks().get(&coords1).is_some());
        let symbol1 = state.grid().ice_blocks().get(&coords1).cloned();
        assert!(state.grid().ice_blocks().get(&coords2).is_some());
        let symbol2 = state.grid().ice_blocks().get(&coords2).cloned();

        let state = MeetState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert!(state.grid().is_modified());
        assert_eq!(state.grid().ice_blocks().get(&coords1).cloned(), symbol1);
        assert_eq!(state.grid().ice_blocks().get(&coords2).cloned(), symbol2);
    }

    #[test]
    fn from_empty_toml() {
        let state = MeetState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert!(!state.grid().is_modified());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = MeetState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert!(state.grid().is_modified());
    }
}

// ========================================================================= //
