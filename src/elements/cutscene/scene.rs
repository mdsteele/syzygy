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
use std::rc::Rc;

use elements::Paragraph;
use gui::{Action, Background, Canvas, Element, Event, FRAME_DELAY_MILLIS,
          Keycode, Point, Sound, Sprite};
use super::theater::{TalkPos, Theater};

// ========================================================================= //

const CLICKS_TO_SHOW_SKIP: i32 = 3;
const FRAMES_BETWEEN_CLICKS: i32 = 10;
const FRAMES_TO_HIDE_SKIP: i32 = 50;

// ========================================================================= //

#[derive(Clone)]
pub struct Scene {
    nodes: Vec<Box<SceneNode>>,
    index: usize,
    began: bool,
    skip_clicks: i32,
    skip_click_frames: i32,
}

impl Scene {
    pub fn new(nodes: Vec<Box<SceneNode>>) -> Scene {
        Scene {
            nodes: nodes,
            index: 0,
            began: false,
            skip_clicks: 0,
            skip_click_frames: 0,
        }
    }

    pub fn empty() -> Scene { Scene::new(Vec::new()) }

    pub fn begin(&mut self, theater: &mut Theater) {
        if !self.began {
            if !self.nodes.is_empty() {
                self.nodes[0].begin(theater, true);
            }
            self.began = true;
        }
    }

    pub fn tick(&mut self, theater: &mut Theater) -> bool {
        if !self.began {
            return false;
        }
        if !theater.drain_queue().is_empty() {
            debug_assert!(false, "Theater queue was not drained.");
        }
        let mut changed = false;
        if self.index < self.nodes.len() {
            changed |= self.nodes[self.index].tick(theater, false);
            while self.nodes[self.index].status() == Status::Done {
                self.index += 1;
                if self.index < self.nodes.len() {
                    self.nodes[self.index].begin(theater, true);
                    changed = true;
                } else {
                    self.skip_clicks = 0;
                    self.skip_click_frames = 0;
                    break;
                }
            }
        }
        changed
    }

    pub fn show_skip(&self) -> bool { self.skip_clicks >= CLICKS_TO_SHOW_SKIP }

    pub fn skip(&mut self, theater: &mut Theater) {
        while self.index < self.nodes.len() {
            self.nodes[self.index].skip(theater);
            self.index += 1;
        }
        self.began = true;
        self.skip_clicks = 0;
        self.skip_click_frames = 0;
    }

    pub fn is_finished(&self) -> bool {
        self.began && self.index == self.nodes.len()
    }

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
        theater.draw_speech_bubbles(canvas);
    }

    fn handle_event(&mut self, event: &Event, theater: &mut Theater)
                    -> Action<()> {
        let action = match event {
            &Event::Quit => Action::ignore(),
            &Event::ClockTick => {
                let mut redraw = self.tick(theater);
                if self.skip_clicks > 0 {
                    self.skip_click_frames -= 1;
                    if self.skip_click_frames <= 0 {
                        redraw = self.skip_clicks >= CLICKS_TO_SHOW_SKIP;
                        self.skip_clicks = 0;
                        self.skip_click_frames = 0;
                    }
                }
                Action::redraw_if(redraw)
            }
            &Event::MouseDown(_) => {
                if !self.began || self.is_finished() {
                    Action::ignore()
                } else {
                    let mut redraw = false;
                    if self.skip_clicks >= CLICKS_TO_SHOW_SKIP {
                        self.skip_click_frames = FRAMES_TO_HIDE_SKIP;
                    } else {
                        self.skip_clicks += 1;
                        if self.skip_clicks >= CLICKS_TO_SHOW_SKIP {
                            self.skip_click_frames = FRAMES_TO_HIDE_SKIP;
                            redraw = true;
                        } else {
                            self.skip_click_frames = FRAMES_BETWEEN_CLICKS;
                        }
                    }
                    if self.is_paused() {
                        self.unpause();
                        redraw = true;
                    }
                    Action::redraw_if(redraw).and_stop()
                }
            }
            &Event::KeyDown(Keycode::Escape, _) => {
                if !self.began || self.is_finished() {
                    Action::ignore()
                } else {
                    self.skip_click_frames = FRAMES_TO_HIDE_SKIP;
                    if self.skip_clicks < CLICKS_TO_SHOW_SKIP {
                        self.skip_clicks = CLICKS_TO_SHOW_SKIP;
                        Action::redraw().and_stop()
                    } else {
                        Action::ignore().and_stop()
                    }
                }
            }
            _ => {
                if !self.began || self.is_finished() {
                    Action::ignore()
                } else {
                    Action::ignore().and_stop()
                }
            }
        };
        action.and_play_sounds(theater.drain_sounds())
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
    fn box_clone(&self) -> Box<SceneNode>;

    fn status(&self) -> Status { Status::Done }

    fn begin(&mut self, _theater: &mut Theater, _terminated_by_pause: bool) {}

    fn tick(&mut self, _theater: &mut Theater, _keep_twiddling: bool) -> bool {
        false
    }

    fn skip(&mut self, _theater: &mut Theater) {}

    fn reset(&mut self) {}

    fn unpause(&mut self) {}
}

