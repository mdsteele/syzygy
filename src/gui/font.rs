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
use sdl2::render::Renderer;
use std::collections::BTreeMap;
use super::sprite::Sprite;

// ========================================================================= //

pub struct Font {
    glyphs: BTreeMap<char, Glyph>,
    default_glyph: Glyph,
    baseline: i32,
}

impl Font {
    pub fn new(renderer: &Renderer, font: &ahi::Font) -> Font {
        let mut glyphs = BTreeMap::new();
        for chr in font.chars() {
            glyphs.insert(chr, Glyph::new(renderer, &font[chr]));
        }
        Font {
            glyphs: glyphs,
            default_glyph: Glyph::new(renderer, font.default_glyph()),
            baseline: font.baseline(),
        }
    }

    pub fn baseline(&self) -> i32 { self.baseline }

    pub fn height(&self) -> u32 { self.default_glyph.sprite().height() }

    pub fn text_width(&self, text: &str) -> i32 {
        let mut width = 0;
        for chr in text.chars() {
            let glyph = self.glyph(chr);
            width += glyph.right_edge - glyph.left_edge;
        }
        width
    }

    pub fn glyph(&self, chr: char) -> &Glyph {
        self.glyphs.get(&chr).unwrap_or(&self.default_glyph)
    }
}

// ========================================================================= //

pub struct Glyph {
    sprite: Sprite,
    left_edge: i32,
    right_edge: i32,
}

impl Glyph {
    fn new(renderer: &Renderer, glyph: &ahi::Glyph) -> Glyph {
        Glyph {
            sprite: Sprite::new(renderer, glyph.image()),
            left_edge: glyph.left_edge(),
            right_edge: glyph.right_edge(),
        }
    }

    pub fn sprite(&self) -> &Sprite { &self.sprite }

    pub fn left_edge(&self) -> i32 { self.left_edge }

    pub fn right_edge(&self) -> i32 { self.right_edge }
}

// ========================================================================= //
