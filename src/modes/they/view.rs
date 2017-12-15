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
use elements::factor::{LettersView, TransformButton};
use gui::{Action, Canvas, Element, Event, Rect, Resources, Sound};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PuzzleState, TheYState};
use super::scenes;

// ========================================================================= //

const RETRY_DELAY: i32 = 20;

pub struct View {
    core: PuzzleCore<Vec<i8>>,
    buttons: Vec<TransformButton>,
    letters: LettersView,
    retry_countdown: i32,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &TheYState)
               -> View {
        let mut core = {
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, intro, outro)
        };
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        let buttons = resources.get_sprites("factor/they");
        let seq = state.sequence();
        View {
            core: core,
            buttons: vec![
                TransformButton::new(&buttons, 0, seq, 128, 48),
                TransformButton::new(&buttons, 1, seq, 208, 48),
                TransformButton::new(&buttons, 2, seq, 288, 48),
                TransformButton::new(&buttons, 3, seq, 128, 96),
                TransformButton::new(&buttons, 4, seq, 208, 96),
                TransformButton::new(&buttons, 5, seq, 288, 96),
            ],
            letters: LettersView::new(resources, state.letters(), 344, 206),
            retry_countdown: 0,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.the_y_factor;
        self.core.draw_back_layer(canvas);
        self.buttons.draw(state.sequence(), canvas);
        self.letters.draw(state.letters(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.the_y_factor;
        let mut action = self.core.handle_event(event, state);
        if event == &Event::ClockTick && self.retry_countdown > 0 {
            self.retry_countdown -= 1;
            if self.retry_countdown == 0 {
                state.set_sequence(Vec::new());
                self.letters.reset(state.letters());
                action.also_play_sound(Sound::talk_annoyed_hi());
                action.also_redraw();
            }
        }
        if !action.should_stop() {
            let mut letters = state.letters().clone();
            action.merge(self.letters
                             .handle_event(event, &mut letters)
                             .but_no_value());
        }
        if !action.should_stop() {
            let mut sequence = state.sequence().clone();
            let subaction = self.buttons.handle_event(event, &mut sequence);
            if let Some(&index) = subaction.value() {
                state.append(index);
                match index {
                    2 => self.letters.hilight_positions(&[1, 2, 5, 6]),
                    3 => self.letters.hilight_positions(&[2]),
                    5 => self.letters.hilight_bars(&[(0, 3), (4, 7)]),
                    _ => self.letters.hilight_changed_letters(state.letters()),
                }
                if state.is_solved() {
                    self.core.begin_outro_scene();
                } else {
                    self.core.push_undo(state.sequence().clone());
                    let sound = Sound::transform_step(state.sequence().len());
                    action.also_play_sound(sound);
                    if state.sequence().len() == 6 {
                        self.retry_countdown = RETRY_DELAY;
                    }
                }
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
        if game.the_y_factor.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(mut seq) = self.core.pop_undo() {
            seq.pop();
            let state = &mut game.the_y_factor;
            state.set_sequence(seq);
            self.letters.reset(state.letters());
            self.retry_countdown = 0;
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(seq) = self.core.pop_redo() {
            let state = &mut game.the_y_factor;
            state.set_sequence(seq);
            self.letters.reset(state.letters());
            self.retry_countdown = if state.sequence().len() == 6 {
                RETRY_DELAY
            } else {
                0
            };
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        let state = &mut game.the_y_factor;
        state.reset();
        self.letters.reset(state.letters());
        self.retry_countdown = 0;
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.the_y_factor;
        state.solve();
        self.letters.reset(state.letters());
        self.retry_countdown = 0;
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) { self.core.drain_queue().clear(); }
}

// ========================================================================= //

const INFO_BOX_TEXT: &str = "\
Your goal is to transform the starting word into a new word.
There is only one possible new word that can be formed.

$M{Tap}{Click} on one of the six buttons at the top to transform the word.
Each button performs a different transformation.

$M{Tap}{Click} on a character in the scene to hear their words of wisdom.";

// ========================================================================= //
