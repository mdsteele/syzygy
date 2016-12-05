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
use sdl2::rect::{Point, Rect};
use sdl2::render::Renderer;
use super::background::Background;
use super::font::Font;
use super::sprite::Sprite;

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

    pub fn width(&self) -> u32 { self.offset_rect.width() }

    pub fn height(&self) -> u32 { self.offset_rect.height() }

    pub fn rect(&self) -> Rect { Rect::new(0, 0, self.width(), self.height()) }

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

    pub fn draw_rect(&mut self, color: (u8, u8, u8), mut rect: Rect) {
        let (r, g, b) = color;
        self.renderer.set_draw_color(Color::RGB(r, g, b));
        rect.offset(self.offset_rect.x(), self.offset_rect.y());
        self.renderer.draw_rect(rect).unwrap();
    }

    pub fn draw_background(&mut self, background: &Background) {
        for (sprite, point) in background.tiles() {
            self.draw_sprite(sprite, point);
        }
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, mut top_left: Point) {
        top_left = top_left.offset(self.offset_rect.x(), self.offset_rect.y());
        self.renderer.copy(&sprite.sdl2_texture(),
                           None,
                           Some(Rect::new(top_left.x(),
                                          top_left.y(),
                                          sprite.width(),
                                          sprite.height())));
    }

    pub fn draw_sprite_transposed(&mut self, sprite: &Sprite,
                                  mut top_left: Point) {
        top_left = top_left.offset(self.offset_rect.x(), self.offset_rect.y());
        self.renderer
            .copy_ex(&sprite.sdl2_texture(),
                     None,
                     Some(Rect::new(top_left.x(),
                                    top_left.y(),
                                    sprite.width(),
                                    sprite.height())),
                     90.0,
                     None,
                     false,
                     true)
            .unwrap();
    }

    pub fn draw_sprite_flipped(&mut self, sprite: &Sprite,
                               mut top_left: Point, flip_horz: bool,
                               flip_vert: bool) {
        top_left = top_left.offset(self.offset_rect.x(), self.offset_rect.y());
        self.renderer
            .copy_ex(&sprite.sdl2_texture(),
                     None,
                     Some(Rect::new(top_left.x(),
                                    top_left.y(),
                                    sprite.width(),
                                    sprite.height())),
                     0.0,
                     None,
                     flip_horz,
                     flip_vert)
            .unwrap();
    }

    pub fn draw_sprite_centered(&mut self, sprite: &Sprite, center: Point) {
        let top_left = Point::new(center.x() - sprite.width() as i32 / 2,
                                  center.y() - sprite.height() as i32 / 2);
        self.draw_sprite(sprite, top_left);
    }

    pub fn draw_sprite_rotated(&mut self, sprite: &Sprite, center: Point,
                               degrees: i32) {
        self.draw_sprite_transformed(sprite, center, degrees, false, false);
    }

    pub fn draw_sprite_transformed(&mut self, sprite: &Sprite,
                                   mut center: Point, degrees: i32,
                                   flip_horz: bool, flip_vert: bool) {
        center = center.offset(self.offset_rect.x(), self.offset_rect.y());
        let dest = Rect::new(center.x() - sprite.width() as i32 / 2,
                             center.y() - sprite.height() as i32 / 2,
                             sprite.width(),
                             sprite.height());
        self.renderer
            .copy_ex(&sprite.sdl2_texture(),
                     None,
                     Some(dest),
                     degrees as f64,
                     None,
                     flip_horz,
                     flip_vert)
            .unwrap();
    }

    pub fn draw_text(&mut self, font: &Font, alignment: Align, start: Point,
                     text: &str) {
        let top = start.y() - font.baseline();
        let mut left = match alignment {
            Align::Left => start.x(),
            Align::Center => start.x() - font.text_width(text) / 2,
            Align::Right => start.x() - font.text_width(text),
        };
        for chr in text.chars() {
            let glyph = font.glyph(chr);
            left -= glyph.left_edge();
            self.draw_sprite(glyph.sprite(), Point::new(left, top));
            left += glyph.right_edge();
        }
    }
}

impl<'a> Drop for Canvas<'a> {
    fn drop(&mut self) { self.renderer.set_clip_rect(self.prev_clip_rect); }
}

// ========================================================================= //

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

// ========================================================================= //
