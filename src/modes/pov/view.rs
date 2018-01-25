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

use elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Canvas, Element, Event, FRAME_DELAY_MILLIS, Font, Point,
          Rect, Resources, Sound, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PovState, PuzzleState};
use super::scenes;

// ========================================================================= //

#[derive(Clone, Copy)]
pub enum PovCmd {
    Moved((i32, i32), (i32, i32)),
    Rotated((i32, i32)),
}

// ========================================================================= //

pub struct View {
    core: PuzzleCore<PovCmd>,
    grid: PovGridView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &PovState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_argony_midscene(resources));
        core.add_extra_scene(scenes::compile_yttris_midscene(resources));
        View {
            core: core,
            grid: PovGridView::new(resources, 128, 64),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.point_of_view;
        self.core.draw_back_layer(canvas);
        self.core.draw_middle_layer(canvas);
        self.grid.draw(state, canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.point_of_view;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() {
            let subaction = self.grid.handle_event(event, state);
            if let Some(&cmd) = subaction.value() {
                if state.is_solved() {
                    self.core.begin_outro_scene();
                } else {
                    self.core.push_undo(cmd);
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
        if game.point_of_view.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(cmd) = self.core.pop_undo() {
            let state = &mut game.point_of_view;
            match cmd {
                PovCmd::Moved(from, to) => {
                    state.move_tile(to, from);
                }
                PovCmd::Rotated(coords) => {
                    state.rotate_tile(coords, -1);
                }
            }
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(cmd) = self.core.pop_redo() {
            let state = &mut game.point_of_view;
            match cmd {
                PovCmd::Moved(from, to) => {
                    state.move_tile(from, to);
                }
                PovCmd::Rotated(coords) => {
                    state.rotate_tile(coords, 1);
                }
            }
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.point_of_view.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.point_of_view.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (command, value) in self.core.drain_queue() {
            if command == 0 && value >= 0 {
                self.grid.num_letters = value as usize;
            }
        }
    }
}

// ========================================================================= //

const GRID_CELL_SIZE: i32 = 32;
const INDICATOR_GOAL_THICKNESS: i32 = 7;
const INDICATOR_COLOR_THICKNESS: i32 = 3;
const INDICATOR_MARGIN: i32 = 2;
const INDICATOR_SPACING: i32 = 2;
const INDICATOR_LENGTH: u32 = (GRID_CELL_SIZE - 2 * INDICATOR_MARGIN) as u32;
const INDICATOR_TOTAL_THICKNESS: i32 =
    INDICATOR_MARGIN + INDICATOR_GOAL_THICKNESS + INDICATOR_SPACING +
        INDICATOR_COLOR_THICKNESS + INDICATOR_MARGIN;
const ROTATE_MAX_MILLIS: u32 = 200;

#[cfg_attr(rustfmt, rustfmt_skip)]
const LETTERS: &[(i32, i32, char, i32)] = &[
    (2, 2, 'M', 270),
    (1, 2, 'I', 90),
    (3, 2, 'R', 180),
    (1, 1, 'N', 0),
    (4, 2, 'E', 270),
    (0, 1, 'E', 90),
    (4, 3, 'T', 180),
    (0, 0, 'D', 180),
    (4, 4, 'E', 180),
    (3, 4, 'D', 270),
];

fn get_color(color: u8) -> (u8, u8, u8) {
    match color {
        0 => (255, 0, 0),
        1 => (255, 255, 0),
        2 => (0, 255, 0),
        3 => (0, 0, 255),
        4 => (255, 0, 255),
        _ => (32, 32, 32),
    }
}

struct GridDrag {
    tile: [u8; 4],
    from_coords: (i32, i32),
    from_pt: Point,
    to_pt: Point,
    millis: u32,
    moved: bool,
}

pub struct PovGridView {
    rect: Rect,
    tile_sprites: Vec<Sprite>,
    drag: Option<GridDrag>,
    font: Rc<Font>,
    num_letters: usize,
}

impl PovGridView {
    fn new(resources: &mut Resources, left: i32, top: i32) -> PovGridView {
        PovGridView {
            rect: Rect::new(left,
                            top,
                            (5 * GRID_CELL_SIZE) as u32,
                            (5 * GRID_CELL_SIZE) as u32),
            tile_sprites: resources.get_sprites("point/view"),
            drag: None,
            font: resources.get_font("block"),
            num_letters: 0,
        }
    }

    fn draw_tile(&self, top_left: Point, tile: [u8; 4], canvas: &mut Canvas) {
        for (index, &color) in tile.iter().enumerate() {
            let sprite_index = 4 * (color as usize) + index;
            canvas.draw_sprite(&self.tile_sprites[sprite_index], top_left);
        }
    }
}

impl Element<PovState, PovCmd> for PovGridView {
    fn draw(&self, state: &PovState, canvas: &mut Canvas) {
        canvas.fill_rect((64, 64, 64), self.rect);
        for index in 0..self.num_letters {
            let (col, row, chr, degrees) = LETTERS[index];
            let center = Point::new(self.rect.left() + GRID_CELL_SIZE * col +
                                        GRID_CELL_SIZE / 2,
                                    self.rect.top() + GRID_CELL_SIZE * row +
                                        GRID_CELL_SIZE / 2);
            let sprite = self.font.glyph(chr).sprite();
            canvas.draw_sprite_rotated(sprite, center, degrees);
        }
        for row in 0..5 {
            // Left:
            let goal = state.row_left_goal(row);
            let color = state.row_left_color(row);
            let left = self.rect.left() - INDICATOR_TOTAL_THICKNESS;
            let top = self.rect.top() + GRID_CELL_SIZE * row;
            canvas.fill_rect(get_color(goal),
                             Rect::new(left + INDICATOR_MARGIN,
                                       top + INDICATOR_MARGIN,
                                       INDICATOR_GOAL_THICKNESS as u32,
                                       INDICATOR_LENGTH));
            canvas.fill_rect(get_color(color),
                             Rect::new(left + INDICATOR_MARGIN +
                                           INDICATOR_GOAL_THICKNESS +
                                           INDICATOR_SPACING,
                                       top + INDICATOR_MARGIN,
                                       INDICATOR_COLOR_THICKNESS as u32,
                                       INDICATOR_LENGTH));
            // Right:
            let goal = state.row_right_goal(row);
            let color = state.row_right_color(row);
            let left = self.rect.right();
            canvas.fill_rect(get_color(color),
                             Rect::new(left + INDICATOR_MARGIN,
                                       top + INDICATOR_MARGIN,
                                       INDICATOR_COLOR_THICKNESS as u32,
                                       INDICATOR_LENGTH));
            canvas.fill_rect(get_color(goal),
                             Rect::new(left + INDICATOR_MARGIN +
                                           INDICATOR_COLOR_THICKNESS +
                                           INDICATOR_SPACING,
                                       top + INDICATOR_MARGIN,
                                       INDICATOR_GOAL_THICKNESS as u32,
                                       INDICATOR_LENGTH));
        }
        for col in 0..5 {
            // Top:
            let goal = state.col_top_goal(col);
            let color = state.col_top_color(col);
            let left = self.rect.left() + GRID_CELL_SIZE * col;
            let top = self.rect.top() - INDICATOR_TOTAL_THICKNESS;
            canvas.fill_rect(get_color(goal),
                             Rect::new(left + INDICATOR_MARGIN,
                                       top + INDICATOR_MARGIN,
                                       INDICATOR_LENGTH,
                                       INDICATOR_GOAL_THICKNESS as u32));
            canvas.fill_rect(get_color(color),
                             Rect::new(left + INDICATOR_MARGIN,
                                       top + INDICATOR_MARGIN +
                                           INDICATOR_GOAL_THICKNESS +
                                           INDICATOR_SPACING,
                                       INDICATOR_LENGTH,
                                       INDICATOR_COLOR_THICKNESS as u32));
            // Bottom:
            let goal = state.col_bottom_goal(col);
            let color = state.col_bottom_color(col);
            let top = self.rect.bottom();
            canvas.fill_rect(get_color(color),
                             Rect::new(left + INDICATOR_MARGIN,
                                       top + INDICATOR_MARGIN,
                                       INDICATOR_LENGTH,
                                       INDICATOR_COLOR_THICKNESS as u32));
            canvas.fill_rect(get_color(goal),
                             Rect::new(left + INDICATOR_MARGIN,
                                       top + INDICATOR_MARGIN +
                                           INDICATOR_COLOR_THICKNESS +
                                           INDICATOR_SPACING,
                                       INDICATOR_LENGTH,
                                       INDICATOR_GOAL_THICKNESS as u32));
        }
        for (&coords, &tile) in state.tiles() {
            if let Some(ref drag) = self.drag {
                if coords == drag.from_coords && drag.from_pt != drag.to_pt {
                    continue;
                }
            }
            let pt = Point::new(self.rect.left() + coords.0 * GRID_CELL_SIZE,
                                self.rect.top() + coords.1 * GRID_CELL_SIZE);
            self.draw_tile(pt, tile, canvas);
        }
        if let Some(ref drag) = self.drag {
            if drag.from_pt != drag.to_pt {
                let pt = self.rect.top_left() + drag.to_pt -
                    Point::new(GRID_CELL_SIZE / 2, GRID_CELL_SIZE / 2);
                self.draw_tile(pt, drag.tile, canvas);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut PovState)
                    -> Action<PovCmd> {
        match event {
            &Event::ClockTick => {
                if let Some(ref mut drag) = self.drag {
                    drag.millis = cmp::min(drag.millis + FRAME_DELAY_MILLIS,
                                           ROTATE_MAX_MILLIS + 1);
                }
            }
            &Event::MouseDown(pt) if !state.is_solved() => {
                if self.rect.contains(pt) {
                    let pt = pt - self.rect.top_left();
                    let col = pt.x() / GRID_CELL_SIZE;
                    let row = pt.y() / GRID_CELL_SIZE;
                    if let Some(tile) = state.tile_at((col, row)) {
                        self.drag = Some(GridDrag {
                                             tile: tile,
                                             from_coords: (col, row),
                                             from_pt: pt,
                                             to_pt: pt,
                                             millis: 0,
                                             moved: false,
                                         });
                    }
                }
            }
            &Event::MouseDrag(pt) => {
                if let Some(ref mut drag) = self.drag {
                    drag.to_pt = pt - self.rect.top_left();
                    let mut action = Action::redraw();
                    if !drag.moved {
                        drag.moved = true;
                        action.also_play_sound(Sound::device_pickup());
                    }
                    return action;
                }
            }
            &Event::MouseUp => {
                if let Some(drag) = self.drag.take() {
                    let to_coords = (drag.to_pt.x() / GRID_CELL_SIZE,
                                     drag.to_pt.y() / GRID_CELL_SIZE);
                    return if to_coords == drag.from_coords {
                        if drag.millis <= ROTATE_MAX_MILLIS {
                            state.rotate_tile(drag.from_coords, 1);
                            Action::redraw()
                                .and_play_sound(Sound::device_rotate())
                                .and_return(PovCmd::Rotated(drag.from_coords))
                        } else {
                            Action::redraw()
                        }
                    } else {
                        let success =
                            state.move_tile(drag.from_coords, to_coords);
                        if success {
                            Action::redraw()
                                .and_play_sound(Sound::device_drop())
                                .and_return(PovCmd::Moved(drag.from_coords,
                                                          to_coords))
                        } else {
                            Action::redraw()
                        }
                    };
                }
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &str = "\
Your goal is to arrange the tiles in the grid so that the
colors seen from the edges of the grid (as shown by
the inner ring of indicators) match the goal pattern
(as shown by the outer ring of indicators).

Drag tiles with $M{your finger}{the mouse} to move their positions in
the grid.  $M{Tap}{Click} tiles to rotate them.

$M{Tap}{Click} on a character in the scene to hear their words
of wisdom.";

// ========================================================================= //
