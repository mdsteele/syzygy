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

use elements::{CrosswordView, Paragraph, PuzzleCmd, PuzzleCore, PuzzleView,
               TalkPos};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources,
          Sound, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PasswordState, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

#[derive(Clone, Copy)]
enum UndoRedo {
    Crossword(i32, i32, i32, char, char),
    Slider(i32, i32, i32),
}

// ========================================================================= //

pub struct View {
    core: PuzzleCore<UndoRedo>,
    speech_bubble: Vec<Sprite>,
    paragraphs: [(Rc<Paragraph>, TalkPos); 6],
    crosswords: [CrosswordView; 6],
    slider: PasswordSlider,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect,
               state: &PasswordState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            speech_bubble: resources.get_sprites("speech/normal"),
            paragraphs: [make_speech(resources, TalkPos::E, ELINSA_SPEECH),
                         make_speech(resources, TalkPos::W, ARGONY_SPEECH),
                         make_speech(resources, TalkPos::NE, MEZURE_SPEECH),
                         make_speech(resources, TalkPos::NW, YTTRIS_SPEECH),
                         make_speech(resources, TalkPos::E, UGRENT_SPEECH),
                         make_speech(resources, TalkPos::W, RELYNG_SPEECH)],
            crosswords: [CrosswordView::new(resources, 264, 116, ELINSA_OFFS),
                         CrosswordView::new(resources, 264, 116, ARGONY_OFFS),
                         CrosswordView::new(resources, 276, 116, MEZURE_OFFS),
                         CrosswordView::new(resources, 276, 116, YTTRIS_OFFS),
                         CrosswordView::new(resources, 212, 116, UGRENT_OFFS),
                         CrosswordView::new(resources, 276, 116, RELYNG_OFFS)],
            slider: PasswordSlider::new(resources),
        };
        for (slot, crossword) in view.crosswords.iter_mut().enumerate() {
            if state.crossword_is_done(slot as i32) {
                crossword.set_center_word_hilighted(true);
            }
        }
        view.drain_queue();
        if state.is_visited() && !state.is_solved() {
            view.display_crossword_speech(state);
        }
        view
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.core.drain_queue() {
            // TODO drain queue
        }
    }

    fn display_crossword_speech(&mut self, state: &PasswordState) -> bool {
        let theater = self.core.theater_mut();
        for other in 0..6 {
            theater.clear_actor_speech(other);
        }
        let slot = state.active_slot();
        if !state.crossword_is_done(slot) {
            let (paragraph, talk_pos) = self.paragraphs[slot as usize].clone();
            theater.set_actor_speech(slot,
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
        if state.all_crosswords_done() {
            self.slider.draw(state, canvas);
        } else {
            let slot = state.active_slot();
            self.crosswords[slot as usize].draw(state.crossword(slot), canvas);
        }
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.password_file;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() {
            if state.all_crosswords_done() {
                let subaction = self.slider.handle_event(event, state);
                if let Some(&(col, new_offset)) = subaction.value() {
                    let old_offset = state.get_slider_offset(col);
                    if new_offset != old_offset {
                        state.set_slider_offset(col, new_offset);
                        if state.is_solved() {
                            self.core.begin_outro_scene();
                        } else {
                            self.core.push_undo(UndoRedo::Slider(col,
                                                                 old_offset,
                                                                 new_offset));
                        }
                    }
                }
                action.merge(subaction.but_no_value());
            } else {
                let slot = state.active_slot();
                let idx = slot as usize;
                if event == &Event::ClockTick ||
                   !state.crossword_is_done(slot) {
                    let subaction = {
                        let crossword = state.crossword_mut(slot);
                        self.crosswords[idx].handle_event(event, crossword)
                    };
                    if let Some(&(row, index, chr)) = subaction.value() {
                        let old_chr = state.crossword(slot)
                                           .get_char(row, index);
                        state.crossword_mut(slot).set_char(row, index, chr);
                        if state.check_crossword(slot) {
                            self.crosswords[idx].reset_cursor();
                            self.crosswords[idx].animate_center_word();
                            let sound = Sound::solve_puzzle_chime();
                            action.also_play_sound(sound);
                            self.display_crossword_speech(state);
                            self.core.clear_undo_redo();
                        } else {
                            self.core.push_undo(UndoRedo::Crossword(slot,
                                                                    row,
                                                                    index,
                                                                    old_chr,
                                                                    chr));
                        }
                    }
                    action.merge(subaction.but_no_value());
                }
            }
        }
        if !action.should_stop() {
            match event {
                &Event::MouseDown(pt) => {
                    if !state.all_crosswords_done() {
                        let opt_slot = self.core.theater().actor_at_point(pt);
                        if let Some(slot) = opt_slot {
                            state.set_active_slot(slot);
                            self.crosswords[slot as usize].reset_cursor();
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
            Some(UndoRedo::Crossword(slot, row, index, old_chr, _new_chr)) => {
                state.set_active_slot(slot);
                state.crossword_mut(slot).set_char(row, index, old_chr);
                self.crosswords[slot as usize].reset_cursor();
            }
            Some(UndoRedo::Slider(col, old_offset, _new_offset)) => {
                state.set_slider_offset(col, old_offset);
            }
            None => {}
        }
    }

    fn redo(&mut self, game: &mut Game) {
        let state = &mut game.password_file;
        match self.core.pop_redo() {
            Some(UndoRedo::Crossword(slot, row, index, _old_chr, new_chr)) => {
                state.set_active_slot(slot);
                state.crossword_mut(slot).set_char(row, index, new_chr);
                self.crosswords[slot as usize].reset_cursor();
            }
            Some(UndoRedo::Slider(col, _old_offset, new_offset)) => {
                state.set_slider_offset(col, new_offset);
            }
            None => {}
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.password_file.reset();
        for crossword in &mut self.crosswords {
            crossword.reset_cursor();
        }
    }

    fn solve(&mut self, game: &mut Game) {
        game.password_file.solve();
        for crossword in &mut self.crosswords {
            crossword.reset_cursor();
        }
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const BOX_USIZE: u32 = 24;
const BOX_SIZE: i32 = BOX_USIZE as i32;
const SLIDER_LEFT: i32 = 216;
const SLIDER_TOP: i32 = 178;
const SLIDER_WORDS: [&str; 6] = ["ELINSA", "ARGONY", "MEZURE", "YTTRIS",
                                 "UGRENT", "RELYNG"];

struct SliderDrag {
    col: i32,
    from_y: i32,
    to_y: i32,
}

struct PasswordSlider {
    font: Rc<Font>,
    drag: Option<SliderDrag>,
}

impl PasswordSlider {
    fn new(resources: &mut Resources) -> PasswordSlider {
        PasswordSlider {
            font: resources.get_font("block"),
            drag: None,
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
        for col in 0..6 {
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
                        _ => (63, 31, 63),
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

    fn handle_event(&mut self, event: &Event, state: &mut PasswordState)
                    -> Action<(i32, i32)> {
        match event {
            &Event::MouseDown(pt) if !state.is_solved() => {
                for col in 0..6 {
                    if self.get_slider_rect(state, col).contains(pt) {
                        self.drag = Some(SliderDrag {
                            col: col,
                            from_y: pt.y(),
                            to_y: pt.y(),
                        });
                        return Action::redraw();
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
                                    .round() as
                                i32;
                    let new_offset = max(-5, min(0, old_offset + delta));
                    return Action::redraw().and_return((drag.col, new_offset));
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

const ELINSA_OFFS: &[(i32, &str)] = &[(5, ""), (5, ""), (5, ""), (6, ""),
                                      (2, ""), (4, "")];
const ARGONY_OFFS: &[(i32, &str)] = &[(5, ""), (2, ""), (7, ""), (4, ""),
                                      (7, ""), (6, "")];
const MEZURE_OFFS: &[(i32, &str)] = &[(5, ""), (1, ""), (6, ""), (1, ""),
                                      (5, ""), (4, "")];
const YTTRIS_OFFS: &[(i32, &str)] = &[(5, ""), (3, ""), (4, ""), (3, ""),
                                      (5, ""), (4, "")];
const UGRENT_OFFS: &[(i32, &str)] = &[(2, ""), (0, ""), (1, ""), (1, ""),
                                      (2, ""), (2, "")];
const RELYNG_OFFS: &[(i32, &str)] = &[(0, ""), (2, ""), (4, ""), (6, ""),
                                      (7, ""), (7, "")];

const CROSSWORDS_INFO_BOX_TEXT: &str = "\
Your goal is to fill in all six crosswords.

$M{Tap}{Click} on each of the six characters to reveal their crossword.
To fill in that crossword, $M{tap}{click} on a box to select it, then type
in the words that properly describe that character, using the
$M{on-screen }{}keyboard.

The words for each character are scattered around other
scenes in the game.  Be sure to check the $iSystem Failure$r area
for hints on how to find all the words.

(Note that for this puzzle, the reset button will affect only the
currently-selected crossword, not any of the others.)";

const SLIDERS_INFO_BOX_TEXT: &str = "\
Now that all six characters' crosswords have been filled in,
you must drag the six columns up and down to form the final,
six-letter password.

There are many possible words you could form this way, but
only one is correct.

(Note that at this point, the reset button will reset only these
sliders, not the already-completed crosswords.)";

// ========================================================================= //
