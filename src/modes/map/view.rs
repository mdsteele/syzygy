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

const NODE_WIDTH: u32 = 28;
const NODE_HEIGHT: u32 = 28;

#[cfg_attr(rustfmt, rustfmt_skip)]
const NODES: &[(Location, (i32, i32), bool)] = &[
    (Location::Prolog, (112, 144), false),
    (Location::ALightInTheAttic, (224, 112), false),
    (Location::AutofacTour, (376, 160), false),
    (Location::BlackAndBlue, (384, 128), true),
    (Location::ColumnAsIcyEm, (416, 160), false),
    (Location::ConnectTheDots, (208, 208), false),
    (Location::CrossSauce, (384, 288), false),
    (Location::CrossTheLine, (160, 176), true),
    (Location::CubeTangle, (304, 256), false),
    (Location::Disconnected, (128, 208), false),
    (Location::DoubleCross, (320, 112), false),
    (Location::FactOrFiction, (352, 256), true),
    (Location::HexSpangled, (328, 160), true),
    (Location::IceToMeetYou, (416, 192), true),
    (Location::IfMemoryServes, (384, 208), false),
    (Location::JogYourMemory, (480, 256), false),
    (Location::LevelHeaded, (440, 128), false),
    (Location::LevelUp, (480, 176), false),
    (Location::LightSyrup, (256, 112), false),
    (Location::LogLevel, (160, 208), false),
    (Location::MemoryLane, (240, 176), false),
    (Location::MissedConnections, (304, 208), true),
    (Location::PasswordFile, (224, 240), false),
    (Location::PlaneAndSimple, (192, 176), false),
    (Location::PlaneAsDay, (480, 224), false),
    (Location::PointOfNoReturn, (448, 256), true),
    (Location::PointOfOrder, (416, 96), false),
    (Location::PointOfView, (336, 288), false),
    (Location::ShiftGears, (416, 288), false),
    (Location::ShiftTheBlame, (384, 96), false),
    (Location::ShiftingGround, (272, 288), false),
    (Location::StarCrossed, (432, 224), true),
    (Location::SystemFailure, (192, 240), true),
    (Location::SystemSyzygy, (224, 272), false),
    (Location::TheIceIsRight, (448, 192), false),
    (Location::TheYFactor, (296, 144), true),
    (Location::ThreeBlindIce, (448, 160), true),
    (Location::TreadLightly, (288, 112), false),
    (Location::WhatchaColumn, (352, 128), false),
    (Location::WreckedAngle, (272, 256), false),
    (Location::Finale, (192, 272), false),
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
    paths_outer: Vec<Rect>,
    paths_inner: Vec<Rect>,
    selected: Option<Location>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, game: &Game) -> View {
        let locations: HashMap<Location, (i32, i32)> =
            NODES.iter().map(|&(loc, pt, _)| (loc, pt)).collect();
        let mut nodes = Vec::new();
        let mut paths_outer = Vec::new();
        let mut paths_inner = Vec::new();
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
                            if w > 0 {
                                paths_outer.push(Rect::new(min(x, px) - 1,
                                                           y - 2,
                                                           w + 2,
                                                           4));
                                paths_inner
                                    .push(Rect::new(min(x, px), y - 1, w, 2));
                            }
                            if h > 0 {
                                paths_outer.push(Rect::new(px - 2,
                                                           min(y, py) - 1,
                                                           4,
                                                           h + 2));
                                paths_inner
                                    .push(Rect::new(px - 1, min(y, py), 2, h));
                            }
                        } else {
                            if h > 0 {
                                paths_outer.push(Rect::new(x - 2,
                                                           min(y, py) - 1,
                                                           4,
                                                           h + 2));
                                paths_inner
                                    .push(Rect::new(x - 1, min(y, py), 2, h));
                            }
                            if w > 0 {
                                paths_outer.push(Rect::new(min(x, px) - 1,
                                                           py - 2,
                                                           w + 2,
                                                           4));
                                paths_inner
                                    .push(Rect::new(min(x, px), py - 1, w, 2));
                            }
                        }
                    }
                }
            }
        }
        let mut map_sprites = Vec::new();
        let biodome_is_open = game.is_unlocked(Location::WhatchaColumn);
        let cold_storage_is_open = game.is_unlocked(Location::IceToMeetYou);
        let main_power_is_open = game.is_unlocked(Location::ALightInTheAttic);
        let lower_decks_are_open = game.is_unlocked(Location::ShiftGears);
        let sewers_are_open = game.is_unlocked(Location::ShiftingGround);
        {
            let sprites = resources.get_sprites("map/aft");
            map_sprites.push((sprites[0].clone(), Point::new(464, 144)));
            map_sprites.push((sprites[1].clone(), Point::new(464, 176)));
            map_sprites.push((sprites[2].clone(), Point::new(464, 208)));
            map_sprites.push((sprites[3].clone(), Point::new(432, 208)));
        }
        {
            let sprites = resources.get_sprites("map/biodome");
            let idx = if biodome_is_open { 0 } else { 2 };
            map_sprites.push((sprites[idx].clone(), Point::new(336, 80)));
            map_sprites.push((sprites[idx + 1].clone(), Point::new(400, 80)));
        }
        {
            let sprites = resources.get_sprites("map/checkpoints");
            map_sprites.push((sprites[0].clone(), Point::new(144, 160)));
            map_sprites.push((sprites[1].clone(), Point::new(304, 96)));
            map_sprites.push((sprites[2].clone(), Point::new(368, 272)));
            map_sprites.push((sprites[3].clone(), Point::new(416, 208)));
        }
        {
            let sprites = resources.get_sprites("map/factory");
            map_sprites.push((sprites[0].clone(), Point::new(272, 128)));
            map_sprites.push((sprites[1].clone(), Point::new(304, 128)));
            map_sprites.push((sprites[2].clone(), Point::new(336, 128)));
            map_sprites.push((sprites[3].clone(), Point::new(368, 128)));
        }
        {
            let mut sprites = resources.get_sprites("map/icebox");
            let idx = if cold_storage_is_open { 1 } else { 0 };
            map_sprites.push((sprites.swap_remove(idx), Point::new(400, 144)));
        }
        {
            let sprites = resources.get_sprites("map/power");
            map_sprites.push((sprites[0].clone(), Point::new(176, 96)));
            map_sprites.push((sprites[1].clone(), Point::new(208, 96)));
            map_sprites.push((sprites[2].clone(), Point::new(240, 96)));
            map_sprites.push((sprites[3].clone(), Point::new(272, 96)));
        }
        if !game.is_unlocked(Location::SystemSyzygy) {
            let sprites = resources.get_sprites("map/secret");
            map_sprites.push((sprites[0].clone(), Point::new(176, 256)));
        }
        {
            let sprites = resources.get_sprites("map/sewer");
            map_sprites.push((sprites[0].clone(), Point::new(240, 272)));
            map_sprites.push((sprites[1].clone(), Point::new(272, 272)));
            map_sprites.push((sprites[2].clone(), Point::new(304, 272)));
            map_sprites.push((sprites[3].clone(), Point::new(336, 272)));
        }
        {
            let sprites = resources.get_sprites("map/yellow");
            map_sprites.push((sprites[0].clone(), Point::new(400, 240)));
            map_sprites.push((sprites[1].clone(), Point::new(432, 240)));
            map_sprites.push((sprites[2].clone(), Point::new(464, 240)));
            map_sprites.push((sprites[3].clone(), Point::new(400, 272)));
            map_sprites.push((sprites[4].clone(), Point::new(432, 272)));
            map_sprites.push((sprites[5].clone(), Point::new(464, 272)));
        }
        {
            let sprites = resources.get_sprites("map/labels");
            if biodome_is_open {
                map_sprites.push((sprites[0].clone(), Point::new(436, 98)));
            }
            if lower_decks_are_open {
                map_sprites.push((sprites[2].clone(), Point::new(440, 285)));
            }
            if main_power_is_open {
                map_sprites.push((sprites[3].clone(), Point::new(220, 91)));
            }
            if sewers_are_open {
                map_sprites.push((sprites[4].clone(), Point::new(272, 301)));
            }
        }
        View {
            screen_fade: ScreenFade::new(resources,
                                         FadeStyle::Radial,
                                         FadeStyle::Radial),
            hud: Hud::new(resources, visible, Location::Map),
            background: resources.get_background("map"),
            map_sprites: map_sprites,
            nodes: nodes,
            paths_outer: paths_outer,
            paths_inner: paths_inner,
            selected: None,
        }
    }

    fn hud_input(&self) -> HudInput {
        HudInput {
            name: self.selected.unwrap_or(Location::Map).name(),
            access: Access::Unvisited,
            is_paused: false,
            show_skip: false,
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
        for &rect in &self.paths_outer {
            canvas.fill_rect((96, 64, 0), rect);
        }
        for &rect in &self.paths_inner {
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
                                 self.screen_fade
                                     .fade_out_and_return(Cmd::ReturnToTitle);
                                 subaction.but_no_value()
                             }
                             Some(&HudCmd::Info) => {
                                 subaction.but_return(Cmd::ShowInfoBox)
                             }
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
    sprites: Vec<Sprite>,
    sprite_index: usize,
    rect: Rect,
    loc: Location,
}

impl PuzzleNode {
    fn new(resources: &mut Resources, rect: Rect, location: Location,
           game: &Game)
           -> PuzzleNode {
        let sprite_index = if game.has_been_solved(location) {
            1
        } else if location == Location::SystemFailure &&
                   !game.system_failure.mid_scene_is_done()
        {
            2
        } else {
            0
        };
        PuzzleNode {
            sprites: resources.get_sprites("map/nodes"),
            sprite_index: sprite_index,
            rect: rect,
            loc: location,
        }
    }
}

impl Element<Option<Location>, Location> for PuzzleNode {
    fn draw(&self, selected: &Option<Location>, canvas: &mut Canvas) {
        let top_left = self.rect.top_left();
        canvas.draw_sprite(&self.sprites[self.sprite_index], top_left);
        if *selected == Some(self.loc) {
            canvas.draw_sprite(&self.sprites[3], top_left);
        }
    }

    fn handle_event(&mut self, event: &Event,
                    selected: &mut Option<Location>)
                    -> Action<Location> {
        match event {
            &Event::MouseDown(pt) if self.rect.contains_point(pt) => {
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
    use std::collections::{HashMap, HashSet};

    use gui::Rect;
    use save::Location;
    use super::{NODES, NODE_HEIGHT, NODE_WIDTH};

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

    #[test]
    fn no_repeated_locations_on_map() {
        let mut locations: HashSet<Location> = HashSet::new();
        for &(loc, _, _) in NODES {
            assert!(!locations.contains(&loc), "Repeated: {:?}", loc);
            locations.insert(loc);
        }
    }

    #[test]
    fn nodes_do_not_overlap_on_map() {
        let rects: HashMap<Location, Rect> = NODES
            .iter()
            .map(|&(loc, (x, y), _)| {
                     (loc,
                      Rect::new(x - (NODE_WIDTH / 2) as i32,
                                y - (NODE_HEIGHT / 2) as i32,
                                NODE_WIDTH,
                                NODE_HEIGHT))
                 })
            .collect();
        for (&loc1, &rect1) in rects.iter() {
            for (&loc2, &rect2) in rects.iter() {
                if loc1 != loc2 {
                    assert!(!rect1.has_intersection(rect2),
                            "{:?} intersects {:?}",
                            loc1,
                            loc2);
                }
            }
        }
    }
}

// ========================================================================= //
