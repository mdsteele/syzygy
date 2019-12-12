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

use num_integer::div_mod_floor;
use std::cmp;
use std::collections::HashSet;

use crate::elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView};
use crate::gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sound};
use crate::gui::Sprite;
use crate::modes::SOLVED_INFO_TEXT;
use crate::save::{Game, OrderState, PuzzleState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(usize, usize)>,
    rows: Vec<TileRow>,
    show_tiles: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &OrderState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::RightToLeft);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_argony_midscene(resources));
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        core.add_extra_scene(scenes::compile_relyng_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        View {
            core: core,
            rows: vec![
                TileRow::new(resources, 0, 321, 95),
                TileRow::new(resources, 1, 321, 127),
                TileRow::new(resources, 2, 321, 159),
                TileRow::new(resources, 3, 322, 191),
                TileRow::new(resources, 4, 322, 223),
                TileRow::new(resources, 5, 322, 255),
            ],
            show_tiles: false,
        }
    }

    fn clear_drag(&mut self) {
        for row in self.rows.iter_mut() {
            row.drag = None;
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.point_of_order;
        self.core.draw_back_layer(canvas);
        if self.show_tiles {
            self.rows.draw(state, canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.point_of_order;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() && self.show_tiles {
            let subaction = self.rows.handle_event(event, state);
            if let Some(&(old_index, new_index)) = subaction.value() {
                let old_row = state.current_row();
                state.move_tile(old_index, new_index);
                if state.is_solved() {
                    self.core.begin_outro_scene();
                    action = action.and_return(PuzzleCmd::Save);
                } else if state.current_row() > old_row {
                    action.also_play_sound(Sound::mid_puzzle_chime());
                    self.core.clear_undo_redo();
                } else {
                    self.core.push_undo((old_index, new_index));
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
        if game.point_of_order.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((old_index, new_index)) = self.core.pop_undo() {
            self.clear_drag();
            game.point_of_order.move_tile(new_index, old_index);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((old_index, new_index)) = self.core.pop_redo() {
            self.clear_drag();
            game.point_of_order.move_tile(old_index, new_index);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        self.clear_drag();
        game.point_of_order.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.point_of_order.solve();
        self.clear_drag();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.show_tiles = value != 0;
            } else if kind == 1 {
                let (row, index) = div_mod_floor(value, 6);
                if row >= 0 && (row as usize) < self.rows.len() {
                    self.rows[row as usize].hilight_tile(index as usize);
                }
            }
        }
    }
}

// ========================================================================= //

struct TileRow {
    symbol_sprites: Vec<Sprite>,
    tile_sprites: Vec<Sprite>,
    row: usize,
    left: i32,
    top: i32,
    drag: Option<TileDrag>,
    hilights: HashSet<usize>,
}

impl TileRow {
    fn new(resources: &mut Resources, row: usize, left: i32, top: i32)
           -> TileRow {
        TileRow {
            symbol_sprites: resources.get_sprites("point/order"),
            tile_sprites: resources.get_sprites("point/tiles"),
            row: row,
            left: left,
            top: top,
            drag: None,
            hilights: HashSet::new(),
        }
    }

    fn hilight_tile(&mut self, index: usize) { self.hilights.insert(index); }
}

impl Element<OrderState, (usize, usize)> for TileRow {
    fn draw(&self, state: &OrderState, canvas: &mut Canvas) {
        if state.current_row() >= self.row {
            for (index, value) in state
                .row_order(self.row)
                .iter()
                .enumerate()
            {
                let mut x = self.left + TILE_SPACING * (index as i32) + 1;
                if let Some(ref drag) = self.drag {
                    if drag.index == index {
                        continue;
                    }
                    x += drag.offset(index);
                }
                let pt = Point::new(x, self.top + 1);
                let symbol_index = 6 * self.row + value;
                let tile_index = if self.hilights.contains(&index) {
                    2
                } else if state.current_row() == self.row {
                    0
                } else {
                    1
                };
                canvas.draw_sprite(&self.tile_sprites[tile_index], pt);
                canvas.draw_sprite(&self.symbol_sprites[symbol_index], pt);
            }
            if let Some(ref drag) = self.drag {
                let value = state.row_order(self.row)[drag.index];
                let x = self.left + TILE_SPACING * (drag.index as i32) + 1 +
                    drag.offset(drag.index);
                let x = x.max(self.left - TILE_SPACING / 2 + 1);
                let x = x.min(self.left + (TILE_SPACING * 11) / 2 + 1);
                let pt = Point::new(x, self.top + 1);
                let symbol_index = 6 * self.row + value;
                canvas.draw_sprite(&self.tile_sprites[0], pt);
                canvas.draw_sprite(&self.symbol_sprites[symbol_index], pt);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut OrderState)
                    -> Action<(usize, usize)> {
        match event {
            &Event::ClockTick => {
                if let Some(ref mut drag) = self.drag {
                    return Action::redraw_if(drag.tick_animation());
                }
            }
            &Event::MouseDown(pt) if state.current_row() == self.row => {
                let num_tiles = state.row_order(self.row).len();
                for index in 0..num_tiles {
                    let tile_left = self.left + TILE_SPACING * (index as i32);
                    let rect = Rect::new(tile_left, self.top, 28, 28);
                    if rect.contains_point(pt) {
                        self.drag =
                            Some(TileDrag::new(index, pt.x(), num_tiles));
                        let sound = Sound::device_pickup();
                        return Action::redraw().and_play_sound(sound);
                    }
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
                    debug_assert_eq!(state.current_row(), self.row);
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

const TILE_SPACING: i32 = 29;
const TILE_SLIDE_SPEED: i32 = 6;

struct TileDrag {
    index: usize,
    new_index: usize,
    from: i32,
    to: i32,
    offsets: Vec<(i32, i32)>,
}

impl TileDrag {
    fn new(index: usize, from: i32, num_tiles: usize) -> TileDrag {
        TileDrag {
            index: index,
            new_index: index,
            from: from,
            to: from,
            offsets: vec![(0, 0); num_tiles],
        }
    }

    fn set_to(&mut self, to: i32) {
        self.to = to;
        let new_index =
            cmp::min(cmp::max(0,
                              self.new_index as i32 +
                                  (self.to - self.from) / TILE_SPACING),
                     self.offsets.len() as i32 - 1) as usize;
        if self.new_index != new_index {
            self.from += (new_index as i32 - self.new_index as i32) *
                TILE_SPACING;
            self.new_index = new_index;
            let old_index = self.index;
            for (index, &mut (_, ref mut goal)) in
                self.offsets.iter_mut().enumerate()
            {
                *goal = if index < old_index && index >= new_index {
                    TILE_SPACING
                } else if index > old_index && index <= new_index {
                    -TILE_SPACING
                } else {
                    0
                }
            }
        }
        let offset = (self.new_index as i32 - self.index as i32) *
            TILE_SPACING + self.to - self.from;
        self.offsets[self.index] = (offset, offset);
    }

    fn offset(&self, index: usize) -> i32 { self.offsets[index].0 }

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

const INFO_BOX_TEXT: &str = "\
Your goal is to put each row of tiles in alphabetical order.

Drag tiles left and right with $M{your finger}{the mouse} to reorder them.
Once a row is ordered correctly, the next row will appear.

$M{Tap}{Click} on a character in the scene to hear their words of
wisdom.";

// ========================================================================= //
