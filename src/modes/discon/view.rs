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
use save::{DisconState, Game, Location};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub enum Cmd {
    ReturnToMap,
    ShowInfoBox,
}

// ========================================================================= //

pub struct View {
    theater: Theater,
    intro_scene: Scene,
    outro_scene: Scene,
    screen_fade: ScreenFade,
    hud: Hud,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &DisconState)
               -> View {
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
            hud: Hud::new(resources, visible, Location::Disconnected),
        };
        view.drain_queue();
        view
    }

    fn hud_input(&self, _state: &DisconState) -> HudInput {
        HudInput {
            name: "Disconnected",
            can_undo: false,
            can_redo: false,
            can_reset: false,
        }
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.theater.drain_queue() {
            // TODO
        }
    }
}

impl Element<Game, Cmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.disconnected;
        self.theater.draw_background(canvas);
        self.theater.draw_foreground(canvas);
        self.hud.draw(&self.hud_input(state), canvas);
        self.screen_fade.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game) -> Action<Cmd> {
        let state = &mut game.disconnected;
        let mut action = {
            let subaction = self.screen_fade.handle_event(event, &mut ());
            match subaction.value() {
                Some(&true) => subaction.but_return(Cmd::ReturnToMap),
                _ => subaction.but_continue(),
            }
        };
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
                    self.screen_fade.set_should_be_opaque(true);
                    subaction.but_no_value()
                }
                Some(&HudCmd::Info) => subaction.but_return(Cmd::ShowInfoBox),
                Some(&HudCmd::Undo) => {
                    // TODO
                    subaction.but_no_value()
                }
                Some(&HudCmd::Redo) => {
                    // TODO
                    subaction.but_no_value()
                }
                Some(&HudCmd::Reset) => {
                    // TODO
                    subaction.but_no_value()
                }
                None => subaction.but_no_value(),
            });
        }
        action
    }
}

// ========================================================================= //

pub const INFO_BOX_TEXT: &'static str = "\
Your goal is to activate each detector on the right with
the appropriate color of laser.

Drag mirrors with $M{your finger}{the mouse} to swap their positions in
the grid.  $M{Tap}{Click} mirrors to rotate them.";

// ========================================================================= //
