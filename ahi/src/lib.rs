// +--------------------------------------------------------------------------+
// | Copyright 2016 Matthew D. Steele <mdsteele@alum.mit.edu>                 |
// |                                                                          |
// | This file is part of AHI.                                                |
// |                                                                          |
// | AHI is free software: you can redistribute it and/or modify it under     |
// | the terms of the GNU General Public License as published by the Free     |
// | Software Foundation, either version 3 of the License, or (at your        |
// | option) any later version.                                               |
// |                                                                          |
// | AHI is distributed in the hope that it will be useful, but WITHOUT ANY   |
// | WARRANTY; without even the implied warranty of MERCHANTABILITY or        |
// | FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License    |
// | for details.                                                             |
// |                                                                          |
// | You should have received a copy of the GNU General Public License along  |
// | with AHI.  If not, see <http://www.gnu.org/licenses/>.                   |
// +--------------------------------------------------------------------------+

//! A library for encoding/decoding ASCII Hex Image (.ahi) and ASCII Hex Font
//! (.ahf) files.
//!
//! # The AHI format
//!
//! ASCII Hex Image (AHI) is a file format for storing collections of small,
//! 16-color images as an ASCII text file.  It is intended for storing sprites
//! for games or other graphical applications, in a way that makes changes to
//! image files result in (semi-)human-readable VCS diffs.
//!
//! Here's what a typical .ahi file looks like:
//!
//! ```text
//! ahi0 w20 h5 n2
//!
//! 0000000000000FFF0000
//! FFFFFFFFFFFFFF11FF00
//! F11111111111111111FF
//! FFFFFFFFFFFFFF11FF00
//! 0000000000000FFF0000
//!
//! 0000FFF0000000000000
//! 00FF11FFFFFFFFFFFFFF
//! FF11111111111111111F
//! 00FF11FFFFFFFFFFFFFF
//! 0000FFF0000000000000
//! ```
//!
//! The start of the .ahi file is the _header line_, which has the form
//! `ahi<version> w<width> h<height> n<num_images>`, where each of the four
//! fields is a decimal number.  So, the above file is AHI version 0 (currently
//! the only valid version), and contains two 20x5-pixel images (all the
//! images in a single file must have the same dimensions).
//!
//! After the header line comes the images, which are separated from the header
//! line and from each other by double-newlines.  Each image has one text line
//! per pixel row, with one hex digit per pixel.  Each pixel row line
//! (including the last one in the file) must be terminated by a newline.
//!
//! To map from hex digits to colors, treat each hex digit as a four-digit
//! binary number; the 1's place controls brightness, the 2's place controls
//! red, the 4's place controls green, and the 8's place controls blue.  For
//! example, `3 = 0b0011` is full-brightness red; `A = 0b1010` is
//! half-brightness magenta; `F = 0b1111` is white; and `E = 0x1110` is
//! "half-brightness white", i.e. gray.  Since "full-brightness black"
//! (`1 = 0b0001`) and "half-brightness black" (`0 = 0b0000`) would be the same
//! color, instead color `0` is special-cased to be transparent (and color `1`
//! is black).
//!
//! # The AHF format
//!
//! ASCII Hex Font (AHF) is a variation on the AHI file format, meant for
//! storing 16-color bitmap fonts as an ASCII text file.
//!
//! Here's what a typical .ahf file looks like:
//!
//! ```text
//! ahf0 h6 b5 n2
//!
//! def w4 s5
//! 1111
//! 1001
//! 1001
//! 1001
//! 1001
//! 1111
//!
//! 'A' w5 s6
//! 01110
//! 10001
//! 11111
//! 10001
//! 10001
//! 00000
//!
//! 'g' w4 s5
//! 0000
//! 0111
//! 1001
//! 0111
//! 0001
//! 0110
//! ```
//!
//! The start of the .ahf file is the _header line_, which has the form
//! `ahf<version> h<height> b<baseline> n<num_glyphs>`, where each of the four
//! fields is a decimal number.  So, the above file is AHF version 0 (currently
//! the only valid version), and contains two 6-pixel high glyphs in addition
//! to the default glyph, with a baseline height of 5 pixels from the top.
//!
//! After the header line comes the glyphs, which are separated from the header
//! line and from each other by double-newlines.  Each glyph has a _subheader
//! line_, followed by one text line per pixel row, with one hex digit per
//! pixel.  Each pixel row line (including the last one in the file) must be
//! terminated by a newline.
//!

//! Each glyph subheader line has the form `<char> w<width> l<left> r<right>`,
//! where the `<char>` field is either `def` for the font's default glyph
//! (which must be present, and must come first), or a single-quoted Rust
//! character literal (e.g. `'g'` or `'\n'` or `'\u{2603}'`).  The `<left>` and
//! `<right>` fields give the number of pixels between the left edge of the
//! glyph's image and the virtual left/right edge of the glyph itself when
//! printing a string.  Color mapping of pixels works the same as for AHI
//! files.

#![warn(missing_docs)]