impl Clone for Box<SceneNode> {
    fn clone(&self) -> Box<SceneNode> { self.box_clone() }
}

// ========================================================================= //

#[derive(Clone)]
pub struct SequenceNode {
    nodes: Vec<Box<SceneNode>>,
    index: usize,
    terminated_by_pause: bool,
}

impl SequenceNode {
    pub fn new(nodes: Vec<Box<SceneNode>>) -> SequenceNode {
        SequenceNode {
            nodes: nodes,
            index: 0,
            terminated_by_pause: false,
        }
    }

    fn on_last_node(&self) -> bool { self.index + 1 == self.nodes.len() }
}

impl SceneNode for SequenceNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn status(&self) -> Status {
        if self.on_last_node() {
            let status = self.nodes[self.index].status();
            debug_assert!(status != Status::Done);
            status
        } else if self.index < self.nodes.len() {
            Status::Active
        } else {
            Status::Done
        }
    }

    fn begin(&mut self, theater: &mut Theater, terminated_by_pause: bool) {
        self.terminated_by_pause = terminated_by_pause;
        while self.index < self.nodes.len() {
            let pause = terminated_by_pause && self.on_last_node();
            let node = &mut self.nodes[self.index];
            node.begin(theater, pause);
            if node.status() != Status::Done {
                break;
            }
            self.index += 1;
        }
    }

    fn tick(&mut self, theater: &mut Theater, keep_twiddling: bool) -> bool {
        let mut changed = false;
        if self.index < self.nodes.len() {
            let twiddle = keep_twiddling && self.on_last_node();
            changed |= self.nodes[self.index].tick(theater, twiddle);
            while self.nodes[self.index].status() == Status::Done {
                self.index += 1;
                if self.index < self.nodes.len() {
                    let pause = self.terminated_by_pause &&
                        self.on_last_node();
                    self.nodes[self.index].begin(theater, pause);
                    changed = true;
                } else {
                    break;
                }
            }
        }
        changed
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

#[derive(Clone)]
pub struct ParallelNode {
    nodes: Vec<Box<SceneNode>>,
}

impl ParallelNode {
    pub fn new(nodes: Vec<Box<SceneNode>>) -> ParallelNode {
        ParallelNode { nodes: nodes }
    }
}

impl SceneNode for ParallelNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn status(&self) -> Status {
        let mut status = Status::Done;
        for node in self.nodes.iter() {
            status = cmp::min(status, node.status());
        }
        status
    }

    fn begin(&mut self, theater: &mut Theater, terminated_by_pause: bool) {
        for node in self.nodes.iter_mut() {
            node.begin(theater, terminated_by_pause);
        }
    }

    fn tick(&mut self, theater: &mut Theater, keep_twiddling: bool) -> bool {
        let mut keep_twiddling = keep_twiddling;
        if !keep_twiddling {
            for node in self.nodes.iter() {
                if node.status() <= Status::Paused {
                    keep_twiddling = true;
                    break;
                }
            }
        }
        let mut changed = false;
        for node in self.nodes.iter_mut() {
            changed |= node.tick(theater, keep_twiddling);
        }
        changed
    }

    fn skip(&mut self, theater: &mut Theater) {
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

#[derive(Clone)]
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
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn status(&self) -> Status {
        if self.iteration < self.min_iterations {
            Status::Active
        } else if self.node.status() == Status::Active {
            Status::Twiddling
        } else {
            Status::Done
        }
    }

    fn begin(&mut self, theater: &mut Theater, _: bool) {
        self.node.begin(theater, false);
    }

    fn tick(&mut self, theater: &mut Theater, keep_twiddling: bool) -> bool {
        let mut changed = false;
        if self.node.status() == Status::Active {
            changed |= self.node.tick(theater, false);
            if self.node.status() == Status::Done {
                if self.iteration < self.min_iterations ||
                    self.max_iterations.is_some()
                {
                    self.iteration += 1;
                }
                if self.iteration < self.min_iterations ||
                    (keep_twiddling && self.can_continue())
                {
                    self.node.reset();
                    self.node.begin(theater, false);
                    changed = true;
                }
            }
        }
        changed
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

#[derive(Clone)]
pub struct AnimNode {
    slot: i32,
    sprites: Vec<Sprite>,
    slowdown: i32,
}

impl AnimNode {
    pub fn new(slot: i32, sprites: Vec<Sprite>, slowdown: i32) -> AnimNode {
        AnimNode {
            slot: slot,
            sprites: sprites,
            slowdown: slowdown,
        }
    }
}

impl SceneNode for AnimNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) {
        theater.set_actor_anim(self.slot, self.sprites.clone(), self.slowdown);
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct DarkNode {
    dark: bool,
}

impl DarkNode {
    pub fn new(dark: bool) -> DarkNode { DarkNode { dark: dark } }
}

impl SceneNode for DarkNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) { theater.set_dark(self.dark); }
}

// ========================================================================= //

const GRAVITY: f64 = 480.0;

#[derive(Clone)]
pub struct JumpNode {
    progress: i32,
    duration: i32,
    slot: i32,
    start: Point,
    end: Point,
}

impl JumpNode {
    pub fn new(slot: i32, end: Point, duration_seconds: f64) -> JumpNode {
        JumpNode {
            progress: 0,
            duration: seconds_to_frames(duration_seconds),
            slot: slot,
            start: end,
            end: end,
        }
    }

    pub fn time_to_fall(dist: i32) -> f64 {
        if dist <= 0 {
            0.0
        } else {
            ((2.0 / GRAVITY) * dist as f64).sqrt()
        }
    }
}

impl SceneNode for JumpNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn status(&self) -> Status {
        if self.progress < self.duration {
            Status::Active
        } else {
            Status::Done
        }
    }

    fn begin(&mut self, theater: &mut Theater, _: bool) {
        if let Some(position) = theater.get_actor_position(self.slot) {
            self.start = position;
        }
    }

    fn tick(&mut self, theater: &mut Theater, _: bool) -> bool {
        if self.progress < self.duration {
            self.progress += 1;
            let frac = self.progress as f64 / self.duration as f64;
            let delta = self.end - self.start;
            let dx = delta.x() as f64 * frac;
            let dy = delta.y() as f64 * frac -
                0.5 * GRAVITY * frames_to_seconds(self.progress) *
                    frames_to_seconds(self.duration - self.progress);
            let delta = Point::new(dx.round() as i32, dy.round() as i32);
            theater.set_actor_position(self.slot, self.start + delta);
            true
        } else {
            false
        }
    }

    fn skip(&mut self, theater: &mut Theater) {
        theater.set_actor_position(self.slot, self.end);
    }

    fn reset(&mut self) { self.progress = 0; }
}

