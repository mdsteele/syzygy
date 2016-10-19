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

use gui::{Action, Canvas, Element, Event, FRAME_DELAY_MILLIS, Point, Sprite};
use super::theater::Theater;

// ========================================================================= //

pub struct Scene {
    nodes: Vec<Box<SceneNode>>,
    index: usize,
}

impl Scene {
    pub fn new(nodes: Vec<Box<SceneNode>>) -> Scene {
        Scene {
            nodes: nodes,
            index: 0,
        }
    }

    pub fn begin(&mut self, theater: &mut Theater) {
        if !self.nodes.is_empty() {
            self.nodes[0].begin(theater);
        }
    }

    pub fn tick(&mut self, theater: &mut Theater) {
        if self.index < self.nodes.len() {
            self.nodes[self.index].tick(theater, true, false);
            while self.nodes[self.index].status() == Status::Done {
                self.index += 1;
                if self.index < self.nodes.len() {
                    self.nodes[self.index].begin(theater);
                } else {
                    break;
                }
            }
        }
    }

    pub fn skip(&mut self, theater: &mut Theater) {
        while self.index < self.nodes.len() {
            self.nodes[self.index].skip(theater);
            self.index += 1;
        }
    }

    pub fn is_finished(&self) -> bool { self.index == self.nodes.len() }

    pub fn is_paused(&self) -> bool {
        self.index < self.nodes.len() &&
        self.nodes[self.index].status() == Status::Paused
    }

    pub fn unpause(&mut self) {
        if self.index < self.nodes.len() {
            self.nodes[self.index].unpause();
        }
    }
}

impl Element<Theater, ()> for Scene {
    fn draw(&self, theater: &Theater, canvas: &mut Canvas) {
        theater.draw_background(canvas);
        theater.draw_foreground(canvas);
    }

    fn handle_event(&mut self, event: &Event, theater: &mut Theater)
                    -> Action<()> {
        match event {
            &Event::Quit => Action::ignore(),
            &Event::ClockTick => {
                self.tick(theater);
                Action::redraw()
            }
            &Event::MouseDown(_) if self.is_paused() => {
                self.unpause();
                Action::redraw().and_stop()
            }
            _ => {
                if self.is_finished() {
                    Action::ignore()
                } else {
                    Action::ignore().and_stop()
                }
            }
        }

    }
}

// ========================================================================= //

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Status {
    Active,
    Paused,
    Twiddling,
    Done,
}

// ========================================================================= //

pub trait SceneNode {
    fn status(&self) -> Status { Status::Done }

    fn begin(&mut self, _theater: &mut Theater) {}

    fn tick(&mut self, _theater: &mut Theater, _terminated_by_pause: bool,
            _keep_twiddling: bool) {
    }

    fn skip(&mut self, _theater: &mut Theater) {}

    fn reset(&mut self) {}

    fn unpause(&mut self) {}
}

// ========================================================================= //

pub struct SequenceNode {
    nodes: Vec<Box<SceneNode>>,
    index: usize,
}

impl SequenceNode {
    pub fn new(nodes: Vec<Box<SceneNode>>) -> SequenceNode {
        SequenceNode {
            nodes: nodes,
            index: 0,
        }
    }
}

impl SceneNode for SequenceNode {
    fn status(&self) -> Status {
        if self.index + 1 < self.nodes.len() {
            Status::Active
        } else if self.index < self.nodes.len() {
            self.nodes[self.index].status()
        } else {
            Status::Done
        }
    }

    fn begin(&mut self, theater: &mut Theater) {
        if !self.nodes.is_empty() {
            self.nodes[0].begin(theater);
        }
    }

    fn tick(&mut self, theater: &mut Theater, terminated_by_pause: bool,
            keep_twiddling: bool) {
        if self.index < self.nodes.len() {
            let last = self.index + 1 == self.nodes.len();
            self.nodes[self.index].tick(theater,
                                        terminated_by_pause && last,
                                        keep_twiddling && last);
            while self.nodes[self.index].status() == Status::Done {
                self.index += 1;
                if self.index < self.nodes.len() {
                    self.nodes[self.index].begin(theater);
                } else {
                    break;
                }
            }
        }
    }

