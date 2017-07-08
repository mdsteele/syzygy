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

use num_integer::mod_floor;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources};
use save::column::Columns;

// ========================================================================= //

const BOX_USIZE: u32 = 24;
const BOX_SIZE: i32 = BOX_USIZE as i32;
const ADJUST_SPEED: i32 = 3;

pub struct ColumnsView {
    font: Rc<Font>,
    left: i32,
    top: i32,
    row_spacing: i32,
    hilights: HashMap<usize, (u8, u8, u8)>,
    adjust: HashMap<usize, i32>,
    drag: Option<(usize, i32, i32)>,
}

impl ColumnsView {
    pub fn new(resources: &mut Resources, left: i32, top: i32,
               row_spacing: i32)
               -> ColumnsView {
        ColumnsView {
            font: resources.get_font("block"),
            left: left,
            top: top,
            row_spacing: row_spacing,
            hilights: HashMap::new(),
            adjust: HashMap::new(),
            drag: None,
        }
    }

    pub fn set_hilight_color(&mut self, col: usize, color: (u8, u8, u8)) {
        self.hilights.insert(col, color);
    }

    fn column_rect(&self, columns: &Columns, col: usize) -> Rect {
        let mut left = self.left + 32 * col as i32;
        let mut top = self.top + BOX_SIZE * columns.column_offset(col);
        let half = columns.num_columns() / 2;
        if self.row_spacing != 0 && col >= half {
            left -= 32 * half as i32;
            top += self.row_spacing;
        }
        let num_chars = columns.column_word_len(col) as u32;
        Rect::new(left, top, BOX_USIZE, BOX_USIZE * num_chars)
    }

    fn column_scroll(&self, columns: &Columns, col: usize) -> i32 {
        let mut scroll = self.adjust.get(&col).cloned().unwrap_or(0);
        if let Some((drag_col, from, to)) = self.drag {
            for &(other, factor) in columns.column_linkages(drag_col) {
                if other == col {
                    scroll += (to - from) * factor;
                    break;
                }
            }
        }
        scroll
    }
}

impl Element<Columns, (usize, i32)> for ColumnsView {
    fn draw(&self, columns: &Columns, canvas: &mut Canvas) {
        for col in 0..columns.num_columns() {
            let mut canvas = canvas.subcanvas(self.column_rect(columns, col));
            canvas.clear((0, 0, 0));
            let hilight_color =
                self.hilights.get(&col).cloned().unwrap_or((63, 31, 63));
            let hilight_rect = Rect::new(0,
                                         -BOX_SIZE *
                                         columns.column_offset(col),
                                         BOX_USIZE,
                                         BOX_USIZE);
            canvas.fill_rect(hilight_color, hilight_rect);
            let height = canvas.height() as i32;
            let offset = mod_floor(self.column_scroll(columns, col), height);
            for (index, &chr) in columns.column_letters(col)
                                        .iter()
                                        .enumerate() {
                let top = BOX_SIZE * index as i32 + offset;
                let pt = Point::new(BOX_SIZE / 2, top + BOX_SIZE - 3);
                canvas.draw_char(&self.font, Align::Center, pt, chr);
                let pt = Point::new(BOX_SIZE / 2, top + BOX_SIZE - 3 - height);
                canvas.draw_char(&self.font, Align::Center, pt, chr);
            }
            let rect = canvas.rect();
            canvas.draw_rect((255, 255, 255), rect);
        }
    }

    fn handle_event(&mut self, event: &Event, columns: &mut Columns)
                    -> Action<(usize, i32)> {
        match event {
            &Event::ClockTick => {
                let any_adjustments = !self.adjust.is_empty();
                let mut done = Vec::new();
                for (&col, adjust) in self.adjust.iter_mut() {
                    if *adjust > 0 {
                        *adjust = cmp::max(0, *adjust - ADJUST_SPEED);
                    } else {
                        *adjust = cmp::min(0, *adjust + ADJUST_SPEED);
                    }
                    if *adjust == 0 {
                        done.push(col);
                    }
                }
                for col in done.into_iter() {
                    self.adjust.remove(&col);
                }
                return Action::redraw_if(any_adjustments);
            }
            &Event::MouseDown(pt) => {
                for col in 0..columns.num_columns() {
                    if self.column_rect(columns, col).contains(pt) &&
                       !columns.column_linkages(col).is_empty() {
                        self.drag = Some((col, pt.y(), pt.y()));
                        return Action::redraw();
                    }
                }
            }
            &Event::MouseDrag(pt) => {
                if let Some((_col, _from, ref mut to)) = self.drag {
                    *to = pt.y();
                    return Action::redraw();
                }
            }
            &Event::MouseUp => {
                if let Some((col, from, to)) = self.drag.take() {
                    let delta = to - from;
                    let boosted = if delta > 0 {
                        delta + BOX_SIZE / 2
                    } else {
                        delta - BOX_SIZE / 2
                    };
                    let by = boosted / BOX_SIZE;
                    let adjust = delta - (by * BOX_SIZE);
                    for &(other, factor) in columns.column_linkages(col) {
                        self.adjust.insert(other, adjust * factor);
                    }
                    return if by == 0 {
                        Action::redraw()
                    } else {
                        Action::redraw().and_return((col, by))
                    };
                }
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //
