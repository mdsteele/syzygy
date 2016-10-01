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

use std::ops::{BitOr, BitOrAssign};

// ========================================================================= //

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Action {
    redraw: bool,
    stop: bool,
}

impl Action {
    pub fn ignore() -> ActionBuilder {
        ActionBuilder { redraw: false }
    }

    pub fn redraw() -> ActionBuilder {
        ActionBuilder { redraw: true }
    }

    pub fn should_redraw(&self) -> bool {
        self.redraw
    }

    pub fn should_stop(&self) -> bool {
        self.stop
    }
}

impl BitOr for Action {
    type Output = Action;
    fn bitor(self, rhs: Action) -> Action {
        Action {
            redraw: self.redraw | rhs.redraw,
            stop: self.stop | rhs.stop,
        }
    }
}

impl BitOrAssign for Action {
    fn bitor_assign(&mut self, rhs: Action) {
        self.redraw |= rhs.redraw;
        self.stop |= rhs.stop;
    }
}

// ========================================================================= //

#[derive(Clone, Copy)]
pub struct ActionBuilder {
    redraw: bool,
}

impl ActionBuilder {
    pub fn and_continue(&self) -> Action {
        Action {
            redraw: self.redraw,
            stop: false,
        }
    }

    pub fn and_stop(&self) -> Action {
        Action {
            redraw: self.redraw,
            stop: true,
        }
    }
}

// ========================================================================= //
