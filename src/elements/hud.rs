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

use elements::Paragraph;
use gui::{Action, Align, Canvas, Element, Event, Font, GroupElement, Point,
          Rect, Resources, Sprite, SubrectElement};
use save::Location;

// ========================================================================= //

const BUTTON_HEIGHT: u32 = 16;
const NAMEBOX_WIDTH: u32 = 114;
const NAMEBOX_HEIGHT: u32 = 16;
const SCROLL_SPEED: i32 = 2;
const PAUSE_TEXT: &'static str = "$M{Tap}{Click} anywhere to continue";
const PAUSE_TEXT_MARGIN: i32 = 2;

// ========================================================================= //

pub struct HudInput {
    pub name: &'static str,
    pub is_paused: bool,
    pub active: bool,
    pub can_undo: bool,
    pub can_redo: bool,
    pub can_reset: bool,
    pub can_replay: bool,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum HudCmd {
    Back,
    Info,
    Undo,
    Redo,
    Reset,
    Replay,
}

// ========================================================================= //

pub struct Hud {
    elements: GroupElement<HudInput, HudCmd>,
}

impl Hud {
    pub fn new(resources: &mut Resources, visible: Rect, location: Location)
               -> Hud {
        let bot = visible.bottom();
        let cx = visible.left() + visible.width() as i32 / 2;
        let elements: Vec<Box<Element<HudInput, HudCmd>>> = vec![
            Box::new(PauseIndicator::new(resources, visible)),
            Hud::button(resources, location, HudCmd::Back, cx - 160, bot),
            Hud::button(resources, location, HudCmd::Info, cx - 95, bot),
            Hud::button(resources, location, HudCmd::Undo, cx + 96, bot),
            Hud::button(resources, location, HudCmd::Redo, cx + 149, bot),
            Hud::button(resources, location, HudCmd::Reset, cx + 210, bot),
            Hud::button(resources, location, HudCmd::Replay, cx + 160, bot),
            Hud::namebox(resources, cx, bot),
        ];
        Hud { elements: GroupElement::new(elements) }
    }

    fn button(resources: &mut Resources, location: Location, value: HudCmd,
              center_x: i32, bottom: i32)
              -> Box<Element<HudInput, HudCmd>> {
        Box::new(HudButton::new(resources, location, value, center_x, bottom))
    }

    fn namebox(resources: &mut Resources, center_x: i32, bottom: i32)
               -> Box<Element<HudInput, HudCmd>> {
        let namebox = HudNamebox::new(resources);
        let left = center_x - NAMEBOX_WIDTH as i32 / 2;
        let top = bottom - NAMEBOX_HEIGHT as i32;
        let rect = Rect::new(left, top, NAMEBOX_WIDTH, NAMEBOX_HEIGHT);
        Box::new(SubrectElement::new(namebox, rect))
    }
}

impl Element<HudInput, HudCmd> for Hud {
    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        self.elements.draw(input, canvas);
    }

    fn handle_event(&mut self, event: &Event, input: &mut HudInput)
                    -> Action<HudCmd> {
        self.elements.handle_event(event, input)
    }
}

// ========================================================================= //

struct HudButton {
    sprite: Sprite,
    rect: Rect,
    value: HudCmd,
    scroll: i32,
}

impl HudButton {
    fn new(resources: &mut Resources, location: Location, value: HudCmd,
           center_x: i32, bottom: i32)
           -> HudButton {
        let sprites = resources.get_sprites("hud_buttons");
        let index = match value {
            HudCmd::Back if location == Location::Map => 0,
            HudCmd::Back => 1,
            HudCmd::Info => 2,
            HudCmd::Undo => 3,
            HudCmd::Redo => 4,
            HudCmd::Reset => 5,
            HudCmd::Replay => 6,
        };
        let sprite = sprites[index].clone();
        let rect = Rect::new(center_x - sprite.width() as i32 / 2,
                             bottom - sprite.height() as i32,
                             sprite.width(),
                             sprite.height());
        HudButton {
            sprite: sprite,
            rect: rect,
            value: value,
            scroll: BUTTON_HEIGHT as i32,
        }
    }

    fn enabled(&self, input: &HudInput) -> bool {
        let active = input.active;
        match self.value {
            HudCmd::Back => active,
            HudCmd::Info => active,
            HudCmd::Undo => active && input.can_undo && !input.can_replay,
            HudCmd::Redo => active && input.can_redo && !input.can_replay,
            HudCmd::Reset => active && input.can_reset && !input.can_replay,
            HudCmd::Replay => active && input.can_replay,
        }
    }
}

impl Element<HudInput, HudCmd> for HudButton {
    fn draw(&self, _: &HudInput, canvas: &mut Canvas) {
        let top_left = Point::new(self.rect.x(), self.rect.y() + self.scroll);
        canvas.draw_sprite(&self.sprite, top_left);
    }

    fn handle_event(&mut self, event: &Event, input: &mut HudInput)
                    -> Action<HudCmd> {
        match event {
            &Event::ClockTick => {
                let enabled = self.enabled(input);
                if enabled && self.scroll > 0 {
                    self.scroll = cmp::max(0, self.scroll - SCROLL_SPEED);
                    Action::redraw()
                } else if !enabled && self.scroll < BUTTON_HEIGHT as i32 {
                    self.scroll = cmp::min(BUTTON_HEIGHT as i32,
                                           self.scroll + SCROLL_SPEED);
                    Action::redraw()
                } else {
                    Action::ignore()
                }
            }
            &Event::MouseDown(pt) if self.scroll == 0 && self.enabled(input) &&
                                     self.rect.contains(pt) => {
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

struct PauseIndicator {
    paragraph: Paragraph,
    outer_rect: Rect,
    inner_rect: Rect,
}

impl PauseIndicator {
    fn new(resources: &mut Resources, visible: Rect) -> PauseIndicator {
        let paragraph = Paragraph::new(resources, PAUSE_TEXT);
        let inner_width = paragraph.min_width();
        let outer_width = inner_width + 2 * PAUSE_TEXT_MARGIN;
        let inner_height = paragraph.height();
        let outer_height = inner_height + 2 * PAUSE_TEXT_MARGIN as u32;
        let outer_rect = Rect::new(visible.x() +
                                   (visible.width() as i32 - outer_width) / 2,
                                   visible.top(),
                                   outer_width as u32,
                                   outer_height);
        let inner_rect = Rect::new(outer_rect.x() + PAUSE_TEXT_MARGIN,
                                   outer_rect.y() + PAUSE_TEXT_MARGIN,
                                   inner_width as u32,
                                   inner_height);
        PauseIndicator {
            paragraph: paragraph,
            outer_rect: outer_rect,
            inner_rect: inner_rect,
        }
    }
}

impl Element<HudInput, HudCmd> for PauseIndicator {
    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        if input.is_paused {
            canvas.fill_rect((255, 255, 255), self.outer_rect);
            let mut canvas = canvas.subcanvas(self.inner_rect);
            self.paragraph.draw(&mut canvas);
        }
    }

    fn handle_event(&mut self, _: &Event, _: &mut HudInput) -> Action<HudCmd> {
        Action::ignore()
    }
}

// ========================================================================= //
