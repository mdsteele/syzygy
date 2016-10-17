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

use std::marker::PhantomData;

use gui::{Action, Canvas, Element, Event, Rect, Resources};
use elements::DialogBox;

// ========================================================================= //

pub struct View<'a, A, E: 'a> {
    original_view: &'a E,
    dialog: DialogBox<()>,
    phantom: PhantomData<A>,
}

impl<'a, A, E> View<'a, A, E> {
    pub fn new(resources: &mut Resources, visible: Rect, original_view: &'a E,
               text: &str)
               -> View<'a, A, E> {
        let buttons = vec![("OK".to_string(), ())];
        let dialog = DialogBox::new(resources, visible, text, buttons);
        View {
            original_view: original_view,
            dialog: dialog,
            phantom: PhantomData,
        }
    }
}

impl<'a, S, A, E: Element<S, A>> Element<S, ()> for View<'a, A, E> {
    fn draw(&self, state: &S, canvas: &mut Canvas) {
        self.original_view.draw(state, canvas);
        self.dialog.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, _: &mut S) -> Action<()> {
        self.dialog.handle_event(event, &mut ())
    }
}

// ========================================================================= //
