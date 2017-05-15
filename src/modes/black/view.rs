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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Canvas, Element, Event, Rect, Resources};
use modes::SOLVED_INFO_TEXT;
use save::{BlackState, Game, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &BlackState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        View { core: core }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.black_and_blue;
        self.core.draw_back_layer(canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.black_and_blue;
        let action = self.core.handle_event(event, state);
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.black_and_blue.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, _game: &mut Game) {
        if let Some(_) = self.core.pop_undo() {
            // TODO undo
        }
    }

    fn redo(&mut self, _game: &mut Game) {
        if let Some(_) = self.core.pop_redo() {
            // TODO redo
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.black_and_blue;
        self.core.clear_undo_redo();
        state.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.black_and_blue;
        state.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.core.drain_queue() {
            // TODO drain queue
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &str = "\
Your goal is to TODO.";

// ========================================================================= //