use std::cmp::{max, min};
use std::collections::{BTreeMap, btree_map};
use std::io::{self, Error, ErrorKind, Read, Write};
use std::ops::Deref;
use std::rc::Rc;

// ========================================================================= //

/// Represents a pixel color for an ASCII Hex Image.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    /// Completely transparent (R=0, G=0, B=0, A=0).
    Transparent,
    /// Solid black (R=0, G=0, B=0, A=255).
    Black,
    /// Half-brightness red (R=127, G=0, B=0, A=255).
    DarkRed,
    /// Full-brightness red (R=255, G=0, B=0, A=255).
    Red,
    /// Half-brightness green (R=0, G=127, B=0, A=255).
    DarkGreen,
    /// Full-brightness green (R=0, G=255, B=0, A=255).
    Green,
    /// Half-brightness yellow (R=127, G=127, B=0, A=255).
    DarkYellow,
    /// Full-brightness yellow (R=255, G=255, B=0, A=255).
    Yellow,
    /// Half-brightness blue (R=0, G=0, B=127, A=255).
    DarkBlue,
    /// Full-brightness blue (R=0, G=0, B=255, A=255).
    Blue,
    /// Half-brightness magenta (R=127, G=0, B=127, A=255).
    DarkMagenta,
    /// Full-brightness magenta (R=255, G=0, B=255, A=255).
    Magenta,
    /// Half-brightness cyan (R=0, G=127, B=127, A=255).
    DarkCyan,
    /// Full-brightness cyan (R=0, G=255, B=255, A=255).
    Cyan,
    /// Gray, i.e. half-brightness white  (R=127, G=127, B=127, A=255).
    Gray,
    /// Solid white (R=255, G=255, B=255, A=255).
    White,
}

impl Color {
    /// Returns the color's RGBA values.
    ///
    /// # Examples
    /// ```
    /// use ahi::Color;
    /// assert_eq!(Color::Transparent.rgba(), (0, 0, 0, 0));
    /// assert_eq!(Color::DarkYellow.rgba(), (127, 127, 0, 255));
    /// assert_eq!(Color::White.rgba(), (255, 255, 255, 255));
    /// ```
    pub fn rgba(self) -> (u8, u8, u8, u8) {
        match self {
            Color::Transparent => (0, 0, 0, 0),
            Color::Black => (0, 0, 0, 255),
            Color::DarkRed => (127, 0, 0, 255),
            Color::Red => (255, 0, 0, 255),
            Color::DarkGreen => (0, 127, 0, 255),
            Color::Green => (0, 255, 0, 255),
            Color::DarkYellow => (127, 127, 0, 255),
            Color::Yellow => (255, 255, 0, 255),
            Color::DarkBlue => (0, 0, 127, 255),
            Color::Blue => (0, 0, 255, 255),
            Color::DarkMagenta => (127, 0, 127, 255),
            Color::Magenta => (255, 0, 255, 255),
            Color::DarkCyan => (0, 127, 127, 255),
            Color::Cyan => (0, 255, 255, 255),
            Color::Gray => (127, 127, 127, 255),
            Color::White => (255, 255, 255, 255),
        }
    }

    fn to_byte(self) -> u8 {
        match self {
            Color::Transparent => b'0',
            Color::Black => b'1',
            Color::DarkRed => b'2',
            Color::Red => b'3',
            Color::DarkGreen => b'4',
            Color::Green => b'5',
            Color::DarkYellow => b'6',
            Color::Yellow => b'7',
            Color::DarkBlue => b'8',
            Color::Blue => b'9',
            Color::DarkMagenta => b'A',
            Color::Magenta => b'B',
            Color::DarkCyan => b'C',
            Color::Cyan => b'D',
            Color::Gray => b'E',
            Color::White => b'F',
        }
    }

    fn from_byte(byte: u8) -> io::Result<Color> {
        match byte {
            b'0' => Ok(Color::Transparent),
            b'1' => Ok(Color::Black),
            b'2' => Ok(Color::DarkRed),
            b'3' => Ok(Color::Red),
            b'4' => Ok(Color::DarkGreen),
            b'5' => Ok(Color::Green),
            b'6' => Ok(Color::DarkYellow),
            b'7' => Ok(Color::Yellow),
            b'8' => Ok(Color::DarkBlue),
            b'9' => Ok(Color::Blue),
            b'A' => Ok(Color::DarkMagenta),
            b'B' => Ok(Color::Magenta),
            b'C' => Ok(Color::DarkCyan),
            b'D' => Ok(Color::Cyan),
            b'E' => Ok(Color::Gray),
            b'F' => Ok(Color::White),
            _ => {
                let msg = format!("invalid pixel character: '{}'",
                                  String::from_utf8_lossy(&[byte]));
                Err(Error::new(ErrorKind::InvalidData, msg))
            }
        }
    }
}

// ========================================================================= //

/// Represents a single ASCII Hex Image.
#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Box<[Color]>,
}

