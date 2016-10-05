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

use super::super::gui::{Action, Canvas, Element, Event, GroupElement, Point,
                        Rect, Resources, Sprite, SubrectElement};
use super::super::save::{Game, Location};

// ========================================================================= //

const PUZZLE_NODE_WIDTH: u32 = 24;
const PUZZLE_NODE_HEIGHT: u32 = 24;

// ========================================================================= //

#[allow(dead_code)]
pub enum MapAction {
    ReturnToTitle,
    GoToPuzzle(Location),
}

// ========================================================================= //

pub struct MapView {
    nodes: GroupElement<Game, MapAction>,
}

impl MapView {
    pub fn new(resources: &mut Resources, _visible: Rect) -> MapView {
        MapView {
            nodes: GroupElement::new(vec![
                MapView::node(resources, Location::ALightInTheAttic, 100, 50),
            ]),
        }
    }

    fn node(resources: &mut Resources, loc: Location, x: i32, y: i32)
            -> Box<Element<Game, MapAction>> {
        let rect = Rect::new(x, y, PUZZLE_NODE_WIDTH, PUZZLE_NODE_HEIGHT);
        Box::new(SubrectElement::new(PuzzleNode::new(resources, loc), rect))
    }
}

impl Element<Game, MapAction> for MapView {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        canvas.clear((64, 128, 64));
        self.nodes.draw(game, canvas);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<MapAction> {
        self.nodes.handle_event(event, game)
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
    fn draw(&self, _game: &Game, canvas: &mut Canvas) {
        // TODO: pick icon based on location access
        canvas.draw_sprite(&self.icons[0], Point::new(0, 0));
    }

    fn handle_event(&mut self, event: &Event, _game: &mut Game)
                    -> Action<MapAction> {
        // TODO: ignore clicks for locked locations
        match event {
            &Event::MouseDown(_) => {
                Action::redraw().and_return(MapAction::GoToPuzzle(self.loc))
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //
