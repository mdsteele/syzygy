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

use elements::{Hud, HudCmd, HudInput, Scene, ScreenFade, Theater};
use gui::{Action, Canvas, Element, Event, Rect, Resources};
use save::{Access, Game, PuzzleState};

// ========================================================================= //

pub enum PuzzleCmd {
    Back,
    Info,
    Undo,
    Redo,
    Reset,
    Replay,
    Solve,
    Next,
}

// ========================================================================= //

pub trait PuzzleView: Element<Game, PuzzleCmd> {
    fn info_text(&self, game: &Game) -> &'static str;

    fn undo(&mut self, game: &mut Game);

    fn redo(&mut self, game: &mut Game);

    fn reset(&mut self, game: &mut Game);

    fn replay(&mut self, game: &mut Game);

    fn solve(&mut self, game: &mut Game);
}

// ========================================================================= //

pub struct PuzzleCore<U> {
    theater: Theater,
    intro_scene: Scene,
    outro_scene: Scene,
    hud: Hud,
    screen_fade: ScreenFade<PuzzleCmd>,
    undo_stack: Vec<U>,
    redo_stack: Vec<U>,
    previously_solved: bool,
}

impl<U: Clone> PuzzleCore<U> {
    pub fn new<S: PuzzleState>(resources: &mut Resources, visible: Rect,
                               state: &S, mut intro_scene: Scene,
                               mut outro_scene: Scene)
                               -> PuzzleCore<U> {
        let mut theater = Theater::new();
        if state.is_visited() {
            intro_scene.skip(&mut theater);
            if state.is_solved() {
                outro_scene.skip(&mut theater);
            }
        } else {
            intro_scene.begin(&mut theater);
        }
        PuzzleCore {
            theater: theater,
            intro_scene: intro_scene,
            outro_scene: outro_scene,
            hud: Hud::new(resources, visible, state.location()),
            screen_fade: ScreenFade::new(resources),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            previously_solved: state.access() >= Access::Solved,
        }
    }

    pub fn flash_info_button(&mut self) { self.hud.flash_info_button(); }

    pub fn theater(&self) -> &Theater { &self.theater }

    pub fn theater_mut(&mut self) -> &mut Theater { &mut self.theater }

    pub fn drain_queue(&mut self) -> Vec<(i32, i32)> {
        self.theater.drain_queue()
    }

    pub fn begin_outro_scene(&mut self) {
        self.outro_scene.begin(&mut self.theater);
    }

    pub fn replay(&mut self) {
        self.theater.reset();
        self.intro_scene.reset();
        self.outro_scene.reset();
        self.intro_scene.begin(&mut self.theater);
        self.screen_fade.fade_in();
    }

    pub fn push_undo(&mut self, change: U) {
        self.undo_stack.push(change);
        self.redo_stack.clear();
    }

    pub fn pop_undo(&mut self) -> Option<U> {
        if let Some(change) = self.undo_stack.pop() {
            self.redo_stack.push(change.clone());
            Some(change)
        } else {
            None
        }
    }

    pub fn pop_redo(&mut self) -> Option<U> {
        if let Some(change) = self.redo_stack.pop() {
            self.undo_stack.push(change.clone());
            Some(change)
        } else {
            None
        }
    }

    pub fn clear_undo_redo(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    fn hud_input<S: PuzzleState>(&self, state: &S) -> HudInput {
        let scene = if state.is_solved() {
            &self.outro_scene
        } else {
            &self.intro_scene
        };
        HudInput {
            name: state.location().name(),
            access: state.access(),
            is_paused: scene.is_paused(),
            active: self.screen_fade.is_transparent() && scene.is_finished(),
            can_undo: !self.undo_stack.is_empty(),
            can_redo: !self.redo_stack.is_empty(),
            can_reset: state.can_reset(),
        }
    }

    pub fn draw_back_layer(&self, canvas: &mut Canvas) {
        self.theater.draw_background(canvas);
    }

    pub fn draw_middle_layer(&self, canvas: &mut Canvas) {
        self.theater.draw_foreground(canvas);
    }

    pub fn draw_front_layer<S: PuzzleState>(&self, canvas: &mut Canvas,
                                            state: &S) {
        self.theater.draw_speech_bubbles(canvas);
        self.hud.draw(&self.hud_input(state), canvas);
        self.screen_fade.draw(&(), canvas);
    }

    pub fn handle_event<S: PuzzleState>(&mut self, event: &Event, state: &S)
                                        -> Action<PuzzleCmd> {
        let mut action = self.screen_fade.handle_event(event, &mut ());
        if !action.should_stop() {
            let subaction = if state.is_solved() {
                self.outro_scene.handle_event(event, &mut self.theater)
            } else {
                self.intro_scene.handle_event(event, &mut self.theater)
            };
            if !self.previously_solved && self.outro_scene.is_finished() &&
               self.screen_fade.is_transparent() {
                self.screen_fade.fade_out_and_return(PuzzleCmd::Next);
            }
            action.merge(subaction.but_no_value());
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
        action
    }
}

// ========================================================================= //
