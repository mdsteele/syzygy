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

use num_integer::div_floor;

use elements::{Paragraph, PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Point, Rect, Resources,
          Sound, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, LaneState, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

const FLIP_SLOWDOWN: i32 = 3;
const FLIP_COUNTDOWN_MAX: i32 = FLIP_SLOWDOWN * 5 - 1;
const REMOVE_DELAY: i32 = FLIP_SLOWDOWN * 5 + 20;
const REMOVE_SOUND_AT: i32 = 20 + FLIP_SLOWDOWN * 2;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    grid: MemoryGridView,
    next: NextShapeView,
    free: FreeSymbolView,
    progress: ProgressBar,
    prompt: PromptView,
    remove_countdown: i32,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &LaneState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            grid: MemoryGridView::new(resources, (208, 64), state),
            next: NextShapeView::new(resources, (96, 64)),
            free: FreeSymbolView::new(resources, (448, 112)),
            progress: ProgressBar::new((112, 176)),
            prompt: PromptView::new(resources),
            remove_countdown: 0,
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.core.drain_queue() {
            // TODO: drain queue
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.memory_lane;
        self.core.draw_back_layer(canvas);
        self.progress.draw(state, canvas);
        self.free.draw(state, canvas);
        self.grid.draw(state, canvas);
        if self.remove_countdown == 0 && self.next.drag.is_none() {
            self.prompt.draw(state, canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.next.draw(state, canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.memory_lane;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if event == &Event::ClockTick && self.remove_countdown > 0 {
            self.remove_countdown -= 1;
            if self.remove_countdown == REMOVE_SOUND_AT {
                let symbol = self.grid.flip_symbol;
                let sound = if state.can_remove_symbol(symbol) {
                    self.progress.adjust = 1;
                    Sound::mid_puzzle_chime()
                } else {
                    Sound::talk_annoyed_hi()
                };
                action.merge(Action::redraw().and_play_sound(sound));
            }
            if self.remove_countdown == 0 {
                self.progress.adjust = 0;
                state.remove_symbol(self.grid.flip_symbol);
                self.grid.clear_flip();
                if state.is_solved() {
                    self.core.begin_outro_scene();
                }
                action.merge(Action::redraw());
            }
        }
        if !action.should_stop() {
            let subaction = self.next.handle_event(event, state);
            if let Some(&pt) = subaction.value() {
                let pt = pt - self.grid.top_left();
                let col = div_floor(pt.x() + 16, 32);
                let row = div_floor(pt.y() + 16, 32);
                if let Some(symbol) = state.try_place_shape(col, row) {
                    action = action.and_play_sound(Sound::device_drop());
                    self.grid.place_symbol(symbol);
                }
            }
            action.merge(subaction.but_no_value());
        }
        if (!action.should_stop() && self.remove_countdown == 0) ||
           event == &Event::ClockTick {
            let subaction = self.grid.handle_event(event, state);
            if let Some(&symbol) = subaction.value() {
                action = action.and_play_sound(Sound::device_rotate());
                self.grid.reveal_symbol(symbol);
                self.remove_countdown = REMOVE_DELAY;
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.memory_lane.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, _: &mut Game) {}

    fn redo(&mut self, _: &mut Game) {}

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.memory_lane.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.memory_lane.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

struct MemoryGridView {
    rect: Rect,
    tile_sprites: Vec<Sprite>,
    symbol_sprites: Vec<Sprite>,
    flip_countdown: i32,
    flip_symbol: i8,
}

impl MemoryGridView {
    fn new(resources: &mut Resources, (left, top): (i32, i32),
           state: &LaneState)
           -> MemoryGridView {
        MemoryGridView {
            rect: Rect::new(left,
                            top,
                            32 * state.num_cols() as u32,
                            32 * state.num_rows() as u32),
            tile_sprites: resources.get_sprites("memory/tiles"),
            symbol_sprites: resources.get_sprites("memory/lane"),
            flip_countdown: 0,
            flip_symbol: 0,
        }
    }

    fn top_left(&self) -> Point { self.rect.top_left() }

    fn place_symbol(&mut self, symbol: i8) {
        self.flip_symbol = symbol;
        self.flip_countdown = FLIP_COUNTDOWN_MAX;
    }

    fn reveal_symbol(&mut self, symbol: i8) {
        self.flip_symbol = symbol;
        self.flip_countdown = 0;
    }

    fn clear_flip(&mut self) {
        self.flip_symbol = 0;
        self.flip_countdown = 0;
    }

    fn flip_tile_offset(&self) -> i32 {
        self.flip_countdown.abs() / FLIP_SLOWDOWN
    }
}

impl Element<LaneState, i8> for MemoryGridView {
    fn draw(&self, state: &LaneState, canvas: &mut Canvas) {
        canvas.fill_rect((31, 31, 31), self.rect);
        for (index, &value) in state.grid().iter().enumerate() {
            if value == 0 {
                continue;
            }
            let pt = Point::new(index as i32 % state.num_cols(),
                                index as i32 / state.num_cols()) *
                     32 + self.rect.top_left();
            let symbol = value.abs();
            let tile_index = if self.flip_symbol == symbol {
                let base = if self.flip_countdown > 0 {
                    5
                } else if value > 0 {
                    10
                } else {
                    0
                };
                base + self.flip_tile_offset()
            } else if value < 0 {
                0
            } else {
                5
            };
            canvas.draw_sprite(&self.tile_sprites[tile_index as usize], pt);
            if tile_index % 5 == 4 {
                let symbol_index = (symbol - 1) as usize * 2;
                canvas.draw_sprite(&self.symbol_sprites[symbol_index], pt);
            } else if tile_index % 5 == 3 {
                let symbol_index = (symbol - 1) as usize * 2 + 1;
                canvas.draw_sprite(&self.symbol_sprites[symbol_index], pt);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut LaneState)
                    -> Action<i8> {
        match event {
            &Event::ClockTick => {
                if self.flip_symbol != 0 &&
                   self.flip_countdown > -FLIP_COUNTDOWN_MAX {
                    let old_offset = self.flip_tile_offset();
                    self.flip_countdown -= 1;
                    let new_offset = self.flip_tile_offset();
                    if self.flip_countdown == 0 {
                        self.flip_symbol = 0;
                    }
                    Action::redraw_if(old_offset != new_offset)
                } else {
                    Action::ignore()
                }
            }
            &Event::MouseDown(pt) if self.rect.contains(pt) &&
                                     self.flip_symbol == 0 &&
                                     state.next_remove().is_some() => {
                let pt = pt - self.top_left();
                let col = pt.x() / 32;
                let row = pt.y() / 32;
                if let Some(symbol) = state.symbol_at(col, row) {
                    Action::redraw().and_return(symbol)
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct ShapeDrag {
    from: Point,
    to: Point,
}

impl ShapeDrag {
    fn new(from: Point) -> ShapeDrag {
        ShapeDrag {
            from: from,
            to: from,
        }
    }
}

struct NextShapeView {
    top_left: Point,
    tile_sprite: Sprite,
    symbol_sprites: Vec<Sprite>,
    drag: Option<ShapeDrag>,
}

impl NextShapeView {
    fn new(resources: &mut Resources, top_left: (i32, i32)) -> NextShapeView {
        NextShapeView {
            top_left: Point::from(top_left),
            tile_sprite: resources.get_sprites("memory/tiles")[4].clone(),
            symbol_sprites: resources.get_sprites("memory/lane"),
            drag: None,
        }
    }

    fn rect(&self) -> Rect {
        Rect::new(self.top_left.x(), self.top_left.y(), 96, 96)
    }

    fn cell_rect(&self, index: usize) -> Rect {
        Rect::new((index as i32 % 3) * 32, (index as i32 / 3) * 32, 32, 32)
    }
}

impl Element<LaneState, Point> for NextShapeView {
    fn draw(&self, state: &LaneState, canvas: &mut Canvas) {
        if let Some(shape) = state.next_shape() {
            let mut top_left = self.top_left;
            if let Some(ref drag) = self.drag {
                top_left = top_left - drag.from + drag.to;
            }
            for (index, &symbol) in shape.iter().enumerate() {
                if symbol > 0 {
                    let pt = self.cell_rect(index).top_left() + top_left;
                    canvas.draw_sprite(&self.tile_sprite, pt);
                    let idx = (symbol - 1) as usize * 2;
                    canvas.draw_sprite(&self.symbol_sprites[idx], pt);
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut LaneState)
                    -> Action<Point> {
        match event {
            &Event::MouseDown(pt) => {
                if let Some(shape) = state.next_shape() {
                    let rect = self.rect();
                    if rect.contains(pt) {
                        let rel_pt = pt - rect.top_left();
                        for (index, &symbol) in shape.iter().enumerate() {
                            if symbol != 0 &&
                               self.cell_rect(index).contains(rel_pt) {
                                self.drag = Some(ShapeDrag::new(pt));
                                let sound = Sound::device_pickup();
                                return Action::ignore().and_play_sound(sound);
                            }
                        }
                    }
                }
            }
            &Event::MouseDrag(pt) => {
                if let Some(ref mut drag) = self.drag {
                    drag.to = pt;
                    return Action::redraw();
                }
            }
            &Event::MouseUp => {
                if let Some(drag) = self.drag.take() {
                    let pt = self.top_left - drag.from + drag.to;
                    return Action::redraw().and_return(pt);
                }
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //

struct FreeSymbolView {
    top_left: Point,
    symbol_sprites: Vec<Sprite>,
}

impl FreeSymbolView {
    fn new(resources: &mut Resources, top_left: (i32, i32)) -> FreeSymbolView {
        FreeSymbolView {
            top_left: Point::from(top_left),
            symbol_sprites: resources.get_sprites("memory/lane"),
        }
    }

    fn draw(&self, state: &LaneState, canvas: &mut Canvas) {
        if let Some(symbol) = state.next_remove() {
            canvas.draw_sprite(&self.symbol_sprites[(symbol - 1) as usize *
                                                    2],
                               self.top_left);
        }
    }
}

// ========================================================================= //

struct ProgressBar {
    left: i32,
    top: i32,
    adjust: usize,
}

impl ProgressBar {
    fn new((left, top): (i32, i32)) -> ProgressBar {
        ProgressBar {
            left: left + 1,
            top: top + 1,
            adjust: 0,
        }
    }

    fn draw(&self, state: &LaneState, canvas: &mut Canvas) {
        if !state.is_solved() {
            let stage = state.current_stage() + self.adjust;
            if stage > 0 {
                let width = (78 * stage / state.total_num_stages()) as u32;
                canvas.fill_rect((191, 191, 0),
                                 Rect::new(self.left, self.top, width, 14));
            }
        }
    }
}

// ========================================================================= //

const PLACE_PROMPT: &'static str = "Drag the shape on the left onto the \
                                    center grid.";
const REMOVE_PROMPT: &'static str = "$M{Tap}{Click} any tile in the center \
                                     grid that had the symbol on the right.";

struct PromptView {
    place: Paragraph,
    remove: Paragraph,
}

impl PromptView {
    fn new(resources: &mut Resources) -> PromptView {
        PromptView {
            place: Paragraph::new(resources,
                                  "roman",
                                  Align::Center,
                                  PLACE_PROMPT),
            remove: Paragraph::new(resources,
                                   "roman",
                                   Align::Center,
                                   REMOVE_PROMPT),
        }
    }

    fn draw(&self, state: &LaneState, canvas: &mut Canvas) {
        let paragraph = if state.next_shape().is_some() {
            Some(&self.place)
        } else if state.next_remove().is_some() {
            Some(&self.remove)
        } else {
            None
        };
        if let Some(paragraph) = paragraph {
            let width = paragraph.min_width() + 8;
            let height = paragraph.height();
            let left = (canvas.width() / 2) as i32 - width / 2;
            let rect = Rect::new(left, 291, width as u32, height + 4);
            canvas.fill_rect((192, 192, 192), rect);
            canvas.draw_rect((128, 128, 128), rect);
            let rect = Rect::new(rect.x(), rect.y() + 2, rect.width(), height);
            let mut canvas = canvas.subcanvas(rect);
            paragraph.draw(&mut canvas);
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to place (and later remove) each group of tiles
on the grid.

When a group of tiles appears on the left, use $M{your finger}{the mouse} to
drag it onto the grid on the right.  The tiles will then flip over;
the backs of the tiles will be green.

Tiles will eventually turn from green to gray.  When all the
tiles with a given symbol are gray, you will be prompted to
remove them from the grid.  $M{Tap}{Click} any of the tiles on the grid
that had that symbol to remove all of them.

If you accidentally remove a tile that's still green, you will
have to start over.";

// ========================================================================= //
