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

use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};

// ========================================================================= //

const TILE_SIZE: i32 = 24;
const NUM_COLS: i32 = 24;
const NUM_ROWS: i32 = 16;
const MAX_CELL_OPACITY: i32 = 7;

// ========================================================================= //

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum FadeStyle {
    Uniform,
    Radial,
    TopBottom,
    BottomTop,
    LeftRight,
    RightLeft,
}

impl FadeStyle {
    fn cell_opacity(self, col: i32, row: i32, opacity: i32) -> i32 {
        match self {
            FadeStyle::Uniform => opacity,
            FadeStyle::Radial => {
                let dx = col - (NUM_COLS / 2);
                let dy = row - (NUM_ROWS / 2);
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                opacity - 8 + (0.5 * dist).ceil() as i32
            }
            FadeStyle::TopBottom => opacity - row / 2,
            FadeStyle::BottomTop => opacity - (NUM_ROWS - row - 1) / 2,
            FadeStyle::LeftRight => opacity - col / 3,
            FadeStyle::RightLeft => opacity - (NUM_COLS - col - 1) / 3,
        }
    }

    fn max_opacity(self) -> i32 {
        match self {
            FadeStyle::Uniform => MAX_CELL_OPACITY,
            FadeStyle::Radial => MAX_CELL_OPACITY + 8,
            FadeStyle::TopBottom => MAX_CELL_OPACITY + NUM_ROWS / 2,
            FadeStyle::BottomTop => MAX_CELL_OPACITY + NUM_ROWS / 2,
            FadeStyle::LeftRight => MAX_CELL_OPACITY + NUM_COLS / 3,
            FadeStyle::RightLeft => MAX_CELL_OPACITY + NUM_COLS / 3,
        }
    }
}

// ========================================================================= //

pub struct ScreenFade<A> {
    sprites: Vec<Sprite>,
    style: FadeStyle,
    opacity: i32,
    fade_out_command: Option<A>,
}

impl<A> ScreenFade<A> {
    pub fn new(resources: &mut Resources, style: FadeStyle) -> ScreenFade<A> {
        ScreenFade {
            sprites: resources.get_sprites("screen_fade"),
            style: style,
            opacity: style.max_opacity(),
            fade_out_command: None,
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.opacity == 0 && self.fade_out_command.is_none()
    }

    pub fn fade_out_and_return(&mut self, command: A) {
        self.fade_out_command = Some(command);
    }
}

impl<A> Element<(), A> for ScreenFade<A> {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        if self.opacity >= self.style.max_opacity() {
            canvas.clear((0, 0, 0));
        } else if self.opacity > 0 {
            debug_assert_eq!(self.sprites.len() as i32, MAX_CELL_OPACITY - 1);
            for row in 0..NUM_ROWS {
                for col in 0..NUM_COLS {
                    let cell_opacity =
                        self.style.cell_opacity(col, row, self.opacity);
                    if cell_opacity >= MAX_CELL_OPACITY {
                        canvas.fill_rect((0, 0, 0),
                                         Rect::new(col * TILE_SIZE,
                                                   row * TILE_SIZE,
                                                   TILE_SIZE as u32,
                                                   TILE_SIZE as u32));
                    } else if cell_opacity > 0 {
                        let sprite = &self.sprites[cell_opacity as usize - 1];
                        canvas.draw_sprite(sprite,
                                           Point::new(col * TILE_SIZE,
                                                      row * TILE_SIZE));
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<A> {
        match event {
            &Event::Quit => Action::ignore(),
            &Event::ClockTick => {
                let should_be_opaque = self.fade_out_command.is_some();
                let max_opacity = self.style.max_opacity();
                if should_be_opaque && self.opacity < max_opacity {
                    self.opacity += 1;
                    if self.opacity == max_opacity {
                        let command = self.fade_out_command.take().unwrap();
                        Action::redraw().and_return(command)
                    } else {
                        Action::redraw()
                    }
                } else if !should_be_opaque && self.opacity > 0 {
                    self.opacity -= 1;
                    Action::redraw()
                } else {
                    Action::ignore()
                }
            }
            _ => {
                if self.opacity == 0 {
                    Action::ignore()
                } else {
                    Action::ignore().and_stop()
                }
            }
        }
    }
}

// ========================================================================= //
