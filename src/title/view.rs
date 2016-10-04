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
use super::super::gui::{Action, Align, Canvas, Element, Event, Font,
                        GroupElement, Point, Rect, Resources, Sprite,
                        SubrectElement};
use super::super::save::SaveData;

// ========================================================================= //

const BOTTOM_BUTTONS_MARGIN: i32 = 20;
const FULLSCREEN_BUTTON_WIDTH: u32 = 32;
const FULLSCREEN_BUTTON_HEIGHT: u32 = 32;

// ========================================================================= //

pub struct TitleView {
    elements: GroupElement<SaveData>,
    font: Rc<Font>,
    sprites: Vec<Sprite>,
}

impl TitleView {
    pub fn new(resources: &mut Resources, visible: Rect) -> TitleView {
        let font = resources.get_font("roman");
        let sprites = resources.get_sprites("chars");
        let fullscreen_button = {
            let rect = Rect::new(visible.left() + BOTTOM_BUTTONS_MARGIN,
                                 visible.bottom() -
                                 FULLSCREEN_BUTTON_HEIGHT as i32 -
                                 BOTTOM_BUTTONS_MARGIN,
                                 FULLSCREEN_BUTTON_WIDTH,
                                 FULLSCREEN_BUTTON_HEIGHT);
            SubrectElement::new(FullscreenButton::new(resources), rect)
        };
        TitleView {
            elements: GroupElement::new(vec![
                Box::new(fullscreen_button),
            ]),
            font: font,
            sprites: sprites,
        }
    }
}

impl Element<SaveData> for TitleView {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        canvas.clear((64, 64, 128));
        let rect = canvas.rect();
        let margin: u32 = 100;
        let rect = Rect::new(rect.x() + margin as i32,
                             rect.y() + margin as i32,
                             rect.width() - 2 * margin,
                             rect.height() - 2 * margin);
        canvas.fill_rect((0, 192, 0), rect);
        for i in 0..6 {
            canvas.draw_sprite(&self.sprites[i as usize],
                               Point::new(150 + 40 * i, 150));
        }
        let center_x = canvas.rect().width() as i32 / 2;
        canvas.draw_text(&self.font,
                         Align::Center,
                         Point::new(center_x, 250),
                         "Hello, world!");
        self.elements.draw(data, canvas);
    }

    fn handle_event(&mut self, event: &Event, data: &mut SaveData) -> Action {
        self.elements.handle_event(event, data)
    }
}

// ========================================================================= //

struct FullscreenButton {
    to_fullscreen_icon: Sprite,
    to_windowed_icon: Sprite,
}

impl FullscreenButton {
    fn new(resources: &mut Resources) -> FullscreenButton {
        let sprites = resources.get_sprites("fullscreen");
        FullscreenButton {
            to_fullscreen_icon: sprites[0].clone(),
            to_windowed_icon: sprites[1].clone(),
        }
    }
}

impl Element<SaveData> for FullscreenButton {
    fn draw(&self, data: &SaveData, canvas: &mut Canvas) {
        let icon = if data.prefs().fullscreen() {
            &self.to_windowed_icon
        } else {
            &self.to_fullscreen_icon
        };
        canvas.draw_sprite(icon, Point::new(0, 0));
    }

    fn handle_event(&mut self, event: &Event, data: &mut SaveData) -> Action {
        match event {
            &Event::MouseDown(_) => {
                let prefs = data.prefs_mut();
                let fullscreen = prefs.fullscreen();
                prefs.set_fullscreen(!fullscreen);
                Action::redraw().and_stop()
            }
            _ => Action::ignore().and_continue(),
        }
    }
}

// ========================================================================= //
