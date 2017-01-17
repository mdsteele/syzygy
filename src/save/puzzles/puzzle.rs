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

use save::{Access, Location};

// ========================================================================= //

pub trait PuzzleState {
    fn location(&self) -> Location;

    fn access(&self) -> Access;

    fn access_mut(&mut self) -> &mut Access;

    fn has_been_visited(&self) -> bool { self.access().has_been_visited() }

    fn is_visited(&self) -> bool { self.access().is_visited() }

    fn visit(&mut self) { self.access_mut().visit(); }

    fn revisit(&mut self) { self.access_mut().revisit(); }

    fn has_been_solved(&self) -> bool { self.access().has_been_solved() }

    fn is_solved(&self) -> bool { self.access().is_solved() }

    fn allow_reset_for_undo_redo(&self) -> bool { true }

    fn can_reset(&self) -> bool;

    fn reset(&mut self);

    fn replay(&mut self) {
        self.reset();
        *self.access_mut() = Access::BeginReplay;
    }

    fn to_toml(&self) -> toml::Value;
}

// ========================================================================= //
