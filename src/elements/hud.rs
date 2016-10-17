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

use gui::{Action, Align, Canvas, Element, Event, Font, GroupElement, Point,
          Rect, Resources, Sprite, SubrectElement};
use save::Location;

// ========================================================================= //

const NAMEBOX_WIDTH: u32 = 120;
const NAMEBOX_HEIGHT: u32 = 16;
const BUTTON_WIDTH: u32 = 24;
const BUTTON_HEIGHT: u32 = 24;
const BUTTON_SPACING: i32 = 3;
const MARGIN_HORZ: i32 = 6;
const MARGIN_VERT: i32 = 4;

// ========================================================================= //

pub struct HudInput {
    pub name: &'static str,
    pub can_undo: bool,
    pub can_redo: bool,
    pub can_reset: bool,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum HudCmd {
    Back,
    Info,
    Undo,
    Redo,
    Reset,
}

// ========================================================================= //

pub struct Hud {
    rect: Rect,
    elements: GroupElement<HudInput, HudCmd>,
}

impl Hud {
    pub fn new(resources: &mut Resources, visible: Rect, location: Location)
               -> Hud {
        let rect = {
            let width = (2 * MARGIN_HORZ + 5 * BUTTON_WIDTH as i32 +
                         NAMEBOX_WIDTH as i32 +
                         5 * BUTTON_SPACING) as u32;
            let height = (2 * MARGIN_VERT + BUTTON_HEIGHT as i32) as u32;
            let left = visible.left() +
                       (visible.width() as i32 - width as i32) / 2;
            let top = visible.bottom() - height as i32;
            Rect::new(left, top, width, height)
        };
        let buttons = {
            let xb1 = rect.left() + MARGIN_HORZ;
            let xb2 = xb1 + BUTTON_WIDTH as i32 + BUTTON_SPACING;
            let xnb = xb2 + BUTTON_WIDTH as i32 + BUTTON_SPACING;
            let xb3 = xnb + NAMEBOX_WIDTH as i32 + BUTTON_SPACING;
            let xb4 = xb3 + BUTTON_WIDTH as i32 + BUTTON_SPACING;
            let xb5 = xb4 + BUTTON_WIDTH as i32 + BUTTON_SPACING;
            let yb = rect.top() + MARGIN_VERT;
            let ynb = yb + (BUTTON_HEIGHT as i32 - NAMEBOX_HEIGHT as i32) / 2;
            vec![
                Hud::button(resources, location, HudCmd::Back, xb1, yb),
                Hud::button(resources, location, HudCmd::Info, xb2, yb),
                Hud::button(resources, location, HudCmd::Undo, xb3, yb),
                Hud::button(resources, location, HudCmd::Redo, xb4, yb),
                Hud::button(resources, location, HudCmd::Reset, xb5, yb),
                Hud::namebox(resources, xnb, ynb),
            ]
        };
        Hud {
            rect: rect,
            elements: GroupElement::new(buttons),
        }
    }

    fn button(resources: &mut Resources, location: Location, value: HudCmd,
              left: i32, top: i32)
              -> Box<Element<HudInput, HudCmd>> {
        let button = HudButton::new(resources, location, value);
        let rect = Rect::new(left, top, BUTTON_WIDTH, BUTTON_HEIGHT);
        Box::new(SubrectElement::new(button, rect))
    }

    fn namebox(resources: &mut Resources, left: i32, top: i32)
               -> Box<Element<HudInput, HudCmd>> {
        let namebox = HudNamebox::new(resources);
        let rect = Rect::new(left, top, NAMEBOX_WIDTH, NAMEBOX_HEIGHT);
        Box::new(SubrectElement::new(namebox, rect))
    }
}

impl Element<HudInput, HudCmd> for Hud {
    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        canvas.fill_rect((64, 64, 64), self.rect);
        self.elements.draw(input, canvas);
    }

    fn handle_event(&mut self, event: &Event, input: &mut HudInput)
                    -> Action<HudCmd> {
        self.elements.handle_event(event, input)
    }
}

// ========================================================================= //

struct HudButton {
    disabled_sprite: Sprite,
    enabled_sprite: Sprite,
    value: HudCmd,
}

impl HudButton {
    fn new(resources: &mut Resources, location: Location, value: HudCmd)
           -> HudButton {
        let sprites = resources.get_sprites("hud_buttons");
        let index = match value {
            HudCmd::Back if location == Location::Map => 1,
            HudCmd::Back => 2,
            HudCmd::Info => 3,
            HudCmd::Undo => 4,
            HudCmd::Redo => 5,
            HudCmd::Reset => 6,
        };
        HudButton {
            disabled_sprite: sprites[0].clone(),
            enabled_sprite: sprites[index].clone(),
            value: value,
        }
    }

    fn enabled(&self, input: &HudInput) -> bool {
        match self.value {
            HudCmd::Back => true,
            HudCmd::Info => true,
            HudCmd::Undo => input.can_undo,
            HudCmd::Redo => input.can_redo,
            HudCmd::Reset => input.can_reset,
        }
    }
}

impl Element<HudInput, HudCmd> for HudButton {
    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        let sprite = if self.enabled(input) {
            &self.enabled_sprite
        } else {
            &self.disabled_sprite
        };
        canvas.draw_sprite(sprite, Point::new(0, 0));
    }

    fn handle_event(&mut self, event: &Event, input: &mut HudInput)
                    -> Action<HudCmd> {
        match event {
            &Event::MouseDown(_) if self.enabled(input) => {
                Action::redraw().and_return(self.value)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct HudNamebox {
    font: Rc<Font>,
}

impl HudNamebox {
    fn new(resources: &mut Resources) -> HudNamebox {
        HudNamebox { font: resources.get_font("roman") }
    }
}

impl Element<HudInput, HudCmd> for HudNamebox {
    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        canvas.clear((200, 200, 200));
        let start = Point::new(canvas.width() as i32 / 2, 12);
        canvas.draw_text(&self.font, Align::Center, start, input.name);
    }

    fn handle_event(&mut self, _event: &Event, _input: &mut HudInput)
                    -> Action<HudCmd> {
        Action::ignore()
    }
}

// ========================================================================= //
