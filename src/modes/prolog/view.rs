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

use elements::{FadeStyle, Hud, HudCmd, HudInput, Scene, ScreenFade, Theater};
use gui::{Action, Canvas, Element, Event, Rect, Resources};
use save::{Game, Location, PrologState, PuzzleState};
use super::scenes::compile_scene;

// ========================================================================= //

pub enum Cmd {
    ReturnToMap,
    ShowInfoBox,
    GoToNextPuzzle,
    Replay,
}

// ========================================================================= //

pub struct View {
    theater: Theater,
    scene: Scene,
    screen_fade: ScreenFade<Cmd>,
    hud: Hud,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &PrologState)
               -> View {
        let mut theater = Theater::new();
        let mut scene = compile_scene(resources);
        if state.is_solved() {
            scene.skip(&mut theater);
        } else {
            scene.begin(&mut theater);
        }
        let mut view = View {
            theater: theater,
            scene: scene,
            screen_fade: ScreenFade::new(resources, FadeStyle::TopBottom),
            hud: Hud::new(resources, visible, Location::ALightInTheAttic),
        };
        view.drain_queue();
        view
    }

    fn hud_input(&self, state: &PrologState) -> HudInput {
        HudInput {
            name: "Prolog",
            access: state.access(),
            is_paused: self.scene.is_paused(),
            active: self.screen_fade.is_transparent() &&
                    self.scene.is_finished(),
            can_undo: false,
            can_redo: false,
            can_reset: false,
        }
    }

    fn drain_queue(&mut self) {
        for (_index, _enable) in self.theater.drain_queue() {
            // TODO drain queue
        }
    }
}

impl Element<Game, Cmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.prolog;
        self.scene.draw(&self.theater, canvas);
        self.hud.draw(&self.hud_input(state), canvas);
        self.screen_fade.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game) -> Action<Cmd> {
        let state = &mut game.prolog;
        let mut action = self.screen_fade.handle_event(event, &mut ());
        if !action.should_stop() {
            let subaction = self.scene.handle_event(event, &mut self.theater);
            action.merge(subaction.but_no_value());
            self.drain_queue();
            if self.scene.is_finished() && !state.is_solved() {
                state.mark_solved();
                self.screen_fade.fade_out_and_return(Cmd::GoToNextPuzzle);
            }
        }
        if !action.should_stop() {
            let mut input = self.hud_input(state);
            let subaction = self.hud.handle_event(event, &mut input);
            action.merge(match subaction.value() {
                Some(&HudCmd::Back) => {
                    self.screen_fade.fade_out_and_return(Cmd::ReturnToMap);
                    subaction.but_no_value()
                }
                Some(&HudCmd::Info) => subaction.but_return(Cmd::ShowInfoBox),
                Some(&HudCmd::Replay) => {
                    self.screen_fade.fade_out_and_return(Cmd::Replay);
                    subaction.but_no_value()
                }
                _ => subaction.but_no_value(),
            });
        }
        action
    }
}

// ========================================================================= //

pub const INFO_BOX_TEXT: &'static str = "\
Return to the map to select another scene.";

// ========================================================================= //
