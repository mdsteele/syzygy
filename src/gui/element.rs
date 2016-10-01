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

#![allow(dead_code)]

use sdl2::rect::Rect;
use super::action::Action;
use super::canvas::Canvas;
use super::event::Event;

// ========================================================================= //

pub trait Element<S> {
    fn draw(&self, state: &S, canvas: &mut Canvas);
    fn handle_event(&mut self, event: &Event, state: &mut S) -> Action;
}

// ========================================================================= //

pub struct GroupElement<S> {
    elements: Vec<Box<Element<S>>>,
}

impl<S> GroupElement<S> {
    pub fn new(elements: Vec<Box<Element<S>>>) -> GroupElement<S> {
        GroupElement { elements: elements }
    }
}

impl<S> Element<S> for GroupElement<S> {
    fn draw(&self, state: &S, canvas: &mut Canvas) {
        for element in self.elements.iter().rev() {
            element.draw(state, canvas);
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut S) -> Action {
        let mut action = Action::ignore().and_continue();
        for element in self.elements.iter_mut() {
            action |= element.handle_event(event, state);
            if action.should_stop() {
                break;
            }
        }
        action
    }
}

// ========================================================================= //

pub struct SubrectElement<E> {
    subrect: Rect,
    element: E,
}

impl<E> SubrectElement<E> {
    pub fn new(element: E, subrect: Rect) -> SubrectElement<E> {
        SubrectElement {
            subrect: subrect,
            element: element,
        }
    }
}

impl<E, S> Element<S> for SubrectElement<E>
    where E: Element<S>
{
    fn draw(&self, state: &S, canvas: &mut Canvas) {
        let mut subcanvas = canvas.subcanvas(self.subrect);
        self.element.draw(state, &mut subcanvas);
    }

    fn handle_event(&mut self, event: &Event, state: &mut S) -> Action {
        match event {
            &Event::MouseDown(pt) => {
                if !self.subrect.contains(pt) {
                    return Action::ignore().and_continue();
                }
            }
            _ => {}
        }
        let dx = self.subrect.x();
        let dy = self.subrect.y();
        let event = event.translate(-dx, -dy);
        self.element.handle_event(&event, state)
    }
}

// ========================================================================= //