impl Image {
    /// Constructs a new image with all pixels transparent.
    pub fn new(width: u32, height: u32) -> Image {
        let num_pixels = (width * height) as usize;
        return Image {
            width: width,
            height: height,
            pixels: vec![Color::Transparent; num_pixels].into_boxed_slice(),
        };
    }

    /// Returns the width of the image, in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image, in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a byte array containing RGBA-order data for the image pixels,
    /// in row-major order.
    pub fn rgba_data(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.pixels.len() * 4);
        for pixel in self.pixels.iter() {
            let (r, g, b, a) = pixel.rgba();
            data.push(r);
            data.push(g);
            data.push(b);
            data.push(a);
        }
        data
    }

    /// Reads a group of images from an AHI file.
    pub fn read_all<R: Read>(mut reader: R) -> io::Result<Vec<Image>> {
        try!(read_exactly(reader.by_ref(), b"ahi"));
        let version = try!(read_header_uint(reader.by_ref(), b' '));
        if version != 0 {
            let msg = format!("unsupported AHI version: {}", version);
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }
        try!(read_exactly(reader.by_ref(), b"w"));
        let width = try!(read_header_uint(reader.by_ref(), b' '));
        try!(read_exactly(reader.by_ref(), b"h"));
        let height = try!(read_header_uint(reader.by_ref(), b' '));
        try!(read_exactly(reader.by_ref(), b"n"));
        let num_images = try!(read_header_uint(reader.by_ref(), b'\n'));
        let mut images = Vec::with_capacity(num_images as usize);
        let mut row_buffer = vec![0u8; width as usize];
        for _ in 0..num_images {
            try!(read_exactly(reader.by_ref(), b"\n"));
            let mut pixels = Vec::with_capacity((width * height) as usize);
            for _ in 0..height {
                try!(reader.read_exact(&mut row_buffer));
                for &byte in &row_buffer {
                    pixels.push(try!(Color::from_byte(byte)));
                }
                try!(read_exactly(reader.by_ref(), b"\n"));
            }
            images.push(Image {
                width: width,
                height: height,
                pixels: pixels.into_boxed_slice(),
            })
        }
        Ok(images)
    }

    /// Writes a group of images to an AHI file.  Returns an error if the
    /// images aren't all the same dimensions.
    pub fn write_all<W: Write>(mut writer: W,
                               images: &[Image])
                               -> io::Result<()> {
        let (width, height) = if images.is_empty() {
            (0, 0)
        } else {
            (images[0].width, images[0].height)
        };
        try!(write!(writer,
                    "ahi0 w{} h{} n{}\n",
                    width,
                    height,
                    images.len()));
        for image in images {
            if image.width != width || image.height != height {
                let msg = format!("images must all have the same dimensions \
                                   (found {}x{} instead of {}x{})",
                                  image.width,
                                  image.height,
                                  width,
                                  height);
                return Err(Error::new(ErrorKind::InvalidInput, msg));
            }
            try!(write!(writer, "\n"));
            for row in 0..height {
                for col in 0..width {
                    let color = image.pixels[(row * width + col) as usize];
                    try!(writer.write_all(&[color.to_byte()]));
                }
                try!(write!(writer, "\n"));
            }
        }
        Ok(())
    }

    /// Sets all pixels in the image to transparent.
    pub fn clear(&mut self) {
        self.pixels = vec![Color::Transparent; self.pixels.len()]
                          .into_boxed_slice();
    }

    /// Sets all pixels in the specified rectangle to the given color.
    pub fn fill_rect(&mut self,
                     x: i32,
                     y: i32,
                     w: u32,
                     h: u32,
                     color: Color) {
        let start_row = min(max(0, y) as u32, self.height);
        let end_row = min(max(0, y + h as i32) as u32, self.height);
        let start_col = min(max(0, x) as u32, self.width);
        let end_col = min(max(0, x + w as i32) as u32, self.width);
        for row in start_row..end_row {
            for col in start_col..end_col {
                self[(col, row)] = color;
            }
        }
    }

    /// Draws pixels from `src` onto this image, placing the top-left corner of
    /// `src` at coordinates `(x, y)`.
    pub fn draw(&mut self, src: &Image, x: i32, y: i32) {
        let src_start_row = min(max(0, -y) as u32, src.height);
        let src_start_col = min(max(0, -x) as u32, src.width);
        let dest_start_row = min(max(0, y) as u32, self.height);
        let dest_start_col = min(max(0, x) as u32, self.width);
        let num_rows = min(src.height - src_start_row,
                           self.height - dest_start_row);
        let num_cols = min(src.width - src_start_col,
                           self.width - dest_start_col);
        for row in 0..num_rows {
            for col in 0..num_cols {
                let color = src[(src_start_col + col, src_start_row + row)];
                if color != Color::Transparent {
                    self[(dest_start_col + col, dest_start_row + row)] = color;
                }
            }
        }
    }

    /// Returns a copy of the image that has been flipped horizontally.
    pub fn flip_horz(&self) -> Image {
        let mut pixels = Vec::with_capacity(self.pixels.len());
        for row in 0..self.height {
            let offset = row * self.width;
            for col in 0..self.width {
                let index = offset + self.width - col - 1;
                pixels.push(self.pixels[index as usize]);
            }
        }
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels.into_boxed_slice(),
        }
    }

    /// Returns a copy of the image that has been flipped vertically.
    pub fn flip_vert(&self) -> Image {
        let mut pixels = Vec::with_capacity(self.pixels.len());
        for row in 0..self.height {
            let offset = (self.height - row - 1) * self.width;
            for col in 0..self.width {
                let index = offset + col;
                pixels.push(self.pixels[index as usize]);
            }
        }
        Image {
            width: self.width,
            height: self.height,
            pixels: pixels.into_boxed_slice(),
        }
    }

    /// Returns a copy of the image that has been rotated 90 degrees clockwise.
    pub fn rotate_cw(&self) -> Image {
        let mut pixels = Vec::with_capacity(self.pixels.len());
        for row in 0..self.width {
            for col in 0..self.height {
                let index = self.width * (self.height - col - 1) + row;
                pixels.push(self.pixels[index as usize]);
            }
        }
        Image {
            width: self.height,
            height: self.width,
            pixels: pixels.into_boxed_slice(),
        }
    }

    /// Returns a copy of the image that has been rotated 90 degrees
    /// counterclockwise.
    pub fn rotate_ccw(&self) -> Image {
        let mut pixels = Vec::with_capacity(self.pixels.len());
        for row in 0..self.width {
            for col in 0..self.height {
                let index = self.width * col + (self.width - row - 1);
                pixels.push(self.pixels[index as usize]);
            }
        }
        Image {
            width: self.height,
            height: self.width,
            pixels: pixels.into_boxed_slice(),
        }
    }

    /// Returns a copy of the image, cropped to the given size.  If the new
    /// width/height is less than the current value, pixels are removed from
    /// the right/bottom; if the new width/height is greater than the current
    /// value, extra transparent pixels are added to the right/bottom.
    pub fn crop(&self, new_width: u32, new_height: u32) -> Image {
        let mut new_image = Image::new(new_width, new_height);
        new_image.draw(self, 0, 0);
        new_image
    }
}

