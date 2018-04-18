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

use elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{BlackState, Game, PuzzleState};
use save::tree::{BasicTree, TreeOp};
use super::scenes;

// ========================================================================= //

#[derive(Clone, Copy)]
enum TreeCmd {
    Insert(i32),
    Remove(i32),
}

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(TreeCmd, [i8; 15])>,
    tree: TreeView,
    tree_visible: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &BlackState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::BottomToTop, FadeStyle::BottomToTop);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        View {
            core: core,
            tree: TreeView::new(resources, 312, 320, state),
            tree_visible: false,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.black_and_blue;
        self.core.draw_back_layer(canvas);
        if self.tree_visible {
            self.tree.draw(state, canvas);
        }
        self.core.draw_middle_layer(canvas);
        if self.tree_visible {
            self.tree.draw_loose_fruits(state, canvas);
        }
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.black_and_blue;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() && self.tree_visible {
            let subaction = self.tree.handle_event(event, state);
            if let Some(&cmd) = subaction.value() {
                let basic = state.tree().as_basic().clone();
                let signature = state.signature();
                let ops = match cmd {
                    TreeCmd::Insert(key) => state.insert(key),
                    TreeCmd::Remove(key) => state.remove(key),
                };
                if !ops.is_empty() {
                    // TODO: play sound
                    self.tree.start_animation(basic, ops);
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                        action = action.and_return(PuzzleCmd::Save);
                    } else {
                        self.core.push_undo((cmd, signature));
                    }
                }
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
        if game.black_and_blue.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((_, signature)) = self.core.pop_undo() {
            let state = &mut game.black_and_blue;
            state.set_from_signature(&signature);
            self.tree.update_fruit_positions(state);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((cmd, _)) = self.core.pop_redo() {
            let state = &mut game.black_and_blue;
            match cmd {
                TreeCmd::Insert(key) => {
                    state.insert(key);
                }
                TreeCmd::Remove(key) => {
                    state.remove(key);
                }
            }
            self.tree.update_fruit_positions(state);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.black_and_blue;
        self.core.clear_undo_redo();
        state.reset();
        self.tree.update_fruit_positions(state);
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.black_and_blue;
        state.solve();
        self.tree.update_fruit_positions(state);
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.tree_visible = value != 0;
            } else if kind == 1 {
                self.tree.override_fruits = value != 0;
            }
        }
    }
}

// ========================================================================= //

const BRANCH_COLOR: (u8, u8, u8) = (127, 63, 0);
const BRANCH_THICKNESS: u32 = 6;
const BRANCH_SEMI: i32 = (BRANCH_THICKNESS as i32) / 2;

const OP_ANIMATION_FRAMES: i32 = 10;

struct TreeView {
    base: Point,
    font: Rc<Font>,
    fruit_sprites: Vec<Sprite>,
    leaf_sprites: Vec<Sprite>,
    fruit: HashMap<i32, (Point, Point, Point)>,
    animation: Option<(BasicTree, Vec<TreeOp>, i32)>,
    override_fruits: bool,
}

impl TreeView {
    fn new(resources: &mut Resources, base_x: i32, base_y: i32,
           state: &BlackState)
           -> TreeView {
        let pt = Point::new(0, 0);
        let fruit = (1..16).map(|key| (key, (pt, pt, pt))).collect();
        let mut view = TreeView {
            base: Point::new(base_x, base_y),
            font: resources.get_font("roman"),
            fruit_sprites: resources.get_sprites("tree/nodes"),
            leaf_sprites: resources.get_sprites("tree/leaves"),
            fruit: fruit,
            animation: None,
            override_fruits: false,
        };
        view.update_fruit_positions(state);
        view
    }

    fn start_animation(&mut self, mut basic: BasicTree, ops: Vec<TreeOp>) {
        debug_assert!(!ops.is_empty());
        basic.perform_op(&ops[0]);
        self.move_fruit_to_goals();
        self.update_fruit_goals(&basic);
        self.animation = Some((basic, ops, OP_ANIMATION_FRAMES));
    }

    fn update_fruit_positions(&mut self, state: &BlackState) {
        self.animation = None;
        self.update_fruit_goals(state.tree().as_basic());
        self.move_fruit_to_goals();
    }

