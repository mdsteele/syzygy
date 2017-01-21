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

use std::cmp::{max, min};
use std::rc::Rc;

use gui::{Action, Align, Canvas, Element, Event, Font, FRAME_DELAY_MILLIS,
          Point, Rect, Resources, Sprite};

// ========================================================================= //

const ARROW_BLINK_FRAMES: i32 = 3;

pub struct ArrowPair {
    sprites: Vec<Sprite>,
    font: Rc<Font>,
    left: i32,
    top: i32,
    row: i32,
    default_delta: i32,
    delta_override: Option<i32>,
    blink_left: i32,
    blink_right: i32,
}

impl ArrowPair {
    pub fn new(resources: &mut Resources, topleft: (i32, i32), row: i32,
               delta: i32)
               -> ArrowPair {
        ArrowPair {
            sprites: resources.get_sprites("shift/arrows"),
            font: resources.get_font("roman"),
            left: topleft.0,
            top: topleft.1,
            row: row,
            default_delta: delta,
            delta_override: None,
            blink_left: 0,
            blink_right: 0,
        }
    }

    pub fn set_delta_override(&mut self, delta_override: Option<i32>) {
        self.delta_override = delta_override;
    }

    fn delta(&self) -> i32 {
        self.delta_override.unwrap_or(self.default_delta)
    }

    fn rect(&self) -> Rect { Rect::new(self.left, self.top, 48, 16) }
}

impl Element<(), (i32, i32)> for ArrowPair {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        let left_arrow = if self.blink_left == 0 {
            &self.sprites[0]
        } else {
            &self.sprites[1]
        };
        let right_arrow = if self.blink_right == 0 {
            &self.sprites[2]
        } else {
            &self.sprites[3]
        };
        let middle = if self.delta_override.is_none() {
            &self.sprites[4]
        } else {
            &self.sprites[5]
        };
        canvas.draw_sprite(left_arrow, Point::new(3, 0));
        canvas.draw_sprite(right_arrow, Point::new(31, 0));
        canvas.draw_sprite(middle, Point::new(17, 0));
        canvas.draw_text(&self.font,
                         Align::Center,
                         Point::new(24, 12),
                         &format!("{}", self.delta()));
    }

    fn handle_event(&mut self, event: &Event, _state: &mut ())
                    -> Action<(i32, i32)> {
        match event {
            &Event::ClockTick => {
                let mut redraw = false;
                if self.blink_left > 0 {
                    self.blink_left -= 1;
                    if self.blink_left == 0 {
                        redraw = true;
                    }
                }
                if self.blink_right > 0 {
                    self.blink_right -= 1;
                    if self.blink_right == 0 {
                        redraw = true;
                    }
                }
                Action::redraw_if(redraw)
            }
            &Event::MouseDown(pt) => {
                let rect = self.rect();
                if rect.contains(pt) {
                    if pt.x() - rect.x() <= (rect.width() / 2) as i32 {
                        self.blink_left = ARROW_BLINK_FRAMES;
                        Action::redraw().and_return((self.row, -self.delta()))
                    } else {
                        self.blink_right = ARROW_BLINK_FRAMES;
                        Action::redraw().and_return((self.row, self.delta()))
                    }
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const PLATFORM_SPEED: i32 = 6;

pub struct Platform {
    sprites: Vec<Sprite>,
    track_left: i32,
    top: i32,
    left: i32,
    goal: i32,
}

impl Platform {
    pub fn new(resources: &mut Resources, topleft: (i32, i32), position: i32)
               -> Platform {
        let mut platform = Platform {
            sprites: resources.get_sprites("shift/platforms"),
            track_left: topleft.0,
            top: topleft.1,
            left: topleft.1,
            goal: topleft.1,
        };
        let left = platform.pos_to_left(position);
        platform.left = left;
        platform.goal = left;
        platform
    }

    pub fn top(&self) -> i32 { self.top }

    pub fn top_point_for_pos(&self, pos: i32) -> Point {
        Point::new(self.pos_to_left(pos) + 16, self.top)
    }

    pub fn set_position(&mut self, pos: i32) {
        self.left = self.pos_to_left(pos);
        self.goal = self.left;
    }

    pub fn set_goal(&mut self, pos: i32) { self.goal = self.pos_to_left(pos); }

    pub fn move_to_goal(&mut self) { self.left = self.goal; }

    pub fn pos_to_left(&self, pos: i32) -> i32 { self.track_left + 32 * pos }

    pub fn travel_time(from_pos: i32, to_pos: i32) -> f64 {
        let dist = 32 * (from_pos - to_pos).abs();
        let num_frames = dist as f64 / PLATFORM_SPEED as f64;
        FRAME_DELAY_MILLIS as f64 * 0.001 * num_frames
    }
}

impl Element<(), ()> for Platform {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        let index = ((self.left % 24) / 12) as usize;
        canvas.draw_sprite(&self.sprites[index],
                           Point::new(self.left, self.top));
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<()> {
        match event {
            &Event::ClockTick => {
                if self.goal != self.left {
                    self.left = if self.goal < self.left {
                        max(self.goal, self.left - PLATFORM_SPEED)
                    } else {
                        min(self.goal, self.left + PLATFORM_SPEED)
                    };
                    Action::redraw()
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //
