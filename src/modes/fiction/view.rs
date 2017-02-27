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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources,
          Sound, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{FictionState, Game, PuzzleState};
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
    pub fn new(resources: &mut Resources, visible: Rect,
               state: &FictionState)
               -> View {
        let intro = scenes::compile_intro_scene(resources);
        let outro = scenes::compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            buttons: vec![TransformButton::new(resources, state, 0),
                          TransformButton::new(resources, state, 1),
                          TransformButton::new(resources, state, 2),
                          TransformButton::new(resources, state, 3),
                          TransformButton::new(resources, state, 4),
                          TransformButton::new(resources, state, 5)],
            letters: LettersView::new(resources, state),
            retry_countdown: 0,
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.core.drain_queue() {
            // TODO: drain queue
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.fact_or_fiction;
        self.core.draw_back_layer(canvas);
        self.buttons.draw(state, canvas);
        self.letters.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.fact_or_fiction;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if event == &Event::ClockTick && self.retry_countdown > 0 {
            self.retry_countdown -= 1;
            if self.retry_countdown == 0 {
                state.set_sequence(Vec::new());
                self.letters.reset(state);
                action = action.and_play_sound(Sound::talk_annoyed_hi());
                action.merge(Action::redraw());
            }
        }
        if !action.should_stop() {
            action.merge(self.letters.handle_event(event, state));
        }
        if !action.should_stop() {
            let subaction = self.buttons.handle_event(event, state);
            if let Some(&index) = subaction.value() {
                state.append(index);
                match index {
                    0 => self.letters.hilight_full(),
                    1 => self.letters.hilight_halves(),
                    5 => self.letters.hilight_all(),
                    _ => self.letters.hilight_changed_letters(state),
                }
                if state.is_solved() {
                    self.core.begin_outro_scene();
                } else {
                    self.core.push_undo(state.sequence().clone());
                    let sound = Sound::transform_step(state.sequence().len());
                    action = action.and_play_sound(sound);
                    if state.sequence().len() == 6 {
                        self.retry_countdown = RETRY_DELAY;
                    }
                }
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.fact_or_fiction.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(mut seq) = self.core.pop_undo() {
            seq.pop();
            let state = &mut game.fact_or_fiction;
            state.set_sequence(seq);
            self.letters.reset(state);
            self.retry_countdown = 0;
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(seq) = self.core.pop_redo() {
            let state = &mut game.fact_or_fiction;
            state.set_sequence(seq);
            self.letters.reset(state);
            self.retry_countdown = if state.sequence().len() == 6 {
                RETRY_DELAY
            } else {
                0
            };
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        let state = &mut game.fact_or_fiction;
        state.reset();
        self.letters.reset(state);
        self.retry_countdown = 0;
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.fact_or_fiction;
        state.solve();
        self.letters.reset(state);
        self.core.begin_outro_scene();
        self.drain_queue();
        self.retry_countdown = 0;
    }
}

// ========================================================================= //

const ANIM_MIN: i32 = 0;
const ANIM_MAX: i32 = 16;
const ANIM_STEP: i32 = 6;

struct TransformButton {
    sprites: Vec<Sprite>,
    index: i8,
    anim: i32,
}

impl TransformButton {
    fn new(resources: &mut Resources, state: &FictionState, index: i8)
           -> TransformButton {
        TransformButton {
            sprites: resources.get_sprites("factor/fiction"),
            index: index,
            anim: if state.has_used(index) {
                ANIM_MAX
            } else {
                ANIM_MIN
            },
        }
    }

    fn rect(&self) -> Rect {
        let index = self.index as i32;
        Rect::new(96 + 336 * (index % 2), 192 + 48 * (index / 2), 64, 32)
    }
}

impl Element<FictionState, i8> for TransformButton {
    fn draw(&self, _state: &FictionState, canvas: &mut Canvas) {
        if self.anim < ANIM_MAX {
            let rect = self.rect();
            let rect = Rect::new(rect.x(),
                                 rect.y() + self.anim,
                                 rect.width(),
                                 rect.height() - 2 * self.anim as u32);
            let mut canvas = canvas.subcanvas(rect);
            canvas.draw_sprite(&self.sprites[self.index as usize],
                               Point::new(0, -self.anim));
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut FictionState)
                    -> Action<i8> {
        match event {
            &Event::ClockTick => {
                if state.has_used(self.index) {
                    if self.anim < ANIM_MAX {
                        self.anim = min(ANIM_MAX, self.anim + ANIM_STEP);
                        return Action::redraw();
                    }
                } else {
                    if self.anim > ANIM_MIN {
                        self.anim = max(ANIM_MIN, self.anim - ANIM_STEP);
                        return Action::redraw();
                    }
                }
                Action::ignore()
            }
            &Event::MouseDown(pt) if self.rect().contains(pt) &&
                                     !state.has_used(self.index) => {
                Action::redraw().and_return(self.index)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const HILIGHT_FRAMES: i32 = 7;

struct LettersView {
    font: Rc<Font>,
    letters: Vec<char>,
    hilights: Vec<Rect>,
    countdown: i32,
}

impl LettersView {
    fn new(resources: &mut Resources, state: &FictionState) -> LettersView {
        LettersView {
            font: resources.get_font("block"),
            letters: state.letters().clone(),
            hilights: Vec::new(),
            countdown: 0,
        }
    }

    fn hilight_changed_letters(&mut self, state: &FictionState) {
        self.hilights.clear();
        for (position, &letter) in state.letters().iter().enumerate() {
            if self.letters[position] != letter {
                let rect = Rect::new(189 + 32 * position as i32, 245, 22, 22);
                self.hilights.push(rect);
            }
        }
        self.countdown = HILIGHT_FRAMES;
    }

    fn hilight_all(&mut self) {
        self.hilights.clear();
        for position in 0..7 {
            let rect = Rect::new(189 + 32 * position, 245, 22, 22);
            self.hilights.push(rect);
        }
        self.countdown = HILIGHT_FRAMES;
    }

    fn hilight_full(&mut self) {
        self.hilights.clear();
        self.hilights.push(Rect::new(189, 245, 214, 22));
        self.countdown = HILIGHT_FRAMES;
    }

    fn hilight_halves(&mut self) {
        self.hilights.clear();
        self.hilights.push(Rect::new(189, 245, 86, 22));
        self.hilights.push(Rect::new(317, 245, 86, 22));
        self.countdown = HILIGHT_FRAMES;
    }

    fn reset(&mut self, state: &FictionState) {
        self.letters = state.letters().clone();
        self.hilights.clear();
        self.countdown = 0;
    }
}

impl Element<FictionState, PuzzleCmd> for LettersView {
    fn draw(&self, _state: &FictionState, canvas: &mut Canvas) {
        for &hilight in &self.hilights {
            canvas.fill_rect((255, 255, 191), hilight);
        }
        for (position, &letter) in self.letters.iter().enumerate() {
            let pt = Point::new(200 + 32 * position as i32, 265);
            canvas.draw_char(&self.font, Align::Center, pt, letter);
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut FictionState)
                    -> Action<PuzzleCmd> {
        match event {
            &Event::ClockTick => {
                if self.countdown > 0 {
                    self.countdown -= 1;
                    if self.countdown == 0 {
                        self.hilights.clear();
                        self.letters = state.letters().clone();
                        return Action::redraw();
                    }
                }
                Action::ignore()
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to transform the starting word into a new word.
There is only one possible new word that can be formed.

$M{Tap}{Click} on one of the six buttons at the top to transform the
word.  Each button performs a different transformation.";

// ========================================================================= //
