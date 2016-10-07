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

// ========================================================================= //

pub struct Action<A> {
    redraw: bool,
    value: Option<A>,
}

impl<A> Action<A> {
    pub fn ignore() -> Action<A> {
        Action {
            redraw: false,
            value: None,
        }
    }

    pub fn redraw() -> Action<A> {
        Action {
            redraw: true,
            value: None,
        }
    }

    pub fn and_return(self, value: A) -> Action<A> {
        Action {
            redraw: self.redraw,
            value: Some(value),
        }
    }

    pub fn should_redraw(&self) -> bool { self.redraw }

    pub fn should_stop(&self) -> bool { self.value.is_some() }

    pub fn value(&self) -> Option<&A> { self.value.as_ref() }

    pub fn merge(&mut self, action: Action<A>) {
        self.redraw |= action.redraw;
        if action.value.is_some() {
            self.value = action.value;
        }
    }

    pub fn map<B, F: FnOnce(A) -> B>(self, f: F) -> Action<B> {
        Action {
            redraw: self.redraw,
            value: self.value.map(f),
        }
    }
}

// ========================================================================= //
