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

use elements::HudAction;
use gui::{Element, Event, Window};
use mode::Mode;
use save::{Game, Location};

use super::view::{View, Cmd};

// ========================================================================= //

pub fn run_a_light_in_the_attic(window: &mut Window, game: &mut Game) -> Mode {
    game.a_light_in_the_attic.visit();
    let mut view = {
        let visible_rect = window.visible_rect();
        View::new(&mut window.resources(),
                  visible_rect,
                  &game.a_light_in_the_attic)
    };
    window.render(game, &view);
    loop {
        let action = match window.next_event() {
            Event::Quit => return Mode::Quit,
            event => view.handle_event(&event, game),
        };
        match action.value() {
            Some(&Cmd::Hud(HudAction::Back)) => {
                return Mode::Location(Location::Map);
            }
            _ => {}
        }
        if action.should_redraw() {
            window.render(game, &view);
        }
    }
}

// ========================================================================= //
