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

use std::cmp::min;
use std::collections::HashMap;

use elements::{FadeStyle, Hud, HudCmd, HudInput, ScreenFade};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite,
          SubrectElement};
use save::{Access, Game, Location};

// ========================================================================= //

const NODE_WIDTH: u32 = 24;
const NODE_HEIGHT: u32 = 24;

#[cfg_attr(rustfmt, rustfmt_skip)]
const NODES: &'static [(Location, (i32, i32))] = &[
    (Location::Prolog, (75, 100)),
    (Location::ALightInTheAttic, (200, 50)),
    (Location::BlackAndBlue, (225, 75)),
    (Location::ConnectTheDots, (150, 150)),
    (Location::CrossSauce, (225, 225)),
    (Location::CrossTheLine, (125, 225)),
    (Location::CubeTangle, (125, 280)),
    (Location::Disconnected, (100, 150)),
    (Location::DoubleCross, (175, 225)),
    (Location::FactOrFiction, (150, 200)),
    (Location::LevelUp, (225, 125)),
    (Location::LightSyrup, (250, 50)),
    (Location::LogLevel, (125, 175)),
    (Location::MemoryLane, (175, 280)),
    (Location::MissedConnections, (200, 150)),
    (Location::PasswordFile, (400, 200)),
    (Location::ShiftGears, (200, 250)),
    (Location::ShiftTheBlame, (275, 75)),
    (Location::ShiftingGround, (150, 250)),
    (Location::SystemFailure, (360, 200)),
    (Location::TheYFactor, (100, 200)),
    (Location::TreadLightly, (300, 50)),
    (Location::WreckedAngle, (100, 250)),
];

// ========================================================================= //

#[derive(Clone, Copy)]
pub enum Cmd {
    ReturnToTitle,
    ShowInfoBox,
    GoToPuzzle(Location),
}

// ========================================================================= //

pub struct View {
    screen_fade: ScreenFade<Cmd>,
    hud: Hud,
    nodes: Vec<SubrectElement<PuzzleNode>>,
    paths: Vec<Rect>,
    selected: Option<Location>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, game: &Game) -> View {
        let locations: HashMap<Location, (i32, i32)> = NODES.iter()
                                                            .cloned()
                                                            .collect();
        let mut nodes = Vec::new();
        let mut paths = Vec::new();
        for &(location, (x, y)) in NODES {
            if game.is_unlocked(location) {
                let node = PuzzleNode::new(resources, location, game);
                let left = x - NODE_WIDTH as i32 / 2;
                let top = y - NODE_HEIGHT as i32 / 2;
                let rect = Rect::new(left, top, NODE_WIDTH, NODE_HEIGHT);
                nodes.push(SubrectElement::new(node, rect));
                for prereq in &location.prereqs() {
                    if let Some(&(px, py)) = locations.get(prereq) {
                        let w = (px - x).abs() as u32;
                        let h = (py - y).abs() as u32;
                        if w < h {
                            paths.push(Rect::new(min(x, px) - 2,
                                                 y - 2,
                                                 w + 4,
                                                 4));
                            paths.push(Rect::new(px - 2, min(y, py), 4, h));
                        } else {
                            paths.push(Rect::new(x - 2,
                                                 min(y, py) - 2,
                                                 4,
                                                 h + 4));
                            paths.push(Rect::new(min(x, px), py - 2, w, 4));
                        }
                    }
                }
            }
        }
        View {
            screen_fade: ScreenFade::new(resources, FadeStyle::Radial),
            hud: Hud::new(resources, visible, Location::Map),
            nodes: nodes,
            paths: paths,
            selected: None,
        }
    }

    fn hud_input(&self) -> HudInput {
        HudInput {
            name: self.selected.unwrap_or(Location::Map).name(),
            access: Access::Unvisited,
            is_paused: false,
            active: self.screen_fade.is_transparent(),
            can_undo: false,
            can_redo: false,
            can_reset: false,
        }
    }
}

impl Element<Game, Cmd> for View {
    fn draw(&self, _: &Game, canvas: &mut Canvas) {
        canvas.clear((64, 128, 64));
        for &rect in &self.paths {
            canvas.fill_rect((128, 0, 128), rect);
        }
        self.nodes.draw(&self.selected, canvas);
        self.hud.draw(&self.hud_input(), canvas);
        self.screen_fade.draw(&(), canvas);
    }

    fn handle_event(&mut self, event: &Event, _: &mut Game) -> Action<Cmd> {
        let mut action = self.screen_fade.handle_event(event, &mut ());
        if !action.should_stop() {
            let mut input = self.hud_input();
            let subaction = self.hud.handle_event(event, &mut input);
            action.merge(match subaction.value() {
                Some(&HudCmd::Back) => {
                    self.screen_fade.fade_out_and_return(Cmd::ReturnToTitle);
                    subaction.but_no_value()
                }
                Some(&HudCmd::Info) => subaction.but_return(Cmd::ShowInfoBox),
                _ => subaction.but_no_value(),
            });
        }
        if !action.should_stop() {
            let subaction = self.nodes.handle_event(event, &mut self.selected);
            if let Some(&loc) = subaction.value() {
                self.screen_fade.fade_out_and_return(Cmd::GoToPuzzle(loc));
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            if let &Event::MouseDown(_) = event {
                self.selected = None;
                action.merge(Action::redraw().and_stop());
            }
        }
        action
    }
}

// ========================================================================= //

struct PuzzleNode {
    icon: Sprite,
    loc: Location,
}

impl PuzzleNode {
    fn new(resources: &mut Resources, location: Location, game: &Game)
           -> PuzzleNode {
        let index = if game.has_been_solved(location) {
            1
        } else if location == Location::SystemFailure &&
                              !game.system_failure.mid_scene_is_done() {
            2
        } else {
            0
        };
        PuzzleNode {
            icon: resources.get_sprites("puzzle_nodes")[index].clone(),
            loc: location,
        }
    }
}

impl Element<Option<Location>, Location> for PuzzleNode {
    fn draw(&self, selected: &Option<Location>, canvas: &mut Canvas) {
        canvas.draw_sprite(&self.icon, Point::new(0, 0));
        if *selected == Some(self.loc) {
            let rect = canvas.rect();
            canvas.draw_rect((255, 255, 255), rect);
        }
    }

    fn handle_event(&mut self, event: &Event,
                    selected: &mut Option<Location>)
                    -> Action<Location> {
        match event {
            &Event::MouseDown(_) => {
                if *selected == Some(self.loc) {
                    Action::redraw().and_return(self.loc)
                } else {
                    *selected = Some(self.loc);
                    Action::redraw().and_stop()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

pub const INFO_BOX_TEXT: &'static str = "\
$M{Tap}{Click} on a system node to select it; $M{tap}{click} on it again to
travel there.

Nodes that still need to be repaired are marked in red.
Repaired nodes are marked in green.";

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use save::Location;
    use super::NODES;

    #[test]
    fn all_locations_represented_on_map() {
        let mut locations: BTreeSet<Location> =
            Location::all().iter().cloned().collect();
        locations.remove(&Location::Map);
        for &(loc, _) in NODES {
            locations.remove(&loc);
        }
        assert!(locations.is_empty(),
                "Unrepresented locations: {:?}",
                locations);
    }
}

// ========================================================================= //
