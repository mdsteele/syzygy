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

use elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PuzzleState, TreadState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32)>,
    toggles: Vec<ToggleLight>,
    passives: Vec<PassiveLight>,
    next: NextLetter,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &TreadState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        core.add_extra_scene(scenes::compile_relyng_midscene(resources));
        View {
            core: core,
            toggles: vec![
                ToggleLight::new(resources, state, (1, 1)),
                ToggleLight::new(resources, state, (2, 1)),
                ToggleLight::new(resources, state, (3, 1)),
                ToggleLight::new(resources, state, (4, 1)),
                ToggleLight::new(resources, state, (1, 2)),
                ToggleLight::new(resources, state, (2, 2)),
                ToggleLight::new(resources, state, (3, 2)),
                ToggleLight::new(resources, state, (4, 2)),
                ToggleLight::new(resources, state, (1, 3)),
                ToggleLight::new(resources, state, (2, 3)),
                ToggleLight::new(resources, state, (3, 3)),
                ToggleLight::new(resources, state, (4, 3)),
            ],
            passives: vec![
                PassiveLight::new(resources, state, (0, 0)),
                PassiveLight::new(resources, state, (1, 0)),
                PassiveLight::new(resources, state, (2, 0)),
                PassiveLight::new(resources, state, (3, 0)),
                PassiveLight::new(resources, state, (4, 0)),
                PassiveLight::new(resources, state, (5, 0)),
                PassiveLight::new(resources, state, (5, 1)),
                PassiveLight::new(resources, state, (5, 2)),
                PassiveLight::new(resources, state, (5, 3)),
                PassiveLight::new(resources, state, (5, 4)),
                PassiveLight::new(resources, state, (4, 4)),
                PassiveLight::new(resources, state, (3, 4)),
                PassiveLight::new(resources, state, (2, 4)),
                PassiveLight::new(resources, state, (1, 4)),
                PassiveLight::new(resources, state, (0, 4)),
                PassiveLight::new(resources, state, (0, 3)),
                PassiveLight::new(resources, state, (0, 2)),
                PassiveLight::new(resources, state, (0, 1)),
            ],
            next: NextLetter::new(resources),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.tread_lightly;
        self.core.draw_back_layer(canvas);
        self.next.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.passives.draw(state, canvas);
        self.toggles.draw(state, canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.tread_lightly;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() {
            let subaction = self.toggles.handle_event(event, state);
            if let Some(&position) = subaction.value() {
                if state.push_toggle(position) {
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                    } else {
                        self.core.push_undo(position);
                    }
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            action.merge(self.passives.handle_event(event, state));
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
        if game.tread_lightly.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(_) = self.core.pop_undo() {
            game.tread_lightly.pop_toggle();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(position) = self.core.pop_redo() {
            game.tread_lightly.push_toggle(position);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.tread_lightly.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.tread_lightly.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (index, enable) in self.core.drain_queue() {
            if index < 0 {
                self.next.visible = enable != 0;
            } else if (index as usize) < self.toggles.len() {
                self.toggles[index as usize].set_hilight(enable != 0);
            }
        }
    }
}

// ========================================================================= //

const LIGHTS_TOP: i32 = 72;
const LIGHTS_LEFT: i32 = 200;
const TOGGLE_MAX_LIGHT_RADIUS: i32 = 12;

struct ToggleLight {
    font: Rc<Font>,
    frame: Sprite,
    position: (i32, i32),
    light_radius: i32,
    hilight: bool,
}

impl ToggleLight {
    fn new(resources: &mut Resources, state: &TreadState,
           position: (i32, i32))
           -> ToggleLight {
        ToggleLight {
            font: resources.get_font("block"),
            frame: resources.get_sprites("light/toggle")[0].clone(),
            position: position,
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

    fn set_hilight(&mut self, hilight: bool) { self.hilight = hilight; }
}

impl Element<TreadState, (i32, i32)> for ToggleLight {
    fn draw(&self, state: &TreadState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        draw_light(&mut canvas,
                   self.light_radius,
                   TOGGLE_MAX_LIGHT_RADIUS,
                   self.hilight);
        let center = canvas.rect().center();
        if let Some(chr) = state.toggled_label(self.position) {
            let pt = center + Point::new(0, 9);
            canvas.draw_char(&self.font, Align::Center, pt, chr);
        }
        canvas.draw_sprite_centered(&self.frame, center);
    }

    fn handle_event(&mut self, event: &Event, state: &mut TreadState)
                    -> Action<(i32, i32)> {
        match event {
            &Event::ClockTick => {
                tick_radius(state.is_lit(self.position),
                            &mut self.light_radius,
                            TOGGLE_MAX_LIGHT_RADIUS)
            }
            &Event::MouseDown(pt)
                if self.rect().contains_point(pt) && !state.is_solved() => {
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
    fn new(resources: &mut Resources, state: &TreadState,
           position: (i32, i32))
           -> PassiveLight {
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
            position: position,
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

impl Element<TreadState, PuzzleCmd> for PassiveLight {
    fn draw(&self, _: &TreadState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        draw_light(&mut canvas,
                   self.light_radius,
                   PASSIVE_MAX_LIGHT_RADIUS,
                   false);
        let center = canvas.rect().center();
        canvas.draw_sprite_centered(&self.frame, center);
    }

    fn handle_event(&mut self, event: &Event, state: &mut TreadState)
                    -> Action<PuzzleCmd> {
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

struct NextLetter {
    font: Rc<Font>,
    visible: bool,
}

impl NextLetter {
    fn new(resources: &mut Resources) -> NextLetter {
        NextLetter {
            font: resources.get_font("block"),
            visible: false,
        }
    }
}

impl Element<TreadState, PuzzleCmd> for NextLetter {
    fn draw(&self, state: &TreadState, canvas: &mut Canvas) {
        if self.visible {
            canvas.fill_rect((0, 0, 127), Rect::new(86, 102, 36, 36));
            if let Some(chr) = state.next_label() {
                let pt = Point::new(104, 130);
                canvas.draw_char(&self.font, Align::Center, pt, chr);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, _state: &mut TreadState)
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
Your goal is to turn all thirty lights OFF by placing seven
letters onto the grid.

$M{Tap}{Click} one of the center twelve lights to place the next
letter on that light; this will toggle that light as well as
some of the surrounding lights, depending on the letter.

If any lights are still on after you place the seventh
letter, you will need to use the reset button or the
undo button to back up and try again.

$M{Tap}{Click} on a character in the scene to hear their words of
wisdom.";

// ========================================================================= //
