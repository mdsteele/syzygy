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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView, Scene};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources};
use save::{Game, PrologState, PuzzleState};
use super::scenes::compile_scene;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    status: StatusIndicator,
    stars_space: MovingStars,
    stars_window1: MovingStars,
    stars_window2: MovingStars,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &PrologState)
               -> View {
        let intro = compile_scene(resources);
        let outro = Scene::empty();
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            status: StatusIndicator::new(resources, 304, 64),
            stars_space: MovingStars::new(0, 0, 576, 384),
            stars_window1: MovingStars::new(144, 144, 64, 32),
            stars_window2: MovingStars::new(336, 144, 64, 32),
        };
        view.drain_queue();
        view
    }

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
                _ => {}
            }
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
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.prolog;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
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
}

// ========================================================================= //

struct MovingStars {
    rect: Rect,
    anim: i32,
    visible: bool,
}

impl MovingStars {
    fn new(left: i32, top: i32, width: u32, height: u32) -> MovingStars {
        MovingStars {
            rect: Rect::new(left, top, width, height),
            anim: 0,
            visible: false,
        }
    }

    fn set_visible(&mut self, visible: bool) { self.visible = visible; }

    fn rand(range: u32, seed: &mut (u32, u32)) -> i32 {
        seed.0 = 36969 * (seed.0 & 0xffff) + (seed.0 >> 16);
        seed.1 = 18000 * (seed.1 & 0xffff) + (seed.1 >> 16);
        let next = (seed.0 << 16) | (seed.1 & 0xffff);
        (next % range) as i32
    }

    fn draw_star(&self, x: i32, y: i32, width: u32, gray: u8,
                 canvas: &mut Canvas) {
        canvas.fill_rect((gray, gray, gray), Rect::new(x, y, width, 1));
    }

    fn draw_layer(&self, spacing: u32, speed: i32, gray: u8,
                  canvas: &mut Canvas) {
        let mut seed = (123456789, 987654321);
        let star_width = (speed / 2) as u32;
        let modulus = (self.rect.width() + spacing) as i32;
        let scroll = (self.anim * speed) % modulus;
        let mut yoff = 0;
        while yoff < modulus {
            let mut xoff = 0;
            while xoff < modulus {
                let x = ((xoff + scroll) % modulus) - spacing as i32 +
                        MovingStars::rand(spacing, &mut seed);
                let y = yoff + MovingStars::rand(spacing, &mut seed);
                self.draw_star(x, y, star_width, gray, canvas);
                xoff += spacing as i32;
            }
            yoff += spacing as i32;
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        if self.visible {
            let mut canvas = canvas.subcanvas(self.rect);
            canvas.clear((0, 0, 0));
            self.draw_layer(16, 8, 63, &mut canvas);
            self.draw_layer(32, 16, 127, &mut canvas);
        }
    }

    fn tick_animation(&mut self) -> bool {
        if self.visible {
            self.anim += 1;
        }
        self.visible
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
            true
        } else if self.anim >= STATUS_ON_FRAMES + STATUS_OFF_FRAMES {
            self.anim = 0;
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
