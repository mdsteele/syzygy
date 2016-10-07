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

use super::super::gui::{Element, Event, Window};
use super::super::mode::Mode;
use super::super::save::Game;
use super::view::{MapAction, MapView};

// ========================================================================= //

pub fn run_map_screen(window: &mut Window, game: &mut Game) -> Mode {
    let mut view = {
        let visible_rect = window.visible_rect();
        MapView::new(&mut window.resources(), visible_rect)
    };
    window.render(game, &view);
    loop {
        let action = match window.next_event() {
            Event::Quit => return Mode::Quit,
            event => view.handle_event(&event, game),
        };
        match action.value() {
            Some(&MapAction::ReturnToTitle) => {
                return Mode::Title;
            }
            Some(&MapAction::ShowInfoBox) => {
                // TODO: Show map screen info box
            }
            Some(&MapAction::GoToPuzzle(loc)) => {
                return Mode::Location(loc);
            }
            None => {}
        }
        if action.should_redraw() {
            window.render(game, &view);
        }
    }
}

// ========================================================================= //
