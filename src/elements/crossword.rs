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

use gui::{Action, Align, Canvas, Element, Event, Font, Keycode, Point, Rect,
          Resources};
use save::CrosswordState;

// ========================================================================= //

const BOX_USIZE: u32 = 24;
const BOX_SIZE: i32 = BOX_USIZE as i32;
const ANIM_WORD_DELAY_FRAMES: i32 = 1;
const ANIM_FADE_FRAMES: i32 = 5;

// ========================================================================= //

pub struct CrosswordView {
    block_font: Rc<Font>,
    clue_font: Rc<Font>,
    crossword_center_x: i32,
    crossword_top: i32,
    offsets_and_clues: &'static [(i32, &'static str)],
    clue_center_x: i32,
    clue_top: i32,
    cursor: Option<(i32, i32)>,
    animation: Option<i32>,
}

impl CrosswordView {
    pub fn new(resources: &mut Resources,
               (crossword_center_x, crossword_top): (i32, i32),
               offsets_and_clues: &'static [(i32, &'static str)],
               (clue_center_x, clue_top): (i32, i32))
               -> CrosswordView {
        assert!(!offsets_and_clues.is_empty());
        CrosswordView {
            block_font: resources.get_font("block"),
            clue_font: resources.get_font("roman"),
            crossword_center_x: crossword_center_x,
            crossword_top: crossword_top,
            offsets_and_clues: offsets_and_clues,
            clue_center_x: clue_center_x,
            clue_top: clue_top,
            cursor: None,
            animation: None,
        }
    }

    pub fn reset_cursor(&mut self) { self.cursor = None; }

    pub fn animate_center_word(&mut self) { self.animation = Some(0); }

    pub fn set_center_word_hilighted(&mut self, hilight: bool) {
        self.animation = if hilight { Some(self.anim_max()) } else { None }
    }

    fn anim_max(&self) -> i32 {
        let num_words = self.offsets_and_clues.len() as i32;
        debug_assert!(num_words > 0);
        (num_words - 1) * ANIM_WORD_DELAY_FRAMES + ANIM_FADE_FRAMES
    }

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

    fn cursor_up(&mut self, state: &CrosswordState) -> bool {
        if let Some((ref mut row, ref mut index)) = self.cursor {
            let old_offset = self.offsets_and_clues[*row as usize].0;
            *row -= 1;
            if *row < 0 {
                *row = state.words().len() as i32 - 1;
            }
            let new_offset = self.offsets_and_clues[*row as usize].0;
            let len = state.words()[*row as usize].len() as i32 - 1;
            *index = max(0, min(len, *index - old_offset + new_offset));
            true
        } else {
            false
        }
    }

    fn cursor_down(&mut self, state: &CrosswordState) -> bool {
        if let Some((ref mut row, ref mut index)) = self.cursor {
            let old_offset = self.offsets_and_clues[*row as usize].0;
            *row += 1;
            if *row >= state.words().len() as i32 {
                *row = 0;
            }
            let new_offset = self.offsets_and_clues[*row as usize].0;
            let len = state.words()[*row as usize].len() as i32 - 1;
            *index = max(0, min(len, *index - old_offset + new_offset));
            true
        } else {
            false
        }
    }

    fn box_color(&self, row: usize, index: i32, chr: char) -> (u8, u8, u8) {
        let offset = self.offsets_and_clues[row].0;
        let row = row as i32;
        let under_cursor = Some((row, index)) == self.cursor;
        if index == offset {
            if let Some(frames) = self.animation {
                let lower = row * ANIM_WORD_DELAY_FRAMES;
                if frames >= lower {
                    let progress = min(ANIM_FADE_FRAMES, frames - lower);
                    let red_blue = 255 - progress * (144 / ANIM_FADE_FRAMES);
                    let green = 255 - progress * (224 / ANIM_FADE_FRAMES);
                    return (red_blue as u8, green as u8, red_blue as u8);
                }
            }
            if under_cursor {
                (255, 128, 255)
            } else if chr == ' ' {
                (0, 0, 0)
            } else {
                (63, 31, 63)
            }
        } else if under_cursor {
            (192, 192, 128)
        } else if chr == ' ' {
            (0, 0, 0)
        } else {
            (63, 63, 31)
        }
    }
}

impl Element<CrosswordState, (i32, i32, char)> for CrosswordView {
    fn draw(&self, state: &CrosswordState, canvas: &mut Canvas) {
        for (row, word) in state.words().iter().enumerate() {
            let top = self.crossword_top + BOX_SIZE * row as i32;
            let offset = self.offsets_and_clues[row].0;
            let word_left = self.crossword_center_x - BOX_SIZE / 2 -
                BOX_SIZE * offset;
            for (index, &chr) in word.iter().enumerate() {
                let index = index as i32;
                let left = word_left + BOX_SIZE * index;
                let rect = Rect::new(left, top, BOX_USIZE + 1, BOX_USIZE + 1);
                canvas.fill_rect(self.box_color(row, index, chr), rect);
                if chr != ' ' {
                    let pt = Point::new(left + BOX_SIZE / 2,
                                        top + BOX_SIZE - 3);
                    canvas.draw_char(&self.block_font, Align::Center, pt, chr);
                }
                canvas.draw_rect((255, 255, 255), rect);
            }
        }
        if let Some((row, _)) = self.cursor {
            let clue = self.offsets_and_clues[row as usize].1;
            if !clue.is_empty() {
                let width = max(0, self.clue_font.text_width(clue)) + 8;
                let rect = Rect::new(self.clue_center_x - width / 2,
                                     self.clue_top,
                                     width as u32,
                                     17);
                canvas.fill_rect((192, 192, 192), rect);
                canvas.draw_rect((128, 128, 128), rect);
                let pt = Point::new(self.clue_center_x, rect.top() + 12);
                canvas.draw_text(&self.clue_font, Align::Center, pt, clue);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut CrosswordState)
                    -> Action<(i32, i32, char)> {
        match event {
            &Event::ClockTick => {
                if let Some(frames) = self.animation {
                    let limit = self.anim_max();
                    if frames < limit {
                        self.animation = Some(frames + 1);
                        return Action::redraw();
                    }
                }
            }
            &Event::MouseDown(pt) => {
                let row = (pt.y() - self.crossword_top) / BOX_SIZE;
                if row < 0 || row >= state.words().len() as i32 {
                    return Action::ignore();
                }
                let offset = self.offsets_and_clues[row as usize].0;
                let word_left = self.crossword_center_x - BOX_SIZE / 2 -
                    BOX_SIZE * offset;
                let index = (pt.x() - word_left) / BOX_SIZE;
                if index < 0 ||
                    index >= state.words()[row as usize].len() as i32
                {
                    return Action::ignore();
                }
                self.cursor = Some((row, index));
                return Action::redraw().and_stop();
            }
            &Event::KeyDown(Keycode::Backspace, _) => {
                self.cursor_prev(state);
                if let Some((row, index)) = self.cursor {
                    return Action::redraw().and_return((row, index, ' '));
                }
            }
            &Event::KeyDown(Keycode::Down, _) => {
                return Action::redraw_if(self.cursor_down(state));
            }
            &Event::KeyDown(Keycode::Left, _) => {
                return Action::redraw_if(self.cursor_prev(state));
            }
            &Event::KeyDown(Keycode::Right, _) => {
                return Action::redraw_if(self.cursor_next(state));
            }
            &Event::KeyDown(Keycode::Up, _) => {
                return Action::redraw_if(self.cursor_up(state));
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
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //
