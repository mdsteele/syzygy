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
use elements::plane::PlaneGridView;
use gui::{Action, Canvas, Element, Event, Rect, Resources, Sound};
use modes::SOLVED_INFO_TEXT;
use save::{DayState, Game, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    grid: PlaneGridView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &DayState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            grid: PlaneGridView::new(resources, 100, 40),
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.core.drain_queue() {
            // TODO drain queue
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.plane_as_day;
        self.core.draw_back_layer(canvas);
        self.grid.draw(state.grid(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.plane_as_day;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() && !state.is_solved() {
            let subaction = self.grid.handle_event(event, state.grid_mut());
            if let Some(&()) = subaction.value() {
                if state.advance_stage_if_done() {
                    self.core.clear_undo_redo();
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                    } else {
                        action.also_play_sound(Sound::mid_puzzle_chime());
                        // TODO animate grid changes
                    }
                } else {
                    // TODO push undo
                }
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.plane_as_day.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, _game: &mut Game) {
        if let Some(_) = self.core.pop_undo() {
            // TODO implement undo
        }
    }

    fn redo(&mut self, _game: &mut Game) {
        if let Some(_) = self.core.pop_redo() {
            // TODO implement redo
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.plane_as_day;
        self.core.clear_undo_redo();
        state.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.plane_as_day;
        state.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to connect each red node to each blue
node.

Drag across the grid with $M{your finger}{the mouse} to create or
remove pipes between the nodes.";

// ========================================================================= //
