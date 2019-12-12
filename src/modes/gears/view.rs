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

use crate::elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView, Scene};
use crate::elements::cutscene::{JumpNode, ParallelNode, QueueNode, SceneNode,
                         SequenceNode, SlideNode, SoundNode, WaitNode};
use crate::elements::shift::{ArrowPair, Platform};
use crate::gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sound};
use crate::modes::SOLVED_INFO_TEXT;
use crate::save::{Game, GearsState, PuzzleState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32, i32)>,
    animation: Scene,
    animating: bool,
    platforms: Vec<Platform>,
    arrows: Vec<ArrowPair>,
    platforms_and_arrows_visible: bool,
}

impl View {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(resources: &mut Resources, visible: Rect, state: &GearsState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::BottomToTop);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_ugrent_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        let mut view = View {
            core: core,
            animation: Scene::empty(),
            animating: false,
            platforms: vec![
                Platform::new(resources, (128, 88), state.get_position(0)),
                Platform::new(resources, (128, 120), state.get_position(1)),
                Platform::new(resources, (128, 152), state.get_position(2)),
                Platform::new(resources, (128, 184), state.get_position(3)),
                Platform::new(resources, (128, 184), state.get_position(4)),
                Platform::new(resources, (128, 216), state.get_position(5)),
                Platform::new(resources, (128, 248), state.get_position(6)),
                Platform::new(resources, (128, 280), state.get_position(7)),
            ],
            arrows: vec![
                ArrowPair::new(resources, (80, 96), 0, 3),
                ArrowPair::new(resources, (80, 128), 1, 3),
                ArrowPair::new(resources, (80, 160), 2, 6),
                ArrowPair::new(resources, (80, 192), 3, 3),
                ArrowPair::new(resources, (480, 192), 4, 2),
                ArrowPair::new(resources, (480, 224), 5, 7),
                ArrowPair::new(resources, (480, 256), 6, 5),
                ArrowPair::new(resources, (480, 288), 7, 3),
            ],
            platforms_and_arrows_visible: true,
        };
        view.animation.begin(view.core.theater_mut());
        if state.is_visited() && !state.is_solved() {
            view.update_ugrent_position(state);
        }
        view
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
            Point::new(496, 64)
        } else if row >= GearsState::num_rows() {
            Point::new(112, 304)
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

    fn update_ugrent_position(&mut self, state: &GearsState) {
        let row = state.get_ugrent_row();
        let position = self.platform_pt(state, row);
        self.core.theater_mut().set_actor_position(scenes::UGRENT, position);
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
        let original_ugrent_row = state.get_ugrent_row();
        let mut top_seq: Vec<Box<dyn SceneNode>> = Vec::new();

        // Move platform:
        let mut ugrent_row = state.get_ugrent_row();
        let mut ugrent_pos = 0;
        if ugrent_row >= 0 && ugrent_row < num_rows {
            ugrent_pos = state.get_position(ugrent_row);
        }
        let mut pos = state.get_position(row);
        while delta != 0 {
            let mut platform_seq: Vec<Box<dyn SceneNode>> = Vec::new();
            let mut ugrent_seq: Vec<Box<dyn SceneNode>> = Vec::new();
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
            if ugrent_row >= 0 && ugrent_row < num_rows &&
                (row == ugrent_row - 1 ||
                     ((row == 2 || row == 3) && row == ugrent_row - 2))
            {
                let impact = if pos > old_pos && ugrent_pos > old_pos &&
                    ugrent_pos <= pos
                {
                    Some((Platform::travel_time(old_pos, ugrent_pos - 1),
                          cmp::min(10, ugrent_pos + 1)))
                } else if pos < old_pos && ugrent_pos >= pos &&
                           ugrent_pos < old_pos
                {
                    Some((Platform::travel_time(old_pos, ugrent_pos + 1),
                          cmp::max(0, ugrent_pos - 1)))
                } else {
                    None
                };
                if let Some((time_to_hit, new_ugrent_pos)) = impact {
                    ugrent_pos = new_ugrent_pos;
                    ugrent_row = state.fall_from(ugrent_row, ugrent_pos);
                    let dest = if ugrent_row < num_rows {
                        self.platform_pt_for_pos(ugrent_row, ugrent_pos)
                    } else {
                        self.floor_pt_for_pos(ugrent_pos)
                    };
                    let fall_dist = dest.y() - self.platform_top(row + 1);
                    let time_to_fall = JumpNode::time_to_fall(fall_dist + 5) +
                        JumpNode::time_to_fall(5);
                    ugrent_seq.push(Box::new(WaitNode::new(time_to_hit)));
                    let sound = Sound::character_collision();
                    ugrent_seq.push(Box::new(SoundNode::new(sound)));
                    ugrent_seq.push(Box::new(JumpNode::new(scenes::UGRENT,
                                                           dest,
                                                           time_to_fall)));
                    ugrent_seq
                        .push(Box::new(QueueNode::new((ugrent_row, -1))));
                }
            } else if row == ugrent_row {
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
                            barrier_pos_2 < barrier_pos_1
                        {
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
                            barrier_pos_2 > barrier_pos_1
                        {
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
                if let Some(new_ugrent_pos) = impact {
                    let time_to_hit = Platform::travel_time(ugrent_pos,
                                                            new_ugrent_pos);
                    ugrent_pos = new_ugrent_pos;
                    ugrent_row = state.fall_from(ugrent_row, ugrent_pos);
                    let slide_dest = self.platform_pt_for_pos(row, ugrent_pos);
                    let jump_dest = if ugrent_row < num_rows {
                        self.platform_pt_for_pos(ugrent_row, ugrent_pos)
                    } else {
                        self.floor_pt_for_pos(ugrent_pos)
                    };
                    let fall_dist = jump_dest.y() - self.platform_top(row);
                    let time_to_fall = JumpNode::time_to_fall(fall_dist + 5) +
                        JumpNode::time_to_fall(5);
                    ugrent_seq.push(Box::new(SlideNode::new(scenes::UGRENT,
                                                            slide_dest,
                                                            false,
                                                            false,
                                                            time_to_hit)));
                    let sound = Sound::character_collision();
                    ugrent_seq.push(Box::new(SoundNode::new(sound)));
                    ugrent_seq.push(Box::new(JumpNode::new(scenes::UGRENT,
                                                           jump_dest,
                                                           time_to_fall)));
                    ugrent_seq
                        .push(Box::new(QueueNode::new((ugrent_row, -1))));
                } else {
                    ugrent_pos = pos;
                    let dest = self.platform_pt_for_pos(row, ugrent_pos);
                    ugrent_seq.push(Box::new(SlideNode::new(scenes::UGRENT,
                                                            dest,
                                                            false,
                                                            false,
                                                            travel_time)));
                }
            }
            top_seq.push(Box::new(ParallelNode::new(vec![
                Box::new(SequenceNode::new(platform_seq)),
                Box::new(SequenceNode::new(ugrent_seq)),
            ])));
        }
        state.set_position(row, pos);

        // If Ugrent fell to the floor, get back on the starting platform:
        if ugrent_row == num_rows && original_ugrent_row != num_rows {
            let slide_time = 0.5 * Platform::travel_time(ugrent_pos, 0);
            top_seq.push(Box::new(SlideNode::new(scenes::UGRENT,
                                                 self.floor_pt_for_pos(0),
                                                 false,
                                                 false,
                                                 slide_time)));
            let dest = self.platform_pt(state, ugrent_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(scenes::UGRENT, dest, 0.5)));
        }

        // Make Ugrent climb upwards:
        let last_row = num_rows - 1;
        let min_pos_for_last_row = GearsState::min_position_for_row(last_row);
        if ugrent_row == num_rows &&
            state.get_position(last_row) == min_pos_for_last_row &&
            state.get_position(last_row - 1) != min_pos_for_last_row
        {
            ugrent_row -= 1;
            let dest = self.platform_pt(state, ugrent_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(scenes::UGRENT, dest, 0.6)));
            top_seq.push(Box::new(QueueNode::new((ugrent_row, -1))));
        }
        if ugrent_row < num_rows {
            while ugrent_row > 0 {
                let ugrent_pos = state.get_position(ugrent_row);
                let pos_1 = state.get_position(ugrent_row - 1);
                let pos_2 = if ugrent_row > 1 {
                    state.get_position(ugrent_row - 2)
                } else {
                    5
                };
                let mut next_row = ugrent_row;
                if (ugrent_pos - pos_1).abs() == 1 && pos_2 != pos_1 &&
                    pos_2 != ugrent_pos &&
                    ((ugrent_row != 5 && ugrent_row != 6) ||
                         state.get_position(ugrent_row - 3) != pos_1)
                {
                    next_row = ugrent_row - 1;
                } else if (ugrent_row == 4 || ugrent_row == 5) &&
                           (ugrent_pos - pos_2).abs() == 1
                {
                    let pos_3 = state.get_position(ugrent_row - 3);
                    if pos_3 != pos_2 && pos_3 != ugrent_pos {
                        next_row = ugrent_row - 2;
                    }
                }
                if next_row == ugrent_row {
                    break;
                }
                ugrent_row = next_row;
                let dest = self.platform_pt(state, ugrent_row);
                top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
                top_seq
                    .push(Box::new(JumpNode::new(scenes::UGRENT, dest, 0.6)));
                top_seq.push(Box::new(QueueNode::new((ugrent_row, -1))));
            }
        }
        if ugrent_row == 0 &&
            state.get_position(0) == GearsState::max_position_for_row(0)
        {
            ugrent_row -= 1;
            let dest = self.platform_pt(state, ugrent_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(scenes::UGRENT, dest, 0.6)));
            top_seq.push(Box::new(QueueNode::new((ugrent_row, -1))));
        }
        state.set_ugrent_row(ugrent_row);
        self.core.push_undo((row,
                             pos - original_position,
                             ugrent_row - original_ugrent_row));

        // Start animation:
        self.animation =
            Scene::new(vec![Box::new(SequenceNode::new(top_seq))]);
        self.animation.begin(self.core.theater_mut());
        self.animating = true;
        self.drain_queue();
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.shift_gears;
        self.core.draw_back_layer(canvas);
        if self.platforms_and_arrows_visible {
            self.arrows.draw(&(), canvas);
            self.platforms.draw(&(), canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.shift_gears;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() && self.platforms_and_arrows_visible {
            let subaction = self.platforms.handle_event(event, &mut ());
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() && self.animating {
            let subaction = self.animation
                .handle_event(event, self.core.theater_mut());
            action.merge(subaction.but_no_value());
            self.drain_queue();
            if self.animation.is_finished() {
                self.animating = false;
                if state.is_solved() {
                    self.core.begin_outro_scene();
                    action = action.and_return(PuzzleCmd::Save);
                }
            }
        }
        if !action.should_stop() && self.platforms_and_arrows_visible &&
            (event == &Event::ClockTick || !state.is_solved())
        {
            let subaction = self.arrows.handle_event(event, &mut ());
            if let Some(&(row, delta)) = subaction.value() {
                self.shift_platform(state, row, delta);
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            self.core.begin_character_scene_on_click(event);
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
            self.animation = Scene::empty();
            self.animating = false;
            let state = &mut game.shift_gears;
            let new_pos = state.get_position(row) - dx;
            let new_ugrent = state.get_ugrent_row() - dy;
            state.set_position(row, new_pos);
            self.platforms[row as usize].set_position(new_pos);
            state.set_ugrent_row(new_ugrent);
            self.update_ugrent_position(state);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((row, dx, dy)) = self.core.pop_redo() {
            let state = &mut game.shift_gears;
            let new_pos = state.get_position(row) + dx;
            let new_ugrent = state.get_ugrent_row() + dy;
            state.set_position(row, new_pos);
            self.platforms[row as usize].set_position(new_pos);
            state.set_ugrent_row(new_ugrent);
            self.update_ugrent_position(state);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.shift_gears;
        self.core.clear_undo_redo();
        state.reset();
        self.update_platform_positions(state);
        self.update_ugrent_position(state);
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.shift_gears;
        state.solve();
        self.update_platform_positions(state);
        self.update_ugrent_position(state);
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (row, pos) in self.core.drain_queue() {
            if pos < 0 {
                self.set_override_row(row);
            } else if row >= 0 && row < self.platforms.len() as i32 {
                let lower = GearsState::min_position_for_row(row);
                let upper = GearsState::max_position_for_row(row);
                let goal = cmp::min(cmp::max(lower, pos), upper);
                self.platforms[row as usize].set_goal(goal);
            } else if row == -1 {
                self.platforms_and_arrows_visible = pos != 0;
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
Your goal is to help Ugrent reach the ledge in the
upper-right corner.

$M{Tap}{Click} the arrow buttons to move a platform to
the left or to the right.  If Ugrent is standing on
that platform, it will move by 4 instead of by
its normal distance.

Ugrent will jump up onto the next platform
when it is adjacent.

$M{Tap}{Click} on a character in the scene to hear their
words of wisdom.";

// ========================================================================= //