impl std::ops::Index<(u32, u32)> for Image {
    type Output = Color;
    fn index(&self, index: (u32, u32)) -> &Color {
        let (col, row) = index;
        if col >= self.width || row >= self.height {
            panic!("index out of range");
        }
        &self.pixels[(row * self.width + col) as usize]
    }
}

impl std::ops::IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Color {
        let (col, row) = index;
        if col >= self.width || row >= self.height {
            panic!("index out of range");
        }
        &mut self.pixels[(row * self.width + col) as usize]
    }
}

// ========================================================================= //

/// An image for a single character in a font, along with spacing information.
#[derive(Clone)]
pub struct Glyph {
    image: Image,
    left: i32,
    right: i32,
}

impl Glyph {
    /// Creates a new glyph with the given image and left/right edges.
    pub fn new(image: Image, left: i32, right: i32) -> Glyph {
        Glyph {
            image: image,
            left: left,
            right: right,
        }
    }

    /// Returns the image for this glyph.
    pub fn image(&self) -> &Image {
        &self.image
    }

    /// Returns a mutable reference to the image for this glyph.
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
    }

    /// Returns the left edge of this glyph, in pixels.  This is the distance
    /// (possibly negative) to the right of the left edge of this glyph's image
    /// at which this glyph should start.  Normally this is zero, but can be
    /// positive if the glyph needs extra space on the left, or negative if the
    /// image should extend to the left (e.g. the tail on a lowercase j).
    pub fn left_edge(&self) -> i32 {
        self.left
    }

    /// Sets the left edge for this glyph.
    pub fn set_left_edge(&mut self, left: i32) {
        self.left = left;
    }

    /// Returns the right edge of this glyph, in pixels.  This is the distance
    /// (possibly negative) to the right of the left edge of this glyph's image
    /// at which the next glyph in the text should start, which may be slightly
    /// different than the width of this glyph's image (e.g. if this is an
    /// italic glyph whose image should extend a bit over that of the next
    /// glyph).
    pub fn right_edge(&self) -> i32 {
        self.right
    }

    /// Sets the right edge for this glyph.
    pub fn set_right_edge(&mut self, right: i32) {
        self.right = right;
    }
}

// ========================================================================= //

/// Represents a mapping from characters to glyphs.
#[derive(Clone)]
pub struct Font {
    glyphs: BTreeMap<char, Rc<Glyph>>,
    default_glyph: Rc<Glyph>,
    baseline: i32,
}

impl Font {
    /// Creates a new, empty font whose glyphs have the given height, in
    /// pixels.  The initial default glyph will be a zero-width space, and the
    /// initial baseline will be equal to the height.
    pub fn with_glyph_height(height: u32) -> Font {
        Font {
            glyphs: BTreeMap::new(),
            default_glyph: Rc::new(Glyph::new(Image::new(0, height), 0, 0)),
            baseline: height as i32,
        }
    }

