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

use elements::{Hud, HudAction, HudInput};
use gui::{Action, Background, Canvas, Element, Event, Rect, Resources, Sprite};
use save::{AtticState, Game, Location};

// ========================================================================= //

pub enum Cmd {
    Hud(HudAction),
}

// ========================================================================= //

pub struct View {
    background: Rc<Background>,
    hud: Hud,
    toggles: Vec<ToggleLight>,
    passives: Vec<PassiveLight>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, attic: &AtticState)
               -> View {
        View {
            background: resources.get_background("a_light_in_the_attic"),
            hud: Hud::new(resources, visible, Location::ALightInTheAttic),
            toggles: vec![
                ToggleLight::new(resources, attic, (1, 1), 'C'),
                ToggleLight::new(resources, attic, (2, 1), 'Z'),
                ToggleLight::new(resources, attic, (3, 1), 'H'),
                ToggleLight::new(resources, attic, (4, 1), 'A'),
                ToggleLight::new(resources, attic, (1, 2), 'U'),
                ToggleLight::new(resources, attic, (2, 2), 'V'),
                ToggleLight::new(resources, attic, (3, 2), 'X'),
                ToggleLight::new(resources, attic, (4, 2), 'S'),
                ToggleLight::new(resources, attic, (1, 3), 'J'),
                ToggleLight::new(resources, attic, (2, 3), 'T'),
                ToggleLight::new(resources, attic, (3, 3), 'I'),
                ToggleLight::new(resources, attic, (4, 3), 'K'),
                ToggleLight::new(resources, attic, (1, 4), 'Y'),
                ToggleLight::new(resources, attic, (2, 4), 'O'),
                ToggleLight::new(resources, attic, (3, 4), 'L'),
                ToggleLight::new(resources, attic, (4, 4), 'N'),
            ],
            passives: vec![
                PassiveLight::new(resources, attic, (1, 0)),
                PassiveLight::new(resources, attic, (2, 0)),
                PassiveLight::new(resources, attic, (3, 0)),
                PassiveLight::new(resources, attic, (4, 0)),
                PassiveLight::new(resources, attic, (1, 5)),
                PassiveLight::new(resources, attic, (2, 5)),
                PassiveLight::new(resources, attic, (3, 5)),
                PassiveLight::new(resources, attic, (4, 5)),
                PassiveLight::new(resources, attic, (0, 1)),
                PassiveLight::new(resources, attic, (0, 2)),
                PassiveLight::new(resources, attic, (0, 3)),
                PassiveLight::new(resources, attic, (0, 4)),
                PassiveLight::new(resources, attic, (5, 1)),
                PassiveLight::new(resources, attic, (5, 2)),
                PassiveLight::new(resources, attic, (5, 3)),
                PassiveLight::new(resources, attic, (5, 4)),
            ],
        }
    }

    fn hud_input(&self) -> HudInput {
        HudInput {
            name: "A Light in the Attic",
            can_undo: false,
            can_redo: false,
            can_reset: false,
        }
    }
}

impl Element<Game, Cmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        canvas.draw_background(&self.background);
        self.passives.draw(&game.a_light_in_the_attic, canvas);
        self.toggles.draw(&game.a_light_in_the_attic, canvas);
        self.hud.draw(&self.hud_input(), canvas);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game) -> Action<Cmd> {
        let mut input = self.hud_input();
        let mut action = self.hud
                             .handle_event(event, &mut input)
                             .map(Cmd::Hud);
        let attic = &mut game.a_light_in_the_attic;
        if !action.should_stop() {
            action.merge(self.toggles.handle_event(event, attic));
        }
        if !action.should_stop() {
            action.merge(self.passives.handle_event(event, attic));
        }
        action
    }
}

// ========================================================================= //

const LIGHTS_TOP: i32 = 56;
const LIGHTS_LEFT: i32 = 312;
const TOGGLE_MAX_LIGHT_RADIUS: i32 = 12;

pub struct ToggleLight {
    frame_off: Sprite,
    frame_on: Sprite,
    label: Sprite,
    position: (i32, i32),
    light_radius: i32,
}

