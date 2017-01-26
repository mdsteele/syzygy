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

use std::cmp;

use elements::{PuzzleCmd, PuzzleCore, PuzzleView, Scene};
use elements::cutscene::{JumpNode, ParallelNode, QueueNode, SceneNode,
                         SequenceNode, SlideNode, SoundNode, WaitNode};
use elements::shift::{ArrowPair, Platform};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sound};
use modes::SOLVED_INFO_TEXT;
use save::{Game, GearsState, PuzzleState};
use super::scenes::{ELINSA_SLOT, compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32, i32)>,
    animation: Scene,
    platforms: Vec<Platform>,
    arrows: Vec<ArrowPair>,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &GearsState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            animation: Scene::new(Vec::new()),
            platforms: vec![
                Platform::new(resources, (96, 88), state.get_position(0)),
                Platform::new(resources, (96, 120), state.get_position(1)),
                Platform::new(resources, (96, 152), state.get_position(2)),
                Platform::new(resources, (96, 184), state.get_position(3)),
                Platform::new(resources, (96, 184), state.get_position(4)),
                Platform::new(resources, (96, 216), state.get_position(5)),
                Platform::new(resources, (96, 248), state.get_position(6)),
                Platform::new(resources, (96, 280), state.get_position(7)),
            ],
            arrows: vec![
                ArrowPair::new(resources, (448, 96), 0, 3),
                ArrowPair::new(resources, (448, 128), 1, 3),
                ArrowPair::new(resources, (448, 160), 2, 6),
                ArrowPair::new(resources, (448, 192), 3, 3),
                ArrowPair::new(resources, (48, 192), 4, 2),
                ArrowPair::new(resources, (48, 224), 5, 7),
                ArrowPair::new(resources, (48, 256), 6, 5),
                ArrowPair::new(resources, (48, 288), 7, 3),
            ],
        };
        view.animation.begin(view.core.theater_mut());
        view.drain_queue();
        if state.is_visited() && !state.is_solved() {
            view.update_elinsa_position(state);
        }
        view
    }

    fn drain_queue(&mut self) {
        for (row, pos) in self.core.drain_queue() {
            if pos < 0 {
                self.set_override_row(row);
            } else if row >= 0 && row < self.platforms.len() as i32 {
                self.platforms[row as usize].set_goal(pos);
            } else if row == -2 {
                for platform in self.platforms.iter_mut() {
                    platform.move_to_goal();
                }
            }
        }
    }

    fn set_override_row(&mut self, row: i32) {
        for (index, arrow) in self.arrows.iter_mut().enumerate() {
            if index as i32 == row {
                arrow.set_delta_override(Some(4));
            } else {
                arrow.set_delta_override(None);
            }
        }
    }

    fn platform_top(&self, row: i32) -> i32 {
        assert!(row >= 0 && row < self.platforms.len() as i32);
        self.platforms[row as usize].top()
    }

    fn platform_pt(&self, state: &GearsState, row: i32) -> Point {
        if row < 0 {
            Point::new(80, 64)
        } else if row >= GearsState::num_rows() {
            Point::new(464, 304)
        } else {
            self.platform_pt_for_pos(row, state.get_position(row))
        }
    }

    fn platform_pt_for_pos(&self, row: i32, pos: i32) -> Point {
        assert!(row >= 0 && row < self.platforms.len() as i32);
        self.platforms[row as usize].top_point_for_pos(pos)
    }

    fn floor_pt_for_pos(&self, pos: i32) -> Point {
        Point::new(self.platforms[0].pos_to_left(pos) + 16, 320)
    }

    fn update_elinsa_position(&mut self, state: &GearsState) {
        let row = state.get_elinsa_row();
        let position = self.platform_pt(state, row);
        self.core.theater_mut().set_actor_position(ELINSA_SLOT, position);
        self.set_override_row(row);
    }

    fn update_platform_positions(&mut self, state: &GearsState) {
        for row in 0..GearsState::num_rows() {
            self.platforms[row as usize].set_position(state.get_position(row));
        }
    }

    fn shift_platform(&mut self, state: &mut GearsState, row: i32,
                      mut delta: i32) {
        let num_rows = GearsState::num_rows() as i32;
        let min_pos = GearsState::min_position_for_row(row);
        let max_pos = GearsState::max_position_for_row(row);
        let original_position = state.get_position(row);
        let original_elinsa_row = state.get_elinsa_row();
        let mut top_seq: Vec<Box<SceneNode>> = Vec::new();

        // Move platform:
        let mut elinsa_row = state.get_elinsa_row();
        let mut elinsa_pos = 0;
        if elinsa_row >= 0 && elinsa_row < num_rows {
            elinsa_pos = state.get_position(elinsa_row);
        }
        let mut pos = state.get_position(row);
        while delta != 0 {
            let mut platform_seq: Vec<Box<SceneNode>> = Vec::new();
            let mut elinsa_seq: Vec<Box<SceneNode>> = Vec::new();
            let old_pos = pos;
            pos += delta;
            if pos < min_pos {
                debug_assert!(delta < 0);
                pos = min_pos;
                delta = -(delta + (old_pos - min_pos));
                debug_assert!(delta > 0);
            } else if pos > max_pos {
                debug_assert!(delta > 0);
                pos = max_pos;
                delta = -(delta - (max_pos - old_pos));
                debug_assert!(delta < 0);
            } else {
                delta = 0;
            }
            let travel_time = Platform::travel_time(old_pos, pos);
            let sound = Sound::platform_shift((pos - old_pos).abs());
            platform_seq.push(Box::new(SoundNode::new(sound)));
            platform_seq.push(Box::new(QueueNode::new((row, pos))));
            platform_seq.push(Box::new(WaitNode::new(travel_time)));
            if elinsa_row >= 0 && elinsa_row < num_rows &&
               (row == elinsa_row - 1 ||
                ((row == 2 || row == 3) && row == elinsa_row - 2)) {
                let impact = if pos > old_pos && elinsa_pos > old_pos &&
                                elinsa_pos <= pos {
                    Some((Platform::travel_time(old_pos, elinsa_pos - 1),
                          cmp::min(10, elinsa_pos + 1)))
                } else if pos < old_pos && elinsa_pos >= pos &&
                                elinsa_pos < old_pos {
                    Some((Platform::travel_time(old_pos, elinsa_pos + 1),
                          cmp::max(0, elinsa_pos - 1)))
                } else {
                    None
                };
                if let Some((time_to_hit, new_elinsa_pos)) = impact {
                    elinsa_pos = new_elinsa_pos;
                    elinsa_row = state.fall_from(elinsa_row, elinsa_pos);
                    let dest = if elinsa_row < num_rows {
                        self.platform_pt_for_pos(elinsa_row, elinsa_pos)
                    } else {
                        self.floor_pt_for_pos(elinsa_pos)
                    };
                    let fall_dist = dest.y() - self.platform_top(row + 1);
                    let time_to_fall = JumpNode::time_to_fall(fall_dist + 5) +
                                       JumpNode::time_to_fall(5);
                    elinsa_seq.push(Box::new(WaitNode::new(time_to_hit)));
                    let sound = Sound::character_collision();
                    elinsa_seq.push(Box::new(SoundNode::new(sound)));
                    elinsa_seq.push(Box::new(JumpNode::new(ELINSA_SLOT,
                                                           dest,
                                                           time_to_fall)));
                    elinsa_seq.push(Box::new(QueueNode::new((elinsa_row,
                                                             -1))));
                }
            } else if row == elinsa_row {
                let barrier_pos_1 = if row > 0 {
                    state.get_position(row - 1)
                } else {
                    5
                };
                let barrier_pos_2 = if row == 4 || row == 5 {
                    state.get_position(row - 2)
                } else {
                    barrier_pos_1
                };
                let impact = if pos > old_pos {
                    if barrier_pos_1 > old_pos && barrier_pos_1 <= pos {
                        if barrier_pos_2 > old_pos &&
                           barrier_pos_2 < barrier_pos_1 {
                            Some(barrier_pos_2 - 1)
                        } else {
                            Some(barrier_pos_1 - 1)
                        }
                    } else if barrier_pos_2 > old_pos && barrier_pos_2 <= pos {
                        Some(barrier_pos_2 - 1)
                    } else {
                        None
                    }
                } else {
                    if barrier_pos_1 < old_pos && barrier_pos_1 >= pos {
                        if barrier_pos_2 < old_pos &&
                           barrier_pos_2 > barrier_pos_1 {
                            Some(barrier_pos_2 + 1)
                        } else {
                            Some(barrier_pos_1 + 1)
                        }
                    } else if barrier_pos_2 < old_pos && barrier_pos_2 >= pos {
                        Some(barrier_pos_2 + 1)
                    } else {
                        None
                    }
                };
                if let Some(new_elinsa_pos) = impact {
                    let time_to_hit = Platform::travel_time(elinsa_pos,
                                                            new_elinsa_pos);
                    elinsa_pos = new_elinsa_pos;
                    elinsa_row = state.fall_from(elinsa_row, elinsa_pos);
                    let slide_dest = self.platform_pt_for_pos(row, elinsa_pos);
                    let jump_dest = if elinsa_row < num_rows {
                        self.platform_pt_for_pos(elinsa_row, elinsa_pos)
                    } else {
                        self.floor_pt_for_pos(elinsa_pos)
                    };
                    let fall_dist = jump_dest.y() - self.platform_top(row);
                    let time_to_fall = JumpNode::time_to_fall(fall_dist + 5) +
                                       JumpNode::time_to_fall(5);
                    elinsa_seq.push(Box::new(SlideNode::new(ELINSA_SLOT,
                                                            slide_dest,
                                                            false,
                                                            false,
                                                            time_to_hit)));
                    let sound = Sound::character_collision();
                    elinsa_seq.push(Box::new(SoundNode::new(sound)));
                    elinsa_seq.push(Box::new(JumpNode::new(ELINSA_SLOT,
                                                           jump_dest,
                                                           time_to_fall)));
                    elinsa_seq.push(Box::new(QueueNode::new((elinsa_row,
                                                             -1))));
                } else {
                    elinsa_pos = pos;
                    let dest = self.platform_pt_for_pos(row, elinsa_pos);
                    elinsa_seq.push(Box::new(SlideNode::new(ELINSA_SLOT,
                                                            dest,
                                                            false,
                                                            false,
                                                            travel_time)));
                }
            }
            top_seq.push(Box::new(ParallelNode::new(vec![
                Box::new(SequenceNode::new(platform_seq)),
                Box::new(SequenceNode::new(elinsa_seq)),
            ])));
        }
        state.set_position(row, pos);

        // If Elinsa fell to the floor, get back on the starting platform:
        if elinsa_row == num_rows && original_elinsa_row != num_rows {
            let slide_time = 0.5 * Platform::travel_time(elinsa_pos, 10);
            top_seq.push(Box::new(SlideNode::new(ELINSA_SLOT,
                                                 self.floor_pt_for_pos(10),
                                                 false,
                                                 false,
                                                 slide_time)));
            let dest = self.platform_pt(state, elinsa_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(ELINSA_SLOT, dest, 0.5)));
        }

        // Make Elinsa climb upwards:
        let last_row = num_rows - 1;
        let max_pos_for_last_row = GearsState::max_position_for_row(last_row);
        if elinsa_row == num_rows &&
           state.get_position(last_row) == max_pos_for_last_row &&
           state.get_position(last_row - 1) != max_pos_for_last_row {
            elinsa_row -= 1;
            let dest = self.platform_pt(state, elinsa_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(ELINSA_SLOT, dest, 0.6)));
            top_seq.push(Box::new(QueueNode::new((elinsa_row, -1))));
        }
        if elinsa_row < num_rows {
            while elinsa_row > 0 {
                let elinsa_pos = state.get_position(elinsa_row);
                let pos_1 = state.get_position(elinsa_row - 1);
                let pos_2 = if elinsa_row > 1 {
                    state.get_position(elinsa_row - 2)
                } else {
                    5
                };
                let mut next_row = elinsa_row;
                if (elinsa_pos - pos_1).abs() == 1 && pos_2 != pos_1 &&
                   pos_2 != elinsa_pos &&
                   ((elinsa_row != 5 && elinsa_row != 6) ||
                    state.get_position(elinsa_row - 3) != pos_1) {
                    next_row = elinsa_row - 1;
                } else if (elinsa_row == 4 || elinsa_row == 5) &&
                   (elinsa_pos - pos_2).abs() == 1 {
                    let pos_3 = state.get_position(elinsa_row - 3);
                    if pos_3 != pos_2 && pos_3 != elinsa_pos {
                        next_row = elinsa_row - 2;
                    }
                }
                if next_row == elinsa_row {
                    break;
                }
                elinsa_row = next_row;
                let dest = self.platform_pt(state, elinsa_row);
                top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
                top_seq.push(Box::new(JumpNode::new(ELINSA_SLOT, dest, 0.6)));
                top_seq.push(Box::new(QueueNode::new((elinsa_row, -1))));
            }
        }
        if elinsa_row == 0 &&
           state.get_position(0) == GearsState::min_position_for_row(0) {
            elinsa_row -= 1;
            let dest = self.platform_pt(state, elinsa_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(ELINSA_SLOT, dest, 0.6)));
            top_seq.push(Box::new(QueueNode::new((elinsa_row, -1))));
        }
        state.set_elinsa_row(elinsa_row);
        self.core.push_undo((row,
                             pos - original_position,
                             elinsa_row - original_elinsa_row));

        // Start animation:
        self.animation = Scene::new(vec![
            Box::new(SequenceNode::new(top_seq)),
        ]);
        self.animation.begin(self.core.theater_mut());
        self.drain_queue();
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.shift_gears;
        self.core.draw_back_layer(canvas);
        self.arrows.draw(&(), canvas);
        self.platforms.draw(&(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.shift_gears;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() {
            let subaction = self.platforms.handle_event(event, &mut ());
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            let subaction = self.animation
                                .handle_event(event, self.core.theater_mut());
            action.merge(subaction.but_no_value());
            self.drain_queue();
            if state.is_solved() && self.animation.is_finished() {
                self.core.begin_outro_scene();
            }
        }
        if !action.should_stop() &&
           (event == &Event::ClockTick || !state.is_solved()) {
            let subaction = self.arrows.handle_event(event, &mut ());
            if let Some(&(row, delta)) = subaction.value() {
                self.shift_platform(state, row, delta);
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.shift_gears.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((row, dx, dy)) = self.core.pop_undo() {
            let state = &mut game.shift_gears;
            let new_pos = state.get_position(row) - dx;
            let new_elinsa = state.get_elinsa_row() - dy;
            state.set_position(row, new_pos);
            self.platforms[row as usize].set_position(new_pos);
            state.set_elinsa_row(new_elinsa);
            self.update_elinsa_position(state);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((row, dx, dy)) = self.core.pop_redo() {
            let state = &mut game.shift_gears;
            let new_pos = state.get_position(row) + dx;
            let new_elinsa = state.get_elinsa_row() + dy;
            state.set_position(row, new_pos);
            self.platforms[row as usize].set_position(new_pos);
            state.set_elinsa_row(new_elinsa);
            self.update_elinsa_position(state);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.shift_gears;
        self.core.clear_undo_redo();
        state.reset();
        self.update_platform_positions(state);
        self.update_elinsa_position(state);
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.shift_gears;
        state.solve();
        self.update_platform_positions(state);
        self.update_elinsa_position(state);
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to help Elinsa reach the ledge in the
upper-left corner.

$M{Tap}{Click} the arrow buttons to move a platform to
the left or to the right.  If Elinsa is standing on
that platform, it will move by 4 instead of by
its normal distance.

Elinsa will jump up onto the next platform when
it is adjacent to her.";

// ========================================================================= //