    /// Returns the image height of the glyphs in this font, in pixels.
    pub fn glyph_height(&self) -> u32 {
        self.default_glyph.image.height()
    }

    /// Returns the baseline height for this font, measured in pixels down from
    /// the top of the glyph.  The baseline is the line on which e.g. most
    /// numerals and capital letters sit, and below which e.g. the tail on a
    /// lowercase 'g' or 'y' extends.
    pub fn baseline(&self) -> i32 {
        self.baseline
    }

    /// Sets the baseline value for this font, measured in pixels down from the
    /// top of the glyph.  It is customary for the baseline to be in the range
    /// (0, `height`], but note that this is not actually required.
    pub fn set_baseline(&mut self, baseline: i32) {
        self.baseline = baseline;
    }

    /// Gets the glyph for the given character, if any.  If you instead want to
    /// get the default glyph for characters that have no glyph, use the index
    /// operator.
    pub fn get_char_glyph(&self, chr: char) -> Option<&Glyph> {
        match self.glyphs.get(&chr) {
            Some(glyph) => Some(glyph.deref()),
            None => None,
        }
    }

    /// Gets a mutable reference to the glyph for the given character, if any.
    pub fn get_char_glyph_mut(&mut self, chr: char) -> Option<&mut Glyph> {
        match self.glyphs.get_mut(&chr) {
            Some(glyph) => Some(Rc::make_mut(glyph)),
            None => None,
        }
    }

    /// Sets the glyph for the given character.  Panics if the new glyph's
    /// height is not equal to the font's glyph height.
    pub fn set_char_glyph(&mut self, chr: char, glyph: Glyph) {
        assert_eq!(glyph.image().height(), self.glyph_height());
        self.glyphs.insert(chr, Rc::new(glyph));
    }

    /// Removes the glyph for the given character from the font.  After calling
    /// this, the font's default glyph will be used for this character.
    pub fn remove_char_glyph(&mut self, chr: char) {
        self.glyphs.remove(&chr);
    }

    /// Gets the default glyph for this font, which is used for characters that
    /// don't have a glyph.
    pub fn default_glyph(&self) -> &Glyph {
        &self.default_glyph
    }

    /// Gets a mutable reference to the default glyph for this font.
    pub fn default_glyph_mut(&mut self) -> &mut Glyph {
        Rc::make_mut(&mut self.default_glyph)
    }

    /// Sets the default glyph for this font.  Panics if the new glyph's height
    /// is not equal to the font's glyph height.
    pub fn set_default_glyph(&mut self, glyph: Glyph) {
        assert_eq!(glyph.image().height(), self.glyph_height());
        self.default_glyph = Rc::new(glyph);
    }

    /// Returns an iterator over the characters that have glyphs in this font.
    pub fn chars(&self) -> Chars {
        Chars { iter: self.glyphs.keys() }
    }

    /// Reads a font from an AHF file.
    pub fn read<R: Read>(mut reader: R) -> io::Result<Font> {
        try!(read_exactly(reader.by_ref(), b"ahf"));
        let version = try!(read_header_uint(reader.by_ref(), b' '));
        if version != 0 {
            let msg = format!("unsupported AHF version: {}", version);
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }
        try!(read_exactly(reader.by_ref(), b"h"));
        let height = try!(read_header_uint(reader.by_ref(), b' '));
        try!(read_exactly(reader.by_ref(), b"b"));
        let baseline = try!(read_header_int(reader.by_ref(), b' '));
        try!(read_exactly(reader.by_ref(), b"n"));
        let num_glyphs = try!(read_header_uint(reader.by_ref(), b'\n'));

        try!(read_exactly(reader.by_ref(), b"\ndef "));
        let default_glyph = try!(Font::read_glyph(reader.by_ref(), height));

        let mut glyphs = BTreeMap::new();
        for _ in 0..num_glyphs {
            try!(read_exactly(reader.by_ref(), b"\n'"));
            let chr = try!(read_char_escape(reader.by_ref()));
            try!(read_exactly(reader.by_ref(), b"' "));
            let glyph = try!(Font::read_glyph(reader.by_ref(), height));
            glyphs.insert(chr, Rc::new(glyph));
        }
        Ok(Font {
            glyphs: glyphs,
            default_glyph: Rc::new(default_glyph),
            baseline: baseline,
        })
    }

    fn read_glyph<R: Read>(mut reader: R, height: u32) -> io::Result<Glyph> {
        try!(read_exactly(reader.by_ref(), b"w"));
        let width = try!(read_header_uint(reader.by_ref(), b' '));
        try!(read_exactly(reader.by_ref(), b"l"));
        let left = try!(read_header_int(reader.by_ref(), b' '));
        try!(read_exactly(reader.by_ref(), b"r"));
        let right = try!(read_header_int(reader.by_ref(), b'\n'));
        let mut row_buffer = vec![0u8; width as usize];
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for _ in 0..height {
            try!(reader.read_exact(&mut row_buffer));
            for &byte in &row_buffer {
                pixels.push(try!(Color::from_byte(byte)));
            }
            try!(read_exactly(reader.by_ref(), b"\n"));
        }
        let image = Image {
            width: width,
            height: height,
            pixels: pixels.into_boxed_slice(),
        };
        Ok(Glyph {
            image: image,
            left: left,
            right: right,
        })
    }

    /// Writes the font to an AHF file.
    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        let height = self.glyph_height();
        try!(write!(writer,
                    "ahf0 h{} b{} n{}\n",
                    height,
                    self.baseline(),
                    self.glyphs.len()));
        try!(write!(writer, "\ndef "));
        try!(Font::write_glyph(writer.by_ref(), &self.default_glyph));
        for (chr, glyph) in self.glyphs.iter() {
            let escaped: String = chr.escape_default().collect();
            try!(write!(writer, "\n'{}' ", escaped));
            try!(Font::write_glyph(writer.by_ref(), glyph));
        }
        Ok(())
    }

    fn write_glyph<W: Write>(mut writer: W, glyph: &Glyph) -> io::Result<()> {
        let image = glyph.image();
        let width = image.width();
        let height = image.height();
        try!(write!(writer,
                    "w{} l{} r{}\n",
                    width,
                    glyph.left_edge(),
                    glyph.right_edge()));
        for row in 0..height {
            for col in 0..width {
                let color = image[(col, row)];
                try!(writer.write_all(&[color.to_byte()]));
            }
            try!(write!(writer, "\n"));
        }
        Ok(())
    }
}

impl std::ops::Index<char> for Font {
    type Output = Glyph;
    fn index(&self, index: char) -> &Glyph {
        self.glyphs.get(&index).unwrap_or(&self.default_glyph)
    }
}

impl std::ops::IndexMut<char> for Font {
    fn index_mut(&mut self, index: char) -> &mut Glyph {
        Rc::make_mut(match self.glyphs.get_mut(&index) {
            Some(glyph) => glyph,
            None => &mut self.default_glyph,
        })
    }
}

// ========================================================================= //

