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

use elements::{DangerSign, LaserCmd, LaserField, PuzzleCmd, PuzzleCore,
               PuzzleView};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, MissedState, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<LaserCmd>,
    laser_field: LaserField,
    danger_sign: DangerSign,
    blinkenlights: Vec<Blinkenlight>,
    box_open: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &MissedState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            laser_field: LaserField::new(resources, 104, 72, state.grid()),
            danger_sign: DangerSign::new(resources,
                                         (224, 144),
                                         "ACHTUNG!",
                                         "DAS BLINKENLICHTEN"),
            blinkenlights: vec![Blinkenlight::new(resources, 176, 112, 0),
                                Blinkenlight::new(resources, 176, 144, 3),
                                Blinkenlight::new(resources, 176, 160, 2),
                                Blinkenlight::new(resources, 176, 192, 1),
                                Blinkenlight::new(resources, 336, 112, 1),
                                Blinkenlight::new(resources, 336, 144, 2),
                                Blinkenlight::new(resources, 336, 160, 3),
                                Blinkenlight::new(resources, 336, 192, 0)],
            box_open: false,
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.box_open = value != 0;
            }
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.missed_connections;
        self.core.draw_back_layer(canvas);
        if self.box_open {
            self.laser_field.draw(state.grid(), canvas);
        } else {
            self.danger_sign.draw(canvas);
            self.blinkenlights.draw(&(), canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.missed_connections;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() && self.box_open &&
           (event == &Event::ClockTick || !state.is_solved()) {
            let subaction = self.laser_field
                                .handle_event(event, state.grid_mut());
            if let Some(&cmd) = subaction.value() {
                if self.laser_field.all_detectors_satisfied(state.grid()) {
                    state.mark_solved();
                    self.core.begin_outro_scene();
                } else {
                    self.core.push_undo(cmd);
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() && !self.box_open {
            action.merge(self.blinkenlights.handle_event(event, &mut ()));
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.missed_connections.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        let state = &mut game.missed_connections;
        if let Some(cmd) = self.core.pop_undo() {
            match cmd {
                LaserCmd::Moved(col1, row1, col2, row2) => {
                    state.grid_mut().move_to(col2, row2, col1, row1);
                }
                LaserCmd::Rotated(col, row) => {
                    state.grid_mut().unrotate(col, row);
                }
            }
            self.laser_field.recalculate_lasers(state.grid());
        }
    }

    fn redo(&mut self, game: &mut Game) {
        let state = &mut game.missed_connections;
        if let Some(cmd) = self.core.pop_redo() {
            match cmd {
                LaserCmd::Moved(col1, row1, col2, row2) => {
                    state.grid_mut().move_to(col1, row1, col2, row2);
                }
                LaserCmd::Rotated(col, row) => {
                    state.grid_mut().rotate(col, row);
                }
            }
            self.laser_field.recalculate_lasers(state.grid());
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.missed_connections;
        self.core.clear_undo_redo();
        state.reset();
        self.laser_field.recalculate_lasers(state.grid());
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.missed_connections;
        state.solve();
        self.laser_field.recalculate_lasers(state.grid());
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const BLINK_FRAMES: i32 = 20;

struct Blinkenlight {
    topleft: Point,
    sprites: Vec<Sprite>,
    anim: i32,
}

impl Blinkenlight {
    fn new(resources: &mut Resources, left: i32, top: i32, phase: i32)
           -> Blinkenlight {
        Blinkenlight {
            topleft: Point::new(left, top),
            sprites: resources.get_sprites("blinkenlights"),
            anim: phase * BLINK_FRAMES,
        }
    }
}

impl Element<(), PuzzleCmd> for Blinkenlight {
    fn draw(&self, _unused: &(), canvas: &mut Canvas) {
        let index = if self.anim < BLINK_FRAMES {
            1
        } else {
            0
        };
        canvas.draw_sprite(&self.sprites[index], self.topleft);
    }

    fn handle_event(&mut self, event: &Event, _unused: &mut ())
                    -> Action<PuzzleCmd> {
        match event {
            &Event::ClockTick => {
                self.anim += 1;
                if self.anim >= 4 * BLINK_FRAMES {
                    self.anim = 0;
                    Action::redraw()
                } else if self.anim == BLINK_FRAMES {
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

const INFO_BOX_TEXT: &'static str = "\
Your goal is to activate each detector on the right with
the appropriate color of laser.

Drag mirrors and other objects with $M{your finger}{the mouse} to
move their positions in the grid.  $M{Tap}{Click} objects to rotate
them.";

// ========================================================================= //
