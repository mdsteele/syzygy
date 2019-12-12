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

use crate::elements::{PuzzleCmd, PuzzleView};
use crate::gui::{Event, Window};
use crate::modes::{Mode, run_info_box};
use crate::save::{Location, SaveData};

// ========================================================================= //

pub fn run_puzzle<V: PuzzleView>(window: &mut Window,
                                 save_data: &mut SaveData, mut view: V)
                                 -> Mode {
    view.drain_queue();
    let location = {
        let game = save_data.game_mut();
        let location = game.location;
        game.puzzle_state_mut(location).revisit();
        window.render(game, &view);
        location
    };
    loop {
        let mut action = match window.next_event() {
            Event::Quit => return Mode::Quit,
            event => {
                let action = view.handle_event(&event, save_data.game_mut());
                view.drain_queue();
                action
            }
        };
        window.play_sounds(action.drain_sounds());
        match action.value() {
            Some(&PuzzleCmd::Back) => return Mode::Location(Location::Map),
            Some(&PuzzleCmd::Info) => {
                let game = save_data.game_mut();
                game.ever_clicked_info = true;
                let text = view.info_text(game);
                if !run_info_box(window, &view, game, text) {
                    return Mode::Quit;
                }
            }
            Some(&PuzzleCmd::Undo) => view.undo(save_data.game_mut()),
            Some(&PuzzleCmd::Redo) => view.redo(save_data.game_mut()),
            Some(&PuzzleCmd::Reset) => view.reset(save_data.game_mut()),
            Some(&PuzzleCmd::Replay) => {
                save_data.game_mut().puzzle_state_mut(location).replay();
                return Mode::Location(location);
            }
            Some(&PuzzleCmd::Solve) => {
                view.solve(save_data.game_mut());
                view.drain_queue();
            }
            Some(&PuzzleCmd::Next) => {
                let mut next = location.next();
                if !save_data.game_mut().is_unlocked(next) {
                    next = Location::Map;
                }
                return Mode::Location(next);
            }
            Some(&PuzzleCmd::Save) => {
                if let Err(error) = save_data.save_to_disk() {
                    println!("Failed to auto-save game: {}", error);
                }
            }
            None => {}
        }
        if action.should_redraw() {
            window.render(save_data.game_mut(), &view);
        }
    }
}

// ========================================================================= //
