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
use elements::plane::{PlaneCmd, PlaneGridView};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sound};
use modes::SOLVED_INFO_TEXT;
use save::{DayState, Game, PuzzleState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<Vec<(Point, Point)>>,
    grid: PlaneGridView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &DayState)
               -> View {
        let mut core = {
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, intro, outro)
        };
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        View {
            core: core,
            grid: PlaneGridView::new(resources, 196, 52),
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
        if !action.should_stop() && !state.is_solved() {
            let mut subaction = self.grid
                .handle_event(event, state.grid_mut());
            match subaction.take_value() {
                Some(PlaneCmd::Changed) => {
                    if state.advance_stage_if_done() {
                        self.core.clear_undo_redo();
                        self.grid.cancel_drag_and_clear_changes();
                        if state.is_solved() {
                            self.core.begin_outro_scene();
                        } else {
                            action.also_play_sound(Sound::mid_puzzle_chime());
                            // TODO animate grid changes
                        }
                    }
                }
                Some(PlaneCmd::PushUndo(changes)) => {
                    self.core.push_undo(changes);
                }
                None => {}
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            self.core.begin_character_scene_on_click(event);
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

    fn undo(&mut self, game: &mut Game) {
        if let Some(changes) = self.core.pop_undo() {
            for &(coords1, coords2) in changes.iter().rev() {
                game.plane_as_day.grid_mut().toggle_pipe(coords1, coords2);
            }
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(changes) = self.core.pop_redo() {
            for &(coords1, coords2) in changes.iter() {
                game.plane_as_day.grid_mut().toggle_pipe(coords1, coords2);
            }
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.plane_as_day;
        self.core.clear_undo_redo();
        state.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.plane_as_day.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (word, letter) in self.core.drain_queue() {
            if word >= 0 && (word as usize) < WORDS.len() {
                let (col, row, letters) = WORDS[word as usize];
                if letter >= 0 && (letter as usize) < letters.len() {
                    self.grid.add_letter(Point::new(col + letter, row),
                                         letters[letter as usize]);
                }
            }
        }
    }
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const WORDS: &[(i32, i32, &[char])] = &[
    (10, 0, &['O', 'R', 'G']),
    (10, 8, &['D', 'U', 'B']),
];

const INFO_BOX_TEXT: &str = "\
Your goal is to connect each red node to each blue
node.

Drag across the grid with $M{your finger}{the mouse} to create or
remove pipes between the nodes.";

// ========================================================================= //
