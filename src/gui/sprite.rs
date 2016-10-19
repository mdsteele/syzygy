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

use ahi;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2::surface::Surface;
use std::rc::Rc;

// ========================================================================= //

#[derive(Clone)]
pub struct Sprite {
    width: u32,
    height: u32,
    texture: Rc<Texture>,
}

impl Sprite {
    pub fn new(renderer: &Renderer, image: &ahi::Image) -> Sprite {
        let width = image.width();
        let height = image.height();
        let mut data = image.rgba_data();
        let bytes_per_pixel = 4;
        let format = if cfg!(target_endian = "big") {
            PixelFormatEnum::RGBA8888
        } else {
            PixelFormatEnum::ABGR8888
        };
        let surface = Surface::from_data(&mut data,
                                         width,
                                         height,
                                         width * bytes_per_pixel,
                                         format)
                          .unwrap();
        Sprite {
            width: width,
            height: height,
            texture: Rc::new(renderer.create_texture_from_surface(&surface)
                                     .unwrap()),
        }
    }

    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }

    pub fn rect(&self) -> Rect { Rect::new(0, 0, self.width, self.height) }

    pub fn sdl2_texture(&self) -> &Texture { &self.texture }
}

// ========================================================================= //
