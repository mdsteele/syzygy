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

use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sprite};

// ========================================================================= //

const HILIGHT_FRAMES: i32 = 7;

pub struct LettersView {
    cx: i32,
    cy: i32,
    font: Rc<Font>,
    letters: Vec<char>,
    hilights: Vec<Rect>,
    countdown: i32,
}

impl LettersView {
    pub fn new(resources: &mut Resources, letters: &Vec<char>, cx: i32,
               cy: i32)
               -> LettersView {
        LettersView {
            cx: cx,
            cy: cy,
            font: resources.get_font("block"),
            letters: letters.clone(),
            hilights: Vec::new(),
            countdown: 0,
        }
    }

    pub fn hilight_positions(&mut self, positions: &[usize]) {
        self.hilights.clear();
        for &position in positions {
            let rect = self.letter_rect(position);
            self.hilights.push(rect);
        }
        self.countdown = HILIGHT_FRAMES;
    }

    pub fn hilight_bars(&mut self, bars: &[(usize, usize)]) {
        self.hilights.clear();
        for &(from, to) in bars {
            let rect1 = self.letter_rect(from);
            let rect2 = self.letter_rect(to);
            self.hilights.push(Rect::new(rect1.left(),
                                         rect1.top(),
                                         (rect2.right() - rect1.left()) as
                                             u32,
                                         rect1.height()));
        }
        self.countdown = HILIGHT_FRAMES;
    }

    pub fn hilight_changed_letters(&mut self, new_letters: &Vec<char>) {
        self.hilights.clear();
        for (position, &letter) in new_letters.iter().enumerate() {
            if self.letters[position] != letter {
                let rect = self.letter_rect(position);
                self.hilights.push(rect);
            }
        }
        self.countdown = HILIGHT_FRAMES;
    }

    pub fn reset(&mut self, letters: &Vec<char>) {
        self.letters = letters.clone();
        self.hilights.clear();
        self.countdown = 0;
    }

    fn letter_rect(&self, index: usize) -> Rect {
        debug_assert!(index < self.letters.len());
        let left = self.cx - 16 * (self.letters.len() as i32 - 1) - 11;
        Rect::new(left + 32 * (index as i32), self.cy - 11, 22, 22)
    }
}

impl Element<Vec<char>, ()> for LettersView {
    fn draw(&self, _letters: &Vec<char>, canvas: &mut Canvas) {
        for &hilight in &self.hilights {
            canvas.fill_rect((255, 255, 191), hilight);
        }
        for (position, &letter) in self.letters.iter().enumerate() {
            let rect = self.letter_rect(position);
            let pt = Point::new(rect.x() + 11, rect.y() + 20);
            canvas.draw_char(&self.font, Align::Center, pt, letter);
        }
    }

    fn handle_event(&mut self, event: &Event, letters: &mut Vec<char>)
                    -> Action<()> {
        match event {
            &Event::ClockTick => {
                if self.countdown > 0 {
                    self.countdown -= 1;
                    if self.countdown == 0 {
                        self.hilights.clear();
                        self.letters = letters.clone();
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

const ANIM_MIN: i32 = 0;
const ANIM_MAX: i32 = 16;
const ANIM_STEP: i32 = 6;

pub struct TransformButton {
    rect: Rect,
    sprite: Sprite,
    index: i8,
    anim: i32,
}

impl TransformButton {
    pub fn new(sprites: &Vec<Sprite>, index: i8, seq: &Vec<i8>, left: i32,
               top: i32)
               -> TransformButton {
        debug_assert!(index >= 0 && (index as usize) < sprites.len());
        let sprite = sprites[index as usize].clone();
        let rect = Rect::new(left, top, sprite.width(), sprite.height());
        TransformButton {
            rect: rect,
            sprite: sprite,
            index: index,
            anim: if seq.contains(&index) {
                ANIM_MAX
            } else {
                ANIM_MIN
            },
        }
    }
}

impl Element<Vec<i8>, i8> for TransformButton {
    fn draw(&self, _seq: &Vec<i8>, canvas: &mut Canvas) {
        if self.anim < ANIM_MAX {
            let rect = Rect::new(self.rect.x(),
                                 self.rect.y() + self.anim,
                                 self.rect.width(),
                                 self.rect.height() - 2 * self.anim as u32);
            let mut canvas = canvas.subcanvas(rect);
            canvas.draw_sprite(&self.sprite, Point::new(0, -self.anim));
        }
    }

    fn handle_event(&mut self, event: &Event, seq: &mut Vec<i8>)
                    -> Action<i8> {
        match event {
            &Event::ClockTick => {
                if seq.contains(&self.index) {
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
            &Event::MouseDown(pt)
                if self.rect.contains_point(pt) &&
                       !seq.contains(&self.index) => {
                Action::redraw().and_return(self.index)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //
