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

use crate::gui::{Element, Event, Window};

use super::view::View;

// ========================================================================= //

pub fn run_info_box<S, A, E: Element<S, A>>(
    window: &mut Window,
    original_view: &E,
    original_input: &mut S,
    text: &str,
) -> bool {
    let mut view = {
        let visible = window.visible_rect();
        View::new(&mut window.resources(), visible, original_view, text)
    };
    window.render(original_input, &view);
    loop {
        let mut action = match window.next_event() {
            Event::Quit => return false,
            event => view.handle_event(&event, original_input),
        };
        window.play_sounds(action.drain_sounds());
        if action.value().is_some() {
            return true;
        } else if action.should_redraw() {
            window.render(original_input, &view);
        }
    }
}

// ========================================================================= //
