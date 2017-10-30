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

use std::cmp::{max, min};

use elements::{PuzzleCmd, PuzzleCore, PuzzleView, Scene};
use elements::cutscene::{JumpNode, ParallelNode, QueueNode, SceneNode,
                         SequenceNode, SlideNode, SoundNode, WaitNode};
use elements::shift::{ArrowPair, Platform};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sound};
use modes::SOLVED_INFO_TEXT;
use save::{BlameState, Game, PuzzleState};
use super::scenes::{ELINSA_SLOT, compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32, i32, i32)>,
    animation: Scene,
    platforms: Vec<Platform>,
    arrows: Vec<ArrowPair>,
}

impl View {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(resources: &mut Resources, visible: Rect, state: &BlameState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            animation: Scene::empty(),
            platforms: vec![
                Platform::new(resources, (112,  88), state.get_position(0)),
                Platform::new(resources, (112, 120), state.get_position(1)),
                Platform::new(resources, (112, 152), state.get_position(2)),
                Platform::new(resources, (112, 184), state.get_position(3)),
                Platform::new(resources, (112, 216), state.get_position(4)),
                Platform::new(resources, (112, 248), state.get_position(5)),
                Platform::new(resources, (112, 280), state.get_position(6)),
            ],
            arrows: vec![
                ArrowPair::new(resources, (64, 112), 0, 3),
                ArrowPair::new(resources, (64, 144), 1, 5),
                ArrowPair::new(resources, (64, 176), 2, 3),
                ArrowPair::new(resources, (64, 208), 3, 7),
                ArrowPair::new(resources, (64, 240), 4, 5),
                ArrowPair::new(resources, (64, 272), 5, 3),
            ],
        };
        view.animation.begin(view.core.theater_mut());
        if state.is_visited() && !state.is_solved() {
            view.update_elinsa_position(state);
        }
        view
    }

    fn platform_top(&self, row: i32) -> i32 {
        assert!(row >= 0 && row < self.platforms.len() as i32);
        self.platforms[row as usize].top()
    }

    fn platform_pt(&self, state: &BlameState, row: i32) -> Point {
        if row < 0 {
            Point::new(416, 64)
        } else if row >= BlameState::num_rows() {
            Point::new(448, 304)
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

    fn update_elinsa_position(&mut self, state: &BlameState) {
        let position = self.platform_pt(state, state.get_elinsa_row());
        self.core.theater_mut().set_actor_position(ELINSA_SLOT, position);
    }

    fn update_platform_positions(&mut self, state: &BlameState) {
        for row in 0..BlameState::num_rows() {
            self.platforms[row as usize].set_position(state.get_position(row));
        }
    }

    fn shift_platform(&mut self, state: &mut BlameState, row_1: i32,
                      delta: i32) {
        let num_rows = BlameState::num_rows() as i32;
        let last_row = num_rows - 1;
        let max_pos_for_last_row = BlameState::max_position_for_row(last_row);
        debug_assert!(row_1 >= 0 && row_1 < num_rows - 1);
        let row_2 = row_1 + 1;
        let max_pos_1 = BlameState::max_position_for_row(row_1);
        let max_pos_2 = BlameState::max_position_for_row(row_2);
        let original_pos_1 = state.get_position(row_1);
        let original_pos_2 = state.get_position(row_2);
        let original_elinsa_row = state.get_elinsa_row();
        let mut top_seq: Vec<Box<SceneNode>> = Vec::new();

        // Move platforms:
        let mut elinsa_row = state.get_elinsa_row();
        let mut elinsa_pos = 0;
        if elinsa_row >= 0 && elinsa_row < num_rows {
            elinsa_pos = state.get_position(elinsa_row);
        }
        let mut delta_1 = delta;
        let mut delta_2 = delta;
        let mut pos_1 = state.get_position(row_1);
        let mut pos_2 = state.get_position(row_2);
        while delta_1 != 0 || delta_2 != 0 {
            let mut platform_1_seq: Vec<Box<SceneNode>> = Vec::new();
            let mut platform_2_seq: Vec<Box<SceneNode>> = Vec::new();
            let mut elinsa_seq: Vec<Box<SceneNode>> = Vec::new();
            if delta_1 < 0 && pos_1 == 0 || delta_1 > 0 && pos_1 == max_pos_1 {
                delta_1 = -delta_1;
            }
            if delta_2 < 0 && pos_2 == 0 || delta_2 > 0 && pos_2 == max_pos_2 {
                delta_2 = -delta_2;
            }
            debug_assert_eq!(delta_1.abs(), delta_2.abs());
            let mut step = delta_1.abs();
            if pos_1 + delta_1 < 0 {
                step = min(step, pos_1);
            }
            if pos_2 + delta_2 < 0 {
                step = min(step, pos_2);
            }
            if pos_1 + delta_1 > max_pos_1 {
                step = min(step, max_pos_1 - pos_1);
            }
            if pos_2 + delta_2 > max_pos_2 {
                step = min(step, max_pos_2 - pos_2);
            }
            debug_assert!(step > 0);
            let old_pos_1 = pos_1;
            let old_pos_2 = pos_2;
            if delta_1 < 0 {
                pos_1 -= step;
                delta_1 += step;
            } else {
                pos_1 += step;
                delta_1 -= step;
            }
            if delta_2 < 0 {
                pos_2 -= step;
                delta_2 += step;
            } else {
                pos_2 += step;
                delta_2 -= step;
            }
            let sound = Sound::platform_shift(step);
            let travel_time = Platform::travel_time(old_pos_1, pos_1);
            platform_1_seq.push(Box::new(SoundNode::new(sound)));
            platform_1_seq.push(Box::new(QueueNode::new((row_1, pos_1))));
            platform_1_seq.push(Box::new(WaitNode::new(travel_time)));
            platform_2_seq.push(Box::new(QueueNode::new((row_2, pos_2))));
            platform_2_seq.push(Box::new(WaitNode::new(travel_time)));
            let max_pos_for_row_5 = BlameState::max_position_for_row(5);
            state.set_position(row_1, pos_1);
            state.set_position(row_2, pos_2);

            // Move Elinsa and/or knock her downwards.
            if elinsa_row == row_1 || elinsa_row == row_2 {
                let impact = if elinsa_row == row_1 {
                    if row_1 == 5 && pos_1 == max_pos_for_row_5 {
                        Some(max_pos_for_row_5 - 1)
                    } else if row_1 > 0 {
                        let pos_0 = state.get_position(row_1 - 1);
                        if pos_1 > old_pos_1 && pos_0 > old_pos_1 &&
                            pos_0 <= pos_1
                        {
                            Some(pos_0 - 1)
                        } else if pos_1 < old_pos_1 && pos_0 < old_pos_1 &&
                                   pos_0 >= pos_1
                        {
                            Some(pos_0 + 1)
                        } else {
                            None
                        }
                    } else if pos_1 == 0 {
                        Some(1)
                    } else {
                        None
                    }
                } else {
                    debug_assert_eq!(elinsa_row, row_2);
                    if old_pos_1 < old_pos_2 && pos_1 >= pos_2 {
                        Some(old_pos_1 + (old_pos_2 - old_pos_1) / 2 + 1)
                    } else if old_pos_1 > old_pos_2 && pos_1 <= pos_2 {
                        Some(old_pos_2 + (old_pos_1 - old_pos_2 + 1) / 2 - 1)
                    } else if row_2 == 5 && pos_2 == max_pos_for_row_5 {
                        Some(max_pos_for_row_5 - 1)
                    } else {
                        None
                    }
                };
                let (row, pos) = if elinsa_row == row_1 {
                    (row_1, pos_1)
                } else {
                    (row_2, pos_2)
                };
                if let Some(new_elinsa_pos) = impact {
                    let mut time_to_hit =
                        Platform::travel_time(elinsa_pos, new_elinsa_pos);
                    elinsa_pos = new_elinsa_pos;
                    elinsa_row = state.fall_from(elinsa_row, elinsa_pos);
                    let mut slide_dest =
                        self.platform_pt_for_pos(row, elinsa_pos);
                    if row == row_2 && (old_pos_1 - old_pos_2).abs() % 2 == 0 {
                        slide_dest = slide_dest +
                            if old_pos_1 < old_pos_2 {
                                Point::new(-16, 0)
                            } else {
                                Point::new(16, 0)
                            };
                        time_to_hit += 0.5 * Platform::travel_time(0, 1);
                    }
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
                } else {
                    elinsa_pos = pos;
                    let dest = self.platform_pt_for_pos(row, elinsa_pos);
                    elinsa_seq.push(Box::new(SlideNode::new(ELINSA_SLOT,
                                                            dest,
                                                            false,
                                                            false,
                                                            travel_time)));
                }
            } else if elinsa_row < num_rows && elinsa_row == row_2 + 1 {
                let impact = if pos_2 > old_pos_2 && elinsa_pos > old_pos_2 &&
                    elinsa_pos <= pos_2
                {
                    Some((Platform::travel_time(old_pos_2, elinsa_pos - 1),
                          min(BlameState::max_position_for_row(elinsa_row),
                              elinsa_pos + 1)))
                } else if pos_2 < old_pos_2 && elinsa_pos >= pos_2 &&
                           elinsa_pos < old_pos_2
                {
                    Some((Platform::travel_time(old_pos_2, elinsa_pos + 1),
                          max(0, elinsa_pos - 1)))
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
                    let fall_dist = dest.y() - self.platform_top(row_2 + 1);
                    let time_to_fall = JumpNode::time_to_fall(fall_dist + 5) +
                        JumpNode::time_to_fall(5);
                    elinsa_seq.push(Box::new(WaitNode::new(time_to_hit)));
                    let sound = Sound::character_collision();
                    elinsa_seq.push(Box::new(SoundNode::new(sound)));
                    elinsa_seq.push(Box::new(JumpNode::new(ELINSA_SLOT,
                                                           dest,
                                                           time_to_fall)));
                }
            }
            top_seq.push(Box::new(ParallelNode::new(vec![
                Box::new(SequenceNode::new(platform_1_seq)),
                Box::new(SequenceNode::new(platform_2_seq)),
                Box::new(SequenceNode::new(elinsa_seq)),
            ])));
        }

        // If Elinsa fell to the floor, get back on the starting platform:
        if elinsa_row == num_rows && original_elinsa_row != num_rows {
            let time = 0.5 *
                Platform::travel_time(elinsa_pos, max_pos_for_last_row);
            let dest = Point::new(408, 320);
            let slide = SlideNode::new(ELINSA_SLOT, dest, false, false, time);
            top_seq.push(Box::new(slide));
            let dest = Point::new(424, 304);
            let slide = SlideNode::new(ELINSA_SLOT, dest, false, false, 0.1);
            top_seq.push(Box::new(slide));
            let dest = self.platform_pt(state, elinsa_row);
            let slide = SlideNode::new(ELINSA_SLOT, dest, false, false, 0.1);
            top_seq.push(Box::new(slide));
        }

        // Make Elinsa climb upwards:
        if elinsa_row == num_rows &&
            state.get_position(last_row) == max_pos_for_last_row &&
            state.get_position(last_row - 1) != max_pos_for_last_row
        {
            elinsa_row -= 1;
            let dest = self.platform_pt(state, elinsa_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(ELINSA_SLOT, dest, 0.6)));
        }
        if elinsa_row < num_rows {
            while elinsa_row > 0 {
                let pos_0 = state.get_position(elinsa_row);
                let pos_1 = state.get_position(elinsa_row - 1);
                if (pos_0 - pos_1).abs() != 1 {
                    break;
                }
                if elinsa_row >= 2 {
                    let pos_2 = state.get_position(elinsa_row - 2);
                    if pos_2 == pos_1 || pos_2 == pos_0 {
                        break;
                    }
                }
                if elinsa_row == last_row &&
                    (pos_0 == max_pos_for_last_row ||
                         pos_1 == max_pos_for_last_row) ||
                    elinsa_row == 1 && (pos_0 == 0 || pos_1 == 0)
                {
                    break;
                }
                elinsa_row -= 1;
                let dest = self.platform_pt(state, elinsa_row);
                top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
                top_seq.push(Box::new(JumpNode::new(ELINSA_SLOT, dest, 0.6)));
            }
        }
        if elinsa_row == 0 &&
            state.get_position(0) == BlameState::max_position_for_row(0)
        {
            elinsa_row -= 1;
            let dest = self.platform_pt(state, elinsa_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(ELINSA_SLOT, dest, 0.6)));
        }
        state.set_elinsa_row(elinsa_row);
        self.core.push_undo((row_1,
                             pos_1 - original_pos_1,
                             pos_2 - original_pos_2,
                             elinsa_row - original_elinsa_row));

        // Start animation:
        self.animation =
            Scene::new(vec![Box::new(SequenceNode::new(top_seq))]);
        self.animation.begin(self.core.theater_mut());
        self.drain_queue();
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.shift_the_blame;
        self.core.draw_back_layer(canvas);
        self.arrows.draw(&(), canvas);
        self.platforms.draw(&(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.shift_the_blame;
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
            (event == &Event::ClockTick || !state.is_solved())
        {
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
        if game.shift_the_blame.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((row_1, dx_1, dx_2, dy)) = self.core.pop_undo() {
            self.animation = Scene::empty();
            let state = &mut game.shift_the_blame;
            let row_2 = row_1 + 1;
            let new_pos_1 = state.get_position(row_1) - dx_1;
            let new_pos_2 = state.get_position(row_2) - dx_2;
            let new_elinsa = state.get_elinsa_row() - dy;
            state.set_position(row_1, new_pos_1);
            state.set_position(row_2, new_pos_2);
            self.platforms[row_1 as usize].set_position(new_pos_1);
            self.platforms[row_2 as usize].set_position(new_pos_2);
            state.set_elinsa_row(new_elinsa);
            self.update_elinsa_position(state);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((row_1, dx_1, dx_2, dy)) = self.core.pop_redo() {
            let state = &mut game.shift_the_blame;
            let row_2 = row_1 + 1;
            let new_pos_1 = state.get_position(row_1) + dx_1;
            let new_pos_2 = state.get_position(row_2) + dx_2;
            let new_elinsa = state.get_elinsa_row() + dy;
            state.set_position(row_1, new_pos_1);
            state.set_position(row_2, new_pos_2);
            self.platforms[row_1 as usize].set_position(new_pos_1);
            self.platforms[row_2 as usize].set_position(new_pos_2);
            state.set_elinsa_row(new_elinsa);
            self.update_elinsa_position(state);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.shift_the_blame;
        self.core.clear_undo_redo();
        state.reset();
        self.update_platform_positions(state);
        self.update_elinsa_position(state);
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.shift_the_blame;
        state.solve();
        self.update_platform_positions(state);
        self.update_elinsa_position(state);
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (row, pos) in self.core.drain_queue() {
            if row >= 0 && row < self.platforms.len() as i32 {
                self.platforms[row as usize].set_goal(pos);
            } else if row == -2 {
                for platform in self.platforms.iter_mut() {
                    platform.move_to_goal();
                }
            }
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &str = "\
Your goal is to help Elinsa reach the ledge in the
upper-right corner.

$M{Tap}{Click} the arrow buttons to move the platforms
to the left or to the right.

Elinsa will jump up onto the next platform when
it is adjacent to her.";

// ========================================================================= //
