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
use super::super::gui::{Action, Align, Canvas, Element, Event, Font,
                        GroupElement, Point, Rect, Resources, SubrectElement};

// ========================================================================= //

const BUTTON_WIDTH: u32 = 64;
const BUTTON_HEIGHT: u32 = 24;
const BUTTON_SPACING: i32 = 12;
const LINE_SPACING: i32 = 20;
const MARGIN: i32 = 24;

// ========================================================================= //

pub struct DialogBox<A> {
    rect: Rect,
    font: Rc<Font>,
    lines: Vec<String>,
    elements: GroupElement<(), A>,
}

impl<A: 'static + Clone> DialogBox<A> {
    pub fn new(resources: &mut Resources, visible: Rect, text: &str,
               buttons: Vec<(String, A)>)
               -> DialogBox<A> {
        let font = resources.get_font("roman");
        let lines: Vec<String> = text.split('\n')
                                     .map(str::to_string)
                                     .collect();
        let rect = {
            let mut inner_width = buttons.len() as i32 *
                                  (BUTTON_WIDTH as i32 + BUTTON_SPACING) -
                                  BUTTON_SPACING;
            for line in lines.iter() {
                inner_width = cmp::max(inner_width, font.text_width(&line));
            }
            let width = (2 * MARGIN + inner_width) as u32;
            let height =
                (2 * MARGIN + LINE_SPACING * lines.len() as i32 +
                 BUTTON_SPACING + BUTTON_HEIGHT as i32) as u32;
            let mut rect = Rect::new(0, 0, width, height);
            rect.center_on(visible.center());
            rect
        };
        let elements = {
            let mut elements: Vec<Box<Element<(), A>>> = Vec::new();
            let top = rect.bottom() - MARGIN - BUTTON_HEIGHT as i32;
            let mut left = rect.right() - MARGIN - BUTTON_WIDTH as i32;
            for (label, value) in buttons.into_iter().rev() {
                let rect = Rect::new(left, top, BUTTON_WIDTH, BUTTON_HEIGHT);
                let button = DialogButton::new(resources, label, value);
                elements.push(Box::new(SubrectElement::new(button, rect)));
                left -= BUTTON_WIDTH as i32 + BUTTON_SPACING;
            }
            elements
        };
        DialogBox {
            rect: rect,
            font: font,
            lines: lines,
            elements: GroupElement::new(elements),
        }
    }
}

impl<A> Element<(), A> for DialogBox<A> {
    fn draw(&self, state: &(), canvas: &mut Canvas) {
        {
            let mut canvas = canvas.subcanvas(self.rect);
            canvas.clear((192, 128, 128));
            for (i, line) in self.lines.iter().enumerate() {
                canvas.draw_text(&self.font,
                                 Align::Left,
                                 Point::new(MARGIN,
                                            MARGIN + self.font.baseline() +
                                            LINE_SPACING * i as i32),
                                 line);
            }
        }
        self.elements.draw(state, canvas);
    }

    fn handle_event(&mut self, event: &Event, state: &mut ()) -> Action<A> {
        self.elements.handle_event(event, state)
    }
}

// ========================================================================= //

struct DialogButton<A> {
    font: Rc<Font>,
    label: String,
    value: A,
}

impl<A> DialogButton<A> {
    fn new(resources: &mut Resources, label: String, value: A)
           -> DialogButton<A> {
        DialogButton {
            font: resources.get_font("roman"),
            label: label,
            value: value,
        }
    }
}

impl<A: Clone> Element<(), A> for DialogButton<A> {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        canvas.clear((32, 192, 192));
        let (width, height) = canvas.size();
        let start = Point::new(width as i32 / 2,
                               (height as i32 - self.font.height() as i32) /
                               2 +
                               self.font.baseline());
        canvas.draw_text(&self.font, Align::Center, start, &self.label);
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<A> {
        match event {
            &Event::MouseDown(_) => {
                Action::redraw().and_return(self.value.clone())
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //
