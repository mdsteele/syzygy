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

use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources,
          Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{FailureState, Game, Location, PuzzleState};
use save::pyramid::{Board, Coords, MAX_REMOVALS, Move, Team};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const DASHBOARD_CHIPS: &[(i32, i32, Location)] = &[
    (167, 71, Location::ALightInTheAttic),
    (209, 71, Location::ALightInTheAttic),
    (251, 71, Location::ALightInTheAttic),
    (293, 71, Location::TreadLightly),
    (335, 71, Location::PointOfOrder),
    (377, 71, Location::MissedConnections),
    (167, 113, Location::JogYourMemory),
    (209, 113, Location::ALightInTheAttic),
    (251, 113, Location::StarCrossed),
    (293, 113, Location::ALightInTheAttic),
    (335, 113, Location::PlaneAsDay),
    (377, 113, Location::ShiftTheBlame),
    (167, 155, Location::LevelUp),
    (209, 155, Location::ALightInTheAttic),
    (251, 155, Location::WhatchaColumn),
    (293, 155, Location::ALightInTheAttic),
    (335, 155, Location::CrossSauce),
    (377, 155, Location::ALightInTheAttic),
    (167, 197, Location::LogLevel),
    (209, 197, Location::ShiftGears),
    (251, 197, Location::ALightInTheAttic),
    (293, 197, Location::BlackAndBlue),
    (335, 197, Location::DoubleCross),
    (377, 197, Location::LightSyrup),
    (167, 239, Location::IfMemoryServes),
    (209, 239, Location::ConnectTheDots),
    (251, 239, Location::CubeTangle),
    (293, 239, Location::FactOrFiction),
    (335, 239, Location::PlaneAndSimple),
    (377, 239, Location::ShiftingGround),
    (167, 281, Location::CrossTheLine),
    (209, 281, Location::ALightInTheAttic),
    (251, 281, Location::WreckedAngle),
    (293, 281, Location::Disconnected),
    (335, 281, Location::MemoryLane),
    (377, 281, Location::TheYFactor),
];

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    dashboard: Vec<DashChip>,
    pyramid: PyramidView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, game: &Game) -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let state = &game.system_failure;
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        View {
            core: core,
            dashboard: DASHBOARD_CHIPS.iter()
                                      .map(|&(x, y, loc)| {
                                          DashChip::new(resources, x, y, loc)
                                      })
                                      .collect(),
            pyramid: PyramidView::new(resources, state),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.system_failure;
        self.core.draw_back_layer(canvas);
        self.core.draw_middle_layer(canvas);
        if state.mid_scene_is_done() {
            self.pyramid.draw(state, canvas);
        } else {
            self.dashboard.draw(game, canvas);
        }
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let mut action = self.core
                             .handle_event(event, &mut game.system_failure);
        if !game.system_failure.mid_scene_is_done() {
            if !action.should_stop() {
                action.merge(self.dashboard
                                 .handle_event(event, game)
                                 .but_no_value());
            }
        } else {
            let state = &mut game.system_failure;
            if !action.should_stop() {
                let subaction = self.pyramid.handle_event(event, state);
                match subaction.value() {
                    Some(&PyramidCmd::Place(coords)) => {
                        state.board_mut().set_piece_at(coords, Team::You);
                        self.pyramid.step = PyramidStep::YouAnimatePlace {
                            anim: 0,
                            at: coords,
                        };
                    }
                    Some(&PyramidCmd::JumpFrom(from)) => {
                        self.pyramid.step = PyramidStep::YouJumping {
                            from: from,
                            possible: state.board().possible_jump_dests(from),
                        };
                    }
                    Some(&PyramidCmd::Jump(from, to)) => {
                        state.board_mut().remove_piece(from);
                        state.board_mut().set_piece_at(to, Team::You);
                        self.pyramid.step = PyramidStep::YouAnimateJump {
                            anim: 0,
                            from: from,
                            to: to,
                        };
                    }
                    Some(&PyramidCmd::Remove(ref formation, ref so_far)) => {
                        debug_assert!(!so_far.is_empty());
                        let &coords = so_far.last().unwrap();
                        state.board_mut().remove_piece(coords);
                        self.pyramid.step = PyramidStep::YouAnimateRemove {
                            anim: 0,
                            from: coords,
                            formation: formation.clone(),
                            so_far: so_far.clone(),
                        };
                    }
                    None => {}
                }
                action.merge(subaction.but_no_value());
            }
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
        if let Some(()) = self.core.pop_undo() {
            // TODO: support undo
        }
    }

    fn redo(&mut self, _game: &mut Game) {
        if let Some(()) = self.core.pop_redo() {
            // TODO: support redo
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.system_failure;
        self.core.clear_undo_redo();
        state.reset();
        self.pyramid.step = PyramidStep::you_ready(state);
    }

    fn solve(&mut self, game: &mut Game) {
        game.system_failure.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.core.drain_queue() {
            // TODO drain queue
        }
    }
}

// ========================================================================= //

const DASH_ANIM_SLOWDOWN: i32 = 4;
const DASH_ANIM_INDICES: &[usize] = &[4, 5, 6, 7, 8, 9, 10, 11, 12, 7, 6, 13,
                                      14, 15];

struct DashChip {
    sprites: Vec<Sprite>,
    topleft: Point,
    location: Location,
    anim: i32,
}

impl DashChip {
    fn new(resources: &mut Resources, left: i32, top: i32,
           location: Location)
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

const ANIM_PLACE_FRAMES: i32 = 12;
const ANIM_JUMP_FRAMES: i32 = 12;
const ANIM_REMOVE_FRAMES: i32 = ANIM_PLACE_FRAMES;
const ANIM_FORMATION_SLOWDOWN: i32 = 2;
const ANIM_VICTORY_SLOWDOWN: i32 = 2;

enum PyramidStep {
    YouReady { possible: HashSet<Coords> },
    YouJumping {
        from: Coords,
        possible: HashSet<Coords>,
    },
    YouAnimatePlace { anim: i32, at: Coords },
    YouAnimateJump { anim: i32, from: Coords, to: Coords },
    YouAnimateFormation { anim: i32, formation: Vec<Coords> },
    YouRemoving {
        formation: Vec<Coords>,
        so_far: Vec<Coords>,
        possible: HashSet<Coords>,
    },
    YouAnimateRemove {
        anim: i32,
        from: Coords,
        formation: Vec<Coords>,
        so_far: Vec<Coords>,
    },
    SrbThinking { result: Arc<Mutex<Option<Move>>> },
    SrbAnimatePlace {
        anim: i32,
        at: Coords,
        formation: Vec<Coords>,
        to_remove: Vec<Coords>,
    },
    SrbAnimateJump {
        anim: i32,
        from: Coords,
        to: Coords,
        formation: Vec<Coords>,
        to_remove: Vec<Coords>,
    },
    SrbAnimateFormation {
        anim: i32,
        formation: Vec<Coords>,
        to_remove: Vec<Coords>,
    },
    SrbAnimateRemove {
        anim: i32,
        formation: Vec<Coords>,
        from: Coords,
        remaining: Vec<Coords>,
    },
    AnimateVictory { anim: i32, team: Team },
}

impl PyramidStep {
    fn you_ready(state: &FailureState) -> PyramidStep {
        let board = state.board();
        if board.srb_supply() > 0 {
            PyramidStep::YouReady {
                possible: state.board().possible_move_starts(Team::You),
            }
        } else {
            PyramidStep::AnimateVictory {
                anim: 0,
                team: Team::You,
            }
        }
    }

    fn srb_thinking(state: &FailureState) -> PyramidStep {
        if state.board().you_supply() == 0 {
            return PyramidStep::AnimateVictory {
                anim: 0,
                team: Team::SRB,
            };
        }
        let result = Arc::new(Mutex::new(None));
        let step = PyramidStep::SrbThinking { result: result.clone() };
        let board = state.board().clone();
        thread::Builder::new()
            .name("SrbThinking".to_string())
            .spawn(move || {
                let start = time::Instant::now();
                let best = board.best_srb_move();
                if cfg!(debug_assertions) {
                    let end = time::Instant::now();
                    let duration = end.duration_since(start);
                    let millis = duration.as_secs() * 1000 +
                                 (duration.subsec_nanos() / 1_000_000) as u64;
                    println!("Found best move in {}ms", millis);
                }
                *result.lock().unwrap() = Some(best);
            })
            .unwrap();
        step
    }

    fn hilighted_tiles(&self) -> HashSet<Coords> {
        match self {
            &PyramidStep::YouJumping { from, .. } => {
                [from].iter().cloned().collect()
            }
            &PyramidStep::YouAnimateFormation { anim, ref formation, .. } |
            &PyramidStep::SrbAnimateFormation { anim, ref formation, .. } => {
                let num = (anim / ANIM_FORMATION_SLOWDOWN) as usize + 1;
                formation.iter().take(num).cloned().collect()
            }
            &PyramidStep::YouRemoving { ref formation, .. } |
            &PyramidStep::YouAnimateRemove { ref formation, .. } |
            &PyramidStep::SrbAnimateRemove { ref formation, .. } => {
                formation.iter().cloned().collect()
            }
            _ => HashSet::new(),
        }
    }

    fn possible_coords(&self) -> HashSet<Coords> {
        match self {
            &PyramidStep::YouReady { ref possible, .. } |
            &PyramidStep::YouJumping { ref possible, .. } |
            &PyramidStep::YouRemoving { ref possible, .. } => possible.clone(),
            _ => HashSet::new(),
        }
    }

    fn animation(&self) -> Option<(Coords, Team, Point)> {
        match self {
            &PyramidStep::YouAnimatePlace { anim, at } => {
                let pt = interpolate(you_supply_pt(),
                                     coords_to_pt(at),
                                     anim,
                                     ANIM_PLACE_FRAMES);
                Some((at, Team::You, pt))
            }
            &PyramidStep::YouAnimateJump { anim, from, to } => {
                let pt = interpolate(coords_to_pt(from),
                                     coords_to_pt(to),
                                     anim,
                                     ANIM_JUMP_FRAMES);
                Some((to, Team::You, pt))
            }
            &PyramidStep::YouAnimateRemove { anim, from, .. } => {
                let pt = interpolate(coords_to_pt(from),
                                     you_supply_pt(),
                                     anim,
                                     ANIM_REMOVE_FRAMES);
                Some((from, Team::You, pt))
            }
            &PyramidStep::SrbAnimatePlace { anim, at, .. } => {
                let pt = interpolate(srb_supply_pt(),
                                     coords_to_pt(at),
                                     anim,
                                     ANIM_PLACE_FRAMES);
                Some((at, Team::SRB, pt))
            }
            &PyramidStep::SrbAnimateJump { anim, from, to, .. } => {
                let pt = interpolate(coords_to_pt(from),
                                     coords_to_pt(to),
                                     anim,
                                     ANIM_JUMP_FRAMES);
                Some((to, Team::SRB, pt))
            }
            &PyramidStep::SrbAnimateRemove { anim, from, .. } => {
                let pt = interpolate(coords_to_pt(from),
                                     srb_supply_pt(),
                                     anim,
                                     ANIM_REMOVE_FRAMES);
                Some((from, Team::SRB, pt))
            }
            _ => None,
        }
    }

    fn clock_tick(&mut self, state: &mut FailureState) -> bool {
        let mut next = None;
        let redraw = match self {
            &mut PyramidStep::YouReady { .. } |
            &mut PyramidStep::YouJumping { .. } |
            &mut PyramidStep::YouRemoving { .. } => false,
            &mut PyramidStep::YouAnimatePlace { ref mut anim, at } => {
                *anim += 1;
                if *anim >= ANIM_PLACE_FRAMES {
                    if let Some(formation) = state.board().formation_at(at) {
                        // TODO: sound effects
                        next = Some(PyramidStep::YouAnimateFormation {
                            anim: 0,
                            formation: formation,
                        });
                    } else {
                        next = Some(PyramidStep::srb_thinking(state));
                    }
                }
                true
            }
            &mut PyramidStep::YouAnimateJump { ref mut anim, to, .. } => {
                *anim += 1;
                if *anim >= ANIM_JUMP_FRAMES {
                    if let Some(formation) = state.board().formation_at(to) {
                        next = Some(PyramidStep::YouAnimateFormation {
                            anim: 0,
                            formation: formation,
                        });
                    } else {
                        next = Some(PyramidStep::srb_thinking(state));
                    }
                }
                true
            }
            &mut PyramidStep::YouAnimateFormation { ref mut anim,
                                                    ref formation } => {
                *anim += 1;
                if *anim >= ANIM_FORMATION_SLOWDOWN * formation.len() as i32 {
                    next = Some(PyramidStep::YouRemoving {
                        formation: formation.clone(),
                        so_far: Vec::new(),
                        possible: state.board().possible_removals(Team::You),
                    });
                }
                *anim % ANIM_FORMATION_SLOWDOWN == 0
            }
            &mut PyramidStep::YouAnimateRemove { ref mut anim,
                                                 ref formation,
                                                 ref so_far,
                                                 .. } => {
                *anim += 1;
                if *anim >= ANIM_REMOVE_FRAMES {
                    if (so_far.len() as i32) < MAX_REMOVALS {
                        let possible = state.board()
                                            .possible_removals(Team::You);
                        if !possible.is_empty() {
                            next = Some(PyramidStep::YouRemoving {
                                formation: formation.clone(),
                                so_far: so_far.clone(),
                                possible: possible,
                            });
                        } else {
                            next = Some(PyramidStep::srb_thinking(state));
                        }
                    } else {
                        next = Some(PyramidStep::srb_thinking(state));
                    }
                }
                true
            }
            &mut PyramidStep::SrbThinking { ref result } => {
                match result.lock().unwrap().take() {
                    Some(Move::Place { at, formation, remove }) => {
                        state.board_mut().set_piece_at(at, Team::SRB);
                        next = Some(PyramidStep::SrbAnimatePlace {
                            anim: 0,
                            at: at,
                            formation: formation,
                            to_remove: remove,
                        });
                        true
                    }
                    Some(Move::Jump { from, to, formation, remove }) => {
                        state.board_mut().remove_piece(from);
                        state.board_mut().set_piece_at(to, Team::SRB);
                        next = Some(PyramidStep::SrbAnimateJump {
                            anim: 0,
                            from: from,
                            to: to,
                            formation: formation,
                            to_remove: remove,
                        });
                        true
                    }
                    None => false,
                }
            }
            &mut PyramidStep::SrbAnimatePlace { ref mut anim,
                                                ref formation,
                                                ref to_remove,
                                                .. } => {
                *anim += 1;
                if *anim >= ANIM_PLACE_FRAMES {
                    if to_remove.is_empty() {
                        next = Some(PyramidStep::you_ready(state));
                    } else {
                        debug_assert!(!formation.is_empty());
                        next = Some(PyramidStep::SrbAnimateFormation {
                            anim: 0,
                            formation: formation.clone(),
                            to_remove: to_remove.clone(),
                        });
                    }
                }
                true
            }
            &mut PyramidStep::SrbAnimateJump { ref mut anim,
                                               ref formation,
                                               ref to_remove,
                                               .. } => {
                *anim += 1;
                if *anim >= ANIM_JUMP_FRAMES {
                    if to_remove.is_empty() {
                        next = Some(PyramidStep::you_ready(state));
                    } else {
                        debug_assert!(!formation.is_empty());
                        next = Some(PyramidStep::SrbAnimateFormation {
                            anim: 0,
                            formation: formation.clone(),
                            to_remove: to_remove.clone(),
                        });
                    }
                }
                true
            }
            &mut PyramidStep::SrbAnimateFormation { ref mut anim,
                                                    ref formation,
                                                    ref to_remove } => {
                *anim += 1;
                if *anim >= ANIM_FORMATION_SLOWDOWN * formation.len() as i32 {
                    debug_assert!(!to_remove.is_empty());
                    let mut remaining = to_remove.clone();
                    let from = remaining.pop().unwrap();
                    state.board_mut().remove_piece(from);
                    next = Some(PyramidStep::SrbAnimateRemove {
                        anim: 0,
                        formation: formation.clone(),
                        from: from,
                        remaining: remaining,
                    });
                }
                *anim % ANIM_FORMATION_SLOWDOWN == 0
            }
            &mut PyramidStep::SrbAnimateRemove { ref mut anim,
                                                 ref formation,
                                                 ref remaining,
                                                 .. } => {
                *anim += 1;
                if *anim >= ANIM_REMOVE_FRAMES {
                    if remaining.is_empty() {
                        next = Some(PyramidStep::you_ready(state));
                    } else {
                        let mut remaining = remaining.clone();
                        let from = remaining.pop().unwrap();
                        state.board_mut().remove_piece(from);
                        next = Some(PyramidStep::SrbAnimateRemove {
                            anim: 0,
                            formation: formation.clone(),
                            from: from,
                            remaining: remaining,
                        });
                    }
                }
                true
            }
            &mut PyramidStep::AnimateVictory { ref mut anim, team } => {
                *anim += 1;
                if *anim >= ANIM_VICTORY_SLOWDOWN {
                    *anim = 0;
                    let board = state.board_mut();
                    let mut changed = false;
                    for coords in Coords::all() {
                        if board.piece_at(coords).is_none() {
                            board.set_piece_at(coords, team);
                            changed = true;
                            break;
                        }
                    }
                    // TODO: handle game over
                    changed
                } else {
                    false
                }
            }
        };
        if let Some(step) = next {
            *self = step;
        }
        redraw
    }
}

// ========================================================================= //

enum PyramidCmd {
    Place(Coords),
    JumpFrom(Coords),
    Jump(Coords, Coords),
    Remove(Vec<Coords>, Vec<Coords>),
}

// ========================================================================= //

const PYRAMID_TILE_SIZE: i32 = 32;
const PYRAMID_BOTTOM_ROW_LEFT: i32 = 160;
const PYRAMID_BOTTOM_ROW_TOP: i32 = 288;
const PYRAMID_BOTTOM: i32 = PYRAMID_BOTTOM_ROW_TOP + PYRAMID_TILE_SIZE;

struct PyramidView {
    sprites: Vec<Sprite>,
    font: Rc<Font>,
    step: PyramidStep,
}

impl PyramidView {
    fn new(resources: &mut Resources, state: &FailureState) -> PyramidView {
        PyramidView {
            sprites: resources.get_sprites("failure/chips"),
            font: resources.get_font("debug"),
            step: PyramidStep::you_ready(state),
        }
    }

    fn draw_supply(&self, team: Team, board: &Board, canvas: &mut Canvas) {
        let (supply, top_left, sprite_index) = match team {
            Team::You => (board.you_supply(), you_supply_pt(), 1),
            Team::SRB => (board.srb_supply(), srb_supply_pt(), 0),
        };
        if supply > 0 {
            canvas.draw_sprite(&self.sprites[sprite_index], top_left);
            let pt = top_left + Point::new(16, 20);
            let text = format!("{}", supply);
            canvas.draw_text(&self.font, Align::Center, pt, &text);
        }
    }
}

impl Element<FailureState, PyramidCmd> for PyramidView {
    fn draw(&self, state: &FailureState, canvas: &mut Canvas) {
        let board = state.board();
        let hilighted_tiles = self.step.hilighted_tiles();
        let animation = self.step.animation();
        let animated_coords = animation.map(|(coords, _, _)| coords);
        for coords in Coords::all() {
            if Some(coords) == animated_coords {
                continue;
            }
            if let Some(team) = board.piece_at(coords) {
                let mut sprite_index = match team {
                    Team::You => 1,
                    Team::SRB => 0,
                };
                if hilighted_tiles.contains(&coords) {
                    sprite_index += 2;
                }
                let top_left = coords_to_pt(coords);
                canvas.draw_sprite(&self.sprites[sprite_index], top_left);
            }
        }
        for coords in self.step.possible_coords() {
            let pt = coords_to_pt(coords);
            let rect = Rect::new(pt.x(),
                                 pt.y(),
                                 PYRAMID_TILE_SIZE as u32,
                                 PYRAMID_TILE_SIZE as u32);
            canvas.draw_rect((255, 255, 0), rect);
        }
        if let Some((_, team, top_left)) = animation {
            let sprite_index = match team {
                Team::You => 1,
                Team::SRB => 0,
            };
            canvas.draw_sprite(&self.sprites[sprite_index], top_left);
        }
        self.draw_supply(Team::You, board, canvas);
        self.draw_supply(Team::SRB, board, canvas);
    }

    fn handle_event(&mut self, event: &Event, state: &mut FailureState)
                    -> Action<PyramidCmd> {
        match event {
            &Event::ClockTick => {
                Action::redraw_if(self.step.clock_tick(state))
            }
            &Event::MouseDown(pt) => {
                if let Some(coords) = pt_to_coords(pt) {
                    match self.step {
                        PyramidStep::YouReady { ref possible } => {
                            if possible.contains(&coords) {
                                if state.board().piece_at(coords).is_none() {
                                    let cmd = PyramidCmd::Place(coords);
                                    return Action::redraw().and_return(cmd);
                                } else {
                                    let cmd = PyramidCmd::JumpFrom(coords);
                                    return Action::redraw().and_return(cmd);
                                }
                            }
                        }
                        PyramidStep::YouJumping { from, ref possible } => {
                            if possible.contains(&coords) {
                                let cmd = PyramidCmd::Jump(from, coords);
                                return Action::redraw().and_return(cmd);
                            }
                        }
                        PyramidStep::YouRemoving { ref formation,
                                                   ref so_far,
                                                   ref possible } => {
                            if possible.contains(&coords) {
                                let mut so_far = so_far.clone();
                                so_far.push(coords);
                                let cmd =
                                    PyramidCmd::Remove(formation.clone(),
                                                       so_far);
                                return Action::redraw().and_return(cmd);
                            }
                        }
                        _ => {}
                    }
                }
                Action::ignore()
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

fn you_supply_pt() -> Point { Point::new(75, 48) }

fn srb_supply_pt() -> Point { Point::new(469, 48) }

fn interpolate(from: Point, to: Point, anim: i32, max_anim: i32) -> Point {
    let x = from.x() + (to.x() - from.x()) * anim / max_anim;
    let y = from.y() + (to.y() - from.y()) * anim / max_anim +
            50 * 4 * anim * (anim - max_anim) / (max_anim * max_anim);
    Point::new(x, y)
}

fn coords_to_pt(coords: Coords) -> Point {
    let left = PYRAMID_BOTTOM_ROW_LEFT + PYRAMID_TILE_SIZE * coords.col() +
               (PYRAMID_TILE_SIZE / 2) * coords.row();
    let top = PYRAMID_BOTTOM_ROW_TOP - PYRAMID_TILE_SIZE * coords.row();
    Point::new(left, top)
}

fn pt_to_coords(pt: Point) -> Option<Coords> {
    if pt.y() > PYRAMID_BOTTOM {
        return None;
    }
    let row = (PYRAMID_BOTTOM - pt.y()) / PYRAMID_TILE_SIZE;
    if row >= 8 {
        return None;
    }
    let left = PYRAMID_BOTTOM_ROW_LEFT + (PYRAMID_TILE_SIZE / 2) * row;
    if pt.x() < left {
        return None;
    }
    let col = (pt.x() - left) / PYRAMID_TILE_SIZE;
    if col >= 8 - row {
        return None;
    }
    Some(Coords::new(row, col))
}

// ========================================================================= //

const INFO_BOX_TEXT_1: &str = "\
Return here later, after you have repaired
more areas of the ship.";

const INFO_BOX_TEXT_2: &str = "\
Your goal is to beat the System Repair Bot at its own game.

Each player starts out with a supply of 18 pieces to place, and
whoever's supply runs out of pieces first loses.

On your turn, you may either place a new piece into an empty
position on the pile, or jump one of your pieces on the pile up
to a higher row.  Either way, if the newly-positioned piece
creates a $iformation$r, then you may remove two of your pieces
from the board and put them back into your supply.

A $iformation$r is four pieces of the same color in a row, either
horizontally or diagonally.";

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use save::Location;
    use super::DASHBOARD_CHIPS;

    #[test]
    fn all_puzzles_represented_on_dashboard() {
        let mut locations: HashSet<Location> =
            Location::all().iter().cloned().collect();
        locations.remove(&Location::Map);
        locations.remove(&Location::Prolog);
        locations.remove(&Location::SystemFailure);
        locations.remove(&Location::PasswordFile);
        locations.remove(&Location::SystemSyzygy);
        for &(_, _, loc) in DASHBOARD_CHIPS {
            locations.remove(&loc);
        }
        assert!(locations.is_empty(),
                "Unrepresented puzzles: {:?}",
                locations);
    }
}

// ========================================================================= //
