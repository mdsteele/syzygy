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
use super::super::save::SaveData;
use super::view::{TitleAction, TitleView};

// ========================================================================= //

pub fn run_title_screen(window: &mut Window, data: &mut SaveData) -> Mode {
    let mut view = {
        let visible_rect = window.visible_rect();
        TitleView::new(&mut window.resources(), visible_rect)
    };
    window.render(data, &view);
    loop {
        let action = match window.next_event() {
            Event::Quit => return Mode::Quit,
            event => view.handle_event(&event, data),
        };
        if let Err(error) = data.save_if_needed() {
            println!("Failed to save game: {}", error);
        }
        match action.value() {
            &Some(TitleAction::SetFullscreen(full)) => {
                window.set_fullscreen(full);
            }
            &Some(TitleAction::StartGame) => {
                if let Some(game) = data.game() {
                    return Mode::Location(game.location());
                }
                let game = data.start_new_game();
                return Mode::Location(game.location());
            }
            &Some(TitleAction::Quit) => return Mode::Quit,
            &None => {}
        }
        if action.should_redraw() {
            window.render(data, &view);
        }
    }
}

// ========================================================================= //
