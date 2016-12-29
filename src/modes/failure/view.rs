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
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, Location, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    dashboard: Vec<DashChip>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, game: &Game) -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let state = &game.system_failure;
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            dashboard: vec![
                DashChip::new(resources, 167, 71, Location::ALightInTheAttic),
                DashChip::new(resources, 209, 71, Location::ALightInTheAttic),
                DashChip::new(resources, 251, 71, Location::ALightInTheAttic),
                DashChip::new(resources, 293, 71, Location::ALightInTheAttic),
                DashChip::new(resources, 335, 71, Location::ALightInTheAttic),
                DashChip::new(resources, 377, 71, Location::MissedConnections),
                DashChip::new(resources, 167, 113, Location::ALightInTheAttic),
                DashChip::new(resources, 209, 113, Location::ALightInTheAttic),
                DashChip::new(resources, 251, 113, Location::ALightInTheAttic),
                DashChip::new(resources, 293, 113, Location::ALightInTheAttic),
                DashChip::new(resources, 335, 113, Location::ALightInTheAttic),
                DashChip::new(resources, 377, 113, Location::ALightInTheAttic),
                DashChip::new(resources, 167, 155, Location::LevelUp),
                DashChip::new(resources, 209, 155, Location::ALightInTheAttic),
                DashChip::new(resources, 251, 155, Location::ALightInTheAttic),
                DashChip::new(resources, 293, 155, Location::ALightInTheAttic),
                DashChip::new(resources, 335, 155, Location::ALightInTheAttic),
                DashChip::new(resources, 377, 155, Location::ALightInTheAttic),
                DashChip::new(resources, 167, 197, Location::LogLevel),
                DashChip::new(resources, 209, 197, Location::ALightInTheAttic),
                DashChip::new(resources, 251, 197, Location::ALightInTheAttic),
                DashChip::new(resources, 293, 197, Location::ALightInTheAttic),
                DashChip::new(resources, 335, 197, Location::ALightInTheAttic),
                DashChip::new(resources, 377, 197, Location::ALightInTheAttic),
                DashChip::new(resources, 167, 239, Location::ALightInTheAttic),
                DashChip::new(resources, 209, 239, Location::ConnectTheDots),
                DashChip::new(resources, 251, 239, Location::CubeTangle),
                DashChip::new(resources, 293, 239, Location::ALightInTheAttic),
                DashChip::new(resources, 335, 239, Location::ALightInTheAttic),
                DashChip::new(resources, 377, 239, Location::ShiftingGround),
                DashChip::new(resources, 167, 281, Location::CrossTheLine),
                DashChip::new(resources, 209, 281, Location::ALightInTheAttic),
                DashChip::new(resources, 251, 281, Location::WreckedAngle),
                DashChip::new(resources, 293, 281, Location::Disconnected),
                DashChip::new(resources, 335, 281, Location::ALightInTheAttic),
                DashChip::new(resources, 377, 281, Location::ALightInTheAttic),
            ],
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
        let state = &game.system_failure;
        self.core.draw_back_layer(canvas);
        self.core.draw_middle_layer(canvas);
        self.dashboard.draw(game, canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let mut action = self.core
                             .handle_event(event, &mut game.system_failure);
        self.drain_queue();
        if !action.should_stop() {
            action.merge(self.dashboard
                             .handle_event(event, game)
                             .but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.system_failure.is_solved() {
            SOLVED_INFO_TEXT
        } else if game.system_failure.mid_scene_is_done() {
            INFO_BOX_TEXT_2
        } else {
            INFO_BOX_TEXT_1
        }
    }

    fn undo(&mut self, _game: &mut Game) {
        if let Some(_) = self.core.pop_undo() {
            // TODO undo
        }
    }

    fn redo(&mut self, _game: &mut Game) {
        if let Some(_) = self.core.pop_redo() {
            // TODO redo
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.system_failure.reset();
    }

    fn replay(&mut self, game: &mut Game) {
        game.system_failure.replay();
        self.core.replay();
        self.drain_queue();
    }

    fn solve(&mut self, game: &mut Game) {
        game.system_failure.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const DASH_ANIM_SLOWDOWN: i32 = 4;
const DASH_ANIM_INDICES: &'static [usize] = &[4, 5, 6, 7, 8, 9, 10, 11, 12,
                                              7, 6, 13, 14, 15];

struct DashChip {
    sprites: Vec<Sprite>,
    topleft: Point,
    location: Location,
    anim: i32,
}

impl DashChip {
    fn new(resources: &mut Resources, left: i32, top: i32, location: Location)
           -> DashChip {
        DashChip {
            sprites: resources.get_sprites("failure/chips"),
            topleft: Point::new(left, top),
            location: location,
            anim: (left + top) %
                  (DASH_ANIM_SLOWDOWN * DASH_ANIM_INDICES.len() as i32),
        }
    }
}

impl Element<Game, ()> for DashChip {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let index = if game.has_been_solved(self.location) {
            DASH_ANIM_INDICES[(self.anim / DASH_ANIM_SLOWDOWN) as usize]
        } else {
            0
        };
        canvas.draw_sprite(&self.sprites[index], self.topleft);
    }

    fn handle_event(&mut self, event: &Event, _: &mut Game) -> Action<()> {
        match event {
            &Event::ClockTick => {
                self.anim += 1;
                if self.anim ==
                   DASH_ANIM_INDICES.len() as i32 * DASH_ANIM_SLOWDOWN {
                    self.anim = 0;
                }
                Action::redraw_if(self.anim % DASH_ANIM_SLOWDOWN == 0)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT_1: &'static str = "\
Return here later, after you have repaired
more areas of the ship.";

const INFO_BOX_TEXT_2: &'static str = "\
Your goal is to TODO.";

// ========================================================================= //
