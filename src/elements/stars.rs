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

use crate::gui::{Canvas, Rect};

// ========================================================================= //

pub struct MovingStars {
    rect: Rect,
    anim: i32,
    visible: bool,
}

impl MovingStars {
    pub fn new(left: i32, top: i32, width: u32, height: u32) -> MovingStars {
        MovingStars {
            rect: Rect::new(left, top, width, height),
            anim: 0,
            visible: false,
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn rand(range: u32, seed: &mut (u32, u32)) -> i32 {
        seed.0 = 36969 * (seed.0 & 0xffff) + (seed.0 >> 16);
        seed.1 = 18000 * (seed.1 & 0xffff) + (seed.1 >> 16);
        let next = (seed.0 << 16) | (seed.1 & 0xffff);
        (next % range) as i32
    }

    fn draw_star(
        &self,
        x: i32,
        y: i32,
        width: u32,
        gray: u8,
        canvas: &mut Canvas,
    ) {
        canvas.fill_rect((gray, gray, gray), Rect::new(x, y, width, 1));
    }

    fn draw_layer(
        &self,
        spacing: u32,
        speed: i32,
        gray: u8,
        canvas: &mut Canvas,
    ) {
        let mut seed = (123456789, 987654321);
        let star_width = (speed / 2) as u32;
        let modulus = (self.rect.width() + spacing) as i32;
        let scroll = (self.anim * speed) % modulus;
        let mut yoff = 0;
        while yoff < modulus {
            let mut xoff = 0;
            while xoff < modulus {
                let x = ((xoff + scroll) % modulus) - spacing as i32
                    + MovingStars::rand(spacing, &mut seed);
                let y = yoff + MovingStars::rand(spacing, &mut seed);
                self.draw_star(x, y, star_width, gray, canvas);
                xoff += spacing as i32;
            }
            yoff += spacing as i32;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        if self.visible {
            let mut canvas = canvas.subcanvas(self.rect);
            canvas.clear((0, 0, 0));
            self.draw_layer(16, 8, 63, &mut canvas);
            self.draw_layer(32, 16, 127, &mut canvas);
        }
    }

    pub fn tick_animation(&mut self) -> bool {
        if self.visible {
            self.anim += 1;
        }
        self.visible
    }
}

// ========================================================================= //
