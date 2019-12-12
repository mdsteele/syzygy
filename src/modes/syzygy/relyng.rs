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
use std::rc::Rc;

use crate::gui::{
    Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources,
    Sprite,
};
use crate::save::SyzygyState;

// ========================================================================= //

pub struct LightsGrid {
    left: i32,
    top: i32,
    lights: Vec<ToggleLight>,
    next: NextShape,
}

impl LightsGrid {
    pub fn new(
        resources: &mut Resources,
        left: i32,
        top: i32,
        state: &SyzygyState,
    ) -> LightsGrid {
        let mut lights = Vec::new();
        for row in 0..4 {
            for col in 0..5 {
                lights.push(ToggleLight::new(resources, state, (col, row)));
            }
        }
        LightsGrid {
            left,
            top,
            lights,
            next: NextShape::new(resources, left + 176, top + 32),
        }
    }
}

impl Element<SyzygyState, (i32, i32)> for LightsGrid {
    fn draw(&self, state: &SyzygyState, canvas: &mut Canvas) {
        self.next.draw(state, canvas);
        let rect = Rect::new(
            self.left,
            self.top,
            5 * TOGGLE_LIGHT_SIZE,
            4 * TOGGLE_LIGHT_SIZE,
        );
        let mut canvas = canvas.subcanvas(rect);
        self.lights.draw(state, &mut canvas);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        state: &mut SyzygyState,
    ) -> Action<(i32, i32)> {
        let mut action = self.next.handle_event(event, state);
        let event = event.translate(-self.left, -self.top);
        action.merge(self.lights.handle_event(&event, state));
        action
    }
}

// ========================================================================= //

const TOGGLE_LIGHT_SIZE: u32 = 32;
const TOGGLE_MAX_LIGHT_RADIUS: i32 = 12;

struct ToggleLight {
    frame: Sprite,
    position: (i32, i32),
    light_radius: i32,
}

impl ToggleLight {
    fn new(
        resources: &mut Resources,
        state: &SyzygyState,
        position: (i32, i32),
    ) -> ToggleLight {
        ToggleLight {
            frame: resources.get_sprites("light/toggle")[0].clone(),
            position,
            light_radius: if state.relyng_is_lit(position) {
                TOGGLE_MAX_LIGHT_RADIUS
            } else {
                0
            },
        }
    }

    fn rect(&self) -> Rect {
        let (col, row) = self.position;
        Rect::new(
            col * TOGGLE_LIGHT_SIZE as i32,
            row * TOGGLE_LIGHT_SIZE as i32,
            TOGGLE_LIGHT_SIZE,
            TOGGLE_LIGHT_SIZE,
        )
    }
}

impl Element<SyzygyState, (i32, i32)> for ToggleLight {
    fn draw(&self, _state: &SyzygyState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        draw_light(&mut canvas, self.light_radius, TOGGLE_MAX_LIGHT_RADIUS);
        let center = canvas.rect().center();
        canvas.draw_sprite_centered(&self.frame, center);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        state: &mut SyzygyState,
    ) -> Action<(i32, i32)> {
        match event {
            &Event::ClockTick => tick_radius(
                state.relyng_is_lit(self.position),
                &mut self.light_radius,
                TOGGLE_MAX_LIGHT_RADIUS,
            ),
            &Event::MouseDown(pt) if self.rect().contains_point(pt) => {
                Action::redraw().and_return(self.position)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct NextShape {
    frame: Sprite,
    spacer: Sprite,
    font: Rc<Font>,
    left: i32,
    top: i32,
}

impl NextShape {
    fn new(resources: &mut Resources, left: i32, top: i32) -> NextShape {
        NextShape {
            frame: resources.get_sprites("light/toggle")[2].clone(),
            spacer: resources.get_sprites("light/spacer")[0].clone(),
            font: resources.get_font("block"),
            left,
            top,
        }
    }
}

impl Element<SyzygyState, (i32, i32)> for NextShape {
    fn draw(&self, state: &SyzygyState, canvas: &mut Canvas) {
        canvas.fill_rect(
            (0, 0, 32),
            Rect::new(self.left + 5, self.top + 5, 22, 22),
        );
        canvas.draw_sprite(&self.frame, Point::new(self.left, self.top));
        canvas.draw_sprite(
            &self.spacer,
            Point::new(self.left - 16, self.top + 8),
        );
        let chr = state.relyng_next_shape();
        let pt = Point::new(self.left + 16, self.top + 25);
        canvas.draw_char(&self.font, Align::Center, pt, chr);
    }

    fn handle_event(
        &mut self,
        _event: &Event,
        _state: &mut SyzygyState,
    ) -> Action<(i32, i32)> {
        Action::ignore()
    }
}

// ========================================================================= //

fn light_rect(center: Point, radius: i32) -> Rect {
    Rect::new(
        center.x() - radius,
        center.y() - radius,
        2 * radius as u32,
        2 * radius as u32,
    )
}

fn draw_light(canvas: &mut Canvas, radius: i32, max: i32) {
    let center = canvas.rect().center();
    if radius < max {
        canvas.fill_rect((0, 0, 32), light_rect(center, max));
    }
    if radius > 0 {
        canvas.fill_rect((255, 255, 192), light_rect(center, radius));
    }
}

fn tick_radius<A>(lit: bool, radius: &mut i32, max: i32) -> Action<A> {
    if lit {
        if *radius < max {
            *radius = cmp::min(max, *radius + 3);
            return Action::redraw();
        }
    } else {
        if *radius > 0 {
            *radius = cmp::max(0, *radius - 3);
            return Action::redraw();
        }
    }
    Action::ignore()
}

// ========================================================================= //
