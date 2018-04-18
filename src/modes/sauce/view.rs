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

use elements::{FadeStyle, ProgressBar, PuzzleCmd, PuzzleCore, PuzzleView};
use elements::cross::{ClueDisplay, InputDisplay};
use gui::{Action, Canvas, Element, Event, Rect, Resources, Sound};
use modes::SOLVED_INFO_TEXT;
use save::{Direction, Game, PuzzleState, SauceState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    progress: ProgressBar,
    input: InputDisplay,
    clue: ClueDisplay,
    text_timer: i32,
    text_prefix: Option<String>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &SauceState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_argony_midscene(resources));
        core.add_extra_scene(scenes::compile_ugrent_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        View {
            core: core,
            progress: ProgressBar::new((240, 96),
                                       Direction::East,
                                       96,
                                       (95, 95, 95)),
            input: InputDisplay::new(resources, 256),
            clue: ClueDisplay::new(resources, 128),
            text_timer: 0,
            text_prefix: None,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.cross_sauce;
        self.core.draw_back_layer(canvas);
        self.input.draw(&(), canvas);
        if !state.is_solved() || self.text_timer > 0 {
            self.progress
                .draw(state.num_clues_done(), state.total_num_clues(), canvas);
            self.clue.draw(&state.current_clue(), canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.cross_sauce;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() && event == &Event::ClockTick {
            if self.text_timer > 0 {
                self.text_timer -= 1;
                if self.text_timer == 0 {
                    if let Some(prefix) = self.text_prefix.take() {
                        self.input.set_text(prefix);
                    } else {
                        self.input.clear_text();
                        state.go_next();
                    }
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                        action = action.and_return(PuzzleCmd::Save);
                    }
                    action.also_redraw();
                }
            }
        }
        if !action.should_stop() && self.text_timer == 0 {
            let subaction =
                self.clue.handle_event(event, &mut state.current_clue());
            if let Some(&next) = subaction.value() {
                if next {
                    state.go_next();
                    self.input.clear_text();
                } else {
                    state.go_prev();
                    self.input.clear_text();
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() && self.text_timer == 0 &&
            !state.is_solved()
        {
            let subaction = self.input.handle_event(event, &mut ());
            if let Some(text) = subaction.value() {
                let (prefix, error, done) = state.try_text(text);
                if done {
                    self.input.set_text(prefix);
                    self.text_timer = 20;
                    action.also_play_sound(Sound::mid_puzzle_chime());
                } else if error {
                    self.text_timer = 5;
                    self.text_prefix = Some(prefix);
                    action.also_play_sound(Sound::talk_annoyed_hi());
                } else {
                    self.input.set_text(prefix);
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() && self.text_timer == 0 {
            self.core.begin_character_scene_on_click(event);
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.cross_sauce.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, _: &mut Game) {}

    fn redo(&mut self, _: &mut Game) {}

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.cross_sauce.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.cross_sauce.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.clue.set_visible(value != 0);
            } else if kind == 1 {
                if value == 0 {
                    self.input.clear_text();
                } else if value == 1 {
                    self.input.set_text("  RHYME TIME  ".to_string());
                } else if value == 2 {
                    self.input.set_text("  THYME CLIMB ".to_string());
                } else if value == 3 {
                    self.input.set_text("SUBLIME ENZYME".to_string());
                } else if value == 4 {
                    self.input.set_text(" TOUGH BLUFF ".to_string());
                } else if value == 5 {
                    self.input.set_text("ENOUGH STUFF ".to_string());
                } else if value == 6 {
                    self.input.set_text(" BOUGH FLUFF?".to_string());
                } else if value == 7 {
                    self.input.set_text("F   G U    F  R".to_string());
                }
            }
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &str = "\
Your goal is to enter the two-word phrases
hinted at by the clues.  Each pair of words will
rhyme.

For each clue, type in the two words in order
(without any spaces or punctuation), using the
$M{on-screen }{}keyboard.

$M{Tap}{Click} the arrows to skip past a particular clue
and come back to it later.

$M{Tap}{Click} on a character in the scene to hear their
words of wisdom.";

// ========================================================================= //
