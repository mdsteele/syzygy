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
use std::rc::Rc;

use elements::{FadeStyle, Hud, HudCmd, HudInput, ScreenFade};
use gui::{Action, Background, Canvas, Element, Event, Point, Rect, Resources,
          Sprite};
use save::{Access, Game, Location};

// ========================================================================= //

const NODE_WIDTH: u32 = 24;
const NODE_HEIGHT: u32 = 24;

#[cfg_attr(rustfmt, rustfmt_skip)]
const NODES: &[(Location, (i32, i32), bool)] = &[
    (Location::Prolog, (108, 160), false),
    (Location::ALightInTheAttic, (224, 112), true),
    (Location::AutofacTour, (240, 144), false),
    (Location::BlackAndBlue, (416, 144), false),
    (Location::ColumnAsIcyEm, (353, 210), false),
    (Location::ConnectTheDots, (224, 48), false),
    (Location::CrossSauce, (352, 48), false),
    (Location::CrossTheLine, (176, 179), true),
    (Location::CubeTangle, (304, 272), false),
    (Location::Disconnected, (128, 224), false),
    (Location::DoubleCross, (320, 48), false),
    (Location::FactOrFiction, (208, 144), false),
    (Location::HexSpangled, (336, 272), false),
    (Location::IceToMeetYou, (353, 236), false),
    (Location::IfMemoryServes, (432, 272), false),
    (Location::JogYourMemory, (464, 272), false),
    (Location::LevelHeaded, (192, 48), false),
    (Location::LevelUp, (160, 48), false),
    (Location::LightSyrup, (256, 112), false),
    (Location::LogLevel, (160, 224), false),
    (Location::MemoryLane, (400, 272), false),
    (Location::MissedConnections, (256, 48), false),
    (Location::PasswordFile, (224, 224), false),
    (Location::PlaneAndSimple, (416, 48), false),
    (Location::PlaneAsDay, (448, 48), false),
    (Location::PointOfOrder, (288, 48), false),
    (Location::ShiftGears, (128, 48), false),
    (Location::ShiftTheBlame, (448, 144), false),
    (Location::ShiftingGround, (272, 288), false),
    (Location::StarCrossed, (384, 48), false),
    (Location::SystemFailure, (192, 224), false),
    (Location::SystemSyzygy, (208, 256), false),
    (Location::TheIceIsRight, (383, 236), false),
    (Location::TheYFactor, (176, 144), true),
    (Location::TreadLightly, (288, 112), false),
    (Location::VirtueOrIce, (383, 210), false),
    (Location::WhatchaColumn, (64, 48), false),
    (Location::WreckedAngle, (272, 256), true),
    (Location::Finale, (176, 256), false),
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
    background: Rc<Background>,
    map_sprites: Vec<(Sprite, Point)>,
    nodes: Vec<PuzzleNode>,
    paths: Vec<Rect>,
    selected: Option<Location>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, game: &Game) -> View {
        let locations: HashMap<Location, (i32, i32)> =
            NODES.iter()
                 .map(|&(loc, pt, _)| (loc, pt))
                 .collect();
        let mut nodes = Vec::new();
        let mut paths = Vec::new();
        for &(location, (x, y), invert) in NODES {
            if game.is_unlocked(location) {
                let left = x - NODE_WIDTH as i32 / 2;
                let top = y - NODE_HEIGHT as i32 / 2;
                let rect = Rect::new(left, top, NODE_WIDTH, NODE_HEIGHT);
                nodes.push(PuzzleNode::new(resources, rect, location, game));
                for prereq in &location.prereqs() {
                    if let Some(&(px, py)) = locations.get(prereq) {
                        let w = (px - x).abs() as u32;
                        let h = (py - y).abs() as u32;
                        if (w < h) ^ invert {
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
        let mut map_sprites = Vec::new();
        {
            let mut sprites = resources.get_sprites("map/icebox");
            let is_open = game.is_unlocked(Location::IceToMeetYou) ||
                          game.is_unlocked(Location::TheIceIsRight) ||
                          game.is_unlocked(Location::VirtueOrIce) ||
                          game.is_unlocked(Location::ColumnAsIcyEm);
            let idx = if is_open { 1 } else { 0 };
            map_sprites.push((sprites.swap_remove(idx), Point::new(336, 192)));
        }
        {
            let sprites = resources.get_sprites("map/checkpoints");
            map_sprites.push((sprites[0].clone(), Point::new(160, 160)));
        }
        if !game.is_unlocked(Location::SystemSyzygy) {
            let sprites = resources.get_sprites("map/secret");
            map_sprites.push((sprites[0].clone(), Point::new(160, 240)));
        }
        View {
            screen_fade: ScreenFade::new(resources, FadeStyle::Radial),
            hud: Hud::new(resources, visible, Location::Map),
            background: resources.get_background("map"),
            map_sprites: map_sprites,
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
        canvas.clear(self.background.color());
        canvas.draw_background(&self.background);
        for &(ref sprite, point) in self.map_sprites.iter() {
            canvas.draw_sprite(sprite, point);
        }
        for &rect in &self.paths {
            canvas.fill_rect((192, 128, 0), rect);
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
    rect: Rect,
    loc: Location,
}

impl PuzzleNode {
    fn new(resources: &mut Resources, rect: Rect, location: Location,
           game: &Game)
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
            icon: resources.get_sprites("map/nodes")[index].clone(),
            rect: rect,
            loc: location,
        }
    }
}

impl Element<Option<Location>, Location> for PuzzleNode {
    fn draw(&self, selected: &Option<Location>, canvas: &mut Canvas) {
        canvas.draw_sprite(&self.icon, self.rect.top_left());
        if *selected == Some(self.loc) {
            canvas.draw_rect((255, 255, 255), self.rect);
        }
    }

    fn handle_event(&mut self, event: &Event,
                    selected: &mut Option<Location>)
                    -> Action<Location> {
        match event {
            &Event::MouseDown(pt) if self.rect.contains(pt) => {
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

pub const INFO_BOX_TEXT: &str = "\
$M{Tap}{Click} on a system node to select it; $M{tap}{click} on it again to
travel there.

Nodes that still need to be repaired are marked in red.
Repaired nodes are marked in green.";

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use save::Location;
    use super::NODES;

    #[test]
    fn all_locations_represented_on_map() {
        let mut locations: HashSet<Location> =
            Location::all().iter().cloned().collect();
        locations.remove(&Location::Map);
        for &(loc, _, _) in NODES {
            locations.remove(&loc);
        }
        assert!(locations.is_empty(),
                "Unrepresented locations: {:?}",
                locations);
    }
}

// ========================================================================= //