/// An iterator over a the characters that have glyphs in a font.
pub struct Chars<'a> {
    iter: btree_map::Keys<'a, char, Rc<Glyph>>,
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.iter.next().map(|&chr| chr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Chars<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

// ========================================================================= //

fn read_exactly<R: Read>(mut reader: R, expected: &[u8]) -> io::Result<()> {
    let mut actual = vec![0u8; expected.len()];
    try!(reader.read_exact(&mut actual));
    if &actual as &[u8] != expected {
        let msg = format!("expected '{}', found '{}'",
                          String::from_utf8_lossy(expected),
                          String::from_utf8_lossy(&actual));
        Err(Error::new(ErrorKind::InvalidData, msg))
    } else {
        Ok(())
    }
}

fn read_char_escape<R: Read>(mut reader: R) -> io::Result<char> {
    let mut buffer = vec![0u8];
    try!(reader.read_exact(&mut buffer));
    let byte = buffer[0];
    if byte == b'\\' {
        try!(reader.read_exact(&mut buffer));
        let esc = buffer[0];
        if esc == b'\\' {
            Ok('\\')
        } else if esc == b'\'' {
            Ok('\'')
        } else if esc == b'"' {
            Ok('"')
        } else if esc == b'n' {
            Ok('\n')
        } else if esc == b'r' {
            Ok('\r')
        } else if esc == b't' {
            Ok('\t')
        } else if esc == b'u' {
            try!(read_exactly(reader.by_ref(), b"{"));
            let value = try!(read_hex_u32(reader.by_ref(), b'}'));
            std::char::from_u32(value).ok_or_else(|| {
                let msg = format!("invalid unicode value: {}", value);
                Error::new(ErrorKind::InvalidData, msg)
            })
        } else {
            let msg = format!("invalid char escape: {}", esc);
            Err(Error::new(ErrorKind::InvalidData, msg))
        }
    } else if byte < b' ' || byte > b'~' || byte == b'\'' {
        let msg = format!("invalid char literal byte: {}", byte);
        Err(Error::new(ErrorKind::InvalidData, msg))
    } else {
        Ok(std::char::from_u32(byte as u32).unwrap())
    }
}

const MAX_HEADER_VALUE: i32 = 0xFFFF;

fn read_header_int<R: Read>(reader: R, terminator: u8) -> io::Result<i32> {
    let mut negative = false;
    let mut any_digits = false;
    let mut value: i32 = 0;
    for next in reader.bytes() {
        let byte = try!(next);
        if byte == terminator {
            if !any_digits {
                let msg = "missing integer field in header";
                return Err(Error::new(ErrorKind::InvalidData, msg));
            }
            break;
        } else if byte == b'-' {
            if negative || any_digits {
                let msg = "misplaced minus sign in header field";
                return Err(Error::new(ErrorKind::InvalidData, msg));
            }
            negative = true;
        } else if byte < b'0' || byte > b'9' {
            let msg = format!("invalid byte in header field: '{}'",
                              String::from_utf8_lossy(&[byte]));
            return Err(Error::new(ErrorKind::InvalidData, msg));
        } else {
            value = value * 10 + (byte - b'0') as i32;
            if value > MAX_HEADER_VALUE {
                let msg = "header value is too large";
                return Err(Error::new(ErrorKind::InvalidData, msg));
            }
            any_digits = true;
        }
    }
    if negative {
        value = -value;
    }
    Ok(value)
}

fn read_header_uint<R: Read>(reader: R, terminator: u8) -> io::Result<u32> {
    let value = try!(read_header_int(reader, terminator));
    if value < 0 {
        let msg = format!("value must be nonnegative (was {})", value);
        return Err(Error::new(ErrorKind::InvalidData, msg));
    }
    Ok(value as u32)
}

fn read_hex_u32<R: Read>(reader: R, terminator: u8) -> io::Result<u32> {
    let mut any_digits = false;
    let mut value: u32 = 0;
    for next in reader.bytes() {
        let byte = try!(next);
        if byte == terminator {
            if !any_digits {
                let msg = "missing hex literal";
                return Err(Error::new(ErrorKind::InvalidData, msg));
            }
            break;
        }
        let digit = if byte >= b'0' && byte <= b'9' {
            byte - b'0'
        } else if byte >= b'a' && byte <= b'f' {
            byte - b'a' + 0xa
        } else if byte >= b'A' && byte <= b'F' {
            byte - b'A' + 0xA
        } else {
            let msg = format!("invalid hex digit: '{}'",
                              String::from_utf8_lossy(&[byte]));
            return Err(Error::new(ErrorKind::InvalidData, msg));
        };
        if value > 0xFFFFFFF {
            let msg = "hex literal is too large";
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }
        value = value * 0x10 + digit as u32;
        any_digits = true;
    }
    Ok(value)
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_rgba_data() {
        let mut image = Image::new(2, 2);
        image[(0, 0)] = Color::DarkRed;
        image[(0, 1)] = Color::Green;
        image[(1, 1)] = Color::Cyan;
        assert_eq!(image.rgba_data(),
                   vec![127, 0, 0, 255, 0, 0, 0, 0, 0, 255, 0, 255, 0, 255,
                        255, 255]);
    }

    #[test]
    fn read_zero_images() {
        let input: &[u8] = b"ahi0 w0 h0 n0";
        let images = Image::read_all(input).expect("failed to read images");
        assert_eq!(images.len(), 0);
    }

    #[test]
    fn read_two_images() {
        let input: &[u8] = b"ahi0 w2 h2 n2\n\
                             \n\
                             20\n\
                             5D\n\
                             \n\
                             E0\n\
                             0E\n";
        let images = Image::read_all(input).expect("failed to read images");
        assert_eq!(images.len(), 2);
        assert_eq!(images[0][(0, 1)], Color::Green);
        assert_eq!(images[1][(0, 0)], Color::Gray);
    }

    #[test]
    fn write_zero_images() {
        let mut output = Vec::<u8>::new();
        Image::write_all(&mut output, &[]).expect("failed to write images");
        assert_eq!(&output as &[u8], b"ahi0 w0 h0 n0\n");
    }

    #[test]
    fn write_two_images() {
        let mut image0 = Image::new(2, 2);
        image0[(0, 0)] = Color::DarkRed;
        image0[(0, 1)] = Color::Green;
        image0[(1, 1)] = Color::Cyan;
        let mut image1 = Image::new(2, 2);
        image1[(0, 0)] = Color::Gray;
        image1[(1, 1)] = Color::Gray;
        let mut output = Vec::<u8>::new();
        Image::write_all(&mut output, &[image0, image1])
            .expect("failed to write images");
        assert_eq!(&output as &[u8],
                   b"ahi0 w2 h2 n2\n\
                     \n\
                     20\n\
                     5D\n\
                     \n\
                     E0\n\
                     0E\n");
    }

    #[test]
    fn clear_image() {
        let mut image = Image::new(2, 2);
        image[(1, 0)] = Color::DarkRed;
        image[(1, 1)] = Color::Green;
        image.clear();
        assert_eq!(image[(1, 0)], Color::Transparent);
        assert_eq!(image[(1, 1)], Color::Transparent);
    }

    #[test]
    fn flip_image_horz() {
        let mut image = Image::new(2, 2);
        image[(0, 1)] = Color::Red;
        image[(1, 1)] = Color::Green;
        let image = image.flip_horz();
        assert_eq!(image[(0, 1)], Color::Green);
        assert_eq!(image[(1, 1)], Color::Red);
    }

    #[test]
    fn flip_image_vert() {
        let mut image = Image::new(2, 2);
        image[(1, 0)] = Color::Red;
        image[(1, 1)] = Color::Green;
        let image = image.flip_vert();
        assert_eq!(image[(1, 0)], Color::Green);
        assert_eq!(image[(1, 1)], Color::Red);
    }

    #[test]
    fn rotate_image_cw() {
        let mut image = Image::new(4, 2);
        image[(1, 0)] = Color::Red;
        image[(1, 1)] = Color::Green;
        let image = image.rotate_cw();
        assert_eq!(2, image.width());
        assert_eq!(4, image.height());
        assert_eq!(image[(1, 1)], Color::Red);
        assert_eq!(image[(0, 1)], Color::Green);
    }

    #[test]
    fn rotate_image_ccw() {
        let mut image = Image::new(4, 2);
        image[(1, 0)] = Color::Red;
        image[(1, 1)] = Color::Green;
        let image = image.rotate_ccw();
        assert_eq!(2, image.width());
        assert_eq!(4, image.height());
        assert_eq!(image[(0, 2)], Color::Red);
        assert_eq!(image[(1, 2)], Color::Green);
    }

    #[test]
    fn fill_contained_rect() {
        let mut image = Image::new(5, 5);
        image.fill_rect(1, 1, 2, 2, Color::Red);
        let mut output = Vec::<u8>::new();
        Image::write_all(&mut output, &[image])
            .expect("failed to write image");
        assert_eq!(&output as &[u8],
                   b"ahi0 w5 h5 n1\n\
                     \n\
                     00000\n\
                     03300\n\
                     03300\n\
                     00000\n\
                     00000\n" as &[u8]);
    }

    #[test]
    fn fill_overlapping_rect() {
        let mut image = Image::new(5, 3);
        image.fill_rect(2, 1, 7, 7, Color::Red);
        let mut output = Vec::<u8>::new();
        Image::write_all(&mut output, &[image])
            .expect("failed to write image");
        assert_eq!(&output as &[u8],
                   b"ahi0 w5 h3 n1\n\
                     \n\
                     00000\n\
                     00333\n\
                     00333\n" as &[u8]);
    }

    #[test]
    fn draw_overlapping() {
        let input: &[u8] = b"ahi0 w5 h3 n2\n\
                             \n\
                             EEEEE\n\
                             EEEEE\n\
                             EEEEE\n\
                             \n\
                             01110\n\
                             11011\n\
                             01110\n";
        let images = Image::read_all(input).expect("failed to read images");
        let mut image = images[0].clone();
        image.draw(&images[1], -1, 1);
        let mut output = Vec::<u8>::new();
        Image::write_all(&mut output, &[image])
            .expect("failed to write image");
        assert_eq!(&output as &[u8],
                   b"ahi0 w5 h3 n1\n\
                     \n\
                     EEEEE\n\
                     111EE\n\
                     1E11E\n" as &[u8]);
    }

    #[test]
    fn read_font() {
        let input: &[u8] = b"ahf0 h3 b2 n2\n\
            \n\
            def w3 l0 r4\n\
            101\n\
            010\n\
            101\n\
            \n\
            '|' w1 l0 r2\n\
            1\n\
            1\n\
            1\n\
            \n\
            '\\u{2603}' w2 l0 r4\n\
            11\n\
            11\n\
            00\n";
        let font = Font::read(input).expect("failed to read font");
        assert_eq!(font.glyph_height(), 3);
        assert_eq!(font.baseline(), 2);
        assert_eq!(font.default_glyph().image().width(), 3);
        assert_eq!(font.default_glyph().left_edge(), 0);
        assert_eq!(font.default_glyph().right_edge(), 4);
        assert_eq!(font['|'].image().width(), 1);
        assert_eq!(font.chars().len(), 2);
    }

    #[test]
    fn write_font() {
        let mut font = Font::with_glyph_height(3);
        font.set_baseline(2);

        let mut img_default = Image::new(3, 3);
        img_default[(0, 0)] = Color::Black;
        img_default[(2, 0)] = Color::Black;
        img_default[(1, 1)] = Color::Black;
        img_default[(0, 2)] = Color::Black;
        img_default[(2, 2)] = Color::Black;
        font.set_default_glyph(Glyph::new(img_default, 0, 4));

        let mut img_snowman = Image::new(2, 3);
        img_snowman[(0, 0)] = Color::Black;
        img_snowman[(1, 0)] = Color::Black;
        img_snowman[(0, 1)] = Color::Black;
        img_snowman[(1, 1)] = Color::Black;
        font.set_char_glyph('\u{2603}', Glyph::new(img_snowman, 0, 4));

        let mut img_vbar = Image::new(1, 3);
        img_vbar[(0, 0)] = Color::Black;
        img_vbar[(0, 1)] = Color::Black;
        img_vbar[(0, 2)] = Color::Black;
        font.set_char_glyph('|', Glyph::new(img_vbar, 0, 2));

        let mut output = Vec::<u8>::new();
        font.write(&mut output).expect("failed to write font");
        let mut expected = Vec::<u8>::new();
        expected.extend_from_slice(b"ahf0 h3 b2 n2\n\
            \n\
            def w3 l0 r4\n\
            101\n\
            010\n\
            101\n\
            \n\
            '|' w1 l0 r2\n\
            1\n\
            1\n\
            1\n\
            \n\
            '\\u{2603}' w2 l0 r4\n\
            11\n\
            11\n\
            00\n");
        assert_eq!(output, expected);
    }
}

// ========================================================================= //
