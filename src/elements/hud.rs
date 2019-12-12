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

use crate::elements::Paragraph;
use crate::gui::{
    Action, Align, Canvas, Element, Event, Font, KeyMod, Keycode, Point, Rect,
    Resources, Sound, Sprite,
};
use crate::save::{Access, Location};

// ========================================================================= //

const BUTTON_HEIGHT: u32 = 16;
const NAMEBOX_WIDTH: u32 = 114;
const NAMEBOX_HEIGHT: u32 = 16;
const SCROLL_SPEED: i32 = 2;
const PAUSE_TEXT: &str = "$M{Tap}{Click} anywhere to continue";
const PAUSE_TEXT_MARGIN_HORZ: i32 = 5;
const PAUSE_TEXT_MARGIN_VERT: i32 = 3;
const SKIP_TEXT: &str = "$M{Tap here}{Click here or press ESC} to skip scene";
const SKIP_TEXT_MARGIN_HORZ: i32 = 5;
const SKIP_TEXT_MARGIN_VERT: i32 = 3;

// ========================================================================= //

pub struct HudInput {
    pub name: &'static str,
    pub access: Access,
    pub is_paused: bool,
    pub show_skip: bool,
    pub active: bool,
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
    Replay,
    Solve,
    Skip,
}

// ========================================================================= //

pub struct Hud {
    namebox: HudNamebox,
    buttons: Vec<HudButton>,
    pause: PauseIndicator,
    skip: SkipIndicator,
}

impl Hud {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(resources: &mut Resources, visible: Rect, location: Location)
               -> Hud {
        let bot = visible.bottom();
        let cx = visible.left() + visible.width() as i32 / 2;
        let buttons = vec![
            HudButton::new(resources, location, HudCmd::Solve, cx - 204, bot),
            HudButton::new(resources, location, HudCmd::Back, cx - 140, bot),
            HudButton::new(resources, location, HudCmd::Info, cx - 84, bot),
            HudButton::new(resources, location, HudCmd::Undo, cx + 97, bot),
            HudButton::new(resources, location, HudCmd::Redo, cx + 150, bot),
            HudButton::new(resources, location, HudCmd::Reset, cx + 210, bot),
            HudButton::new(resources, location, HudCmd::Replay, cx + 160, bot),
        ];
        Hud {
            namebox: Hud::namebox(resources, cx, bot),
            buttons: buttons,
            pause: PauseIndicator::new(resources, visible),
            skip: SkipIndicator::new(resources, visible),
        }
    }

    pub fn flash_info_button(&mut self) {
        self.buttons[2].set_flashing(true);
    }

    fn namebox(
        resources: &mut Resources,
        center_x: i32,
        bottom: i32,
    ) -> HudNamebox {
        let left = center_x - NAMEBOX_WIDTH as i32 / 2;
        let top = bottom - NAMEBOX_HEIGHT as i32;
        let rect = Rect::new(left, top, NAMEBOX_WIDTH, NAMEBOX_HEIGHT);
        HudNamebox::new(resources, rect)
    }
}

impl Element<HudInput, HudCmd> for Hud {
    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        self.namebox.draw(input, canvas);
        self.buttons.draw(input, canvas);
        self.pause.draw(input, canvas);
        self.skip.draw(input, canvas);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        input: &mut HudInput,
    ) -> Action<HudCmd> {
        let mut action = self.skip.handle_event(event, input);
        if !action.should_stop() || event == &Event::ClockTick {
            action.merge(self.buttons.handle_event(event, input));
        }
        action
    }
}

// ========================================================================= //

const BLINK_FRAMES: i32 = 3;

struct HudButton {
    base_sprite: Sprite,
    blink_sprite: Sprite,
    rect: Rect,
    value: HudCmd,
    scroll: i32,
    blink_frames: i32,
    flashing: bool,
}

impl HudButton {
    fn new(
        resources: &mut Resources,
        location: Location,
        value: HudCmd,
        center_x: i32,
        bottom: i32,
    ) -> HudButton {
        let sprites = resources.get_sprites("hud/buttons");
        let (index, width) = match value {
            HudCmd::Back if location == Location::Map => (0, 48),
            HudCmd::Back => (2, 46),
            HudCmd::Info => (4, 46),
            HudCmd::Undo => (6, 50),
            HudCmd::Redo => (8, 50),
            HudCmd::Reset => (10, 54),
            HudCmd::Replay => (12, 60),
            HudCmd::Solve => (14, 54),
            HudCmd::Skip => panic!("HudButton HudCmd::Skip"),
        };
        let sprite = sprites[index].clone();
        let rect = Rect::new(
            center_x - sprite.width() as i32 / 2,
            bottom - sprite.height() as i32,
            width,
            sprite.height(),
        );
        HudButton {
            base_sprite: sprite,
            blink_sprite: sprites[index + 1].clone(),
            rect,
            value,
            scroll: BUTTON_HEIGHT as i32,
            blink_frames: 0,
            flashing: false,
        }
    }

    fn set_flashing(&mut self, flashing: bool) {
        self.flashing = flashing;
    }

    fn is_visible(&self) -> bool {
        self.scroll < BUTTON_HEIGHT as i32
    }

    fn is_enabled(&self, input: &HudInput) -> bool {
        let active = input.active;
        let solved = input.access == Access::Solved;
        match self.value {
            HudCmd::Back => active,
            HudCmd::Info => active,
            HudCmd::Undo => active && input.can_undo && !solved,
            HudCmd::Redo => active && input.can_redo && !solved,
            HudCmd::Reset => active && input.can_reset && !solved,
            HudCmd::Replay => active && solved,
            HudCmd::Solve => active && input.access == Access::Replaying,
            HudCmd::Skip => panic!("HudButton HudCmd::Skip"),
        }
    }

    fn click_action(&self) -> Action<HudCmd> {
        let mut action = Action::redraw().and_return(self.value);
        if self.value == HudCmd::Undo {
            action.also_play_sound(Sound::undo());
        } else if self.value == HudCmd::Redo {
            action.also_play_sound(Sound::redo());
        } else if self.value == HudCmd::Reset {
            action.also_play_sound(Sound::reset());
        }
        action
    }
}

impl Element<HudInput, HudCmd> for HudButton {
    fn draw(&self, _: &HudInput, canvas: &mut Canvas) {
        if self.is_visible() {
            let pt = Point::new(self.rect.x(), self.rect.y() + self.scroll);
            if self.blink_frames > 0 && self.blink_frames <= BLINK_FRAMES {
                canvas.draw_sprite(&self.blink_sprite, pt);
            } else {
                canvas.draw_sprite(&self.base_sprite, pt);
            }
        }
    }

