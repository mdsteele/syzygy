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

use gui::{Canvas, Rect};

// ========================================================================= //

pub struct ProgressBar {
    left: i32,
    top: i32,
    width: u32,
    color: (u8, u8, u8),
}

impl ProgressBar {
    pub fn new((left, top): (i32, i32), width: u32, color: (u8, u8, u8))
               -> ProgressBar {
        ProgressBar {
            left: left + 1,
            top: top + 1,
            width: if width > 2 { width - 2 } else { 0 },
            color: color,
        }
    }

    pub fn draw(&self, value: u32, maximum: u32, canvas: &mut Canvas) {
        let value = cmp::min(value, maximum);
        if value > 0 {
            let width = self.width * value / maximum;
            let rect = Rect::new(self.left, self.top, width, 14);
            canvas.fill_rect(self.color, rect);
        }
    }
}

// ========================================================================= //
