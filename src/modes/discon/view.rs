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
use gui::{Action, Canvas, Element, Event, Rect, Resources};
use modes::SOLVED_INFO_TEXT;
use save::{DisconState, Game, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<LaserCmd>,
    laser_field: LaserField,
    danger_sign: DangerSign,
    box_open: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &DisconState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            laser_field: LaserField::new(resources, 120, 72, state.grid()),
            danger_sign: DangerSign::new(resources,
                                         (224, 160),
                                         "%DANGER",
                                         "HIGH VOLTAGE"),
            box_open: false,
        };
        view.drain_queue();
        view
    }

    pub fn flash_info_button(&mut self) { self.core.flash_info_button(); }

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
        let state = &game.disconnected;
        self.core.draw_back_layer(canvas);
        if self.box_open {
            self.laser_field.draw(state.grid(), canvas);
        } else {
            self.danger_sign.draw(canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.disconnected;
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
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.disconnected.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        let state = &mut game.disconnected;
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
        let state = &mut game.disconnected;
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
        let state = &mut game.disconnected;
        self.core.clear_undo_redo();
        state.reset();
        self.laser_field.recalculate_lasers(state.grid());
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.disconnected;
        state.solve();
        self.laser_field.recalculate_lasers(state.grid());
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to activate each detector on the right with
the appropriate color of laser.

Drag mirrors with $M{your finger}{the mouse} to move their positions in
the grid.  $M{Tap}{Click} mirrors to rotate them.";

// ========================================================================= //
