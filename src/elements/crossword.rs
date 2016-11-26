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

use gui::{Action, Align, Canvas, Element, Event, Font, Keycode, Point, Rect,
          Resources};
use save::CrosswordState;

// ========================================================================= //

const BOX_USIZE: u32 = 24;
const BOX_SIZE: i32 = BOX_USIZE as i32;
const CLUE_MARGIN_VERT: i32 = 8;

// ========================================================================= //

pub struct CrosswordView {
    block_font: Rc<Font>,
    clue_font: Rc<Font>,
    center_x: i32,
    top: i32,
    offsets_and_clues: &'static [(i32, &'static str)],
    cursor: Option<(i32, i32)>,
}

impl CrosswordView {
    pub fn new(resources: &mut Resources, center_x: i32, top: i32,
               offsets_and_clues: &'static [(i32, &'static str)])
               -> CrosswordView {
        CrosswordView {
            block_font: resources.get_font("block"),
            clue_font: resources.get_font("roman"),
            center_x: center_x,
            top: top,
            offsets_and_clues: offsets_and_clues,
            cursor: None,
        }
    }

    pub fn reset_cursor(&mut self) { self.cursor = None; }

    fn cursor_next(&mut self, state: &CrosswordState) -> bool {
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

    fn cursor_prev(&mut self, state: &CrosswordState) -> bool {
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

impl Element<CrosswordState, (i32, i32, char)> for CrosswordView {
    fn draw(&self, state: &CrosswordState, canvas: &mut Canvas) {
        for (row, word) in state.words().iter().enumerate() {
            let top = self.top + BOX_SIZE * row as i32;
            let offset = self.offsets_and_clues[row].0;
            let word_left = self.center_x - BOX_SIZE / 2 - BOX_SIZE * offset;
            for (index, &chr) in word.iter().enumerate() {
                let index = index as i32;
                let left = word_left + BOX_SIZE * index;
                let rect = Rect::new(left, top, BOX_USIZE + 1, BOX_USIZE + 1);
                let color = if Some((row as i32, index)) == self.cursor {
                    if index == offset {
                        (255, 128, 255)
                    } else {
                        (192, 192, 128)
                    }
                } else {
                    if chr == ' ' {
                        (0, 0, 0)
                    } else if index == offset {
                        (63, 31, 63)
                    } else {
                        (63, 63, 31)
                    }
                };
                canvas.fill_rect(color, rect);
                if chr != ' ' {
                    let pt = Point::new(left + BOX_SIZE / 2,
                                        top + BOX_SIZE - 3);
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
            let clue = self.offsets_and_clues[row as usize].1;
            let width = cmp::max(0, self.clue_font.text_width(clue)) + 8;
            let rect = Rect::new(self.center_x - width / 2,
                                 self.top +
                                 BOX_SIZE * state.words().len() as i32 +
                                 CLUE_MARGIN_VERT,
                                 width as u32,
                                 17);
            canvas.fill_rect((192, 192, 192), rect);
            canvas.draw_rect((128, 128, 128), rect);
            let pt = Point::new(self.center_x, rect.top() + 12);
            canvas.draw_text(&self.clue_font, Align::Center, pt, clue);
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut CrosswordState)
                    -> Action<(i32, i32, char)> {
        match event {
            &Event::MouseDown(pt) => {
                let row = (pt.y() - self.top) / BOX_SIZE;
                if row < 0 || row >= state.words().len() as i32 {
                    return Action::ignore();
                }
                let offset = self.offsets_and_clues[row as usize].0;
                let word_left = self.center_x - BOX_SIZE / 2 -
                                BOX_SIZE * offset;
                let index = (pt.x() - word_left) / BOX_SIZE;
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
                        if state.valid_chars().contains(chr) {
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
