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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PrimaryColor, PuzzleState, SyrupState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32)>,
    toggles: Vec<ToggleLight>,
    next: NextColor,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &SyrupState)
               -> View {
        let mut core = {
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources, visible);
            PuzzleCore::new(resources, visible, state, intro, outro)
        };
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        View {
            core: core,
            toggles: vec![
                ToggleLight::new(resources, state, (1, 0)),
                ToggleLight::new(resources, state, (2, 0)),
                ToggleLight::new(resources, state, (3, 0)),
                ToggleLight::new(resources, state, (0, 1)),
                ToggleLight::new(resources, state, (1, 1)),
                ToggleLight::new(resources, state, (2, 1)),
                ToggleLight::new(resources, state, (3, 1)),
                ToggleLight::new(resources, state, (4, 1)),
                ToggleLight::new(resources, state, (0, 2)),
                ToggleLight::new(resources, state, (1, 2)),
                ToggleLight::new(resources, state, (2, 2)),
                ToggleLight::new(resources, state, (3, 2)),
                ToggleLight::new(resources, state, (4, 2)),
                ToggleLight::new(resources, state, (0, 3)),
                ToggleLight::new(resources, state, (1, 3)),
                ToggleLight::new(resources, state, (2, 3)),
                ToggleLight::new(resources, state, (3, 3)),
                ToggleLight::new(resources, state, (4, 3)),
                ToggleLight::new(resources, state, (1, 4)),
                ToggleLight::new(resources, state, (2, 4)),
                ToggleLight::new(resources, state, (3, 4)),
            ],
            next: NextColor::new(resources),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.light_syrup;
        self.core.draw_back_layer(canvas);
        self.next.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.toggles.draw(state, canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.light_syrup;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() {
            let subaction = self.toggles.handle_event(event, state);
            if let Some(&position) = subaction.value() {
                state.toggle(position);
                if state.is_solved() {
                    self.core.begin_outro_scene();
                } else {
                    self.core.push_undo(position);
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            action.merge(self.next.handle_event(event, state));
        }
        if !action.should_stop() {
            self.core.begin_character_scene_on_click(event);
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.light_syrup.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(position) = self.core.pop_undo() {
            game.light_syrup.untoggle(position);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(position) = self.core.pop_redo() {
            game.light_syrup.toggle(position);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.light_syrup.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.light_syrup.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (index, color) in self.core.drain_queue() {
            if index < 0 {
                self.next.visible = color != 0;
            } else if (index as usize) < self.toggles.len() {
                self.toggles[index as usize].set_hilight(color);
            }
        }
    }
}

// ========================================================================= //

const LIGHTS_TOP: i32 = 72;
const LIGHTS_LEFT: i32 = 200;
const MAX_LIGHT_RADIUS: i32 = 12;

struct ToggleLight {
    frame: Sprite,
    position: (i32, i32),
    red_radius: i32,
    green_radius: i32,
    blue_radius: i32,
    hilight: i32,
}

impl ToggleLight {
    fn new(resources: &mut Resources, state: &SyrupState,
           position: (i32, i32))
           -> ToggleLight {
        let (red, green, blue) = state.light_colors(position);
        ToggleLight {
            frame: resources.get_sprites("light/toggle")[0].clone(),
            position: position,
            red_radius: if red { MAX_LIGHT_RADIUS } else { 0 },
            green_radius: if green { MAX_LIGHT_RADIUS } else { 0 },
            blue_radius: if blue { MAX_LIGHT_RADIUS } else { 0 },
            hilight: 0,
        }
    }

    fn rect(&self) -> Rect {
        let (col, row) = self.position;
        Rect::new(LIGHTS_LEFT + 32 * col, LIGHTS_TOP + 32 * row, 32, 32)
    }

    fn set_hilight(&mut self, color: i32) { self.hilight = color; }
}

impl Element<SyrupState, (i32, i32)> for ToggleLight {
    fn draw(&self, _state: &SyrupState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        if self.hilight == 1 {
            draw_light(&mut canvas,
                       0,
                       0,
                       (0, 0, 0),
                       MAX_LIGHT_RADIUS,
                       (255, 0, 0));
        } else if self.hilight == 2 {
            draw_light(&mut canvas,
                       0,
                       0,
                       (0, 0, 0),
                       MAX_LIGHT_RADIUS,
                       (0, 255, 0));
        } else if self.hilight == 3 {
            draw_light(&mut canvas,
                       0,
                       0,
                       (0, 0, 0),
                       MAX_LIGHT_RADIUS,
                       (0, 0, 255));
        } else if self.red_radius <= self.green_radius &&
                   self.red_radius <= self.blue_radius
        {
            // Red is smallest.
            if self.green_radius <= self.blue_radius {
                draw_light(&mut canvas,
                           self.red_radius,
                           self.green_radius,
                           (0, 255, 255),
                           self.blue_radius,
                           (0, 0, 255));
            } else {
                draw_light(&mut canvas,
                           self.red_radius,
                           self.blue_radius,
                           (0, 255, 255),
                           self.green_radius,
                           (0, 255, 0));
            }
        } else if self.green_radius <= self.blue_radius {
            // Green is smallest.
            if self.red_radius <= self.blue_radius {
                draw_light(&mut canvas,
                           self.green_radius,
                           self.red_radius,
                           (255, 0, 255),
                           self.blue_radius,
                           (0, 0, 255));
            } else {
                draw_light(&mut canvas,
                           self.green_radius,
                           self.blue_radius,
                           (255, 0, 255),
                           self.red_radius,
                           (255, 0, 0));
            }
        } else {
            // Blue is smallest.
            if self.red_radius <= self.green_radius {
                draw_light(&mut canvas,
                           self.blue_radius,
                           self.red_radius,
                           (255, 255, 0),
                           self.green_radius,
                           (0, 255, 0));
            } else {
                draw_light(&mut canvas,
                           self.blue_radius,
                           self.green_radius,
                           (255, 255, 0),
                           self.red_radius,
                           (255, 0, 0));
            }
        }
        let center = canvas.rect().center();
        canvas.draw_sprite_centered(&self.frame, center);
    }

    fn handle_event(&mut self, event: &Event, state: &mut SyrupState)
                    -> Action<(i32, i32)> {
        match event {
            &Event::ClockTick => {
                let (red, green, blue) = state.light_colors(self.position);
                // N.B. The below ORs must be non-short-circuiting.
                Action::redraw_if(tick_radius(red, &mut self.red_radius) |
                                      tick_radius(green,
                                                  &mut self.green_radius) |
                                      tick_radius(blue, &mut self.blue_radius))
            }
            &Event::MouseDown(pt)
                if self.rect().contains(pt) && !state.is_solved() => {
                Action::redraw().and_return(self.position)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct NextColor {
    sprites: Vec<Sprite>,
    visible: bool,
}

impl NextColor {
    fn new(resources: &mut Resources) -> NextColor {
        NextColor {
            sprites: resources.get_sprites("light/color"),
            visible: false,
        }
    }
}

impl Element<SyrupState, PuzzleCmd> for NextColor {
    fn draw(&self, state: &SyrupState, canvas: &mut Canvas) {
        if self.visible {
            canvas.fill_rect((0, 0, 127), Rect::new(454, 70, 36, 36));
            let sprite_index = match state.next_color() {
                PrimaryColor::Red => 0,
                PrimaryColor::Green => 1,
                PrimaryColor::Blue => 2,
            };
            canvas
                .draw_sprite(&self.sprites[sprite_index], Point::new(461, 77));
        }
    }

    fn handle_event(&mut self, event: &Event, _state: &mut SyrupState)
                    -> Action<PuzzleCmd> {
        match event {
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

fn light_rect(center: Point, radius: i32) -> Rect {
    Rect::new(center.x() - radius,
              center.y() - radius,
              2 * radius as u32,
              2 * radius as u32)
}

fn draw_light(canvas: &mut Canvas, radius1: i32, radius2: i32,
              color2: (u8, u8, u8), radius3: i32, color3: (u8, u8, u8)) {
    debug_assert!(0 <= radius1 && radius1 <= radius2 && radius2 <= radius3 &&
                      radius3 <= MAX_LIGHT_RADIUS);
    let center = canvas.rect().center();
    if radius3 < MAX_LIGHT_RADIUS {
        canvas.fill_rect((0, 0, 32), light_rect(center, MAX_LIGHT_RADIUS));
    }
    if radius3 > 0 && radius3 > radius2 {
        canvas.fill_rect(color3, light_rect(center, radius3));
    }
    if radius2 > 0 && radius2 > radius1 {
        canvas.fill_rect(color2, light_rect(center, radius2));
    }
    if radius1 > 0 {
        canvas.fill_rect((255, 255, 255), light_rect(center, radius1));
    }
}

fn tick_radius(lit: bool, radius: &mut i32) -> bool {
    if lit {
        if *radius < MAX_LIGHT_RADIUS {
            *radius = cmp::min(MAX_LIGHT_RADIUS, *radius + 3);
            return true;
        }
    } else {
        if *radius > 0 {
            *radius = cmp::max(0, *radius - 3);
            return true;
        }
    }
    false
}

// ========================================================================= //

const INFO_BOX_TEXT: &str = "\
Your goal is to turn all twenty-one lights WHITE.

$M{Tap}{Click} one of the lights to change the color of that
light and of the adjacent lights.  Your first move will
toggle the redness of the lights; the second move will toggle
greenness, the third blueness, and then back to red.";

// ========================================================================= //
