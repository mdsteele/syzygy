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

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

// ========================================================================= //

pub struct Canvas<'a> {
    renderer: &'a mut Renderer<'static>,
    offset_rect: Rect,
    clip_rect: Option<Rect>,
    prev_clip_rect: Option<Rect>,
}

impl<'a> Canvas<'a> {
    pub fn new(renderer: &'a mut Renderer<'static>, rect: Rect) -> Canvas<'a> {
        Canvas {
            renderer: renderer,
            offset_rect: rect,
            clip_rect: None,
            prev_clip_rect: None,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.offset_rect.width(), self.offset_rect.height())
    }

    pub fn rect(&self) -> Rect {
        let (width, height) = self.size();
        Rect::new(0, 0, width, height)
    }

    pub fn subcanvas(&mut self, mut rect: Rect) -> Canvas {
        rect.offset(self.offset_rect.x(), self.offset_rect.y());
        let new_clip_rect = if let Some(clip) = self.clip_rect {
            if let Some(intersection) = clip.intersection(rect) {
                Some(intersection)
            } else {
                Some(Rect::new(0, 0, 0, 0))
            }
        } else {
            Some(rect)
        };
        self.renderer.set_clip_rect(new_clip_rect);
        Canvas {
            renderer: self.renderer,
            offset_rect: rect,
            clip_rect: new_clip_rect,
            prev_clip_rect: self.clip_rect,
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        let (r, g, b) = color;
        self.renderer.set_draw_color(Color::RGB(r, g, b));
        if let Some(clip) = self.clip_rect {
            self.renderer.fill_rect(clip).unwrap();
        } else {
            self.renderer.clear();
        }
    }

    pub fn fill_rect(&mut self, color: (u8, u8, u8), mut rect: Rect) {
        let (r, g, b) = color;
        self.renderer.set_draw_color(Color::RGB(r, g, b));
        rect.offset(self.offset_rect.x(), self.offset_rect.y());
        if let Some(clip) = self.clip_rect {
            if let Some(intersection) = clip.intersection(rect) {
                rect = intersection;
            } else {
                rect.resize(0, 0);
            }
        }
        self.renderer.fill_rect(rect).unwrap();
    }
}

impl<'a> Drop for Canvas<'a> {
    fn drop(&mut self) {
        self.renderer.set_clip_rect(self.prev_clip_rect);
    }
}

// ========================================================================= //
