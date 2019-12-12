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

use std::collections::HashMap;

use crate::elements::{FadeStyle, Hud, HudCmd, HudInput, Scene, ScreenFade, Theater};
use crate::gui::{Action, Canvas, Element, Event, Rect, Resources};
use crate::save::{Access, Game, Location, PuzzleState};

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
    Save,
}

// ========================================================================= //

pub trait PuzzleView: Element<Game, PuzzleCmd> {
    fn info_text(&self, game: &Game) -> &'static str;

    fn undo(&mut self, game: &mut Game);

    fn redo(&mut self, game: &mut Game);

    fn reset(&mut self, game: &mut Game);

    fn solve(&mut self, game: &mut Game);

    fn drain_queue(&mut self);
}

// ========================================================================= //

pub struct PuzzleCore<U> {
    theater: Theater,
    intro_scene: Scene,
    middle_scene: Option<Scene>,
    outro_scene: Scene,
    extra_scenes: HashMap<i32, Scene>,
    hud: Hud,
    screen_fade: ScreenFade<PuzzleCmd>,
    undo_stack: Vec<U>,
    redo_stack: Vec<U>,
    previously_solved: bool,
}

impl<U: Clone> PuzzleCore<U> {
    pub fn new<S: PuzzleState>(resources: &mut Resources, visible: Rect,
                               state: &S, fade: (FadeStyle, FadeStyle),
                               mut intro_scene: Scene,
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
            middle_scene: None,
            outro_scene: outro_scene,
            extra_scenes: HashMap::new(),
            hud: Hud::new(resources, visible, S::location()),
            screen_fade: ScreenFade::new(resources, fade.0, fade.1),
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

    pub fn add_extra_scene(&mut self, (key, scene): (i32, Scene)) {
        self.extra_scenes.insert(key, scene);
    }

    pub fn begin_extra_scene(&mut self, key: i32) {
        if let Some(scene) = self.extra_scenes.get(&key) {
            let mut scene = scene.clone();
            scene.begin(&mut self.theater);
            self.middle_scene = Some(scene);
        }
    }

    pub fn skip_extra_scene(&mut self, key: i32) {
        if let Some(scene) = self.extra_scenes.get(&key) {
            scene.clone().skip(&mut self.theater);
        }
    }

    pub fn begin_character_scene_on_click(&mut self, event: &Event) {
        if let &Event::MouseDown(pt) = event {
            if let Some(key) = self.theater.actor_at_point(pt) {
                self.begin_extra_scene(key);
            }
        }
    }

    pub fn begin_outro_scene(&mut self) {
        self.clear_undo_redo();
        self.outro_scene.begin(&mut self.theater);
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
        let scene = if !self.intro_scene.is_finished() {
            &self.intro_scene
        } else if let Some(ref scene) = self.middle_scene {
            scene
        } else if state.is_solved() {
            &self.outro_scene
        } else {
            &self.intro_scene
        };
        let mut can_reset = state.can_reset();
        if !can_reset && state.allow_reset_for_undo_redo() {
            can_reset = !self.undo_stack.is_empty() ||
                !self.redo_stack.is_empty();
        }
        HudInput {
            name: S::location().name(),
            access: state.access(),
            is_paused: scene.is_paused(),
            show_skip: scene.show_skip(),
            active: self.screen_fade.is_transparent() && scene.is_finished(),
            can_undo: !self.undo_stack.is_empty(),
            can_redo: !self.redo_stack.is_empty(),
            can_reset: can_reset,
        }
    }

    pub fn clear_screen(&self, canvas: &mut Canvas) {
        self.theater.clear_screen(canvas);
    }

    pub fn draw_back_layer_no_clear(&self, canvas: &mut Canvas) {
        self.theater.draw_background(canvas);
    }

    pub fn draw_back_layer(&self, canvas: &mut Canvas) {
        self.clear_screen(canvas);
        self.draw_back_layer_no_clear(canvas);
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

    pub fn handle_event<S: PuzzleState>(&mut self, event: &Event,
                                        state: &mut S)
                                        -> Action<PuzzleCmd> {
        let mut action = self.screen_fade.handle_event(event, &mut ());
        if event == &Event::ClockTick {
            if self.theater.tick_animations() {
                action.also_redraw();
            }
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
                Some(&HudCmd::Skip) => {
                    if !self.intro_scene.is_finished() {
                        self.intro_scene.skip(&mut self.theater);
                    } else if let Some(ref mut scene) = self.middle_scene {
                        scene.skip(&mut self.theater);
                    } else if state.is_solved() {
                        self.outro_scene.skip(&mut self.theater);
                    };
                    subaction.but_no_value()
                }
                None => subaction.but_no_value(),
            });
        }
        if !action.should_stop() {
            let subaction = if !self.intro_scene.is_finished() {
                self.intro_scene.handle_event(event, &mut self.theater)
            } else if let Some(ref mut scene) = self.middle_scene {
                scene.handle_event(event, &mut self.theater)
            } else if state.is_solved() {
                self.outro_scene.handle_event(event, &mut self.theater)
            } else {
                Action::ignore()
            };
            if self.intro_scene.is_finished() {
                state.visit();
            }
            if self.middle_scene
                .as_ref()
                .map(Scene::is_finished)
                .unwrap_or(false)
            {
                self.middle_scene = None;
            }
            if !self.previously_solved && self.outro_scene.is_finished() &&
                self.screen_fade.is_transparent() &&
                S::location() != Location::Finale
            {
                self.screen_fade.fade_out_and_return(PuzzleCmd::Next);
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

// ========================================================================= //
