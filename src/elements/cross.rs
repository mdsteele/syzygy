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

use std::rc::Rc;

use crate::gui::{
    Action, Align, Canvas, Element, Event, Font, Keycode, Point, Rect,
    Resources, Sprite,
};

// ========================================================================= //

pub struct InputDisplay {
    top: i32,
    font: Rc<Font>,
    text: String,
}

impl InputDisplay {
    pub fn new(resources: &mut Resources, top: i32) -> InputDisplay {
        InputDisplay {
            top,
            font: resources.get_font("block"),
            text: String::new(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn clear_text(&mut self) {
        self.text.clear();
    }
}

impl Element<(), String> for InputDisplay {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        if !self.text.is_empty() {
            canvas.draw_text(
                &self.font,
                Align::Center,
                Point::new(288, self.top + 25),
                &self.text,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<String> {
        match event {
            &Event::TextInput(ref text) => {
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

pub struct ClueDisplay {
    top: i32,
    font: Rc<Font>,
    arrows: Vec<ArrowButton>,
    visible: bool,
}

impl ClueDisplay {
    pub fn new(resources: &mut Resources, top: i32) -> ClueDisplay {
        ClueDisplay {
            top,
            font: resources.get_font("system"),
            arrows: vec![
                ArrowButton::new(resources, false, 96, top),
                ArrowButton::new(resources, true, 464, top),
            ],
            visible: false,
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Element<&'static str, bool> for ClueDisplay {
    fn draw(&self, clue: &&'static str, canvas: &mut Canvas) {
        if self.visible {
            canvas.draw_text(
                &self.font,
                Align::Center,
                Point::new(288, self.top + 12),
                *clue,
            );
            self.arrows.draw(&(), canvas);
        }
    }

    fn handle_event(
        &mut self,
        event: &Event,
        _: &mut &'static str,
    ) -> Action<bool> {
        if self.visible {
            self.arrows.handle_event(event, &mut ())
        } else {
            Action::ignore()
        }
    }
}

// ========================================================================= //

struct ArrowButton {
    left: i32,
    top: i32,
    sprites: Vec<Sprite>,
    next: bool,
    blink: i32,
}

impl ArrowButton {
    fn new(
        resources: &mut Resources,
        next: bool,
        left: i32,
        top: i32,
    ) -> ArrowButton {
        ArrowButton {
            left,
            top,
            sprites: resources.get_sprites("shift/arrows"),
            next,
            blink: 0,
        }
    }

    fn rect(&self) -> Rect {
        Rect::new(self.left, self.top, 16, 16)
    }

    fn activate(&mut self) -> Action<bool> {
        self.blink = 3;
        Action::redraw().and_return(self.next)
    }
}

impl Element<(), bool> for ArrowButton {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        let mut idx = if self.next { 2 } else { 0 };
        if self.blink > 0 {
            idx += 1;
        }
        canvas
            .draw_sprite(&self.sprites[idx], Point::new(self.left, self.top));
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<bool> {
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
            &Event::MouseDown(pt) if self.rect().contains_point(pt) => {
                self.activate()
            }
            &Event::KeyDown(Keycode::Left, _) if !self.next => self.activate(),
            &Event::KeyDown(Keycode::Right, _) if self.next => self.activate(),
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //
