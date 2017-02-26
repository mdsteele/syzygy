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

use std::ascii::AsciiExt;
use std::rc::Rc;

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Keycode, Point, Rect,
          Resources, Sound, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PuzzleState, SauceState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    input: InputDisplay,
    clue: ClueDisplay,
    arrows: Vec<ArrowButton>,
    text_timer: i32,
    text_prefix: Option<String>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &SauceState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            input: InputDisplay::new(resources),
            clue: ClueDisplay::new(resources),
            arrows: vec![ArrowButton::new(resources, false),
                         ArrowButton::new(resources, true)],
            text_timer: 0,
            text_prefix: None,
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
        let state = &game.cross_sauce;
        self.core.draw_back_layer(canvas);
        if !state.is_solved() || self.text_timer > 0 {
            self.input.draw(state, canvas);
            self.clue.draw(state, canvas);
            self.arrows.draw(state, canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.cross_sauce;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
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
                    }
                    action.merge(Action::redraw());
                }
            }
        }
        if !action.should_stop() && self.text_timer == 0 {
            let subaction = self.arrows.handle_event(event, state);
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
        if !action.should_stop() {
            action.merge(self.clue.handle_event(event, state).but_no_value());
        }
        if !action.should_stop() && self.text_timer == 0 {
            let subaction = self.input.handle_event(event, state);
            if let Some(text) = subaction.value() {
                let (prefix, error, done) = state.try_text(text);
                if done {
                    self.input.set_text(prefix);
                    self.text_timer = 20;
                    action = action.and_play_sound(Sound::mid_puzzle_chime());
                } else if error {
                    self.text_timer = 5;
                    self.text_prefix = Some(prefix);
                    action = action.and_play_sound(Sound::talk_annoyed_hi());
                } else {
                    self.input.set_text(prefix);
                }
            }
            action.merge(subaction.but_no_value());
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
        self.drain_queue();
    }
}

// ========================================================================= //

struct InputDisplay {
    font: Rc<Font>,
    text: String,
}

impl InputDisplay {
    fn new(resources: &mut Resources) -> InputDisplay {
        InputDisplay {
            font: resources.get_font("block"),
            text: String::new(),
        }
    }

    fn set_text(&mut self, text: String) { self.text = text; }

    fn clear_text(&mut self) { self.text.clear(); }
}

impl Element<SauceState, String> for InputDisplay {
    fn draw(&self, _: &SauceState, canvas: &mut Canvas) {
        if !self.text.is_empty() {
            canvas.draw_text(&self.font,
                             Align::Center,
                             Point::new(288, 281),
                             &self.text);
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut SauceState)
                    -> Action<String> {
        match event {
            &Event::TextInput(ref text) if !state.is_solved() => {
                let mut any = false;
                for chr in text.chars() {
                    if self.text.len() >= 16 {
                        break;
                    }
                    let chr = chr.to_ascii_uppercase();
                    if 'A' <= chr && chr <= 'Z' {
                        self.text.push(chr);
                        any = true;
                    }
                }
                if any {
                    Action::redraw().and_return(self.text.clone())
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct ClueDisplay {
    font: Rc<Font>,
}

impl ClueDisplay {
    fn new(resources: &mut Resources) -> ClueDisplay {
        ClueDisplay { font: resources.get_font("system") }
    }
}

impl Element<SauceState, ()> for ClueDisplay {
    fn draw(&self, state: &SauceState, canvas: &mut Canvas) {
        canvas.draw_text(&self.font,
                         Align::Center,
                         Point::new(288, 142),
                         state.current_clue());
        let num_done = state.num_clues_done();
        if num_done > 0 {
            let width = 94 * num_done / state.total_num_clues();
            canvas.fill_rect((95, 95, 95), Rect::new(241, 97, width, 14));
        }
    }

    fn handle_event(&mut self, _: &Event, _: &mut SauceState) -> Action<()> {
        Action::ignore()
    }
}

// ========================================================================= //

struct ArrowButton {
    sprites: Vec<Sprite>,
    next: bool,
    blink: i32,
}

impl ArrowButton {
    fn new(resources: &mut Resources, next: bool) -> ArrowButton {
        ArrowButton {
            sprites: resources.get_sprites("shift/arrows"),
            next: next,
            blink: 0,
        }
    }

    fn rect(&self) -> Rect {
        let left = if self.next { 464 } else { 96 };
        Rect::new(left, 128, 16, 16)
    }

    fn activate(&mut self) -> Action<bool> {
        self.blink = 3;
        Action::redraw().and_return(self.next)
    }
}

impl Element<SauceState, bool> for ArrowButton {
    fn draw(&self, _: &SauceState, canvas: &mut Canvas) {
        let mut idx = if self.next { 2 } else { 0 };
        if self.blink > 0 {
            idx += 1;
        }
        canvas.draw_sprite(&self.sprites[idx], self.rect().top_left());
    }

    fn handle_event(&mut self, event: &Event, _state: &mut SauceState)
                    -> Action<bool> {
        match event {
            &Event::ClockTick => {
                if self.blink > 0 {
                    self.blink -= 1;
                    if self.blink == 0 {
                        return Action::redraw();
                    }
                }
                Action::ignore()
            }
            &Event::MouseDown(pt) if self.rect().contains(pt) => {
                self.activate()
            }
            &Event::KeyDown(Keycode::Left, _) if !self.next => self.activate(),
            &Event::KeyDown(Keycode::Right, _) if self.next => self.activate(),
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to enter the two-word phrases
hinted at by the clues.  Each pair of words will
rhyme.

For each clue, type in the two words in order
(without any spaces or punctuation), using the
$M{on-screen }{}keyboard.

$M{Tap}{Click} the arrows to skip past a particular clue
and come back to it later.";

// ========================================================================= //
