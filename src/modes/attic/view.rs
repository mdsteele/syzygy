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

use super::scenes;
use crate::elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView};
use crate::gui::{
    Action, Canvas, Element, Event, Point, Rect, Resources, Sound, Sprite,
};
use crate::modes::SOLVED_INFO_TEXT;
use crate::save::{AtticState, Game, PuzzleState};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32)>,
    grid: AtticGrid,
}

impl View {
    pub fn new(
        resources: &mut Resources,
        visible: Rect,
        state: &AtticState,
    ) -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_argony_midscene(resources));
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        View { core, grid: AtticGrid::new(resources, state) }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.a_light_in_the_attic;
        self.core.draw_back_layer(canvas);
        self.core.draw_middle_layer(canvas);
        self.grid.draw(state, canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        game: &mut Game,
    ) -> Action<PuzzleCmd> {
        let state = &mut game.a_light_in_the_attic;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() {
            let subaction = self.grid.handle_event(event, state);
            if let Some(&position) = subaction.value() {
                action.also_play_sound(Sound::device_rotate());
                state.toggle(position);
                if state.is_solved() {
                    self.core.begin_outro_scene();
                    action = action.and_return(PuzzleCmd::Save);
                } else {
                    self.core.push_undo(position);
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            self.core.begin_character_scene_on_click(event);
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.a_light_in_the_attic.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(position) = self.core.pop_undo() {
            game.a_light_in_the_attic.toggle(position);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(position) = self.core.pop_redo() {
            game.a_light_in_the_attic.toggle(position);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.a_light_in_the_attic.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.a_light_in_the_attic.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (index, enable) in self.core.drain_queue() {
            self.grid.toggles[index as usize].set_hilight(enable != 0);
        }
    }
}

// ========================================================================= //

const LIGHTS_TOP: i32 = 56;
const LIGHTS_LEFT: i32 = 296;
const TOGGLE_MAX_LIGHT_RADIUS: i32 = 12;

pub struct AtticGrid {
    toggles: Vec<ToggleLight>,
    passives: Vec<PassiveLight>,
}

impl AtticGrid {
    pub fn new(resources: &mut Resources, state: &AtticState) -> AtticGrid {
        AtticGrid {
            toggles: vec![
                ToggleLight::new(resources, state, (1, 1), 'C'),
                ToggleLight::new(resources, state, (2, 1), 'Z'),
                ToggleLight::new(resources, state, (3, 1), 'H'),
                ToggleLight::new(resources, state, (4, 1), 'A'),
                ToggleLight::new(resources, state, (1, 2), 'U'),
                ToggleLight::new(resources, state, (2, 2), 'V'),
                ToggleLight::new(resources, state, (3, 2), 'X'),
                ToggleLight::new(resources, state, (4, 2), 'S'),
                ToggleLight::new(resources, state, (1, 3), 'J'),
                ToggleLight::new(resources, state, (2, 3), 'T'),
                ToggleLight::new(resources, state, (3, 3), 'I'),
                ToggleLight::new(resources, state, (4, 3), 'K'),
                ToggleLight::new(resources, state, (1, 4), 'Y'),
                ToggleLight::new(resources, state, (2, 4), 'O'),
                ToggleLight::new(resources, state, (3, 4), 'L'),
                ToggleLight::new(resources, state, (4, 4), 'N'),
            ],
            passives: vec![
                PassiveLight::new(resources, state, (1, 0)),
                PassiveLight::new(resources, state, (2, 0)),
                PassiveLight::new(resources, state, (3, 0)),
                PassiveLight::new(resources, state, (4, 0)),
                PassiveLight::new(resources, state, (1, 5)),
                PassiveLight::new(resources, state, (2, 5)),
                PassiveLight::new(resources, state, (3, 5)),
                PassiveLight::new(resources, state, (4, 5)),
                PassiveLight::new(resources, state, (0, 1)),
                PassiveLight::new(resources, state, (0, 2)),
                PassiveLight::new(resources, state, (0, 3)),
                PassiveLight::new(resources, state, (0, 4)),
                PassiveLight::new(resources, state, (5, 1)),
                PassiveLight::new(resources, state, (5, 2)),
                PassiveLight::new(resources, state, (5, 3)),
                PassiveLight::new(resources, state, (5, 4)),
            ],
        }
    }

    pub fn do_not_show_corner_lights(&mut self) {
        for toggle in self.toggles.iter_mut() {
            toggle.frame_on = toggle.frame_off.clone();
        }
    }
}

impl Element<AtticState, (i32, i32)> for AtticGrid {
    fn draw(&self, state: &AtticState, canvas: &mut Canvas) {
        self.passives.draw(state, canvas);
        self.toggles.draw(state, canvas);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        state: &mut AtticState,
    ) -> Action<(i32, i32)> {
        let mut action = self.toggles.handle_event(event, state);
        if !action.should_stop() {
            action.merge(self.passives.handle_event(event, state));
        }
        action
    }
}

// ========================================================================= //

struct ToggleLight {
    frame_off: Sprite,
    frame_on: Sprite,
    label: Sprite,
    position: (i32, i32),
    light_radius: i32,
    hilight: bool,
}

impl ToggleLight {
    fn new(
        resources: &mut Resources,
        state: &AtticState,
        position: (i32, i32),
        label: char,
    ) -> ToggleLight {
        let sprites = resources.get_sprites("light/toggle");
        ToggleLight {
            frame_off: sprites[0].clone(),
            frame_on: sprites[1].clone(),
            label: resources.get_font("block").glyph(label).sprite().clone(),
            position,
            light_radius: if state.is_lit(position) {
                TOGGLE_MAX_LIGHT_RADIUS
            } else {
                0
            },
            hilight: false,
        }
    }

    fn rect(&self) -> Rect {
        let (col, row) = self.position;
        Rect::new(LIGHTS_LEFT + 32 * col, LIGHTS_TOP + 32 * row, 32, 32)
    }

    fn set_hilight(&mut self, hilight: bool) {
        self.hilight = hilight;
    }
}

impl Element<AtticState, (i32, i32)> for ToggleLight {
    fn draw(&self, state: &AtticState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        draw_light(
            &mut canvas,
            self.light_radius,
            TOGGLE_MAX_LIGHT_RADIUS,
            self.hilight,
        );
        let center = canvas.rect().center();
        canvas.draw_sprite_centered(&self.label, center);
        let frame = if state.is_toggled(self.position) {
            &self.frame_on
        } else {
            &self.frame_off
        };
        canvas.draw_sprite_centered(frame, center);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        state: &mut AtticState,
    ) -> Action<(i32, i32)> {
        match event {
            &Event::ClockTick => tick_radius(
                state.is_lit(self.position),
                &mut self.light_radius,
                TOGGLE_MAX_LIGHT_RADIUS,
            ),
            &Event::MouseDown(pt)
                if self.rect().contains_point(pt) && !state.is_solved() =>
            {
                Action::redraw().and_return(self.position)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const PASSIVE_MAX_LIGHT_RADIUS: i32 = 11;

struct PassiveLight {
    frame: Sprite,
    position: (i32, i32),
    light_radius: i32,
}

impl PassiveLight {
    fn new(
        resources: &mut Resources,
        state: &AtticState,
        position: (i32, i32),
    ) -> PassiveLight {
        let sprites = resources.get_sprites("light/toggle");
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
            position,
            light_radius: if state.is_lit(position) {
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

impl Element<AtticState, (i32, i32)> for PassiveLight {
    fn draw(&self, _: &AtticState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        draw_light(
            &mut canvas,
            self.light_radius,
            PASSIVE_MAX_LIGHT_RADIUS,
            false,
        );
        let center = canvas.rect().center();
        canvas.draw_sprite_centered(&self.frame, center);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        state: &mut AtticState,
    ) -> Action<(i32, i32)> {
        match event {
            &Event::ClockTick => tick_radius(
                state.is_lit(self.position),
                &mut self.light_radius,
                PASSIVE_MAX_LIGHT_RADIUS,
            ),
            _ => Action::ignore(),
        }
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

fn draw_light(canvas: &mut Canvas, radius: i32, max: i32, hilight: bool) {
    let center = canvas.rect().center();
    if hilight {
        canvas.fill_rect((255, 64, 255), light_rect(center, max));
    } else {
        if radius < max {
            canvas.fill_rect((0, 0, 32), light_rect(center, max));
        }
        if radius > 0 {
            canvas.fill_rect((255, 255, 192), light_rect(center, radius));
        }
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

const INFO_BOX_TEXT: &str = "\
Your goal is to turn all thirty-two lights ON.

$M{Tapp}{Click}ing on one of the lights labelled with a
letter will toggle some of the nearby lights.

The letter labels give a hint as to which other
lights will be toggled by $M{tapp}{click}ing on that light.

$M{Tap}{Click} on a character in the scene to hear their
words of wisdom.";

// ========================================================================= //
