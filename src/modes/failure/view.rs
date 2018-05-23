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

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sound, Sprite};
use save::{Access, FailureState, Game, Location, PuzzleState};
use save::pyramid::{Board, Coords, MAX_REMOVALS, Move, Team};
use super::coords::{coords_to_pt, pt_to_coords};
use super::scenes;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const DASHBOARD_CHIPS: &[(i32, i32, char, Location)] = &[
    (167, 71,  'B', Location::ThreeBlindIce),
    (209, 71,  'R', Location::LevelHeaded),
    (251, 71,  'I', Location::AutofacTour),
    (293, 71,  'D', Location::TreadLightly),
    (335, 71,  'G', Location::PointOfOrder),
    (377, 71,  'E', Location::MissedConnections),
    (167, 113, 'S', Location::JogYourMemory),
    (209, 113, 'H', Location::ColumnAsIcyEm),
    (251, 113, 'A', Location::StarCrossed),
    (293, 113, 'L', Location::TheIceIsRight),
    (335, 113, 'L', Location::PlaneAsDay),
    (377, 113, ' ', Location::ShiftTheBlame),
    (167, 155, 'E', Location::LevelUp),
    (209, 155, 'X', Location::HexSpangled),
    (251, 155, 'T', Location::WhatchaColumn),
    (293, 155, 'E', Location::PointOfNoReturn),
    (335, 155, 'N', Location::CrossSauce),
    (377, 155, 'D', Location::PointOfView),
    (167, 197, ' ', Location::ShiftGears),
    (209, 197, 'A', Location::LogLevel),
    (251, 197, 'F', Location::IceToMeetYou),
    (293, 197, 'T', Location::BlackAndBlue),
    (335, 197, 'E', Location::DoubleCross),
    (377, 197, 'R', Location::LightSyrup),
    (167, 239, 'F', Location::IfMemoryServes),
    (209, 239, 'I', Location::ConnectTheDots),
    (251, 239, 'N', Location::CubeTangle),
    (293, 239, 'I', Location::FactOrFiction),
    (335, 239, 'S', Location::PlaneAndSimple),
    (377, 239, 'H', Location::ShiftingGround),
    (167, 281, 'R', Location::CrossTheLine),
    (209, 281, 'E', Location::ALightInTheAttic),
    (251, 281, 'P', Location::Disconnected),
    (293, 281, 'A', Location::WreckedAngle),
    (335, 281, 'I', Location::MemoryLane),
    (377, 281, 'R', Location::TheYFactor),
];

// ========================================================================= //

#[derive(Clone)]
enum UndoRedo {
    Place(Coords),
    Jumping(Coords),
    Jump(Coords, Coords),
    Remove(Vec<Coords>, Vec<Coords>),
}

// ========================================================================= //

pub struct View {
    core: PuzzleCore<UndoRedo>,
    dashboard: Vec<DashChip>,
    pyramid: PyramidView,
    show_pyramid: bool,
    should_mark_mid_scene_done: bool,
    should_reset: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, game: &Game) -> View {
        let mut all_puzzles_solved = true;
        for &(_, _, _, location) in DASHBOARD_CHIPS.iter() {
            if !game.has_been_solved(location) {
                all_puzzles_solved = false;
                break;
            }
        }
        let state = &game.system_failure;
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_middle_scene(resources));
        core.add_extra_scene(scenes::compile_argony_midscene(resources));
        core.add_extra_scene(scenes::compile_elinsa_midscene(resources));
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        core.add_extra_scene(scenes::compile_srb_midscene(resources));
        core.add_extra_scene(scenes::compile_ugrent_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        core.add_extra_scene(scenes::compile_lose_game_scene(resources));
        for index in 0..scenes::num_hints() {
            core.add_extra_scene(scenes::compile_hint_scene(resources, index));
        }
        if !state.is_solved() {
            if state.mid_scene_is_done() {
                if state.access() == Access::BeginReplay {
                    // The puzzle core will run the middle scene once the intro
                    // scene finishes.
                    core.begin_extra_scene(scenes::MIDDLE_SCENE);
                } else {
                    core.skip_extra_scene(scenes::MIDDLE_SCENE);
                }
            } else if all_puzzles_solved {
                core.begin_extra_scene(scenes::MIDDLE_SCENE);
            }
        }
        View {
            core: core,
            dashboard: DASHBOARD_CHIPS
                .iter()
                .map(|&(x, y, chr, loc)| {
                         DashChip::new(resources, x, y, loc, chr)
                     })
                .collect(),
            pyramid: PyramidView::new(resources, state),
            show_pyramid: false,
            should_mark_mid_scene_done: false,
            should_reset: false,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.system_failure;
        self.core.clear_screen(canvas);
        if self.show_pyramid {
            self.pyramid.draw(state, canvas);
        } else {
            self.dashboard.draw(game, canvas);
        }
        self.core.draw_back_layer_no_clear(canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let mut action = self.core
            .handle_event(event, &mut game.system_failure);
        if self.should_mark_mid_scene_done {
            game.system_failure.set_mid_scene_is_done(true);
            self.should_mark_mid_scene_done = false;
        }
        if self.should_reset {
            self.should_reset = false;
            self.reset(game);
            action.also_play_sound(Sound::reset());
            self.pyramid.hilight_override.clear();
        }
        if !self.show_pyramid {
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
                        if state.board().formation_at(coords).is_some() {
                            self.core.push_undo(UndoRedo::Place(coords));
                        } else {
                            self.core.clear_undo_redo();
                        }
                    }
                    Some(&PyramidCmd::JumpFrom(from)) => {
                        self.pyramid.step = PyramidStep::YouJumping {
                            from: from,
                            possible: state.board().possible_jump_dests(from),
                        };
                        self.core.push_undo(UndoRedo::Jumping(from));
                    }
                    Some(&PyramidCmd::Jump(from, to)) => {
                        state.board_mut().remove_piece(from);
                        state.board_mut().set_piece_at(to, Team::You);
                        self.pyramid.step = PyramidStep::YouAnimateJump {
                            anim: 0,
                            from: from,
                            to: to,
                        };
                        if state.board().formation_at(to).is_some() {
                            self.core.push_undo(UndoRedo::Jump(from, to));
                        } else {
                            self.core.clear_undo_redo();
                        }
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
                        if (so_far.len() as i32) < MAX_REMOVALS &&
                            !state
                                .board()
                                .possible_removals(Team::You)
                                .is_empty()
                        {
                            self.core
                                .push_undo(UndoRedo::Remove(formation.clone(),
                                                            so_far.clone()));
                        } else {
                            self.core.clear_undo_redo();
                        }
                    }
                    Some(&PyramidCmd::Win) => {
                        state.solve();
                        self.core.begin_outro_scene();
                        action = action.and_return(PuzzleCmd::Save);
                    }
                    Some(&PyramidCmd::Lose) => {
                        self.core.begin_extra_scene(scenes::LOSE_GAME_SCENE);
                    }
                    Some(&PyramidCmd::PasswordHint(coords)) => {
                        self.core.begin_extra_scene(
                            scenes::hint_scene_for_coords(coords));
                    }
                    None => {}
                }
                action.merge(subaction.but_no_value());
            }
        }
        if !action.should_stop() {
            self.core.begin_character_scene_on_click(event);
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.system_failure.is_solved() {
            INFO_BOX_TEXT_3
        } else if game.system_failure.mid_scene_is_done() {
            INFO_BOX_TEXT_2
        } else {
            INFO_BOX_TEXT_1
        }
    }

    fn undo(&mut self, game: &mut Game) {
        let state = &mut game.system_failure;
        match self.core.pop_undo() {
            Some(UndoRedo::Place(at)) => {
                state.board_mut().remove_piece(at);
                self.pyramid.step = PyramidStep::you_ready(state);
            }
            Some(UndoRedo::Jumping(_from)) => {
                self.pyramid.step = PyramidStep::you_ready(state);
            }
            Some(UndoRedo::Jump(from, to)) => {
                state.board_mut().remove_piece(to);
                state.board_mut().set_piece_at(from, Team::You);
                self.pyramid.step = PyramidStep::YouJumping {
                    from: from,
                    possible: state.board().possible_jump_dests(from),
                };
            }
            Some(UndoRedo::Remove(formation, mut so_far)) => {
                debug_assert!(!so_far.is_empty());
                let coords = so_far.pop().unwrap();
                state.board_mut().set_piece_at(coords, Team::You);
                self.pyramid.step = PyramidStep::YouRemoving {
                    formation: formation,
                    so_far: so_far,
                    possible: state.board().possible_removals(Team::You),
                };
            }
            None => {}
        }
    }

    fn redo(&mut self, game: &mut Game) {
        let state = &mut game.system_failure;
        match self.core.pop_redo() {
            Some(UndoRedo::Place(at)) => {
                state.board_mut().set_piece_at(at, Team::You);
                self.pyramid.step = PyramidStep::YouRemoving {
                    formation: state.board().formation_at(at).unwrap(),
                    so_far: Vec::new(),
                    possible: state.board().possible_removals(Team::You),
                };
            }
            Some(UndoRedo::Jumping(from)) => {
                self.pyramid.step = PyramidStep::YouJumping {
                    from: from,
                    possible: state.board().possible_jump_dests(from),
                };
            }
            Some(UndoRedo::Jump(from, to)) => {
                state.board_mut().remove_piece(from);
                state.board_mut().set_piece_at(to, Team::You);
                self.pyramid.step = PyramidStep::YouRemoving {
                    formation: state.board().formation_at(to).unwrap(),
                    so_far: Vec::new(),
                    possible: state.board().possible_removals(Team::You),
                };
            }
            Some(UndoRedo::Remove(formation, so_far)) => {
                debug_assert!(!so_far.is_empty());
                let &coords = so_far.last().unwrap();
                state.board_mut().remove_piece(coords);
                self.pyramid.step = PyramidStep::YouRemoving {
                    formation: formation,
                    so_far: so_far,
                    possible: state.board().possible_removals(Team::You),
                };
            }
            None => {}
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
        self.pyramid.step = PyramidStep::GameOver;
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.show_pyramid = value != 0;
            } else if kind == 1 || kind == 2 {
                let team = if kind == 1 { Team::You } else { Team::SRB };
                if value < 0 {
                    self.pyramid.hilight_override.clear();
                } else if let Some(coords) = Coords::from_index(value as
                                                                    usize)
                {
                    if self.pyramid.hilight_override.get(&coords) ==
                        Some(&team)
                    {
                        self.pyramid.hilight_override.remove(&coords);
                    } else {
                        self.pyramid.hilight_override.insert(coords, team);
                    }
                }
            } else if kind == 3 {
                self.pyramid.team_override = if value == 1 {
                    Some(Team::You)
                } else if value == 2 {
                    Some(Team::SRB)
                } else {
                    None
                };
            } else if kind == 4 {
                self.should_mark_mid_scene_done = value != 0;
            } else if kind == 5 {
                self.should_reset = value != 0;
            } else if kind == 6 {
                if value < 0 {
                    for chip in self.dashboard.iter_mut() {
                        chip.force_red = false;
                        chip.hide_letter = false;
                    }
                } else {
                    let index = value as usize;
                    if index < self.dashboard.len() {
                        self.dashboard[index].force_red = true;
                        self.dashboard[index].hide_letter = true;
                    }
                }
            } else if kind == 7 {
                self.dashboard[9].force_red = true;
                self.dashboard[9].letter = 'N';
                self.dashboard[10].force_red = true;
                self.dashboard[10].letter = 'T';
            } else if kind == 8 {
                for (index, chip) in self.dashboard.iter_mut().enumerate() {
                    chip.hide_letter = true;
                    if (index % 6) < 3 {
                        chip.force_red = false;
                        chip.goal_topleft = you_supply_pt();
                    } else {
                        chip.force_red = true;
                        chip.goal_topleft = srb_supply_pt();
                    }
                }
            }
        }
    }
}