impl ToggleLight {
    fn new(resources: &mut Resources, attic: &AtticState,
           position: (i32, i32), label: char)
           -> ToggleLight {
        let sprites = resources.get_sprites("toggle_light");
        ToggleLight {
            frame_off: sprites[0].clone(),
            frame_on: sprites[1].clone(),
            label: resources.get_font("block").glyph(label).sprite().clone(),
            position: position,
            light_radius: if attic.is_lit(position) {
                TOGGLE_MAX_LIGHT_RADIUS
            } else {
                0
            },
        }
    }

    fn rect(&self) -> Rect {
        let (col, row) = self.position;
        Rect::new(LIGHTS_LEFT + 32 * col, LIGHTS_TOP + 32 * row, 32, 32)
    }
}

impl Element<AtticState, Cmd> for ToggleLight {
    fn draw(&self, state: &AtticState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        draw_light(&mut canvas, self.light_radius, TOGGLE_MAX_LIGHT_RADIUS);
        let center = canvas.rect().center();
        canvas.draw_sprite_centered(&self.label, center);
        let frame = if state.is_toggled(self.position) {
            &self.frame_on
        } else {
            &self.frame_off
        };
        canvas.draw_sprite_centered(frame, center);
    }

    fn handle_event(&mut self, event: &Event, state: &mut AtticState)
                    -> Action<Cmd> {
        match event {
            &Event::ClockTick => {
                tick_radius(state.is_lit(self.position),
                            &mut self.light_radius,
                            TOGGLE_MAX_LIGHT_RADIUS)
            }
            &Event::MouseDown(pt) if self.rect().contains(pt) &&
                                     !state.is_solved() => {
                state.toggle(self.position);
                Action::redraw()
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const PASSIVE_MAX_LIGHT_RADIUS: i32 = 11;

pub struct PassiveLight {
    frame: Sprite,
    position: (i32, i32),
    light_radius: i32,
}

impl PassiveLight {
    fn new(resources: &mut Resources, attic: &AtticState, position: (i32, i32))
           -> PassiveLight {
        let sprites = resources.get_sprites("toggle_light");
        let (col, row) = position;
        let sprite_index = if col == 5 {
            2
        } else if row == 0 {
            3
        } else if col == 0 {
            4
        } else {
            5
        };
        PassiveLight {
            frame: sprites[sprite_index].clone(),
            position: position,
            light_radius: if attic.is_lit(position) {
                PASSIVE_MAX_LIGHT_RADIUS
            } else {
                0
            },
        }
    }

    fn rect(&self) -> Rect {
        let (col, row) = self.position;
        Rect::new(LIGHTS_LEFT + 32 * col, LIGHTS_TOP + 32 * row, 32, 32)
    }
}

impl Element<AtticState, Cmd> for PassiveLight {
    fn draw(&self, _: &AtticState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        draw_light(&mut canvas, self.light_radius, PASSIVE_MAX_LIGHT_RADIUS);
        let center = canvas.rect().center();
        canvas.draw_sprite_centered(&self.frame, center);
    }

    fn handle_event(&mut self, event: &Event, state: &mut AtticState)
                    -> Action<Cmd> {
        match event {
            &Event::ClockTick => {
                tick_radius(state.is_lit(self.position),
                            &mut self.light_radius,
                            PASSIVE_MAX_LIGHT_RADIUS)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

fn draw_light(canvas: &mut Canvas, radius: i32, max: i32) {
    let center = canvas.rect().center();
    if radius < max {
        let dark_rect = Rect::new(center.x() - max,
                                  center.y() - max,
                                  2 * max as u32,
                                  2 * max as u32);
        canvas.fill_rect((0, 0, 32), dark_rect);
    }
    if radius > 0 {
        let light_rect = Rect::new(center.x() - radius,
                                   center.y() - radius,
                                   2 * radius as u32,
                                   2 * radius as u32);
        canvas.fill_rect((255, 255, 192), light_rect);
    }
}

fn tick_radius(lit: bool, radius: &mut i32, max: i32) -> Action<Cmd> {
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
