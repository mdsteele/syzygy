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

use sdl2::rect::Point;
use std::io::{self, Read};
use std::path::Path;

use gui::Sprite;

// ========================================================================= //

const NUM_COLS: u32 = 36;
const NUM_ROWS: u32 = 24;
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;

// ========================================================================= //

pub struct Background {
    color: (u8, u8, u8),
    tiles: Vec<Option<Sprite>>,
}

impl Background {
    pub fn load<R, F>(path: &Path, mut file: R, mut get_sprites: F)
                      -> io::Result<Background>
    where
        R: Read,
        F: FnMut(&str) -> Vec<Sprite>,
    {
        read_exactly(file.by_ref(), b"@BG ")?;
        let red = read_int(file.by_ref(), b' ')? as u8;
        let green = read_int(file.by_ref(), b' ')? as u8;
        let blue = read_int(file.by_ref(), b'\n')? as u8;
        let mut tileset: Vec<(String, Vec<Sprite>)> = Vec::new();
        loop {
            match read_byte(file.by_ref())? {
                b'>' => {
                    let filename = read_string(file.by_ref(), b'\n')?;
                    let sprites = get_sprites(&filename);
                    tileset.push((filename, sprites));
                }
                b'\n' => break,
                byte => {
                    let msg = format!("unexpected byte: {}", byte);
                    return Err(io::Error::new(io::ErrorKind::InvalidData,
                                              msg));
                }
            }
        }
        let mut used_file = vec![false; tileset.len()];
        let mut tiles: Vec<Option<Sprite>> = Vec::new();
        for _ in 0..NUM_ROWS {
            let mut col = 0;
            loop {
                let byte1 = read_byte(file.by_ref())?;
                if byte1 == b'\n' {
                    for _ in col..NUM_COLS {
                        tiles.push(None);
                    }
                    break;
                }
                if col >= NUM_COLS {
                    return Err(io::Error::new(io::ErrorKind::InvalidData,
                                              "too many columns"));
                }
                let byte2 = read_byte(file.by_ref())?;
                if byte1 == b' ' && byte2 == b' ' {
                    tiles.push(None);
                } else {
                    let file_index = base62_index(byte1, tileset.len())?;
                    let sprites = &tileset[file_index].1;
                    let tile_index = base62_index(byte2, sprites.len())?;
                    let sprite = &sprites[tile_index];
                    tiles.push(Some(sprite.clone()));
                    used_file[file_index] = true;
                }
                col += 1;
            }
        }
        for file_index in 0..tileset.len() {
            if !used_file[file_index] {
                println!("WARNING: {:?} doesn't use {}",
                         path,
                         tileset[file_index].0);
            }
        }
        Ok(Background {
               color: (red, green, blue),
               tiles: tiles,
           })
    }

    pub fn color(&self) -> (u8, u8, u8) { self.color }

    pub fn tiles(&self) -> Tiles {
        Tiles {
            background: self,
            col: 0,
            row: 0,
        }
    }
}

// ========================================================================= //

pub struct Tiles<'a> {
    background: &'a Background,
    col: u32,
    row: u32,
}

impl<'a> Iterator for Tiles<'a> {
    type Item = (&'a Sprite, Point);

    fn next(&mut self) -> Option<(&'a Sprite, Point)> {
        loop {
            if self.col >= NUM_COLS {
                self.col = 0;
                self.row += 1;
            }
            if self.row >= NUM_ROWS {
                return None;
            }
            let index = (self.row * NUM_COLS + self.col) as usize;
            if let Some(ref sprite) = self.background.tiles[index] {
                let point = Point::new((self.col * TILE_WIDTH) as i32,
                                       (self.row * TILE_HEIGHT) as i32);
                self.col += 1;
                return Some((sprite, point));
            } else {
                self.col += 1;
            }
        }
    }
}

// ========================================================================= //

fn base62_index(byte: u8, max: usize) -> io::Result<usize> {
    let index = match byte {
        b'A'...b'Z' => (byte - b'A') as usize,
        b'a'...b'z' => (byte - b'a') as usize + 26,
        b'0'...b'9' => (byte - b'0') as usize + 52,
        _ => {
            let msg = format!("invalid index byte: {}", byte);
            return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
        }
    };
    if index >= max {
        let msg = format!("index {} out of range 0..{}", index, max);
        return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
    }
    Ok(index)
}

fn read_byte<R: io::Read>(reader: R) -> io::Result<u8> {
    match reader.bytes().next() {
        Some(result) => result,
        None => {
            let msg = "unexpected EOF";
            Err(io::Error::new(io::ErrorKind::InvalidData, msg))
        }
    }
}

fn read_exactly<R: io::Read>(mut reader: R, string: &[u8]) -> io::Result<()> {
    let mut actual = vec![0u8; string.len()];
    reader.read_exact(&mut actual)?;
    if &actual as &[u8] != string {
        let msg = format!("expected '{}', found '{}'",
                          String::from_utf8_lossy(string),
                          String::from_utf8_lossy(&actual));
        Err(io::Error::new(io::ErrorKind::InvalidData, msg))
    } else {
        Ok(())
    }
}

fn read_int<R: io::Read>(reader: R, terminator: u8) -> io::Result<u32> {
    let mut value: u32 = 0;
    for next in reader.bytes() {
        let byte = next?;
        if byte == terminator {
            break;
        }
        let digit: u8;
        if b'0' <= byte && byte <= b'9' {
            digit = byte - b'0';
        } else {
            let msg = format!("invalid character in header field: '{}'",
                              String::from_utf8_lossy(&[byte]));
            return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
        }
        value = value * 10 + digit as u32;
        if value > 0xFFFF {
            let msg = "value is too large";
            return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
        }
    }
    Ok(value)
}

fn read_string<R: io::Read>(reader: R, terminator: u8) -> io::Result<String> {
    let mut result = Vec::new();
    for next in reader.bytes() {
        let byte = next?;
        if byte == terminator {
            break;
        }
        result.push(byte);
    }
    String::from_utf8(result).map_err(|_| {
        let msg = "invalid utf8";
        io::Error::new(io::ErrorKind::InvalidData, msg)
    })
}

// ========================================================================= //