    fn update_fruit_currents(&mut self, remaining_frames: i32) {
        debug_assert!(remaining_frames >= 0);
        debug_assert!(remaining_frames <= OP_ANIMATION_FRAMES);
        let progress = OP_ANIMATION_FRAMES - remaining_frames;
        for (_, &mut (base, ref mut curr, goal)) in self.fruit.iter_mut() {
            *curr = base + ((goal - base) * progress) / OP_ANIMATION_FRAMES;
        }
    }

    fn update_fruit_goals(&mut self, tree: &BasicTree) {
        let mut positions: HashMap<i32, Point> = HashMap::new();
        let mut stack: Vec<(i32, i32)> = Vec::new();
        if let Some(root_key) = tree.root() {
            stack.push((root_key, 0));
        }
        while let Some((key, rank)) = stack.pop() {
            let position = Point::new(104 + 24 * key, 264 - 32 * rank);
            positions.insert(key, position);
            if let Some(left_key) = tree.left_child(key) {
                stack.push((left_key, rank + 1));
            }
            if let Some(right_key) = tree.right_child(key) {
                stack.push((right_key, rank + 1));
            }
        }
        for key in 1..16 {
            if !positions.contains_key(&key) {
                let y = match key {
                    15 => 277,
                    1 | 2 | 13 | 14 => 293,
                    _ => 309,
                };
                positions.insert(key, Point::new(104 + 24 * key, y));
            }
        }
        for (key, &mut (_, _, ref mut goal)) in self.fruit.iter_mut() {
            *goal = *positions.get(&key).unwrap();
        }
    }

    fn move_fruit_to_goals(&mut self) {
        for (_, &mut (ref mut base, ref mut curr, goal)) in
            self.fruit.iter_mut()
        {
            *base = goal;
            *curr = goal;
        }
    }

    fn draw_fruits(&self, on_tree: bool, tree: &BasicTree,
                   canvas: &mut Canvas) {
        for (&key, &(_, position, _)) in self.fruit.iter() {
            if tree.contains(key) != on_tree {
                continue;
            }
            if on_tree {
                if tree.left_child(key).is_none() {
                    canvas.draw_sprite(&self.leaf_sprites[0],
                                       position + Point::new(-16, -14));
                }
                if tree.right_child(key).is_none() {
                    canvas.draw_sprite(&self.leaf_sprites[1],
                                       position + Point::new(7, -14));
                }
            }
            let idx = if self.override_fruits {
                if LETTERS[(key - 1) as usize].1 { 2 } else { 0 }
            } else {
                if !on_tree || tree.is_red(key) { 1 } else { 0 }
            };
            canvas.draw_sprite_centered(&self.fruit_sprites[idx], position);
            if self.override_fruits {
                canvas.draw_char(&self.font,
                                 Align::Center,
                                 position + Point::new(0, 4),
                                 LETTERS[(key - 1) as usize].0);
            } else {
                canvas.draw_text(&self.font,
                                 Align::Center,
                                 position + Point::new(0, 4),
                                 &format!("{}", key));
            }
        }
    }

    fn draw_loose_fruits(&self, state: &BlackState, canvas: &mut Canvas) {
        let tree = if let Some((ref basic, _, _)) = self.animation {
            basic
        } else {
            state.tree().as_basic()
        };
        self.draw_fruits(false, tree, canvas);
    }
}

