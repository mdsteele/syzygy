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

use elements::{PuzzleCmd, PuzzleView};
use gui::{Event, Window};
use modes::{Mode, run_info_box};
use save::{Game, Location};

// ========================================================================= //

pub fn run_puzzle<V: PuzzleView>(window: &mut Window, game: &mut Game,
                                 mut view: V)
                                 -> Mode {
    let location = game.location;
    game.puzzle_state_mut(location).revisit();
    window.render(game, &view);
    loop {
        let mut action = match window.next_event() {
            Event::Quit => return Mode::Quit,
            event => view.handle_event(&event, game),
        };
        window.play_sounds(action.drain_sounds());
        match action.value() {
            Some(&PuzzleCmd::Back) => return Mode::Location(Location::Map),
            Some(&PuzzleCmd::Info) => {
                game.ever_clicked_info = true;
                let text = view.info_text(game);
                if !run_info_box(window, &view, game, text) {
                    return Mode::Quit;
                }
            }
            Some(&PuzzleCmd::Undo) => view.undo(game),
            Some(&PuzzleCmd::Redo) => view.redo(game),
            Some(&PuzzleCmd::Reset) => view.reset(game),
            Some(&PuzzleCmd::Replay) => {
                game.puzzle_state_mut(location).replay();
                return Mode::Location(location);
            }
            Some(&PuzzleCmd::Solve) => view.solve(game),
            Some(&PuzzleCmd::Next) => {
                let mut next = location.next();
                if !game.is_unlocked(next) {
                    next = Location::Map;
                }
                return Mode::Location(next);
            }
            None => {}
        }
        if action.should_redraw() {
            window.render(game, &view);
        }
    }
}

// ========================================================================= //
