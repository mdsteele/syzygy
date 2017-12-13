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

use super::action::Action;
use super::canvas::Canvas;
use super::event::Event;

// ========================================================================= //

pub trait Element<S, A> {
    fn draw(&self, state: &S, canvas: &mut Canvas);
    fn handle_event(&mut self, event: &Event, state: &mut S) -> Action<A>;
}

impl<S, A, E: Element<S, A>> Element<S, A> for Vec<E> {
    fn draw(&self, state: &S, canvas: &mut Canvas) {
        for element in self.iter().rev() {
            element.draw(state, canvas);
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut S) -> Action<A> {
        let mut action = Action::ignore();
        for element in self.iter_mut() {
            action.merge(element.handle_event(event, state));
            if action.should_stop() {
                break;
            }
        }
        action
    }
}

// ========================================================================= //
