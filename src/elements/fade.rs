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

use gui::{Action, Canvas, Element, Event, Point, Resources, Sprite};

// ========================================================================= //

const MAX_OPACITY: i32 = 7;
const TILE_SIZE: i32 = 24;

pub struct ScreenFade {
    sprites: Vec<Sprite>,
    opacity: i32,
    should_be_opaque: bool,
}

impl ScreenFade {
    pub fn new(resources: &mut Resources) -> ScreenFade {
        ScreenFade {
            sprites: resources.get_sprites("screen_fade"),
            opacity: MAX_OPACITY,
            should_be_opaque: false,
        }
    }

    pub fn set_should_be_opaque(&mut self, opaque: bool) {
        self.should_be_opaque = opaque;
    }
}

impl Element<(), bool> for ScreenFade {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        if self.opacity >= MAX_OPACITY {
            canvas.clear((0, 0, 0));
        } else if self.opacity > 0 {
            let width = canvas.width() as i32;
            let height = canvas.height() as i32;
            let sprite = &self.sprites[self.opacity as usize - 1];
            let mut y = 0;
            while y < height {
                let mut x = 0;
                while x < width {
                    canvas.draw_sprite(sprite, Point::new(x, y));
                    x += TILE_SIZE;
                }
                y += TILE_SIZE;
            }
        }
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<bool> {
        match event {
            &Event::Quit => Action::ignore(),
            &Event::ClockTick => {
                if self.should_be_opaque && self.opacity < MAX_OPACITY {
                    self.opacity += 1;
                    if self.opacity == MAX_OPACITY {
                        Action::redraw().and_return(true)
                    } else {
                        Action::redraw()
                    }
                } else if !self.should_be_opaque && self.opacity > 0 {
                    self.opacity -= 1;
                    if self.opacity == 0 {
                        Action::redraw().and_return(false)
                    } else {
                        Action::redraw()
                    }
                } else {
                    Action::ignore()
                }
            }
            _ => {
                if self.opacity == 0 {
                    Action::ignore()
                } else {
                    Action::ignore().and_stop()
                }
            }
        }
    }
}

// ========================================================================= //
