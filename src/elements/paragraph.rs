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

use std::cmp;
use std::mem;
use std::rc::Rc;
use std::str::Chars;

use crate::gui::{Align, Canvas, Font, Point, Resources};

// ========================================================================= //

// Formatting:
// $$ -- literal $
// $L -- align left (default)
// $C -- align centered
// $R -- align right
// $r -- use roman font (default)
// $i -- use italic font
// $f{name} -- use <name> font
// $M{foo}{bar} -- expands to "foo" on mobile or "bar" on desktop
//
// Example:
// "$M{Tap}{Click} the button to be $ireally$r awesome."

const LINE_SPACING: i32 = 4;
const MIN_LINE_HEIGHT: u32 = 10;

// ========================================================================= //

pub struct Paragraph {
    lines: Vec<Line>,
}

impl Paragraph {
    pub fn new(resources: &mut Resources, init_font: &str,
               init_align: Align, text: &str)
               -> Paragraph {
        let mut parser = Parser::new(resources, init_font, init_align);
        let mut chars = text.chars();
        while let Some(chr) = chars.next() {
            if chr == '$' {
                match chars.next() {
                    Some('$') => parser.push('$'),
                    Some('L') => parser.set_align(Align::Left),
                    Some('C') => parser.set_align(Align::Center),
                    Some('R') => parser.set_align(Align::Right),
                    Some('r') => parser.set_font("roman".to_string()),
                    Some('i') => parser.set_font("italic".to_string()),
                    Some('f') => parser.set_font(parse_arg(&mut chars)),
                    Some('M') => {
                        let mobile = parse_arg(&mut chars);
                        let desktop = parse_arg(&mut chars);
                        if cfg!(any(target_os = "android",
                                    target_os = "ios"))
                        {
                            parser.push_str(&mobile);
                        } else {
                            parser.push_str(&desktop);
                        }
                    }
                    _ => {}
                }
            } else if chr == '\n' {
                parser.newline();
            } else {
                parser.push(chr);
            }
        }
        parser.finish()
    }

    pub fn min_width(&self) -> i32 {
        let mut width = 0;
        for line in self.lines.iter() {
            width = cmp::max(width, line.min_width());
        }
        width
    }

    pub fn height(&self) -> u32 {
        let mut height = 0;
        for line in self.lines.iter() {
            if height > 0 {
                height += LINE_SPACING as u32;
            }
            height += line.height();
        }
        height
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        let mut top = 0;
        for line in self.lines.iter() {
            line.draw(canvas, top);
            top += line.height() as i32 + LINE_SPACING;
        }
    }
}

// ========================================================================= //

struct Line {
    left: Vec<Piece>,
    center: Vec<Piece>,
    right: Vec<Piece>,
}

impl Line {
    fn new() -> Line {
        Line {
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.left.is_empty() && self.center.is_empty() && self.right.is_empty()
    }

    fn baseline(&self) -> i32 {
        let mut baseline = 0;
        for piece in (self.left.iter())
            .chain(self.center.iter())
            .chain(self.right.iter())
        {
            baseline = cmp::max(baseline, piece.baseline());
        }
        baseline
    }

    fn min_width(&self) -> i32 {
        let mut width = 0;
        for piece in (self.left.iter())
            .chain(self.center.iter())
            .chain(self.right.iter())
        {
            width += piece.width();
        }
        width
    }

    fn height(&self) -> u32 {
        let baseline = self.baseline();
        let mut height = MIN_LINE_HEIGHT;
        for piece in (self.left.iter())
            .chain(self.center.iter())
            .chain(self.right.iter())
        {
            height = cmp::max(height,
                              (baseline - piece.baseline()) as u32 +
                                  piece.height());
        }
        height
    }

    fn draw(&self, canvas: &mut Canvas, top: i32) {
        let baseline = top + self.baseline();
        if !self.left.is_empty() {
            let mut left = 0;
            for piece in self.left.iter() {
                piece.draw(canvas, left, baseline);
                left += piece.width();
            }
        }
        if !self.center.is_empty() {
            let mut width = 0;
            for piece in self.center.iter() {
                width += piece.width();
            }
            let mut left = (canvas.width() as i32 - width) / 2;
            for piece in self.center.iter() {
                piece.draw(canvas, left, baseline);
                left += piece.width();
            }
        }
        if !self.right.is_empty() {
            let mut left = canvas.rect().right();
            for piece in self.right.iter() {
                left -= piece.width();
                piece.draw(canvas, left, baseline);
            }
        }
    }
}

// ========================================================================= //

struct Piece {
    font: Rc<Font>,
    text: String,
}

impl Piece {
    fn baseline(&self) -> i32 { self.font.baseline() }

    fn width(&self) -> i32 { self.font.text_width(&self.text) }

    fn height(&self) -> u32 { self.font.height() }

    fn draw(&self, canvas: &mut Canvas, left: i32, baseline: i32) {
        canvas.draw_text(&self.font,
                         Align::Left,
                         Point::new(left, baseline),
                         &self.text);
    }
}

// ========================================================================= //

struct Parser<'a, 'b: 'a> {
    resources: &'a mut Resources<'b>,
    current_font: String,
    current_align: Align,
    current_line: Line,
    current_piece: String,
    lines: Vec<Line>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn new(resources: &'a mut Resources<'b>, font: &str, align: Align)
           -> Parser<'a, 'b> {
        Parser {
            resources: resources,
            current_font: font.to_string(),
            current_align: align,
            current_line: Line::new(),
            current_piece: String::new(),
            lines: Vec::new(),
        }
    }

    fn push(&mut self, chr: char) { self.current_piece.push(chr); }

    fn push_str(&mut self, string: &str) {
        self.current_piece.push_str(string);
    }

    fn newline(&mut self) {
        self.shift_piece();
        let mut line = Line::new();
        mem::swap(&mut line, &mut self.current_line);
        self.lines.push(line);
    }

    fn set_font(&mut self, font: String) {
        if font != self.current_font {
            self.shift_piece();
            self.current_font = font;
        }
    }

    fn set_align(&mut self, align: Align) {
        if align != self.current_align {
            self.shift_piece();
            self.current_align = align;
        }
    }

    fn shift_piece(&mut self) {
        if !self.current_piece.is_empty() {
            let mut text = String::new();
            mem::swap(&mut text, &mut self.current_piece);
            let piece = Piece {
                font: self.resources.get_font(&self.current_font),
                text: text,
            };
            let pieces = match self.current_align {
                Align::Left => &mut self.current_line.left,
                Align::Center => &mut self.current_line.center,
                Align::Right => &mut self.current_line.right,
            };
            pieces.push(piece);
        }
    }

    fn finish(mut self) -> Paragraph {
        self.shift_piece();
        if !self.current_line.is_empty() {
            self.newline();
        }
        Paragraph { lines: self.lines }
    }
}

// ========================================================================= //

fn parse_arg(chars: &mut Chars) -> String {
    let mut result = String::new();
    if chars.next() == Some('{') {
        while let Some(chr) = chars.next() {
            if chr == '}' {
                break;
            } else {
                result.push(chr);
            }
        }
    }
    result
}

// ========================================================================= //
