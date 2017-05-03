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

use num_integer::mod_floor;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PuzzleState, WhatchaState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(usize, i32)>,
    columns: Columns,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect,
               state: &WhatchaState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            columns: Columns::new(resources, 300, 200),
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for _ in self.core.drain_queue() {
            // TODO drain queue
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.whatcha_column;
        self.core.draw_back_layer(canvas);
        self.columns.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.whatcha_column;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() {
            let subaction = self.columns.handle_event(event, state);
            if let Some(&(col, by)) = subaction.value() {
                state.rotate_column(col, by);
                if state.is_solved() {
                    self.core.begin_outro_scene();
                } else {
                    self.core.push_undo((col, by));
                }
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.whatcha_column.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((col, by)) = self.core.pop_undo() {
            game.whatcha_column.rotate_column(col, -by);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((col, by)) = self.core.pop_redo() {
            game.whatcha_column.rotate_column(col, by);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.whatcha_column.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.whatcha_column.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const BOX_USIZE: u32 = 24;
const BOX_SIZE: i32 = BOX_USIZE as i32;
const ADJUST_SPEED: i32 = 3;

struct Columns {
    font: Rc<Font>,
    left: i32,
    top: i32,
    adjust: HashMap<usize, i32>,
    drag: Option<(usize, i32, i32)>,
}

impl Columns {
    fn new(resources: &mut Resources, left: i32, top: i32) -> Columns {
        Columns {
            font: resources.get_font("block"),
            left: left,
            top: top,
            adjust: HashMap::new(),
            drag: None,
        }
    }

    fn column_rect(&self, state: &WhatchaState, col: usize) -> Rect {
        let left = self.left + 32 * col as i32;
        let top = self.top + BOX_SIZE * state.column_offset(col);
        let num_chars = state.column_word_len(col) as u32;
        Rect::new(left, top, BOX_USIZE, BOX_USIZE * num_chars)
    }

    fn column_scroll(&self, state: &WhatchaState, col: usize) -> i32 {
        let mut scroll = self.adjust.get(&col).cloned().unwrap_or(0);
        if let Some((drag_col, from, to)) = self.drag {
            for &other in state.column_linkages(drag_col) {
                if other == col {
                    scroll += to - from;
                    break;
                }
            }
        }
        scroll
    }
}

impl Element<WhatchaState, (usize, i32)> for Columns {
    fn draw(&self, state: &WhatchaState, canvas: &mut Canvas) {
        for col in 0..state.num_columns() {
            let mut canvas = canvas.subcanvas(self.column_rect(state, col));
            canvas.clear((0, 0, 0));
            canvas.fill_rect((63, 31, 63),
                             Rect::new(0,
                                       -BOX_SIZE * state.column_offset(col),
                                       BOX_USIZE,
                                       BOX_USIZE));
            let height = canvas.height() as i32;
            let offset = mod_floor(self.column_scroll(state, col), height);
            for (index, &chr) in state.column_letters(col).iter().enumerate() {
                let top = BOX_SIZE * index as i32 + offset;
                let pt = Point::new(BOX_SIZE / 2, top + BOX_SIZE - 3);
                canvas.draw_char(&self.font, Align::Center, pt, chr);
                let pt = Point::new(BOX_SIZE / 2, top + BOX_SIZE - 3 - height);
                canvas.draw_char(&self.font, Align::Center, pt, chr);
            }
            let rect = canvas.rect();
            canvas.draw_rect((255, 255, 255), rect);
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut WhatchaState)
                    -> Action<(usize, i32)> {
        match event {
            &Event::ClockTick => {
                let any_adjustments = !self.adjust.is_empty();
                let mut done = Vec::new();
                for (&col, adjust) in self.adjust.iter_mut() {
                    if *adjust > 0 {
                        *adjust = cmp::max(0, *adjust - ADJUST_SPEED);
                    } else {
                        *adjust = cmp::min(0, *adjust + ADJUST_SPEED);
                    }
                    if *adjust == 0 {
                        done.push(col);
                    }
                }
                for col in done.into_iter() {
                    self.adjust.remove(&col);
                }
                return Action::redraw_if(any_adjustments);
            }
            &Event::MouseDown(pt) => {
                for col in 0..state.num_columns() {
                    if self.column_rect(state, col).contains(pt) {
                        self.drag = Some((col, pt.y(), pt.y()));
                        return Action::redraw();
                    }
                }
            }
            &Event::MouseDrag(pt) => {
                if let Some((_col, _from, ref mut to)) = self.drag {
                    *to = pt.y();
                    return Action::redraw();
                }
            }
            &Event::MouseUp => {
                if let Some((col, from, to)) = self.drag.take() {
                    let delta = to - from;
                    let boosted = if delta > 0 {
                        delta + BOX_SIZE / 2
                    } else {
                        delta - BOX_SIZE / 2
                    };
                    let by = boosted / BOX_SIZE;
                    let adjust = delta - (by * BOX_SIZE);
                    for &other in state.column_linkages(col) {
                        self.adjust.insert(other, adjust);
                    }
                    return if by == 0 {
                        Action::redraw()
                    } else {
                        Action::redraw().and_return((col, by))
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
Your goal is to slide the columns of letters until the
hilighted letters form a word horizontally across.
There is only one possible word that can be formed.

Drag a column up or down with $M{your finger}{the mouse} to rotate
its letters.  Moving one column may also cause other
columns to move at the same time.";

// ========================================================================= //