// ========================================================================= //

#[derive(Clone)]
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
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) {
        theater.set_actor_light(self.slot, self.light.clone());
    }
}

// ========================================================================= //

#[derive(Clone)]
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
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) {
        theater.place_actor(self.slot, self.sprite.clone(), self.position);
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct QueueNode {
    entry: (i32, i32),
}

impl QueueNode {
    pub fn new(entry: (i32, i32)) -> QueueNode { QueueNode { entry: entry } }
}

impl SceneNode for QueueNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) { theater.enqueue(self.entry); }
}

// ========================================================================= //

#[derive(Clone)]
pub struct RemoveNode {
    slot: i32,
}

impl RemoveNode {
    pub fn new(slot: i32) -> RemoveNode { RemoveNode { slot: slot } }
}

impl SceneNode for RemoveNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) {
        theater.remove_actor(self.slot);
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct SetBgNode {
    background: Rc<Background>,
}

impl SetBgNode {
    pub fn new(background: Rc<Background>) -> SetBgNode {
        SetBgNode { background: background }
    }
}

impl SceneNode for SetBgNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) {
        theater.set_background(self.background.clone());
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct SetPosNode {
    slot: i32,
    position: Point,
}

impl SetPosNode {
    pub fn new(slot: i32, position: Point) -> SetPosNode {
        SetPosNode {
            slot: slot,
            position: position,
        }
    }
}

impl SceneNode for SetPosNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) {
        theater.set_actor_position(self.slot, self.position);
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct SetSpriteNode {
    slot: i32,
    sprite: Sprite,
}

impl SetSpriteNode {
    pub fn new(slot: i32, sprite: Sprite) -> SetSpriteNode {
        SetSpriteNode {
            slot: slot,
            sprite: sprite,
        }
    }
}

impl SceneNode for SetSpriteNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) {
        theater.set_actor_sprite(self.slot, self.sprite.clone());
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct ShakeNode {
    amount: i32,
}

impl ShakeNode {
    pub fn new(amount: i32) -> ShakeNode { ShakeNode { amount: amount } }
}

impl SceneNode for ShakeNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) {
        theater.add_shake(self.amount);
    }
}

// ========================================================================= //

#[derive(Clone)]
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
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn status(&self) -> Status {
        if self.progress < self.duration {
            Status::Active
        } else {
            Status::Done
        }
    }

    fn begin(&mut self, theater: &mut Theater, _: bool) {
        if let Some(position) = theater.get_actor_position(self.slot) {
            self.start = position;
        }
    }

    fn tick(&mut self, theater: &mut Theater, _: bool) -> bool {
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
            true
        } else {
            false
        }
    }

    fn skip(&mut self, theater: &mut Theater) {
        theater.set_actor_position(self.slot, self.end);
    }

    fn reset(&mut self) { self.progress = 0; }
}

// ========================================================================= //

#[derive(Clone)]
pub struct SoundNode {
    sound: Sound,
}

impl SoundNode {
    pub fn new(sound: Sound) -> SoundNode { SoundNode { sound: sound } }
}

impl SceneNode for SoundNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) {
        theater.add_sound(self.sound.clone());
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct SwapNode {
    slot1: i32,
    slot2: i32,
}

impl SwapNode {
    pub fn new(slot1: i32, slot2: i32) -> SwapNode {
        SwapNode {
            slot1: slot1,
            slot2: slot2,
        }
    }
}

impl SceneNode for SwapNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn begin(&mut self, theater: &mut Theater, _: bool) { self.skip(theater); }

    fn skip(&mut self, theater: &mut Theater) {
        theater.swap_actors(self.slot1, self.slot2);
    }
}

// ========================================================================= //

#[derive(Clone)]
pub struct TalkNode {
    slot: i32,
    bubble_sprites: Vec<Sprite>,
    bg_color: (u8, u8, u8),
    talk_pos: TalkPos,
    paragraph: Rc<Paragraph>,
    status: Status,
    terminated_by_pause: bool,
}

impl TalkNode {
    pub fn new(slot: i32, bubble_sprites: Vec<Sprite>,
               bg_color: (u8, u8, u8), talk_pos: TalkPos,
               paragraph: Paragraph)
               -> TalkNode {
        TalkNode {
            slot: slot,
            bubble_sprites: bubble_sprites,
            bg_color: bg_color,
            talk_pos: talk_pos,
            paragraph: Rc::new(paragraph),
            status: Status::Active,
            terminated_by_pause: false,
        }
    }
}

impl SceneNode for TalkNode {
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn status(&self) -> Status { self.status }

    fn begin(&mut self, theater: &mut Theater, terminated_by_pause: bool) {
        self.terminated_by_pause = terminated_by_pause;
        if terminated_by_pause {
            self.status = Status::Paused;
        } else {
            self.status = Status::Twiddling;
        }
        theater.set_actor_speech(self.slot,
                                 self.bubble_sprites.clone(),
                                 self.bg_color,
                                 self.talk_pos,
                                 self.paragraph.clone());
    }

    fn tick(&mut self, theater: &mut Theater, keep_twiddling: bool) -> bool {
        if self.status == Status::Twiddling &&
            (self.terminated_by_pause || !keep_twiddling)
        {
            self.skip(theater);
            true
        } else {
            false
        }
    }

    fn skip(&mut self, theater: &mut Theater) {
        theater.clear_actor_speech(self.slot);
        self.status = Status::Done;
    }

    fn reset(&mut self) { self.status = Status::Active; }

    fn unpause(&mut self) {
        if self.status == Status::Paused {
            self.status = Status::Twiddling;
        }
    }
}

// ========================================================================= //

#[derive(Clone)]
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
    fn box_clone(&self) -> Box<SceneNode> { Box::new(self.clone()) }

    fn status(&self) -> Status {
        if self.progress < self.duration {
            Status::Active
        } else {
            Status::Done
        }
    }

    fn tick(&mut self, _: &mut Theater, _: bool) -> bool {
        if self.progress < self.duration {
            self.progress += 1;
        }
        false
    }

    fn skip(&mut self, _: &mut Theater) { self.progress = self.duration; }

    fn reset(&mut self) { self.progress = 0; }
}

// ========================================================================= //

fn frames_to_seconds(frames: i32) -> f64 {
    0.001 * FRAME_DELAY_MILLIS as f64 * frames as f64
}

fn seconds_to_frames(seconds: f64) -> i32 {
    (seconds / (0.001 * FRAME_DELAY_MILLIS as f64)).floor() as i32
}

// ========================================================================= //
