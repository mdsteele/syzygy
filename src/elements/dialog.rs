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

use std::cmp;
use std::rc::Rc;

use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sprite};
use super::paragraph::Paragraph;

// ========================================================================= //

const BUTTON_WIDTH: u32 = 50;
const BUTTON_HEIGHT: u32 = 20;
const BUTTON_SPACING: i32 = 6;
const MARGIN: i32 = 20;

// ========================================================================= //

pub struct DialogBox<A> {
    rect: Rect,
    bg_sprites: Vec<Sprite>,
    paragraph: Paragraph,
    buttons: Vec<DialogButton<A>>,
}

impl<A: 'static + Clone> DialogBox<A> {
    pub fn new(resources: &mut Resources, visible: Rect, text: &str,
               buttons: Vec<(String, A)>)
               -> DialogBox<A> {
        let paragraph = Paragraph::new(resources, "roman", Align::Left, text);
        let rect = {
            let buttons_width = buttons.len() as i32 *
                (BUTTON_WIDTH as i32 + BUTTON_SPACING) -
                BUTTON_SPACING;
            let width = {
                let inner_width = cmp::max(buttons_width,
                                           paragraph.min_width());
                round_up_to_16(2 * MARGIN + inner_width)
            };
            let height = {
                round_up_to_16(2 * MARGIN + paragraph.height() as i32 +
                                   BUTTON_SPACING +
                                   BUTTON_HEIGHT as i32)
            };
            let mut rect = Rect::new(0, 0, width, height);
            rect.center_on(visible.center());
            rect
        };
        let elements = {
            let mut elements: Vec<DialogButton<A>> = Vec::new();
            let top = rect.bottom() - MARGIN - BUTTON_HEIGHT as i32;
            let mut left = rect.right() - MARGIN - BUTTON_WIDTH as i32;
            for (label, value) in buttons.into_iter().rev() {
                let rect = Rect::new(left, top, BUTTON_WIDTH, BUTTON_HEIGHT);
                let button = DialogButton::new(resources, rect, label, value);
                elements.push(button);
                left -= BUTTON_WIDTH as i32 + BUTTON_SPACING;
            }
            elements
        };
        DialogBox {
            rect: rect,
            bg_sprites: resources.get_sprites("dialog_box"),
            paragraph: paragraph,
            buttons: elements,
        }
    }
}

impl<A: 'static + Clone> Element<(), A> for DialogBox<A> {
    fn draw(&self, state: &(), canvas: &mut Canvas) {
        {
            let mut canvas = canvas.subcanvas(self.rect);
            canvas.fill_rect((200, 200, 200),
                             Rect::new(12,
                                       12,
                                       self.rect.width() - 24,
                                       self.rect.height() - 24));
            let right = self.rect.width() as i32 - 16;
            let bottom = self.rect.height() as i32 - 16;
            canvas.draw_sprite(&self.bg_sprites[0], Point::new(0, 0));
            canvas.draw_sprite(&self.bg_sprites[2], Point::new(right, 0));
            canvas.draw_sprite(&self.bg_sprites[5], Point::new(0, bottom));
            canvas.draw_sprite(&self.bg_sprites[7], Point::new(right, bottom));
            for col in 1..(right / 16) {
                let x = 16 * col;
                canvas.draw_sprite(&self.bg_sprites[1], Point::new(x, 0));
                canvas.draw_sprite(&self.bg_sprites[6], Point::new(x, bottom));
            }
            for row in 1..(bottom / 16) {
                let y = 16 * row;
                canvas.draw_sprite(&self.bg_sprites[3], Point::new(0, y));
                canvas.draw_sprite(&self.bg_sprites[4], Point::new(right, y));
            }
            {
                let rect = Rect::new(MARGIN,
                                     MARGIN,
                                     self.rect.width() - 2 * MARGIN as u32,
                                     self.rect.height() - 2 * MARGIN as u32);
                let mut canvas = canvas.subcanvas(rect);
                self.paragraph.draw(&mut canvas);
            }
        }
        self.buttons.draw(state, canvas);
    }

    fn handle_event(&mut self, event: &Event, state: &mut ()) -> Action<A> {
        self.buttons.handle_event(event, state)
    }
}

// ========================================================================= //

struct DialogButton<A> {
    sprite: Sprite,
    font: Rc<Font>,
    rect: Rect,
    label: String,
    value: A,
}

impl<A> DialogButton<A> {
    fn new(resources: &mut Resources, rect: Rect, label: String, value: A)
           -> DialogButton<A> {
        DialogButton {
            sprite: resources.get_sprites("dialog_button")[0].clone(),
            font: resources.get_font("roman"),
            rect: rect,
            label: label,
            value: value,
        }
    }
}

impl<A: Clone> Element<(), A> for DialogButton<A> {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect);
        canvas.draw_sprite(&self.sprite, Point::new(0, 0));
        let start = Point::new(self.sprite.width() as i32 / 2, 13);
        canvas.draw_text(&self.font, Align::Center, start, &self.label);
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<A> {
        match event {
            &Event::MouseDown(pt) if self.rect.contains(pt) => {
                Action::redraw().and_return(self.value.clone())
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

fn round_up_to_16(mut size: i32) -> u32 {
    let remainder = size % 16;
    if remainder != 0 {
        size += 16 - remainder;
    }
    size as u32
}

// ========================================================================= //
