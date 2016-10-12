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

use elements::{Hud, HudAction, HudInput};
use gui::{Action, Canvas, Element, Event, GroupElement, Point, Rect,
          Resources, Sprite, SubrectElement};
use save::{Game, Location};

// ========================================================================= //

const PUZZLE_NODE_WIDTH: u32 = 24;
const PUZZLE_NODE_HEIGHT: u32 = 24;

// ========================================================================= //

pub enum MapAction {
    ReturnToTitle,
    ShowInfoBox,
    GoToPuzzle(Location),
}

// ========================================================================= //

pub struct MapView {
    nodes: GroupElement<Game, MapAction>,
    hud: Hud,
}

impl MapView {
    pub fn new(resources: &mut Resources, visible: Rect) -> MapView {
        MapView {
            nodes: GroupElement::new(vec![
                MapView::node(resources, Location::ALightInTheAttic, 100, 50),
            ]),
            hud: Hud::new(resources, visible, Location::Map),
        }
    }

    fn node(resources: &mut Resources, loc: Location, x: i32, y: i32)
            -> Box<Element<Game, MapAction>> {
        let rect = Rect::new(x, y, PUZZLE_NODE_WIDTH, PUZZLE_NODE_HEIGHT);
        Box::new(SubrectElement::new(PuzzleNode::new(resources, loc), rect))
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

impl Element<Game, MapAction> for MapView {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        canvas.clear((64, 128, 64));
        self.nodes.draw(game, canvas);
        self.hud.draw(&self.hud_input(), canvas);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<MapAction> {
        let mut input = self.hud_input();
        let mut action = self.hud.handle_event(event, &mut input).map(|act| {
            if act == HudAction::Back {
                MapAction::ReturnToTitle
            } else {
                MapAction::ShowInfoBox
            }
        });
        if !action.should_stop() {
            action.merge(self.nodes.handle_event(event, game));
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

impl Element<Game, MapAction> for PuzzleNode {
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
                    -> Action<MapAction> {
        match event {
            &Event::MouseDown(_) if game.is_unlocked(self.loc) => {
                Action::redraw().and_return(MapAction::GoToPuzzle(self.loc))
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //
