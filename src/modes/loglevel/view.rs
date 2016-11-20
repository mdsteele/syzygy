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
use std::cmp;
use std::rc::Rc;

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Keycode, Point, Rect,
          Resources};
use modes::SOLVED_INFO_TEXT;
use save::{Game, LogLevelState, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32, char, char)>,
    crossword: Crossword,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &LogLevelState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            crossword: Crossword::new(resources),
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
        let state = &game.log_level;
        self.core.draw_back_layer(canvas);
        self.crossword.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.log_level;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() {
            let subaction = self.crossword.handle_event(event, state);
            if let Some(&(row, index, chr)) = subaction.value() {
                let old_chr = state.get_char(row, index);
                state.set_char(row, index, chr);
                if state.is_solved() {
                    self.crossword.reset_cursor();
                    self.core.begin_outro_scene();
                    self.core.clear_undo_redo();
                } else {
                    self.core.push_undo((row, index, old_chr, chr));
                }
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.log_level.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((row, index, chr, _)) = self.core.pop_undo() {
            game.log_level.set_char(row, index, chr);
            self.crossword.reset_cursor();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((row, index, _, chr)) = self.core.pop_redo() {
            game.log_level.set_char(row, index, chr);
            self.crossword.reset_cursor();
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.log_level.reset();
        self.crossword.reset_cursor();
    }

    fn replay(&mut self, game: &mut Game) {
        game.log_level.replay();
        self.crossword.reset_cursor();
        self.core.replay();
        self.drain_queue();
    }

    fn solve(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.log_level.solve();
        self.crossword.reset_cursor();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const BOX_SIZE: u32 = 24;
const CROSSWORD_TOP: i32 = 56;
const CROSSWORD_LEFT: i32 = 352;
const OFFSETS: &'static [i32] = &[5, 1, 1, 1, 1, 2, 6, 1, 2, 0];

const CLUE_TOP: i32 = CROSSWORD_TOP + 10 * BOX_SIZE as i32 + 8;
const CLUE_CENTER: i32 = CROSSWORD_LEFT + BOX_SIZE as i32 / 2;

struct Crossword {
    block_font: Rc<Font>,
    clue_font: Rc<Font>,
    cursor: Option<(i32, i32)>,
}

impl Crossword {
    fn new(resources: &mut Resources) -> Crossword {
        Crossword {
            block_font: resources.get_font("block"),
            clue_font: resources.get_font("roman"),
            cursor: None,
        }
    }

    fn reset_cursor(&mut self) { self.cursor = None; }

    fn cursor_next(&mut self, state: &LogLevelState) -> bool {
        if let Some((ref mut row, ref mut index)) = self.cursor {
            *index += 1;
            if *index >= state.words()[*row as usize].len() as i32 {
                *index = 0;
                *row += 1;
                if *row >= state.words().len() as i32 {
                    *row = 0;
                }
            }
            true
        } else {
            false
        }
    }

    fn cursor_prev(&mut self, state: &LogLevelState) -> bool {
        if let Some((ref mut row, ref mut index)) = self.cursor {
            *index -= 1;
            if *index < 0 {
                *row -= 1;
                if *row < 0 {
                    *row = state.words().len() as i32 - 1;
                }
                *index = state.words()[*row as usize].len() as i32 - 1;
            }
            true
        } else {
            false
        }
    }
}

impl Element<LogLevelState, (i32, i32, char)> for Crossword {
    fn draw(&self, state: &LogLevelState, canvas: &mut Canvas) {
        for (row, word) in state.words().iter().enumerate() {
            let top = CROSSWORD_TOP + BOX_SIZE as i32 * row as i32;
            let word_left = CROSSWORD_LEFT - BOX_SIZE as i32 * OFFSETS[row];
            for (index, &chr) in word.iter().enumerate() {
                let index = index as i32;
                let left = word_left + BOX_SIZE as i32 * index;
                let rect = Rect::new(left, top, BOX_SIZE + 1, BOX_SIZE + 1);
                let color = if Some((row as i32, index)) == self.cursor {
                    if index == OFFSETS[row] {
                        (255, 128, 255)
                    } else {
                        (192, 192, 128)
                    }
                } else {
                    if chr == ' ' {
                        (0, 0, 0)
                    } else if index == OFFSETS[row] {
                        (63, 31, 63)
                    } else {
                        (63, 63, 31)
                    }
                };
                canvas.fill_rect(color, rect);
                if chr != ' ' {
                    let pt = Point::new(left + BOX_SIZE as i32 / 2,
                                        top + BOX_SIZE as i32 - 3);
                    let mut string = String::new();
                    string.push(chr);
                    canvas.draw_text(&self.block_font,
                                     Align::Center,
                                     pt,
                                     &string);
                }
                canvas.draw_rect((255, 255, 255), rect);
            }
        }
        if let Some((row, _)) = self.cursor {
            let clue = &WORD_CLUES[row as usize];
            let width = cmp::max(0, self.clue_font.text_width(clue)) + 8;
            let rect = Rect::new(CLUE_CENTER - width / 2,
                                 CLUE_TOP,
                                 width as u32,
                                 16);
            canvas.fill_rect((192, 192, 192), rect);
            canvas.draw_rect((128, 128, 128), rect);
            let pt = Point::new(CLUE_CENTER, CLUE_TOP + 12);
            canvas.draw_text(&self.clue_font, Align::Center, pt, clue);
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut LogLevelState)
                    -> Action<(i32, i32, char)> {
        match event {
            &Event::MouseDown(pt) if !state.is_solved() => {
                let row = (pt.y() - CROSSWORD_TOP) / BOX_SIZE as i32;
                if row < 0 || row >= state.words().len() as i32 {
                    return Action::ignore();
                }
                let index = (pt.x() - CROSSWORD_LEFT +
                             BOX_SIZE as i32 * OFFSETS[row as usize]) /
                            BOX_SIZE as i32;
                if index < 0 ||
                   index >= state.words()[row as usize].len() as i32 {
                    return Action::ignore();
                }
                self.cursor = Some((row, index));
                Action::redraw().and_stop()
            }
            &Event::KeyDown(Keycode::Backspace, _) => {
                self.cursor_prev(state);
                if let Some((row, index)) = self.cursor {
                    Action::redraw().and_return((row, index, ' '))
                } else {
                    Action::ignore()
                }
            }
            &Event::KeyDown(Keycode::Left, _) => {
                Action::redraw_if(self.cursor_prev(state))
            }
            &Event::KeyDown(Keycode::Right, _) => {
                Action::redraw_if(self.cursor_next(state))
            }
            &Event::TextInput(ref text) => {
                if let Some((row, index)) = self.cursor {
                    for chr in text.chars() {
                        let chr = chr.to_ascii_uppercase();
                        if LogLevelState::is_valid_char(chr) {
                            let cmd = (row, index, chr);
                            self.cursor_next(state);
                            return Action::redraw().and_return(cmd);
                        }
                    }
                }
                Action::ignore()
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const WORD_CLUES: &'static [&'static str] = &["relaxed or casual in manner \
                                               or dress",
                                              "e.g. ``la'' in Spanish or \
                                               French",
                                              "an animated film",
                                              "very good; marvelous",
                                              "crime-solving science",
                                              "to make holes in",
                                              "coincidental; serendipitous",
                                              "the study of matter and \
                                               energy",
                                              "an ingredient in tonic water",
                                              "to separate words with \
                                               symbols"];

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to fill in the crossword.

Click on a box to select it, then type in the
word that matches the given clue, using the
$M{on-screen }{}keyboard.

If the word won't fit, you may need to get
creative.";

// ========================================================================= //
