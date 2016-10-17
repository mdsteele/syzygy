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

use elements::{Hud, HudCmd, HudInput, ScreenFade};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite,
          SubrectElement};
use save::{Game, Location};

// ========================================================================= //

const PUZZLE_NODE_WIDTH: u32 = 24;
const PUZZLE_NODE_HEIGHT: u32 = 24;

// ========================================================================= //

#[derive(Clone, Copy)]
pub enum Cmd {
    ReturnToTitle,
    ShowInfoBox,
    GoToPuzzle(Location),
}

// ========================================================================= //

pub struct View {
    screen_fade: ScreenFade,
    fade_command: Cmd,
    hud: Hud,
    nodes: Vec<SubrectElement<PuzzleNode>>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect) -> View {
        View {
            screen_fade: ScreenFade::new(resources),
            fade_command: Cmd::ReturnToTitle,
            hud: Hud::new(resources, visible, Location::Map),
            nodes: vec![
                View::node(resources, Location::ALightInTheAttic, 100, 50),
            ],
        }
    }

    fn node(resources: &mut Resources, loc: Location, x: i32, y: i32)
            -> SubrectElement<PuzzleNode> {
        let rect = Rect::new(x, y, PUZZLE_NODE_WIDTH, PUZZLE_NODE_HEIGHT);
        SubrectElement::new(PuzzleNode::new(resources, loc), rect)
    }

    fn hud_input(&self) -> HudInput {
        HudInput {
            name: "The Map",
            can_undo: false,
            can_redo: false,
            can_reset: false,
        }
    }
}

impl Element<Game, Cmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        canvas.clear((64, 128, 64));
        self.nodes.draw(game, canvas);
        self.hud.draw(&self.hud_input(), canvas);
        self.screen_fade.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game) -> Action<Cmd> {
        let mut action = {
            let subaction = self.screen_fade.handle_event(event, &mut ());
            match subaction.value() {
                Some(&true) => subaction.but_return(self.fade_command),
                _ => subaction.but_continue(),
            }
        };
        if !action.should_stop() {
            let mut input = self.hud_input();
            let subaction = self.hud.handle_event(event, &mut input);
            action.merge(match subaction.value() {
                Some(&HudCmd::Back) => {
                    self.screen_fade.set_should_be_opaque(true);
                    self.fade_command = Cmd::ReturnToTitle;
                    subaction.but_no_value()
                }
                Some(&HudCmd::Info) => subaction.but_return(Cmd::ShowInfoBox),
                _ => subaction.but_no_value(),
            });
        }
        if !action.should_stop() {
            let subaction = self.nodes.handle_event(event, game);
            if let Some(&location) = subaction.value() {
                self.screen_fade.set_should_be_opaque(true);
                self.fade_command = Cmd::GoToPuzzle(location);
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

// ========================================================================= //

struct PuzzleNode {
    icons: Vec<Sprite>,
    loc: Location,
}

impl PuzzleNode {
    fn new(resources: &mut Resources, loc: Location) -> PuzzleNode {
        PuzzleNode {
            icons: resources.get_sprites("puzzle_nodes"),
            loc: loc,
        }
    }
}

impl Element<Game, Location> for PuzzleNode {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        if game.is_unlocked(self.loc) {
            let icon = if game.is_solved(self.loc) {
                &self.icons[1]
            } else {
                &self.icons[0]
            };
            canvas.draw_sprite(icon, Point::new(0, 0));
        }
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<Location> {
        match event {
            &Event::MouseDown(_) if game.is_unlocked(self.loc) => {
                Action::redraw().and_return(self.loc)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

pub const INFO_BOX_TEXT: &'static str = "\
$M{Tap}{Click} on a system node to travel there.

Systems that still need to be repaired are marked in red.
Repaired systems are marked in green.";

// ========================================================================= //
