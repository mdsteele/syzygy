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

use elements::{FadeStyle, MovingStars, PuzzleCmd, PuzzleCore, PuzzleView,
               Scene};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use modes::syzygy::Atlatl;
use save::{FinaleState, Game, PuzzleState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    stars_space: MovingStars,
    sun_sprites: Vec<Sprite>,
    xanadu3_sprites: Vec<Sprite>,
    xanadu4_sprites: Vec<Sprite>,
    planets_visible: bool,
    atlatl: Atlatl,
    atlatl_visible: bool,
    atlatl_beam: AtlatlBeam,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &FinaleState)
               -> View {
        let core = {
            let fade = (FadeStyle::BottomToTop, FadeStyle::TopToBottom);
            let intro = scenes::compile_scene(resources);
            let outro = Scene::empty();
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        View {
            core: core,
            stars_space: MovingStars::new(0, 0, 576, 384),
            sun_sprites: resources.get_sprites("title/sun"),
            xanadu3_sprites: resources.get_sprites("title/xanadu3"),
            xanadu4_sprites: resources.get_sprites("title/xanadu4"),
            planets_visible: false,
            atlatl: Atlatl::new(resources),
            atlatl_visible: false,
            atlatl_beam: AtlatlBeam::new(),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.finale;
        self.core.draw_back_layer(canvas);
        self.stars_space.draw(canvas);
        if self.planets_visible {
            canvas.fill_rect((255, 255, 255), Rect::new(0, 0, 64, 64));
            canvas.draw_sprite(&self.sun_sprites[0], Point::new(64, 0));
            canvas.draw_sprite(&self.sun_sprites[1], Point::new(64, 64));
            canvas.draw_sprite(&self.sun_sprites[2], Point::new(0, 64));
            canvas.draw_sprite_centered(&self.xanadu3_sprites[0],
                                        Point::new(288, 225));
            canvas.draw_sprite_centered(&self.xanadu4_sprites[0],
                                        Point::new(421, 166));
        }
        self.core.draw_middle_layer(canvas);
        if self.atlatl_visible {
            self.atlatl.draw(&(), canvas);
        }
        self.atlatl_beam.draw(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.finale;
        let mut action = self.core.handle_event(event, state);
        if event == &Event::ClockTick {
            if self.stars_space.tick_animation() {
                action.also_redraw();
            }
            if self.atlatl_beam.tick_animation() {
                action.also_redraw();
            }
        }
        if !action.should_stop() || event == &Event::ClockTick {
            let subaction = self.atlatl.handle_event(event, &mut ());
            action.merge(subaction.but_no_value());
        }
        if state.is_solved() {
            self.core.begin_outro_scene();
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, _game: &Game) -> &'static str { INFO_BOX_TEXT }

    fn undo(&mut self, _game: &mut Game) {}

    fn redo(&mut self, _game: &mut Game) {}

    fn reset(&mut self, _game: &mut Game) {}

    fn solve(&mut self, _game: &mut Game) {}

    fn drain_queue(&mut self) {
        for (device, value) in self.core.drain_queue() {
            match device {
                1 => self.stars_space.set_visible(value != 0),
                2 => self.planets_visible = value != 0,
                3 => self.atlatl_visible = value != 0,
                4 => self.atlatl.set_all_indicators(value != 0),
                5 => {
                    if value == 1 {
                        self.atlatl_beam.turn_on(258, 258);
                    } else if value == 2 {
                        self.atlatl_beam.turn_on(576, 576);
                    } else if value == 3 {
                        self.atlatl_beam.turn_on(576, 280);
                    } else {
                        self.atlatl_beam.turn_off();
                    }
                }
                _ => {}
            }
        }
    }
}

// ========================================================================= //

const BEAM_SPEED: u32 = 32; // pixels/frame
const BEAM_THICKNESS: u32 = 3;

struct AtlatlBeam {
    start: i32,
    length: u32,
    max_length: u32,
    anim: u32,
}

impl AtlatlBeam {
    fn new() -> AtlatlBeam {
        AtlatlBeam {
            start: 0,
            length: 0,
            max_length: 0,
            anim: 0,
        }
    }

    fn turn_on(&mut self, start: i32, max_length: u32) {
        self.start = start;
        self.length = 0;
        self.max_length = max_length;
        self.anim = 0;
    }

    fn turn_off(&mut self) {
        self.length = 0;
        self.max_length = 0;
    }

    fn draw(&self, canvas: &mut Canvas) {
        if self.length > 0 {
            let color = (if self.anim != 0 { 255 } else { 128 },
                         if self.anim != 1 { 255 } else { 128 },
                         if self.anim != 2 { 255 } else { 128 });
            let rect = Rect::new(self.start - (self.length as i32),
                                 197 - (BEAM_THICKNESS / 2) as i32,
                                 self.length,
                                 BEAM_THICKNESS);
            canvas.fill_rect(color, rect);
        }
    }

    fn tick_animation(&mut self) -> bool {
        if self.max_length == 0 {
            return false;
        }
        self.length = (self.length + BEAM_SPEED).min(self.max_length);
        self.anim = (self.anim + 1) % 3;
        true
    }
}

// ========================================================================= //

pub const INFO_BOX_TEXT: &str = "\
Return to the map to select another scene.";

// ========================================================================= //
