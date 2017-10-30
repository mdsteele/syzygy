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
use save::{Game, GroundState, PuzzleState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32, i32)>,
    animation: Scene,
    platforms: Vec<Platform>,
    arrows: Vec<ArrowPair>,
    platforms_and_arrows_visible: bool,
}

impl View {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new(resources: &mut Resources, visible: Rect, state: &GroundState)
               -> View {
        let mut core = {
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, intro, outro)
        };
        core.add_extra_scene(scenes::compile_elinsa_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        let mut view = View {
            core: core,
            animation: Scene::empty(),
            platforms: vec![
                Platform::new(resources, (160, 88), state.get_position(0)),
                Platform::new(resources, (160, 120), state.get_position(1)),
                Platform::new(resources, (160, 152), state.get_position(2)),
                Platform::new(resources, (160, 184), state.get_position(3)),
                Platform::new(resources, (160, 216), state.get_position(4)),
                Platform::new(resources, (160, 248), state.get_position(5)),
                Platform::new(resources, (160, 280), state.get_position(6)),
            ],
            arrows: vec![
                ArrowPair::new(resources, (480, 96), 0, 8),
                ArrowPair::new(resources, (480, 128), 1, 5),
                ArrowPair::new(resources, (480, 160), 2, 6),
                ArrowPair::new(resources, (480, 192), 3, 7),
                ArrowPair::new(resources, (480, 224), 4, 5),
                ArrowPair::new(resources, (480, 256), 5, 3),
                ArrowPair::new(resources, (480, 288), 6, 4),
            ],
            platforms_and_arrows_visible: true,
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

    fn platform_pt(&self, state: &GroundState, row: i32) -> Point {
        if row < 0 {
            Point::new(496, 64)
        } else if row >= GroundState::num_rows() {
            Point::new(144, 304)
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

    fn update_elinsa_position(&mut self, state: &GroundState) {
        let position = self.platform_pt(state, state.get_elinsa_row());
        self.core.theater_mut().set_actor_position(scenes::ELINSA, position);
    }

    fn update_platform_positions(&mut self, state: &GroundState) {
        for row in 0..GroundState::num_rows() {
            self.platforms[row as usize].set_position(state.get_position(row));
        }
    }

    fn shift_platform(&mut self, state: &mut GroundState, row: i32,
                      mut delta: i32) {
        let max_pos = GroundState::max_position();
        let num_rows = GroundState::num_rows() as i32;
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
            if pos < 0 {
                debug_assert!(delta < 0);
                pos = 0;
                delta = -(delta + old_pos);
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
                row == elinsa_row - 1
            {
                let impact = if pos > old_pos && elinsa_pos > old_pos &&
                    elinsa_pos <= pos
                {
                    Some((Platform::travel_time(old_pos, elinsa_pos - 1),
                          cmp::min(max_pos, elinsa_pos + 1)))
                } else if pos < old_pos && elinsa_pos >= pos &&
                           elinsa_pos < old_pos
                {
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
                    elinsa_seq.push(Box::new(JumpNode::new(scenes::ELINSA,
                                                           dest,
                                                           time_to_fall)));
                }
            } else if row == elinsa_row {
                let impact = if row > 0 {
                    let barrier_pos = state.get_position(row - 1);
                    if pos > old_pos && barrier_pos > old_pos &&
                        barrier_pos <= pos
                    {
                        Some(barrier_pos - 1)
                    } else if pos < old_pos && barrier_pos < old_pos &&
                               barrier_pos >= pos
                    {
                        Some(barrier_pos + 1)
                    } else {
                        None
                    }
                } else {
                    None
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
                    elinsa_seq.push(Box::new(SlideNode::new(scenes::ELINSA,
                                                            slide_dest,
                                                            false,
                                                            false,
                                                            time_to_hit)));
                    let sound = Sound::character_collision();
                    elinsa_seq.push(Box::new(SoundNode::new(sound)));
                    elinsa_seq.push(Box::new(JumpNode::new(scenes::ELINSA,
                                                           jump_dest,
                                                           time_to_fall)));
                } else {
                    elinsa_pos = pos;
                    let dest = self.platform_pt_for_pos(row, elinsa_pos);
                    elinsa_seq.push(Box::new(SlideNode::new(scenes::ELINSA,
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
            let slide_time = 0.5 * Platform::travel_time(elinsa_pos, 0);
            top_seq.push(Box::new(SlideNode::new(scenes::ELINSA,
                                                 self.floor_pt_for_pos(0),
                                                 false,
                                                 false,
                                                 slide_time)));
            let dest = self.platform_pt(state, elinsa_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(scenes::ELINSA, dest, 0.5)));
        }

        // Make Elinsa climb upwards:
        if elinsa_row == num_rows && state.get_position(num_rows - 1) == 0 &&
            state.get_position(num_rows - 2) != 0
        {
            elinsa_row -= 1;
            let dest = self.platform_pt(state, elinsa_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(scenes::ELINSA, dest, 0.6)));
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
                elinsa_row -= 1;
                let dest = self.platform_pt(state, elinsa_row);
                top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
                top_seq
                    .push(Box::new(JumpNode::new(scenes::ELINSA, dest, 0.6)));
            }
        }
        if elinsa_row == 0 && state.get_position(0) == max_pos {
            elinsa_row -= 1;
            let dest = self.platform_pt(state, elinsa_row);
            top_seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            top_seq.push(Box::new(JumpNode::new(scenes::ELINSA, dest, 0.6)));
        }
        state.set_elinsa_row(elinsa_row);
        self.core.push_undo((row,
                             pos - original_position,
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
        let state = &game.shifting_ground;
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
        let state = &mut game.shifting_ground;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() && self.platforms_and_arrows_visible {
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
        if game.shifting_ground.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((row, dx, dy)) = self.core.pop_undo() {
            self.animation = Scene::empty();
            let state = &mut game.shifting_ground;
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
            let state = &mut game.shifting_ground;
            let new_pos = state.get_position(row) + dx;
            let new_elinsa = state.get_elinsa_row() + dy;
            state.set_position(row, new_pos);
            self.platforms[row as usize].set_position(new_pos);
            state.set_elinsa_row(new_elinsa);
            self.update_elinsa_position(state);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.shifting_ground;
        self.core.clear_undo_redo();
        state.reset();
        self.update_platform_positions(state);
        self.update_elinsa_position(state);
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.shifting_ground;
        state.solve();
        self.update_platform_positions(state);
        self.update_elinsa_position(state);
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (row, pos) in self.core.drain_queue() {
            if row >= 0 && row < self.platforms.len() as i32 {
                self.platforms[row as usize].set_goal(pos);
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
Your goal is to help Elinsa reach the ledge in the
upper-right corner.

$M{Tap}{Click} the arrow buttons to move a platform to
the left or to the right.

Elinsa will jump up onto the next platform when
it is adjacent to her.";

// ========================================================================= //