impl Element<BlackState, TreeCmd> for TreeView {
    fn draw(&self, state: &BlackState, canvas: &mut Canvas) {
        let tree = if let Some((ref basic, _, _)) = self.animation {
            basic
        } else {
            state.tree().as_basic()
        };
        // Trunk:
        if let Some(root_key) = tree.root() {
            let &(_, root_pos, _) = self.fruit.get(&root_key).unwrap();
            let mid_y = (root_pos.y() + self.base.y()) / 2;
            let rect = Rect::new(self.base.x() - BRANCH_SEMI,
                                 min(self.base.y(), mid_y) + 1 - BRANCH_SEMI,
                                 BRANCH_THICKNESS,
                                 (self.base.y() - mid_y).abs() as u32 +
                                     (BRANCH_THICKNESS / 2 - 1));
            canvas.fill_rect(BRANCH_COLOR, rect);
            let rect =
                Rect::new(min(root_pos.x(), self.base.x()) + 1 - BRANCH_SEMI,
                          mid_y - BRANCH_SEMI,
                          (root_pos.x() - self.base.x()).abs() as u32 +
                              BRANCH_THICKNESS - 2,
                          BRANCH_THICKNESS);
            canvas.fill_rect(BRANCH_COLOR, rect);
            let rect = Rect::new(root_pos.x() - BRANCH_SEMI,
                                 min(root_pos.y(), mid_y) + 1 - BRANCH_SEMI,
                                 BRANCH_THICKNESS,
                                 (root_pos.y() - mid_y).abs() as u32 +
                                     (BRANCH_THICKNESS - 2));
            canvas.fill_rect(BRANCH_COLOR, rect);
        }
        // Branches:
        for (&child_key, &(_, cpos, _)) in self.fruit.iter() {
            if let Some(parent_key) = tree.parent(child_key) {
                let &(_, ppos, _) = self.fruit.get(&parent_key).unwrap();
                let rect = Rect::new(cpos.x() - BRANCH_SEMI,
                                     min(cpos.y(), ppos.y()) + 1 -
                                         BRANCH_SEMI,
                                     BRANCH_THICKNESS,
                                     (cpos.y() - ppos.y()).abs() as u32 +
                                         (BRANCH_THICKNESS - 2));
                canvas.fill_rect(BRANCH_COLOR, rect);
                let rect = Rect::new(min(cpos.x(), ppos.x()) + 1 -
                                         BRANCH_SEMI,
                                     ppos.y() - BRANCH_SEMI,
                                     (cpos.x() - ppos.x()).abs() as u32 +
                                         (BRANCH_THICKNESS - 2),
                                     BRANCH_THICKNESS);
                canvas.fill_rect(BRANCH_COLOR, rect);
            }
        }
        // Fruit:
        self.draw_fruits(true, tree, canvas);
    }

    fn handle_event(&mut self, event: &Event, state: &mut BlackState)
                    -> Action<TreeCmd> {
        match event {
            &Event::ClockTick => {
                let mut redraw = false;
                if let Some((mut basic, mut ops, mut frames)) =
                    self.animation.take()
                {
                    debug_assert!(!ops.is_empty());
                    frames -= 1;
                    if frames <= 0 {
                        redraw = true;
                        ops.remove(0);
                        // TODO: play sound
                        if ops.is_empty() ||
                            (ops.len() == 1 && ops[0].is_set_red())
                        {
                            self.update_fruit_positions(state);
                        } else {
                            basic.perform_op(&ops[0]);
                            frames = OP_ANIMATION_FRAMES;
                            self.move_fruit_to_goals();
                            self.update_fruit_goals(&basic);
                            self.animation = Some((basic, ops, frames));
                        }
                    } else {
                        if !ops[0].is_set_red() {
                            self.update_fruit_currents(frames);
                            redraw = true;
                        }
                        self.animation = Some((basic, ops, frames));
                    }
                }
                Action::redraw_if(redraw)
            }
            &Event::MouseDown(pt)
                if self.animation.is_none() && !state.is_solved() => {
                for (&key, &(_, position, _)) in self.fruit.iter() {
                    let delta = pt - position;
                    let sqdist = delta.x() * delta.x() + delta.y() * delta.y();
                    if sqdist <= 11 * 11 {
                        let cmd = if state.tree().contains(key) {
                            TreeCmd::Remove(key)
                        } else {
                            TreeCmd::Insert(key)
                        };
                        // TODO: play sound
                        return Action::redraw().and_return(cmd);
                    }
                }
                Action::ignore()
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const LETTERS: [(char, bool); 15] = [
    ('A', true),
    ('S', false),
    ('R', true),
    ('T', true),
    ('I', true),
    ('A', false),
    ('R', false),
    ('S', true),
    ('T', true),
    ('C', false),
    ('I', true),
    ('A', false),
    ('S', false),
    ('C', true),
    ('M', false),
];

const INFO_BOX_TEXT: &str = "\
Your goal is to get fruit #10 on the tree high enough
to reach the girder.

$M{Tap}{Click} a fruit on the tree to pluck it off; $M{tap}{click} a fruit on
the ground to put it back on the tree.  You may only
have three fruits removed from the tree at a time.

After each move, the tree will automatically try to
rebalance/recolor itself, following two rules:
1) Each path from a leaf down to the trunk must have
     the same number of black fruits, and
2) No path from a leaf down to the trunk can have
     two red fruits in a row.

$M{Tap}{Click} on a character in the scene to hear their words
of wisdom.";

// ========================================================================= //