    fn handle_event(
        &mut self,
        event: &Event,
        input: &mut HudInput,
    ) -> Action<HudCmd> {
        match event {
            &Event::ClockTick => {
                let was_visible = self.is_visible();
                let mut redraw = false;
                if self.blink_frames > 0 {
                    self.blink_frames -= 1;
                    if self.blink_frames == 0
                        || self.blink_frames == BLINK_FRAMES
                    {
                        redraw = true;
                    }
                } else if self.flashing {
                    self.blink_frames = 3 * BLINK_FRAMES;
                }
                let enabled = self.is_enabled(input);
                if enabled && self.scroll > 0 {
                    self.scroll = cmp::max(0, self.scroll - SCROLL_SPEED);
                    redraw = true;
                } else if !enabled && self.scroll < BUTTON_HEIGHT as i32 {
                    self.scroll = cmp::min(
                        BUTTON_HEIGHT as i32,
                        self.scroll + SCROLL_SPEED,
                    );
                    redraw = true;
                }
                Action::redraw_if(redraw && (was_visible || self.is_visible()))
            }
            &Event::MouseDown(pt)
                if self.scroll == 0
                    && self.is_enabled(input)
                    && self.rect.contains_point(pt) =>
            {
                self.blink_frames = BLINK_FRAMES;
                self.flashing = false;
                self.click_action()
            }
            &Event::KeyDown(Keycode::Z, keymod) if self.is_enabled(input) => {
                if keymod == KeyMod::command() && self.value == HudCmd::Undo
                    || keymod == (KeyMod::command() | KeyMod::shift())
                        && self.value == HudCmd::Redo
                {
                    self.click_action()
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct HudNamebox {
    sprites: Vec<Sprite>,
    font: Rc<Font>,
    rect: Rect,
}

impl HudNamebox {
    fn new(resources: &mut Resources, rect: Rect) -> HudNamebox {
        HudNamebox {
            sprites: resources.get_sprites("hud/namebox"),
            font: resources.get_font("roman"),
            rect,
        }
    }

    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect);
        canvas.draw_sprite(&self.sprites[0], Point::new(0, 0));
        canvas.fill_rect((200, 200, 200), Rect::new(2, 2, 110, 14));
        let start = Point::new(canvas.width() as i32 / 2, 12);
        canvas.draw_text(&self.font, Align::Center, start, input.name);
    }
}

// ========================================================================= //

struct PauseIndicator {
    paragraph: Paragraph,
    outer_rect: Rect,
    mid_rect: Rect,
    inner_rect: Rect,
}

impl PauseIndicator {
    fn new(resources: &mut Resources, visible: Rect) -> PauseIndicator {
        let paragraph =
            Paragraph::new(resources, "roman", Align::Center, PAUSE_TEXT);
        let inner_width = paragraph.min_width();
        let outer_width = inner_width + 2 * PAUSE_TEXT_MARGIN_HORZ;
        let inner_height = paragraph.height();
        let outer_height = inner_height + 2 * PAUSE_TEXT_MARGIN_VERT as u32;
        let outer_rect = Rect::new(
            visible.x() + (visible.width() as i32 - outer_width) / 2,
            visible.top(),
            outer_width as u32,
            outer_height,
        );
        let mid_rect = Rect::new(
            outer_rect.x() + 1,
            outer_rect.y() - 1,
            outer_rect.width() - 2,
            outer_rect.height(),
        );
        let inner_rect = Rect::new(
            outer_rect.x() + PAUSE_TEXT_MARGIN_HORZ,
            outer_rect.y() + PAUSE_TEXT_MARGIN_VERT - 1,
            inner_width as u32,
            inner_height,
        );
        PauseIndicator { paragraph, outer_rect, mid_rect, inner_rect }
    }

    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        if input.is_paused {
            canvas.fill_rect((255, 255, 255), self.outer_rect);
            canvas.draw_rect((0, 0, 0), self.mid_rect);
            let mut canvas = canvas.subcanvas(self.inner_rect);
            self.paragraph.draw(&mut canvas);
        }
    }
}

// ========================================================================= //

struct SkipIndicator {
    paragraph: Paragraph,
    outer_rect: Rect,
    mid_rect: Rect,
    inner_rect: Rect,
}

impl SkipIndicator {
    fn new(resources: &mut Resources, visible: Rect) -> SkipIndicator {
        let paragraph =
            Paragraph::new(resources, "roman", Align::Center, SKIP_TEXT);
        let inner_width = paragraph.min_width();
        let outer_width = inner_width + 2 * SKIP_TEXT_MARGIN_HORZ;
        let inner_height = paragraph.height();
        let outer_height = inner_height + 2 * SKIP_TEXT_MARGIN_VERT as u32;
        let outer_rect = Rect::new(
            visible.x() + (visible.width() as i32 - outer_width) / 2,
            visible.top() + ((visible.height() - outer_height) as i32) / 3,
            outer_width as u32,
            outer_height,
        );
        let mid_rect = Rect::new(
            outer_rect.x() + 1,
            outer_rect.y() + 1,
            outer_rect.width() - 2,
            outer_rect.height() - 2,
        );
        let inner_rect = Rect::new(
            outer_rect.x() + SKIP_TEXT_MARGIN_HORZ,
            outer_rect.y() + SKIP_TEXT_MARGIN_VERT + 1,
            inner_width as u32,
            inner_height,
        );
        SkipIndicator { paragraph, outer_rect, mid_rect, inner_rect }
    }
}

impl Element<HudInput, HudCmd> for SkipIndicator {
    fn draw(&self, input: &HudInput, canvas: &mut Canvas) {
        if input.show_skip {
            canvas.fill_rect((255, 255, 255), self.outer_rect);
            canvas.draw_rect((0, 0, 0), self.mid_rect);
            let mut canvas = canvas.subcanvas(self.inner_rect);
            self.paragraph.draw(&mut canvas);
        }
    }

    fn handle_event(
        &mut self,
        event: &Event,
        input: &mut HudInput,
    ) -> Action<HudCmd> {
        match event {
            &Event::MouseDown(pt)
                if input.show_skip && self.outer_rect.contains_point(pt) =>
            {
                Action::redraw().and_return(HudCmd::Skip)
            }
            &Event::KeyDown(Keycode::Escape, _) if input.show_skip => {
                Action::redraw().and_return(HudCmd::Skip)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //
