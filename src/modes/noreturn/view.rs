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
use std::collections::HashSet;
use std::rc::Rc;

use super::scenes::{self, DOOR_LOWER, DOOR_UPPER, YTTRIS};
use crate::elements::cutscene::WaitNode;
use crate::elements::cutscene::{
    JumpNode, ParallelNode, QueueNode, SceneNode, SequenceNode, SetPosNode,
    ShakeNode, SlideNode, SoundNode,
};
use crate::elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView, Scene};
use crate::gui::{
    Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources,
    Sound, Sprite,
};
use crate::modes::SOLVED_INFO_TEXT;
use crate::save::{Game, NoReturnState, PuzzleState};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(usize, usize)>,
    bridge: TileBridge,
    button: StartStopButton,
    animation: Scene,
    running: bool,
}

impl View {
    pub fn new(
        resources: &mut Resources,
        visible: Rect,
        state: &NoReturnState,
    ) -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_relyng_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        let mut view = View {
            core,
            bridge: TileBridge::new(resources, 112, 176),
            button: StartStopButton::new(resources, 96, 208),
            animation: Scene::empty(),
            running: false,
        };
        view.animation.begin(view.core.theater_mut());
        view
    }

    fn yttris_land_on_floor(&self, seq: &mut Vec<Box<dyn SceneNode>>) {
        let sound = Sound::character_collision();
        seq.push(Box::new(SoundNode::new(sound)));
        seq.push(Box::new(ShakeNode::new(4)));
        seq.push(Box::new(WaitNode::new(1.0)));
        seq.push(Box::new(SetPosNode::new(YTTRIS, Point::new(-16, 176))));
        let pt = self.bridge.tile_center_top(-1);
        seq.push(Box::new(SlideNode::new(YTTRIS, pt, true, true, 0.75)));
        seq.push(Box::new(QueueNode::new((-1, 0))));
    }

    fn start_animation(&mut self, state: &NoReturnState) {
        let values = state.col_values();
        let num_cols = values.len() as i32;
        let mut next_col: i32 = 0;
        let mut visited: HashSet<i32> = HashSet::new();
        let mut seq: Vec<Box<dyn SceneNode>> = Vec::new();
        loop {
            seq.push(Box::new(SoundNode::new(Sound::small_jump())));
            if next_col == -1 {
                let pt = self.bridge.tile_center_top(-1);
                seq.push(Box::new(JumpNode::new(YTTRIS, pt, 0.5)));
                seq.push(Box::new(QueueNode::new((-1, 0))));
            } else if next_col < 0 {
                let pt = Point::new(91, 160);
                let time = 0.6 + 0.1 * next_col as f64;
                seq.push(Box::new(JumpNode::new(YTTRIS, pt, time)));
                let sound = Sound::character_collision();
                seq.push(Box::new(SoundNode::new(sound)));
                let pt = self.bridge.tile_center_top(-1);
                seq.push(Box::new(JumpNode::new(YTTRIS, pt, 0.2)));
                seq.push(Box::new(QueueNode::new((-1, 0))));
            } else if next_col == num_cols && visited.len() == values.len() {
                let pt = self.bridge.tile_center_top(num_cols);
                let mut par: Vec<Box<dyn SceneNode>> = Vec::new();
                par.push(Box::new(SlideNode::new(
                    DOOR_UPPER,
                    Point::new(505, 144),
                    false,
                    false,
                    0.3,
                )));
                par.push(Box::new(SlideNode::new(
                    DOOR_LOWER,
                    Point::new(505, 192),
                    false,
                    false,
                    0.3,
                )));
                par.push(Box::new(JumpNode::new(YTTRIS, pt, 0.5)));
                seq.push(Box::new(ParallelNode::new(par)));
            } else if next_col >= num_cols {
                let pt = Point::new(483, 160);
                let time = 0.5 - 0.1 * (next_col - num_cols) as f64;
                seq.push(Box::new(JumpNode::new(YTTRIS, pt, time)));
                let sound = Sound::character_collision();
                seq.push(Box::new(SoundNode::new(sound)));
                let pt = Point::new(450, 416);
                let fall_time = JumpNode::time_to_fall(416 - 160);
                seq.push(Box::new(JumpNode::new(YTTRIS, pt, fall_time)));
                self.yttris_land_on_floor(&mut seq);
            } else if visited.contains(&next_col) {
                let pt = self.bridge.tile_center_top(next_col);
                seq.push(Box::new(JumpNode::new(YTTRIS, pt, 0.5)));
                let pt = Point::new(pt.x(), 416);
                seq.push(Box::new(JumpNode::new(YTTRIS, pt, 0.85)));
                self.yttris_land_on_floor(&mut seq);
            } else {
                let pt = self.bridge.tile_center_top(next_col);
                seq.push(Box::new(JumpNode::new(YTTRIS, pt, 0.5)));
                seq.push(Box::new(QueueNode::new((0, next_col))));
                visited.insert(next_col);
                next_col += values[next_col as usize];
                continue;
            }
            break;
        }
        self.animation = Scene::new(vec![Box::new(SequenceNode::new(seq))]);
        self.animation.begin(self.core.theater_mut());
        self.running = true;
    }

    fn stop_and_refresh(&mut self) {
        let pt = self.bridge.tile_center_top(-1);
        self.core.theater_mut().set_actor_position(YTTRIS, pt);
        self.animation = Scene::empty();
        self.animation.begin(self.core.theater_mut());
        self.bridge.drag = None;
        self.bridge.visited.clear();
        self.running = false;
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.point_of_no_return;
        self.core.draw_back_layer(canvas);
        self.button
            .draw(&(self.running, self.core.theater().shake_offset()), canvas);
        self.bridge.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        game: &mut Game,
    ) -> Action<PuzzleCmd> {
        let state = &mut game.point_of_no_return;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() && !state.is_solved() {
            let mut input = (self.running, self.core.theater().shake_offset());
            let subaction = self.button.handle_event(event, &mut input);
            if let Some(&run) = subaction.value() {
                self.stop_and_refresh();
                if run {
                    state.check_if_solved();
                    self.start_animation(state);
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() && self.running {
            let subaction =
                self.animation.handle_event(event, self.core.theater_mut());
            action.merge(subaction.but_no_value());
            self.drain_queue();
            if self.animation.is_finished() {
                self.running = false;
                if state.is_solved() {
                    self.core.begin_outro_scene();
                    action = action.and_return(PuzzleCmd::Save);
                }
            }
        }
        if !action.should_stop()
            && (!self.running && !state.is_solved()
                || event == &Event::ClockTick)
        {
            let subaction = self.bridge.handle_event(event, state);
            if let Some(&(old_index, new_index)) = subaction.value() {
                state.move_tile(old_index, new_index);
                self.core.push_undo((old_index, new_index));
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
        if game.point_of_no_return.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((old_index, new_index)) = self.core.pop_undo() {
            game.point_of_no_return.move_tile(new_index, old_index);
            self.stop_and_refresh();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((old_index, new_index)) = self.core.pop_redo() {
            game.point_of_no_return.move_tile(old_index, new_index);
            self.stop_and_refresh();
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.point_of_no_return.reset();
        self.stop_and_refresh();
    }

    fn solve(&mut self, game: &mut Game) {
        game.point_of_no_return.solve();
        self.stop_and_refresh();
        self.start_animation(&game.point_of_no_return);
    }

    fn drain_queue(&mut self) {
        for (command, index) in self.core.drain_queue() {
            if command < 0 {
                self.bridge.visited.clear();
            } else if command == 0 {
                self.bridge.visited.insert(index);
            } else if command == 1 {
                if index >= 0 && (index as usize) < LETTERS.len() {
                    self.bridge.letter_cols.insert(index);
                }
            }
        }
    }
}

// ========================================================================= //

const TILE_USIZE: u32 = 24;
const TILE_SIZE: i32 = TILE_USIZE as i32;

struct TileBridge {
    sprites: Vec<Sprite>,
    numbers_font: Rc<Font>,
    left: i32,
    top: i32,
    drag: Option<TileDrag>,
    visited: HashSet<i32>,
    letters_font: Rc<Font>,
    letter_cols: HashSet<i32>,
}

impl TileBridge {
    fn new(resources: &mut Resources, left: i32, top: i32) -> TileBridge {
        TileBridge {
            sprites: resources.get_sprites("point/no_return"),
            numbers_font: resources.get_font("no_return"),
            left,
            top,
            drag: None,
            visited: HashSet::new(),
            letters_font: resources.get_font("block"),
            letter_cols: HashSet::new(),
        }
    }

    fn tile_center_top(&self, col: i32) -> Point {
        Point::new(self.left + TILE_SIZE / 2 + TILE_SIZE * col, self.top)
    }

    fn draw_tile(
        &self,
        start_col: i32,
        tile: &[i32],
        x: i32,
        canvas: &mut Canvas,
    ) {
        for (index, &value) in tile.iter().enumerate() {
            let col = start_col + index as i32;
            let pt = Point::new(x + TILE_SIZE * (index as i32), self.top);
            let mut arrow_index = if value < 0 { 4 } else { 5 };
            if self.visited.contains(&col) {
                arrow_index += 2;
            } else {
                let bg_index = if tile.len() == 1 {
                    0
                } else if index == 0 {
                    2
                } else if index + 1 == tile.len() {
                    3
                } else {
                    1
                };
                canvas.draw_sprite(&self.sprites[bg_index], pt);
            }
            canvas.draw_sprite(&self.sprites[arrow_index], pt);
            canvas.draw_text(
                &self.numbers_font,
                Align::Center,
                pt + Point::new(12, 20),
                &format!("{}", value.abs()),
            );
            if self.letter_cols.contains(&col) {
                canvas.draw_char(
                    &self.letters_font,
                    Align::Center,
                    pt + Point::new(12, -2),
                    LETTERS[col as usize],
                );
            }
        }
    }
}

impl Element<NoReturnState, (usize, usize)> for TileBridge {
    fn draw(&self, state: &NoReturnState, canvas: &mut Canvas) {
        let tiles = state.tiles();
        let mut start_cols = vec![0; tiles.len()];
        let mut col: i32 = 0;
        for (tile_index, &tile) in tiles.iter().enumerate() {
            start_cols[tile_index] = col;
            let mut x = self.left + TILE_SIZE * col;
            if let Some(ref drag) = self.drag {
                if drag.index == tile_index {
                    col += tile.len() as i32;
                    continue;
                }
                x += drag.offset(tile_index);
            }
            self.draw_tile(col, tile, x, canvas);
            col += tile.len() as i32;
        }
        canvas.draw_sprite(
            &self.sprites[8],
            Point::new(self.left - TILE_SIZE, self.top),
        );
        canvas.draw_sprite(
            &self.sprites[9],
            Point::new(self.left + TILE_SIZE * col, self.top),
        );
        if let Some(ref drag) = self.drag {
            let col = start_cols[drag.index];
            let tile = tiles[drag.index];
            let x = self.left + TILE_SIZE * col + drag.offset(drag.index);
            self.draw_tile(col, tile, x, canvas);
        }
    }

    fn handle_event(
        &mut self,
        event: &Event,
        state: &mut NoReturnState,
    ) -> Action<(usize, usize)> {
        match event {
            &Event::ClockTick => {
                if let Some(ref mut drag) = self.drag {
                    return Action::redraw_if(drag.tick_animation());
                }
            }
            &Event::MouseDown(pt) => {
                let tiles = state.tiles();
                let mut tile_left = self.left;
                for (index, &tile) in tiles.iter().enumerate() {
                    let rect = Rect::new(
                        tile_left,
                        self.top,
                        TILE_USIZE * tile.len() as u32,
                        TILE_USIZE,
                    );
                    if rect.contains_point(pt) {
                        self.drag = Some(TileDrag::new(index, pt.x(), state));
                        let sound = Sound::device_pickup();
                        return Action::redraw().and_play_sound(sound);
                    }
                    tile_left += rect.width() as i32;
                }
            }
            &Event::MouseDrag(pt) => {
                if let Some(ref mut drag) = self.drag {
                    drag.set_to(pt.x());
                    return Action::redraw();
                }
            }
            &Event::MouseUp => {
                if let Some(drag) = self.drag.take() {
                    let sound = Sound::device_drop();
                    let action = Action::redraw().and_play_sound(sound);
                    let old_index = drag.index;
                    let new_index = drag.new_index;
                    if new_index == old_index {
                        return action;
                    }
                    return action.and_return((old_index, new_index));
                }
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //

const TILE_SLIDE_SPEED: i32 = 10;

struct TileDrag {
    index: usize, // original index in state.tiles() of tile being dragged
    new_index: usize, // new index where dragged tile will be inserted
    new_col: i32, // new start_col of dragged tile once inserted
    from: i32,    // mouse x where drag started
    to: i32,      // current mouse x
    tile_spans: Vec<(i32, usize)>, // original (start_col, size) of each tile
    offsets: Vec<(i32, i32)>, // (current_dx, goal_dx) of each tile
}

impl TileDrag {
    fn new(index: usize, from: i32, state: &NoReturnState) -> TileDrag {
        let tiles = state.tiles();
        let tile_spans = {
            let mut tile_spans = Vec::new();
            let mut col: i32 = 0;
            for &tile in tiles.iter() {
                tile_spans.push((col, tile.len()));
                col += tile.len() as i32;
            }
            tile_spans
        };
        let col = tile_spans[index].0;
        TileDrag {
            index,
            new_index: index,
            new_col: col,
            from,
            to: from,
            tile_spans,
            offsets: vec![(0, 0); tiles.len()],
        }
    }

    fn set_to(&mut self, to: i32) {
        self.to = to;
        let left = self.new_col * TILE_SIZE + self.to - self.from;
        let tile_width = self.tile_spans[self.index].1 as i32;

        let mut new_index = self.new_index;
        let mut new_col = self.new_col;
        let mut smallest_gap = (self.to - self.from).abs();
        for index in 0..self.tile_spans.len() {
            let col = {
                let mut col: i32 = 0;
                let mut num_tiles: usize = 0;
                for (idx, &(_, width)) in self.tile_spans.iter().enumerate() {
                    if num_tiles == index {
                        break;
                    }
                    if idx != self.index {
                        col += width as i32;
                        num_tiles += 1;
                    }
                }
                col
            };
            let gap = (col * TILE_SIZE - left).abs();
            if gap < smallest_gap {
                new_index = index;
                new_col = col;
                smallest_gap = gap;
            }
        }
        debug_assert!(new_index < self.tile_spans.len());

        // If tile has been dragged far enough so far to change index, update
        // fields accordingly.
        if self.new_index != new_index {
            self.from += (new_col - self.new_col) * TILE_SIZE;
            self.new_index = new_index;
            self.new_col = new_col;
            let old_index = self.index;
            // Update draw offset goals for other tiles that got shifted.
            for (index, &mut (_, ref mut goal)) in
                self.offsets.iter_mut().enumerate()
            {
                *goal = if index < old_index && index >= new_index {
                    TILE_SIZE * tile_width
                } else if index > old_index && index <= new_index {
                    -TILE_SIZE * tile_width
                } else {
                    0
                }
            }
        }

        // Set draw offset of tile being dragged.
        let old_col = self.tile_spans[self.index].0;
        let offset =
            (self.new_col - old_col) * TILE_SIZE + self.to - self.from;
        self.offsets[self.index] = (offset, offset);
    }

    fn offset(&self, index: usize) -> i32 {
        self.offsets[index].0
    }

    fn tick_animation(&mut self) -> bool {
        let mut redraw = false;
        for &mut (ref mut offset, goal) in self.offsets.iter_mut() {
            if *offset < goal {
                *offset = cmp::min(goal, *offset + TILE_SLIDE_SPEED);
                redraw = true;
            } else if *offset > goal {
                *offset = cmp::max(goal, *offset - TILE_SLIDE_SPEED);
                redraw = true;
            }
        }
        redraw
    }
}

// ========================================================================= //

struct StartStopButton {
    rect: Rect,
    sprites: Vec<Sprite>,
}

impl StartStopButton {
    fn new(resources: &mut Resources, cx: i32, cy: i32) -> StartStopButton {
        let sprites = resources.get_sprites("point/button");
        let width = sprites[0].width();
        let height = sprites[0].height();
        StartStopButton {
            rect: Rect::new(
                cx - (width as i32 / 2),
                cy - (height as i32 / 2),
                width,
                height,
            ),
            sprites,
        }
    }
}

impl Element<(bool, Point), bool> for StartStopButton {
    fn draw(&self, input: &(bool, Point), canvas: &mut Canvas) {
        let index = if input.0 { 0 } else { 1 };
        canvas
            .draw_sprite(&self.sprites[index], self.rect.top_left() + input.1);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        input: &mut (bool, Point),
    ) -> Action<bool> {
        match event {
            &Event::MouseDown(pt) if self.rect.contains_point(pt) => {
                let mut action = Action::redraw().and_return(!input.0);
                if input.0 {
                    action.also_play_sound(Sound::talk_annoyed_hi())
                }
                action
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const LETTERS: &[char] = &[
    '*', 'S', 'E', 'I', 'V', 'T', 'I', 'N', 'O', 'G', 'N', 'I', '*', 'A', '?',
    'T',
];

const INFO_BOX_TEXT: &str = "\
Your goal is to reorder the pieces of the walkway so that
Yttris can cross from ``BEGIN'' to ``END'' while landing on
each square exactly once.

Once the last remaining square has been eliminated, the
door on the right-hand side will open, allowing Yttris to
land on the ``END'' platform.

Drag pieces left and right with $M{your finger}{the mouse} to reorder them.
When you're ready, $M{tap}{click} the green triangle button on the
left to see if your solution works.

$M{Tap}{Click} on a character in the scene to hear their words of
wisdom.";

// ========================================================================= //
