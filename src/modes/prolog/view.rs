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

use std::rc::Rc;

use elements::{MovingStars, PuzzleCmd, PuzzleCore, PuzzleView, Scene};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sprite};
use modes::attic::AtticGrid;
use modes::wrecked::{WreckedDisplay, WreckedGrid};
use save::{AtticState, Game, PrologState, PuzzleState, WreckedState};
use super::scenes::compile_scene;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    status: StatusIndicator,
    stars_space: MovingStars,
    stars_window1: MovingStars,
    stars_window2: MovingStars,
    wrecked_state: WreckedState,
    wrecked_grid: WreckedGrid,
    wrecked_display: WreckedDisplay,
    wrecked_visible: bool,
    attic_state: AtticState,
    attic_grid: AtticGrid,
    attic_visible: bool,
    spawn_point: SpawnPoint,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &PrologState)
               -> View {
        let intro = compile_scene(resources);
        let outro = Scene::empty();
        let core = PuzzleCore::new(resources, visible, state, intro, outro);

        let mut wrecked_display = WreckedDisplay::new(resources);
        wrecked_display.set_panic(true);

        let mut attic_state = AtticState::new();
        attic_state.solve();
        let mut attic_grid = AtticGrid::new(resources, &attic_state);
        attic_grid.do_not_show_corner_lights();

        View {
            core: core,
            status: StatusIndicator::new(resources, 304, 64),
            stars_space: MovingStars::new(0, 0, 576, 384),
            stars_window1: MovingStars::new(144, 144, 64, 32),
            stars_window2: MovingStars::new(336, 144, 64, 32),
            wrecked_state: WreckedState::new(),
            wrecked_grid: WreckedGrid::new(resources),
            wrecked_display: wrecked_display,
            wrecked_visible: false,
            attic_state: attic_state,
            attic_grid: attic_grid,
            attic_visible: false,
            spawn_point: SpawnPoint::new(resources, 240, 248),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.prolog;
        self.core.draw_back_layer(canvas);
        self.stars_space.draw(canvas);
        self.stars_window1.draw(canvas);
        self.stars_window2.draw(canvas);
        self.status.draw(self.core.theater().shake_offset(), canvas);
        if self.wrecked_visible {
            self.wrecked_display.draw(&self.wrecked_state, canvas);
            self.wrecked_grid.draw(&self.wrecked_state, canvas);
        }
        self.spawn_point.draw(canvas);
        self.core.draw_middle_layer(canvas);
        if self.attic_visible {
            self.attic_grid.draw(&self.attic_state, canvas);
        }
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.prolog;
        let mut action = self.core.handle_event(event, state);
        if event == &Event::ClockTick {
            if self.status.tick_animation() {
                action.also_redraw();
            }
            if self.stars_space.tick_animation() {
                action.also_redraw();
            }
            if self.stars_window1.tick_animation() {
                action.also_redraw();
            }
            if self.stars_window2.tick_animation() {
                action.also_redraw();
            }
            if self.attic_visible {
                let subaction =
                    self.attic_grid.handle_event(event, &mut self.attic_state);
                action.merge(subaction.but_no_value());
            }
            if self.spawn_point.tick_animation() {
                action.also_redraw();
            }
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
                1 => self.status.set_mode(value),
                2 => {
                    match value {
                        1 => {
                            self.stars_space.set_visible(true);
                            self.stars_window1.set_visible(false);
                            self.stars_window2.set_visible(false);
                        }
                        2 => {
                            self.stars_space.set_visible(false);
                            self.stars_window1.set_visible(true);
                            self.stars_window2.set_visible(true);
                        }
                        _ => {
                            self.stars_space.set_visible(false);
                            self.stars_window1.set_visible(false);
                            self.stars_window2.set_visible(false);
                        }
                    }
                }
                3 => self.wrecked_visible = value != 0,
                4 => {
                    self.attic_visible = value != 0;
                    if value < 0 {
                        self.attic_state.reset();
                    }
                }
                5 => self.spawn_point.visible = value != 0,
                _ => {}
            }
        }
    }
}

// ========================================================================= //

const STATUS_ON_FRAMES: i32 = 16;
const STATUS_OFF_FRAMES: i32 = 8;

struct StatusIndicator {
    font: Rc<Font>,
    left: i32,
    top: i32,
    mode: i32,
    anim: i32,
}

impl StatusIndicator {
    fn new(resources: &mut Resources, left: i32, top: i32) -> StatusIndicator {
        StatusIndicator {
            font: resources.get_font("roman"),
            left: left,
            top: top,
            mode: 0,
            anim: 0,
        }
    }

    fn is_visible(&self) -> bool { self.mode > 0 }

    fn set_mode(&mut self, mode: i32) {
        self.mode = mode;
        self.anim = 0;
    }

    fn draw(&self, offset: Point, canvas: &mut Canvas) {
        let (color, msg1, msg2) = match self.mode {
            1 => ((63, 255, 63), "EVERYTHING", "IS FINE"),
            2 => ((255, 63, 63), "NOTHING", "IS FINE"),
            3 => ((255, 63, 63), "EVERYTHING", "IS RUINED"),
            4 => ((255, 63, 63), "SOMETHING", "IS ON FIRE"),
            _ => return,
        };
        let mut canvas = canvas.subcanvas(Rect::new(self.left + offset.x(),
                                                    self.top + offset.y(),
                                                    96,
                                                    32));
        if self.anim < STATUS_ON_FRAMES {
            canvas.fill_rect(color, Rect::new(3, 3, 90, 14));
            canvas.draw_text(&self.font,
                             Align::Center,
                             Point::new(48, 14),
                             msg1);
        } else {
            canvas.fill_rect((47, 47, 63), Rect::new(3, 3, 90, 14));
        }
        canvas.draw_text(&self.font, Align::Center, Point::new(48, 28), msg2);
    }

    fn tick_animation(&mut self) -> bool {
        self.anim += 1;
        if self.anim == STATUS_ON_FRAMES {
            self.is_visible()
        } else if self.anim >= STATUS_ON_FRAMES + STATUS_OFF_FRAMES {
            self.anim = 0;
            self.is_visible()
        } else {
            false
        }
    }
}

// ========================================================================= //

const SPAWN_DELAY_FRAMES: u32 = 25;

struct SpawnPoint {
    center: Point,
    lightning_sprites: Vec<Sprite>,
    mezure_sprites: Vec<Sprite>,
    anim: u32,
    visible: bool,
}

impl SpawnPoint {
    fn new(resources: &mut Resources, cx: i32, cy: i32) -> SpawnPoint {
        SpawnPoint {
            center: Point::new(cx, cy),
            lightning_sprites: resources.get_sprites("prolog/spawn"),
            mezure_sprites: resources.get_sprites("chars/mezure"),
            anim: 0,
            visible: false,
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        if self.visible {
            let sprite = &self.lightning_sprites[((self.anim / 2) % 4) as
                                                     usize];
            canvas.draw_sprite_centered(sprite, self.center);
            if self.anim > SPAWN_DELAY_FRAMES {
                let height = (self.anim - SPAWN_DELAY_FRAMES).min(32);
                let rect = Rect::new(self.center.x() - 16,
                                     self.center.y() + 16 - height as i32,
                                     32,
                                     height);
                let mut subcanvas = canvas.subcanvas(rect);
                let sprite = &self.mezure_sprites[0];
                let pt = Point::new(-((self.anim % 2) as i32),
                                    (height as i32) - 32);
                subcanvas.draw_sprite(sprite, pt);
            }
        }
    }

    fn tick_animation(&mut self) -> bool {
        if self.visible {
            self.anim += 1;
            true
        } else {
            false
        }
    }
}

// ========================================================================= //

pub const INFO_BOX_TEXT: &str = "\
Return to the map to select another scene.";

// ========================================================================= //
