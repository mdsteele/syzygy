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

use elements::{Hud, HudCmd, HudInput, LaserCmd, LaserField, PuzzleCmd,
               PuzzleView, Scene, ScreenFade, Theater};
use gui::{Action, Canvas, Element, Event, Rect, Resources};
use modes::SOLVED_INFO_TEXT;
use save::{Game, Location, MissedState, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    theater: Theater,
    intro_scene: Scene,
    outro_scene: Scene,
    screen_fade: ScreenFade<PuzzleCmd>,
    hud: Hud,
    laser_field: LaserField,
    undo_stack: Vec<LaserCmd>,
    redo_stack: Vec<LaserCmd>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &MissedState)
               -> View {
        // TODO: Make a background for "Missed Connections".
        let background = resources.get_background("disconnected");
        let mut theater = Theater::new(background);
        let mut intro_scene = compile_intro_scene(resources);
        let mut outro_scene = compile_outro_scene(resources);
        if state.is_visited() {
            intro_scene.skip(&mut theater);
            if state.is_solved() {
                outro_scene.skip(&mut theater);
            }
        } else {
            intro_scene.begin(&mut theater);
        }
        let mut view = View {
            theater: theater,
            intro_scene: intro_scene,
            outro_scene: outro_scene,
            screen_fade: ScreenFade::new(resources),
            hud: Hud::new(resources, visible, Location::MissedConnections),
            laser_field: LaserField::new(resources, 120, 72, state.grid()),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        };
        view.drain_queue();
        view
    }

    fn current_scene(&self, state: &MissedState) -> &Scene {
        if state.is_solved() {
            &self.outro_scene
        } else {
            &self.intro_scene
        }
    }

    fn hud_input(&self, state: &MissedState) -> HudInput {
        let scene = self.current_scene(state);
        HudInput {
            name: "Missed Connections",
            access: state.access(),
            is_paused: scene.is_paused(),
            active: self.screen_fade.is_transparent() && scene.is_finished(),
            can_undo: !self.undo_stack.is_empty(),
            can_redo: !self.redo_stack.is_empty(),
            can_reset: state.can_reset(),
        }
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.theater.drain_queue() {
            // TODO drain queue
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.missed_connections;
        self.theater.draw_background(canvas);
        self.laser_field.draw(state.grid(), canvas);
        self.theater.draw_foreground(canvas);
        self.theater.draw_speech_bubbles(canvas);
        self.hud.draw(&self.hud_input(state), canvas);
        self.screen_fade.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.missed_connections;
        let mut action = self.screen_fade.handle_event(event, &mut ());
        if !action.should_stop() {
            let subaction = if state.is_solved() {
                self.outro_scene.handle_event(event, &mut self.theater)
            } else {
                self.intro_scene.handle_event(event, &mut self.theater)
            };
            action.merge(subaction.but_no_value());
            self.drain_queue();
        }
        if !action.should_stop() {
            let mut input = self.hud_input(state);
            let subaction = self.hud.handle_event(event, &mut input);
            action.merge(match subaction.value() {
                Some(&HudCmd::Back) => {
                    self.screen_fade.fade_out_and_return(PuzzleCmd::Back);
                    subaction.but_no_value()
                }
                Some(&HudCmd::Info) => subaction.but_return(PuzzleCmd::Info),
                Some(&HudCmd::Undo) => subaction.but_return(PuzzleCmd::Undo),
                Some(&HudCmd::Redo) => subaction.but_return(PuzzleCmd::Redo),
                Some(&HudCmd::Reset) => subaction.but_return(PuzzleCmd::Reset),
                Some(&HudCmd::Replay) => {
                    self.screen_fade.fade_out_and_return(PuzzleCmd::Replay);
                    subaction.but_no_value()
                }
                Some(&HudCmd::Solve) => subaction.but_return(PuzzleCmd::Solve),
                None => subaction.but_no_value(),
            });
        }
        if !action.should_stop() &&
           (event == &Event::ClockTick || !state.is_solved()) {
            let subaction = self.laser_field
                                .handle_event(event, state.grid_mut());
            if let Some(&cmd) = subaction.value() {
                if self.laser_field.all_detectors_satisfied(state.grid()) {
                    state.mark_solved();
                    if cfg!(debug_assertions) {
                        println!("Puzzle solved, beginning outro.");
                    }
                    self.outro_scene.begin(&mut self.theater);
                    self.undo_stack.clear();
                } else {
                    self.undo_stack.push(cmd);
                }
                self.redo_stack.clear();
            }
            action.merge(subaction.but_no_value());
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
        if let Some(cmd) = self.undo_stack.pop() {
            self.redo_stack.push(cmd);
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
        if let Some(cmd) = self.redo_stack.pop() {
            self.undo_stack.push(cmd);
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
        self.undo_stack.clear();
        self.redo_stack.clear();
        state.reset();
        self.laser_field.recalculate_lasers(state.grid());
    }

    fn replay(&mut self, game: &mut Game) {
        game.missed_connections.replay();
        self.laser_field.recalculate_lasers(game.missed_connections.grid());
        self.theater.reset();
        self.intro_scene.reset();
        self.outro_scene.reset();
        self.intro_scene.begin(&mut self.theater);
        self.drain_queue();
        self.screen_fade.fade_in();
    }

    fn solve(&mut self, _game: &mut Game) {
        // TODO solve
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
