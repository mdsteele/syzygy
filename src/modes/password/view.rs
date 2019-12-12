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

use std::cmp::{max, min};
use std::rc::Rc;
use std::f64;

use crate::elements::{CrosswordView, FadeStyle, Paragraph, PuzzleCmd, PuzzleCore,
               PuzzleView, TalkPos};
use crate::gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sound, Sprite};
use crate::modes::SOLVED_INFO_TEXT;
use crate::save::{Game, PasswordState, PuzzleState};
use super::scenes;

// ========================================================================= //

#[derive(Clone, Copy)]
enum UndoRedo {
    Crossword(usize, i32, i32, char, char),
    Slider(i32, i32, i32),
}

// ========================================================================= //

pub struct View {
    core: PuzzleCore<UndoRedo>,
    speech_bubble: Vec<Sprite>,
    paragraphs: [(Rc<Paragraph>, TalkPos); 6],
    crosswords: [CrosswordView; 6],
    slider: PasswordSlider,
    show_crosswords: bool,
    sliders_anim: i32,
    should_display_speech: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect,
               state: &PasswordState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::TopToBottom);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_pre_sliders_scene(resources));
        if state.all_crosswords_done() && !state.is_solved() {
            core.skip_extra_scene(scenes::PRE_SLIDERS_SCENE);
        }
        let mut view = View {
            core: core,
            speech_bubble: resources.get_sprites("speech/normal"),
            paragraphs: [
                make_speech(resources, TalkPos::E, ELINSA_SPEECH),
                make_speech(resources, TalkPos::W, ARGONY_SPEECH),
                make_speech(resources, TalkPos::NE, MEZURE_SPEECH),
                make_speech(resources, TalkPos::NW, YTTRIS_SPEECH),
                make_speech(resources, TalkPos::E, UGRENT_SPEECH),
                make_speech(resources, TalkPos::W, RELYNG_SPEECH),
            ],
            crosswords: [
                CrosswordView::new(resources,
                                   (264, 116),
                                   ELINSA_OFFS,
                                   (264, 80)),
                CrosswordView::new(resources,
                                   (264, 116),
                                   ARGONY_OFFS,
                                   (264, 80)),
                CrosswordView::new(resources,
                                   (276, 116),
                                   MEZURE_OFFS,
                                   (276, 80)),
                CrosswordView::new(resources,
                                   (276, 116),
                                   YTTRIS_OFFS,
                                   (276, 80)),
                CrosswordView::new(resources,
                                   (212, 116),
                                   UGRENT_OFFS,
                                   (212, 80)),
                CrosswordView::new(resources,
                                   (276, 116),
                                   RELYNG_OFFS,
                                   (276, 80)),
            ],
            slider: PasswordSlider::new(resources),
            show_crosswords: false,
            sliders_anim: 0,
            should_display_speech: false,
        };
        for (index, crossword) in view.crosswords.iter_mut().enumerate() {
            if state.crossword_is_done(index) {
                crossword.set_center_word_hilighted(true);
            }
        }
        if state.is_visited() && !state.is_solved() {
            view.display_crossword_speech(state);
        }
        view
    }

    fn display_crossword_speech(&mut self, state: &PasswordState) -> bool {
        let theater = self.core.theater_mut();
        for other in 0..6 {
            let slot = scenes::slot_for_crossword_index(other);
            theater.clear_actor_speech(slot);
        }
        let index = state.active_index();
        if !state.crossword_is_done(index) {
            let (paragraph, talk_pos) = self.paragraphs[index].clone();
            theater.set_actor_speech(scenes::slot_for_crossword_index(index),
                                     self.speech_bubble.clone(),
                                     (255, 255, 255),
                                     talk_pos,
                                     paragraph);
            true
        } else {
            false
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.password_file;
        self.core.draw_back_layer(canvas);
        self.core.draw_middle_layer(canvas);
        if self.show_crosswords {
            let index = state.active_index();
            let crossword = state.crossword(index);
            self.crosswords[index].draw(crossword, canvas);
        }
        if self.slider.show_num_cols > 0 {
            self.slider.draw(state, canvas);
        }
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.password_file;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() && self.should_display_speech {
            self.should_display_speech = false;
            self.display_crossword_speech(state);
        }
        if !action.should_stop() && self.show_crosswords {
            let idx = state.active_index();
            if event == &Event::ClockTick || !state.crossword_is_done(idx) {
                let subaction = {
                    let crossword = state.crossword_mut(idx);
                    self.crosswords[idx].handle_event(event, crossword)
                };
                if let Some(&(row, index, chr)) = subaction.value() {
                    let old_chr = state.crossword(idx).get_char(row, index);
                    state.crossword_mut(idx).set_char(row, index, chr);
                    if state.check_crossword(idx) {
                        self.crosswords[idx].reset_cursor();
                        self.crosswords[idx].animate_center_word();
                        self.display_crossword_speech(state);
                        self.core.clear_undo_redo();
                        if state.all_crosswords_done() {
                            self.core
                                .begin_extra_scene(scenes::PRE_SLIDERS_SCENE);
                        } else {
                            let sound = Sound::solve_puzzle_chime();
                            action.also_play_sound(sound);
                        }
                    } else {
                        self.core.push_undo(UndoRedo::Crossword(idx,
                                                                row,
                                                                index,
                                                                old_chr,
                                                                chr));
                    }
                }
                action.merge(subaction.but_no_value());
            }
        }
        if !action.should_stop() && self.slider.show_num_cols >= 6 &&
            self.sliders_anim == 0
        {
            let subaction = self.slider.handle_event(event, state);
            if let Some(&(col, new_offset)) = subaction.value() {
                let old_offset = state.get_slider_offset(col);
                if new_offset != old_offset {
                    state.set_slider_offset(col, new_offset);
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                        action = action.and_return(PuzzleCmd::Save);
                    } else {
                        self.core.push_undo(UndoRedo::Slider(col,
                                                             old_offset,
                                                             new_offset));
                    }
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            match event {
                &Event::MouseDown(pt) => {
                    if !state.all_crosswords_done() {
                        let opt_index =
                            self.core
                                .theater()
                                .actor_at_point(pt)
                                .and_then(scenes::crossword_index_for_slot);
                        if let Some(index) = opt_index {
                            state.set_active_index(index);
                            self.crosswords[index].reset_cursor();
                            if self.display_crossword_speech(state) {
                                action.also_play_sound(Sound::talk_hi());
                            }
                            action.merge(Action::redraw().and_stop());
                        }
                    }
                }
                _ => {}
            }
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.password_file.is_solved() {
            SOLVED_INFO_TEXT
        } else if game.password_file.all_crosswords_done() {
            SLIDERS_INFO_BOX_TEXT
        } else {
            CROSSWORDS_INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        let state = &mut game.password_file;
        match self.core.pop_undo() {
            Some(UndoRedo::Crossword(index, row, col, old_chr, _new_chr)) => {
                state.set_active_index(index);
                state.crossword_mut(index).set_char(row, col, old_chr);
                self.crosswords[index].reset_cursor();
            }
            Some(UndoRedo::Slider(col, old_offset, _new_offset)) => {
                self.slider.drag = None;
                state.set_slider_offset(col, old_offset);
            }
            None => {}
        }
    }

    fn redo(&mut self, game: &mut Game) {
        let state = &mut game.password_file;
        match self.core.pop_redo() {
            Some(UndoRedo::Crossword(index, row, col, _old_chr, new_chr)) => {
                state.set_active_index(index);
                state.crossword_mut(index).set_char(row, col, new_chr);
                self.crosswords[index].reset_cursor();
            }
            Some(UndoRedo::Slider(col, _old_offset, new_offset)) => {
                self.slider.drag = None;
                state.set_slider_offset(col, new_offset);
            }
            None => {}
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.slider.drag = None;
        self.core.clear_undo_redo();
        game.password_file.reset();
        for crossword in &mut self.crosswords {
            crossword.reset_cursor();
        }
    }

    fn solve(&mut self, game: &mut Game) {
        self.slider.drag = None;
        for crossword in &mut self.crosswords {
            crossword.reset_cursor();
        }
        let state = &mut game.password_file;
        if !state.all_crosswords_done() {
            state.solve_all_crosswords();
            self.display_crossword_speech(state);
            self.core.begin_extra_scene(scenes::PRE_SLIDERS_SCENE);
        } else {
            state.solve();
            self.display_crossword_speech(state);
            self.core.begin_outro_scene();
        }
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.show_crosswords = value != 0;
            } else if kind == 1 {
                self.should_display_speech = value != 0;
            } else if kind == 2 {
                self.slider.show_num_cols = value.max(0).min(6);
            } else if kind == 3 {
                self.slider.glow_anim =
                    if value != 0 { GLOW_ANIM_FRAMES } else { 0 };
            }
        }
    }
}

// ========================================================================= //

const BOX_USIZE: u32 = 24;
const BOX_SIZE: i32 = BOX_USIZE as i32;
const GLOW_ANIM_FRAMES: i32 = 50;
const SLIDER_LEFT: i32 = 216;
const SLIDER_TOP: i32 = 178;
const SLIDER_WORDS: [&str; 6] =
    ["ELINSA", "ARGONY", "MEZURE", "YTTRIS", "UGRENT", "RELYNG"];

struct SliderDrag {
    col: i32,
    from_y: i32,
    to_y: i32,
}

struct PasswordSlider {
    font: Rc<Font>,
    drag: Option<SliderDrag>,
    show_num_cols: i32,
    glow_anim: i32,
}

impl PasswordSlider {
    fn new(resources: &mut Resources) -> PasswordSlider {
        PasswordSlider {
            font: resources.get_font("block"),
            drag: None,
            show_num_cols: 0,
            glow_anim: 0,
        }
    }

    fn get_slider_rect(&self, state: &PasswordState, col: i32) -> Rect {
        Rect::new(SLIDER_LEFT + BOX_SIZE * col,
                  SLIDER_TOP + BOX_SIZE * state.get_slider_offset(col),
                  BOX_USIZE,
                  BOX_USIZE * 6)
    }
}

impl Element<PasswordState, (i32, i32)> for PasswordSlider {
    fn draw(&self, state: &PasswordState, canvas: &mut Canvas) {
        let hilight_color = {
            let glow = ((self.glow_anim as f64) * f64::consts::PI /
                            (GLOW_ANIM_FRAMES as f64))
                .sin();
            (63 + (192.0 * glow) as u8, 31, 63)
        };
        for col in 0..self.show_num_cols {
            let rect = self.get_slider_rect(state, col);
            let left = rect.left();
            let mut word_top = rect.top();
            if let Some(ref drag) = self.drag {
                if col == drag.col {
                    word_top += drag.to_y - drag.from_y;
                    word_top = max(SLIDER_TOP - 5 * BOX_SIZE,
                                   min(SLIDER_TOP, word_top));
                }
            }
            let word = SLIDER_WORDS[col as usize];
            for (index, chr) in word.chars().enumerate() {
                let index = index as i32;
                let top = word_top + BOX_SIZE * index;
                let rect = Rect::new(left, top, BOX_USIZE + 1, BOX_USIZE + 1);
                let color = if index == -state.get_slider_offset(col) {
                    match self.drag {
                        Some(ref drag) if col == drag.col => (0, 0, 0),
                        _ => hilight_color,
                    }
                } else {
                    (0, 0, 0)
                };
                canvas.fill_rect(color, rect);
                let pt = Point::new(left + BOX_SIZE / 2, top + BOX_SIZE - 3);
                let mut string = String::new();
                string.push(chr);
                canvas.draw_text(&self.font, Align::Center, pt, &string);
                canvas.draw_rect((255, 255, 255), rect);
            }
        }
        if self.show_num_cols >= 6 {
            canvas.draw_rect((255, 128, 128),
                             Rect::new(SLIDER_LEFT - 1,
                                       SLIDER_TOP - 1,
                                       6 * BOX_USIZE + 3,
                                       BOX_USIZE + 3));
            canvas.draw_rect((192, 64, 64),
                             Rect::new(SLIDER_LEFT - 2,
                                       SLIDER_TOP - 2,
                                       6 * BOX_USIZE + 5,
                                       BOX_USIZE + 5));
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut PasswordState)
                    -> Action<(i32, i32)> {
        match event {
            &Event::ClockTick => {
                if self.glow_anim > 0 {
                    self.glow_anim -= 1;
                    return Action::redraw();
                }
            }
            &Event::MouseDown(pt) if !state.is_solved() => {
                for col in 0..6 {
                    if self.get_slider_rect(state, col).contains_point(pt) {
                        self.drag = Some(SliderDrag {
                                             col: col,
                                             from_y: pt.y(),
                                             to_y: pt.y(),
                                         });
                        return Action::redraw()
                            .and_play_sound(Sound::device_pickup());
                    }
                }
            }
            &Event::MouseDrag(pt) => {
                if let Some(ref mut drag) = self.drag {
                    drag.to_y = pt.y();
                    return Action::redraw();
                }
            }
            &Event::MouseUp => {
                if let Some(drag) = self.drag.take() {
                    let old_offset = state.get_slider_offset(drag.col);
                    let delta = ((drag.to_y - drag.from_y) as f64 /
                                     BOX_SIZE as f64)
                        .round() as i32;
                    let new_offset = max(-5, min(0, old_offset + delta));
                    return Action::redraw()
                        .and_play_sound(Sound::device_drop())
                        .and_return((drag.col, new_offset));
                }
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //

const ELINSA_SPEECH: &str = "\
If you don't do something yourself, it's not
like you can expect it to work out well.";
const ARGONY_SPEECH: &str = "\
Those who don't learn from the past won't
have much luck in the future either.";
const MEZURE_SPEECH: &str = "\
I may be new around here, but I'm committed
to helping us all to get through this.";
const YTTRIS_SPEECH: &str = "\
I feel frightened, yet inspired!
Who knows what will happen next?";
const UGRENT_SPEECH: &str = "\
Don't get sloppy, or you're
going to endanger us all.";
const RELYNG_SPEECH: &str = "\
Sometimes you have to hide your
own cards to uncover the truth.";

fn make_speech(resources: &mut Resources, pos: TalkPos, text: &str)
               -> (Rc<Paragraph>, TalkPos) {
    (Rc::new(Paragraph::new(resources, "roman", Align::Center, text)), pos)
}

const ELINSA_OFFS: &[(i32, &str)] =
    &[(5, ""), (5, ""), (5, ""), (6, ""), (2, ""), (4, "")];
const ARGONY_OFFS: &[(i32, &str)] =
    &[(5, ""), (2, ""), (7, ""), (4, ""), (7, ""), (6, "")];
const MEZURE_OFFS: &[(i32, &str)] =
    &[(5, ""), (1, ""), (6, ""), (1, ""), (5, ""), (4, "")];
const YTTRIS_OFFS: &[(i32, &str)] =
    &[(5, ""), (3, ""), (4, ""), (3, ""), (5, ""), (4, "")];
const UGRENT_OFFS: &[(i32, &str)] =
    &[(2, ""), (0, ""), (1, ""), (1, ""), (2, ""), (2, "")];
const RELYNG_OFFS: &[(i32, &str)] =
    &[(0, ""), (2, ""), (4, ""), (6, ""), (7, ""), (7, "")];

const CROSSWORDS_INFO_BOX_TEXT: &str = "\
Your goal is to fill in all six crosswords.

$M{Tap}{Click} on each of the six characters to reveal their crossword.
To fill in that crossword, $M{tap}{click} on a box to select it, then type
in the words that properly describe that character, using the
$M{on-screen }{}keyboard.

The words for each character are scattered around other
scenes in the game.  Be sure to check the $iSystem Failure$r
area for hints on how to find all the words.

(Note that for this puzzle, the reset button will affect only
the currently-selected crossword, not any of the others.)";

const SLIDERS_INFO_BOX_TEXT: &str = "\
Now that all six characters' crosswords have been filled in,
you must drag the six columns up and down to form the final,
six-letter password.

There are many possible words you could form this way, but
only one is correct.

(Note that at this point, the reset button will reset only these
sliders, not the already-completed crosswords.)";

// ========================================================================= //