    fn skip(&mut self, theater: &mut Theater) {
        while self.index < self.nodes.len() {
            self.nodes[self.index].skip(theater);
            self.index += 1;
        }
    }

    fn reset(&mut self) {
        for node in self.nodes.iter_mut() {
            node.reset();
        }
        self.index = 0;
    }

    fn unpause(&mut self) {
        if self.index < self.nodes.len() {
            self.nodes[self.index].unpause();
        }
    }
}

// ========================================================================= //

pub struct ParallelNode {
    nodes: Vec<Box<SceneNode>>,
}

impl ParallelNode {
    pub fn new(nodes: Vec<Box<SceneNode>>) -> ParallelNode {
        ParallelNode { nodes: nodes }
    }
}

impl SceneNode for ParallelNode {
    fn status(&self) -> Status {
        let mut status = Status::Done;
        for node in self.nodes.iter() {
            status = cmp::min(status, node.status());
        }
        status
    }

    fn begin(&mut self, theater: &mut Theater) {
        if cfg!(debug_assertions) {
            println!("Beginning cutscene.");
        }
        for node in self.nodes.iter_mut() {
            node.begin(theater);
        }
    }

    fn tick(&mut self, theater: &mut Theater, terminated_by_pause: bool,
            mut keep_twiddling: bool) {
        if !keep_twiddling {
            for node in self.nodes.iter() {
                if node.status() <= Status::Paused {
                    keep_twiddling = true;
                    break;
                }
            }
        }
        for node in self.nodes.iter_mut() {
            node.tick(theater, terminated_by_pause, keep_twiddling);
        }
    }

    fn skip(&mut self, theater: &mut Theater) {
        if cfg!(debug_assertions) {
            println!("Skipping cutscene.");
        }
        for node in self.nodes.iter_mut() {
            node.skip(theater);
        }
    }

    fn reset(&mut self) {
        for node in self.nodes.iter_mut() {
            node.reset();
        }
    }

    fn unpause(&mut self) {
        for node in self.nodes.iter_mut() {
            node.unpause();
        }
    }
}

// ========================================================================= //

pub struct LoopNode {
    node: Box<SceneNode>,
    min_iterations: i32,
    max_iterations: Option<i32>,
    iteration: i32,
}

impl LoopNode {
    pub fn new(node: Box<SceneNode>, min_iterations: i32,
               max_iterations: Option<i32>)
               -> LoopNode {
        LoopNode {
            node: node,
            min_iterations: min_iterations,
            max_iterations: max_iterations,
            iteration: 0,
        }
    }

    fn can_continue(&self) -> bool {
        if let Some(max) = self.max_iterations {
            self.iteration < max
        } else {
            true
        }
    }
}

impl SceneNode for LoopNode {
    fn status(&self) -> Status {
        if self.iteration < self.min_iterations {
            Status::Active
        } else if self.node.status() == Status::Active {
            Status::Twiddling
        } else {
            Status::Done
        }
    }

    fn begin(&mut self, theater: &mut Theater) { self.node.begin(theater); }

    fn tick(&mut self, theater: &mut Theater, _: bool, keep_twiddling: bool) {
        if self.node.status() == Status::Active {
            self.node.tick(theater, false, false);
            if self.node.status() == Status::Done {
                if self.iteration < self.min_iterations ||
                   self.max_iterations.is_some() {
                    self.iteration += 1;
                }
                if self.iteration < self.min_iterations ||
                   (keep_twiddling && self.can_continue()) {
                    self.node.reset();
                    self.node.begin(theater);
                }
            }
        }
    }

    fn skip(&mut self, theater: &mut Theater) {
        self.node.skip(theater);
        self.iteration = self.min_iterations;
    }

    fn reset(&mut self) {
        self.node.reset();
        self.iteration = 0;
    }

    fn unpause(&mut self) { self.node.unpause(); }
}

// ========================================================================= //

pub struct DarkNode {
    dark: bool,
}

impl DarkNode {
    pub fn new(dark: bool) -> DarkNode { DarkNode { dark: dark } }
}

impl SceneNode for DarkNode {
    fn begin(&mut self, theater: &mut Theater) { theater.set_dark(self.dark); }

    fn skip(&mut self, theater: &mut Theater) { self.begin(theater); }
}

// ========================================================================= //

pub struct LightNode {
    slot: i32,
    light: Option<Sprite>,
}

impl LightNode {
    pub fn new(slot: i32, light: Option<Sprite>) -> LightNode {
        LightNode {
            slot: slot,
            light: light,
        }
    }
}

impl SceneNode for LightNode {
    fn begin(&mut self, theater: &mut Theater) {
        theater.set_actor_light(self.slot, self.light.clone());
    }

    fn skip(&mut self, theater: &mut Theater) { self.begin(theater); }
}

// ========================================================================= //

pub struct PlaceNode {
    slot: i32,
    sprite: Sprite,
    position: Point,
}

impl PlaceNode {
    pub fn new(slot: i32, sprite: Sprite, position: Point) -> PlaceNode {
        PlaceNode {
            slot: slot,
            sprite: sprite,
            position: position,
        }
    }
}

impl SceneNode for PlaceNode {
    fn begin(&mut self, theater: &mut Theater) {
        theater.place_actor(self.slot, self.sprite.clone(), self.position);
    }

    fn skip(&mut self, theater: &mut Theater) { self.begin(theater); }
}

// ========================================================================= //

pub struct SlideNode {
    progress: i32,
    duration: i32,
    slot: i32,
    start: Point,
    end: Point,
    accel: bool,
    decel: bool,
}

impl SlideNode {
    pub fn new(slot: i32, end: Point, accel: bool, decel: bool,
               duration_seconds: f64)
               -> SlideNode {
        SlideNode {
            progress: 0,
            duration: seconds_to_frames(duration_seconds),
            slot: slot,
            start: end,
            end: end,
            accel: accel,
            decel: decel,
        }
    }
}

impl SceneNode for SlideNode {
    fn status(&self) -> Status {
        if self.progress < self.duration {
            Status::Active
        } else {
            Status::Done
        }
    }

    fn begin(&mut self, theater: &mut Theater) {
        if let Some(position) = theater.get_actor_position(self.slot) {
            self.start = position;
        }
    }

    fn tick(&mut self, theater: &mut Theater, _: bool, _: bool) {
        if self.progress < self.duration {
            self.progress += 1;
            let param = self.progress as f64 / self.duration as f64;
            let frac = if self.accel {
                if self.decel {
                    if param < 0.5 {
                        2.0 * param * param
                    } else {
                        1.0 - 2.0 * (1.0 - param) * (1.0 - param)
                    }
                } else {
                    param * param
                }
            } else {
                if self.decel {
                    1.0 - (1.0 - param) * (1.0 - param)
                } else {
                    param
                }
            };
            let delta = self.end - self.start;
            let delta = Point::new((delta.x() as f64 * frac).round() as i32,
                                   (delta.y() as f64 * frac).round() as i32);
            theater.set_actor_position(self.slot, self.start + delta);
        }
    }

    fn skip(&mut self, theater: &mut Theater) {
        theater.set_actor_position(self.slot, self.end);
    }

    fn reset(&mut self) { self.progress = 0; }
}

// ========================================================================= //

pub struct WaitNode {
    progress: i32,
    duration: i32,
}

impl WaitNode {
    pub fn new(duration_seconds: f64) -> WaitNode {
        WaitNode {
            progress: 0,
            duration: seconds_to_frames(duration_seconds),
        }
    }
}

impl SceneNode for WaitNode {
    fn status(&self) -> Status {
        if self.progress < self.duration {
            Status::Active
        } else {
            Status::Done
        }
    }

    fn tick(&mut self, _: &mut Theater, _: bool, _: bool) {
        if self.progress < self.duration {
            self.progress += 1;
        }
    }

    fn reset(&mut self) { self.progress = 0; }
}

// ========================================================================= //

fn seconds_to_frames(seconds: f64) -> i32 {
    (seconds / (0.001 * FRAME_DELAY_MILLIS as f64)).floor() as i32
}

// ========================================================================= //
