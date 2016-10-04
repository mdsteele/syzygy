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
use super::super::gui::{Action, Align, Canvas, Element, Event, Font, Point,
                        Rect, Resources, Sprite};
use super::super::save::SaveData;

// ========================================================================= //

pub struct TitleView {
    font: Rc<Font>,
    sprites: Vec<Sprite>,
    counter: i32,
    blink: bool,
}

impl TitleView {
    pub fn new(resources: &mut Resources) -> TitleView {
        let font = resources.get_font("roman");
        let sprites = resources.get_sprites("chars");
        TitleView {
            font: font,
            sprites: sprites,
            counter: 0,
            blink: false,
        }
    }
}

impl Element<SaveData> for TitleView {
    fn draw(&self, _: &SaveData, canvas: &mut Canvas) {
        canvas.clear((64, 64, 128));
        let rect = canvas.rect();
        let margin: u32 = 100;
        let rect = Rect::new(rect.x() + margin as i32,
                             rect.y() + margin as i32,
                             rect.width() - 2 * margin,
                             rect.height() - 2 * margin);
        if self.blink {
            canvas.fill_rect((192, 0, 0), rect);
        } else {
            canvas.fill_rect((0, 192, 0), rect);
        }
        for i in 0..6 {
            canvas.draw_sprite(&self.sprites[i as usize],
                               Point::new(150 + 40 * i, 150));
        }
        let center_x = canvas.rect().width() as i32 / 2;
        canvas.draw_text(&self.font,
                         Align::Center,
                         Point::new(center_x, 250),
                         "Hello, world!");
    }

    fn handle_event(&mut self, event: &Event, _: &mut SaveData) -> Action {
        match event {
            &Event::ClockTick => {
                self.counter += 1;
                if self.counter >= 10 {
                    self.counter = 0;
                    self.blink = !self.blink;
                    Action::redraw().and_continue()
                } else {
                    Action::ignore().and_continue()
                }
            }
            &Event::MouseDown(_) => {
                self.counter = 0;
                self.blink = !self.blink;
                Action::redraw().and_stop()
            }
            _ => Action::ignore().and_continue(),
        }
    }
}

// ========================================================================= //