// ========================================================================= //

const DASH_ANIM_SLOWDOWN: i32 = 4;
const DASH_ANIM_INDICES: &[usize] =
    &[4, 5, 6, 7, 8, 9, 10, 11, 12, 7, 6, 13, 14, 15];
const DASH_SLIDE_SPEED: f64 = 30.0; // pixels/frame

struct DashChip {
    sprites: Vec<Sprite>,
    font: Rc<Font>,
    topleft: Point,
    goal_topleft: Point,
    location: Location,
    letter: char,
    anim: i32,
    hide_letter: bool,
    force_red: bool,
}

impl DashChip {
    fn new(resources: &mut Resources, left: i32, top: i32,
           location: Location, letter: char)
           -> DashChip {
        let topleft = Point::new(left, top);
        DashChip {
            sprites: resources.get_sprites("failure/chips"),
            font: resources.get_font("roman"),
            topleft: topleft,
            goal_topleft: topleft,
            location: location,
            letter: letter,
            anim: (left + top) %
                (DASH_ANIM_SLOWDOWN * DASH_ANIM_INDICES.len() as i32),
            hide_letter: false,
            force_red: false,
        }
    }
}

impl Element<Game, ()> for DashChip {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let solved = game.has_been_solved(self.location);
        let index = if solved && !self.force_red {
            if self.hide_letter {
                1
            } else {
                DASH_ANIM_INDICES[(self.anim / DASH_ANIM_SLOWDOWN) as usize]
            }
        } else {
            0
        };
        canvas.draw_sprite(&self.sprites[index], self.topleft);
        if solved && !self.hide_letter && self.letter != ' ' {
            canvas.fill_rect((191, 191, 191),
                             Rect::new(self.topleft.x() + 13,
                                       self.topleft.y() + 11,
                                       6,
                                       10));
            canvas.fill_rect((191, 191, 191),
                             Rect::new(self.topleft.x() + 12,
                                       self.topleft.y() + 12,
                                       8,
                                       8));
            canvas.draw_char(&self.font,
                             Align::Center,
                             self.topleft + Point::new(16, 20),
                             self.letter);
        }
    }

    fn handle_event(&mut self, event: &Event, _: &mut Game) -> Action<()> {
        match event {
            &Event::ClockTick => {
                self.anim += 1;
                if self.anim ==
                    DASH_ANIM_INDICES.len() as i32 * DASH_ANIM_SLOWDOWN
                {
                    self.anim = 0;
                }
                if self.topleft != self.goal_topleft {
                    let dx = (self.goal_topleft.x() - self.topleft.x()) as f64;
                    let dy = (self.goal_topleft.y() - self.topleft.y()) as f64;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist <= DASH_SLIDE_SPEED {
                        self.topleft = self.goal_topleft;
                    } else {
                        let scale = DASH_SLIDE_SPEED / dist;
                        self.topleft = self.topleft +
                            Point::new((dx * scale).round() as i32,
                                       (dy * scale).round() as i32);
                    }
                    return Action::redraw();
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
    Victory { winner: Team },
    GameOver,
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

    fn srb_thinking(state: &mut FailureState) -> PyramidStep {
        if state.board().you_supply() == 0 {
            state.clear_committed_board();
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
            &PyramidStep::YouAnimateFormation {
                anim,
                ref formation,
                ..
            } |
            &PyramidStep::SrbAnimateFormation {
                anim,
                ref formation,
                ..
            } => {
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
            &mut PyramidStep::YouAnimateFormation {
                ref mut anim,
                ref formation,
            } => {
                *anim += 1;
                if *anim >= ANIM_FORMATION_SLOWDOWN * formation.len() as i32 {
                    next = Some(PyramidStep::YouRemoving {
                                    formation: formation.clone(),
                                    so_far: Vec::new(),
                                    possible:
                                        state
                                            .board()
                                            .possible_removals(Team::You),
                                });
                }
                *anim % ANIM_FORMATION_SLOWDOWN == 0
            }
            &mut PyramidStep::YouAnimateRemove {
                ref mut anim,
                ref formation,
                ref so_far,
                ..
            } => {
                *anim += 1;
                if *anim >= ANIM_REMOVE_FRAMES {
                    if (so_far.len() as i32) < MAX_REMOVALS {
                        let possible =
                            state.board().possible_removals(Team::You);
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
                    Some(Move::Place {
                             at,
                             formation,
                             remove,
                         }) => {
                        state.board_mut().set_piece_at(at, Team::SRB);
                        next = Some(PyramidStep::SrbAnimatePlace {
                                        anim: 0,
                                        at: at,
                                        formation: formation,
                                        to_remove: remove,
                                    });
                        true
                    }
                    Some(Move::Jump {
                             from,
                             to,
                             formation,
                             remove,
                         }) => {
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
            &mut PyramidStep::SrbAnimatePlace {
                ref mut anim,
                ref formation,
                ref to_remove,
                ..
            } => {
                *anim += 1;
                if *anim >= ANIM_PLACE_FRAMES {
                    if to_remove.is_empty() {
                        state.commit_board();
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
            &mut PyramidStep::SrbAnimateJump {
                ref mut anim,
                ref formation,
                ref to_remove,
                ..
            } => {
                *anim += 1;
                if *anim >= ANIM_JUMP_FRAMES {
                    if to_remove.is_empty() {
                        state.commit_board();
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
            &mut PyramidStep::SrbAnimateFormation {
                ref mut anim,
                ref formation,
                ref to_remove,
            } => {
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
            &mut PyramidStep::SrbAnimateRemove {
                ref mut anim,
                ref formation,
                ref remaining,
                ..
            } => {
                *anim += 1;
                if *anim >= ANIM_REMOVE_FRAMES {
                    if remaining.is_empty() {
                        state.commit_board();
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
                    if !changed {
                        next = Some(PyramidStep::Victory { winner: team });
                    }
                    changed
                } else {
                    false
                }
            }
            &mut PyramidStep::Victory { .. } => false,
            &mut PyramidStep::GameOver => false,
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
    Win,
    Lose,
    PasswordHint(Coords),
}

// ========================================================================= //

struct PyramidView {
    chip_sprites: Vec<Sprite>,
    possible_sprites: Vec<Sprite>,
    font: Rc<Font>,
    step: PyramidStep,
    team_override: Option<Team>,
    hilight_override: HashMap<Coords, Team>,
}

impl PyramidView {
    fn new(resources: &mut Resources, state: &FailureState) -> PyramidView {
        PyramidView {
            chip_sprites: resources.get_sprites("failure/chips"),
            possible_sprites: resources.get_sprites("failure/possible"),
            font: resources.get_font("debug"),
            step: PyramidStep::you_ready(state),
            team_override: None,
            hilight_override: HashMap::new(),
        }
    }

    fn draw_supply(&self, team: Team, board: &Board, canvas: &mut Canvas) {
        let (supply, top_left, sprite_index) = match team {
            Team::You => (board.you_supply(), you_supply_pt(), 1),
            Team::SRB => (board.srb_supply(), srb_supply_pt(), 0),
        };
        if supply > 0 {
            canvas.draw_sprite(&self.chip_sprites[sprite_index], top_left);
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
                let hilight = self.hilight_override.get(&coords).cloned();
                let team =
                    hilight
                        .unwrap_or_else(|| self.team_override.unwrap_or(team));
                let mut sprite_index = match team {
                    Team::You => 1,
                    Team::SRB => 0,
                };
                if hilight.is_some() || hilighted_tiles.contains(&coords) {
                    sprite_index += 2;
                }
                let top_left = coords_to_pt(coords);
                canvas.draw_sprite(&self.chip_sprites[sprite_index], top_left);
            }
        }
        // Outline possible moves:
        for coords in self.step.possible_coords() {
            let pt = coords_to_pt(coords);
            let index = if state.board().piece_at(coords).is_some() {
                1
            } else {
                0
            };
            canvas.draw_sprite(&self.possible_sprites[index], pt);
        }
        // Draw animated piece (if any):
        if let Some((_, team, top_left)) = animation {
            let sprite_index = match team {
                Team::You => 1,
                Team::SRB => 0,
            };
            canvas.draw_sprite(&self.chip_sprites[sprite_index], top_left);
        }
        self.draw_supply(Team::You, board, canvas);
        self.draw_supply(Team::SRB, board, canvas);
    }

    fn handle_event(&mut self, event: &Event, state: &mut FailureState)
                    -> Action<PyramidCmd> {
        match event {
            &Event::ClockTick => {
                let mut action = Action::redraw_if(self.step
                                                       .clock_tick(state));
                match self.step {
                    PyramidStep::Victory { winner: Team::You } => {
                        action = action.and_return(PyramidCmd::Win);
                        self.step = PyramidStep::GameOver;
                    }
                    PyramidStep::Victory { winner: Team::SRB } => {
                        action = action.and_return(PyramidCmd::Lose);
                        self.step = PyramidStep::GameOver;
                    }
                    _ => {}
                }
                action
            }
            &Event::MouseDown(pt) => {
                if let Some(coords) = pt_to_coords(pt) {
                    if state.is_solved() {
                        let cmd = PyramidCmd::PasswordHint(coords);
                        return Action::redraw().and_return(cmd);
                    }
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
                        PyramidStep::YouRemoving {
                            ref formation,
                            ref so_far,
                            ref possible,
                        } => {
                            if possible.contains(&coords) {
                                let mut so_far = so_far.clone();
                                so_far.push(coords);
                                let cmd = PyramidCmd::Remove(formation
                                                                 .clone(),
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

// ========================================================================= //

const INFO_BOX_TEXT_1: &str = "\
Return here later, after you have repaired
more areas of the ship.";

const INFO_BOX_TEXT_2: &str = "\
Your goal is to beat the System Repair Bot at its own game.

Each player starts out with a supply of 18 pieces to place, and
whoever's supply runs out of pieces first loses.

On your turn, you may either place a new piece into an open
position on the pile, or jump one of your uncovered pieces on
the pile up to a higher row.  Either way, if the newly-positioned
piece creates a line of four pieces of the same color, then you
may remove two of your pieces from the board and put them
back into your supply.

$M{Tap}{Click} on a character in the scene to hear their words of wisdom.";

const INFO_BOX_TEXT_3: &str = "\
$M{Tap}{Click} on a tile to get a password hint.";

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
        locations.remove(&Location::Finale);
        for &(_, _, _, loc) in DASHBOARD_CHIPS {
            locations.remove(&loc);
        }
        assert!(locations.is_empty(),
                "Unrepresented puzzles: {:?}",
                locations);
    }

    #[test]
    fn no_repeated_locations_on_dashboard() {
        let mut locations: HashSet<Location> = HashSet::new();
        for &(_, _, _, loc) in DASHBOARD_CHIPS {
            assert!(!locations.contains(&loc), "Repeated: {:?}", loc);
            locations.insert(loc);
        }
    }
}

// ========================================================================= //
