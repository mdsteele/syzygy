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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use elements::lasers::{LaserCmd, LaserField};
use elements::plane::{PlaneCmd, PlaneGridView};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sound};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PuzzleState, SyzygyStage, SyzygyState};
use super::relyng::LightsGrid;
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

#[derive(Clone)]
enum UndoRedo {
    Elinsa(Vec<(Point, Point)>),
    Ugrent(LaserCmd),
    Relyng((i32, i32)),
}

// ========================================================================= //

pub struct View {
    core: PuzzleCore<UndoRedo>,
    elinsa: PlaneGridView,
    ugrent: LaserField,
    relyng: LightsGrid,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &SyzygyState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            elinsa: PlaneGridView::new(resources, 150, 140),
            ugrent: LaserField::new(resources, 175, 140, state.ugrent_grid()),
            relyng: LightsGrid::new(resources, 168, 140, state),
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.core.drain_queue() {
            // TODO drain queue
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.system_syzygy;
        self.core.draw_back_layer(canvas);
        match state.stage() {
            SyzygyStage::Elinsa => {
                self.elinsa.draw(state.elinsa_grid(), canvas);
            }
            SyzygyStage::Ugrent => {
                self.ugrent.draw(state.ugrent_grid(), canvas);
            }
            SyzygyStage::Relyng => {
                self.relyng.draw(state, canvas);
            }
            _ => {} // TODO
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.system_syzygy;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() && !state.is_solved() {
            match state.stage() {
                SyzygyStage::Elinsa => {
                    let mut subaction =
                        self.elinsa
                            .handle_event(event, state.elinsa_grid_mut());
                    match subaction.take_value() {
                        Some(PlaneCmd::Changed) => {
                            if state.advance_stage_if_done() {
                                self.core.clear_undo_redo();
                                self.elinsa.cancel_drag_and_clear_changes();
                                let sound = Sound::solve_puzzle_chime();
                                action.also_play_sound(sound);
                            }
                        }
                        Some(PlaneCmd::PushUndo(changes)) => {
                            self.core.push_undo(UndoRedo::Elinsa(changes));
                        }
                        None => {}
                    }
                    action.merge(subaction.but_no_value());
                }
                SyzygyStage::Ugrent => {
                    let subaction = self.ugrent
                                        .handle_event(event,
                                                      state.ugrent_grid_mut());
                    if let Some(&cmd) = subaction.value() {
                        let grid = state.ugrent_grid();
                        if self.ugrent.all_detectors_satisfied(grid) {
                            self.core.clear_undo_redo();
                            // TODO advance stage
                            let sound = Sound::solve_puzzle_chime();
                            action.also_play_sound(sound);
                        } else {
                            self.core.push_undo(UndoRedo::Ugrent(cmd));
                        }
                    }
                    action.merge(subaction.but_no_value());
                }
                SyzygyStage::Relyng => {
                    let subaction = self.relyng.handle_event(event, state);
                    if let Some(&pos) = subaction.value() {
                        state.relyng_toggle(pos);
                        if state.relyng_is_done() {
                            self.core.clear_undo_redo();
                            // TODO advance stage
                            let sound = Sound::solve_puzzle_chime();
                            action.also_play_sound(sound);
                        } else {
                            self.core.push_undo(UndoRedo::Relyng(pos));
                        }
                    }
                    action.merge(subaction.but_no_value());
                }
                _ => {} // TODO
            }
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.system_syzygy.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            match game.system_syzygy.stage() {
                SyzygyStage::Yttris => YTTRIS_INFO_BOX_TEXT,
                SyzygyStage::Argony => ARGONY_INFO_BOX_TEXT,
                SyzygyStage::Elinsa => ELINSA_INFO_BOX_TEXT,
                SyzygyStage::Ugrent => UGRENT_INFO_BOX_TEXT,
                SyzygyStage::Relyng => RELYNG_INFO_BOX_TEXT,
                SyzygyStage::Mezure => MEZURE_INFO_BOX_TEXT,
            }
        }
    }

    fn undo(&mut self, game: &mut Game) {
        let state = &mut game.system_syzygy;
        match self.core.pop_undo() {
            Some(UndoRedo::Relyng(pos)) => state.relyng_untoggle(pos),
            Some(_) => {} // TODO other undos
            None => {}
        }
    }

    fn redo(&mut self, game: &mut Game) {
        let state = &mut game.system_syzygy;
        match self.core.pop_redo() {
            Some(UndoRedo::Relyng(pos)) => state.relyng_toggle(pos),
            Some(_) => {} // TODO other redos
            None => {}
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.system_syzygy;
        self.core.clear_undo_redo();
        state.reset();
        self.ugrent.recalculate_lasers(state.ugrent_grid());
    }

    fn solve(&mut self, game: &mut Game) {
        game.system_syzygy.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const YTTRIS_INFO_BOX_TEXT: &'static str = "\
Your goal is to turn the word TANGENT into the word COSINE.

$M{Tap}{Click} on one of the six buttons at the top to transform the
word.  Each button performs a different transformation.";

const ARGONY_INFO_BOX_TEXT: &'static str = "\
Your goal is to TODO.";

const ELINSA_INFO_BOX_TEXT: &'static str = "\
Your goal is to connect each red node to each blue
node.  The purple node counts as both red and blue.

Drag across the grid with $M{your finger}{the mouse} to create or
remove pipes between the nodes.";

const UGRENT_INFO_BOX_TEXT: &'static str = "\
Your goal is to activate each detector on the right with
the appropriate color of laser.

Drag mirrors and other objects with $M{your finger}{the mouse} to
move their positions in the grid.  $M{Tap}{Click} objects to rotate
them.";

const RELYNG_INFO_BOX_TEXT: &'static str = "\
Your goal is to turn all twenty lights OFF.

$M{Tap}{Click} one of the lights to toggle that light and some
of the adjacent lights.  The pattern of adjancent lights
toggled will change after each move.";

const MEZURE_INFO_BOX_TEXT: &'static str = "\
Your goal is to form the final, missing word.";

// ========================================================================= //
