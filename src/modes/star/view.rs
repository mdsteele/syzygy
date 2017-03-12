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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources,
          Sound, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PuzzleState, StarState, WordDir};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    wordlist: WordList,
    columns: LetterColumns,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &StarState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            wordlist: WordList::new(resources),
            columns: LetterColumns::new(resources),
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for (command, enable) in self.core.drain_queue() {
            if command == 0 {
                self.columns.animate_hilight(enable != 0);
            }
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.star_crossed;
        self.core.draw_back_layer(canvas);
        self.columns.draw(state, canvas);
        self.wordlist.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.star_crossed;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() {
            let subaction = self.wordlist.handle_event(event, state);
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            let subaction = self.columns.handle_event(event, state);
            if let Some(&(col, row, dir, len)) = subaction.value() {
                if state.try_remove_word(col, row, dir, len) {
                    action = action.and_play_sound(Sound::mid_puzzle_chime());
                    self.columns.animate_fall(col, row, dir, len);
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                        self.drain_queue();
                    }
                }
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.star_crossed.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, _: &mut Game) {}

    fn redo(&mut self, _: &mut Game) {}

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.star_crossed.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.star_crossed.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const WORD_LIST_NUM_ROWS: i32 = 12;

struct WordList {
    font: Rc<Font>,
}

impl WordList {
    fn new(resources: &mut Resources) -> WordList {
        WordList { font: resources.get_font("system") }
    }
}

impl Element<StarState, ()> for WordList {
    fn draw(&self, state: &StarState, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(Rect::new(80, 48, 144, 160));
        for index in 0..state.num_words() {
            if !state.word_is_found(index) {
                let word = state.word(index);
                let index = index as i32;
                let pt = Point::new(4 + 78 * (index / WORD_LIST_NUM_ROWS),
                                    13 + 13 * (index % WORD_LIST_NUM_ROWS));
                canvas.draw_text(&self.font, Align::Left, pt, word);
            }
        }
    }

    fn handle_event(&mut self, _: &Event, _: &mut StarState) -> Action<()> {
        Action::ignore()
    }
}

// ========================================================================= //

const BLOCK_WIDTH: i32 = 24;
const BLOCK_HEIGHT: i32 = 24;

struct LetterColumns {
    font: Rc<Font>,
    sprites: Vec<Sprite>,
    drag: Option<Drag>,
    fall_anim: [(i32, i32, i32); 10],
    hilight_anim: i32,
}

impl LetterColumns {
    fn new(resources: &mut Resources) -> LetterColumns {
        LetterColumns {
            font: resources.get_font("block"),
            sprites: resources.get_sprites("cross/star"),
            drag: None,
            fall_anim: [(0, 0, 0); 10],
            hilight_anim: 0,
        }
    }

    fn animate_fall(&mut self, start_col: i32, start_row: i32, dir: WordDir,
                    length: i32) {
        match dir {
            WordDir::Vertical => {
                self.fall_anim[start_col as usize] =
                    (start_row + 1 - length, length * BLOCK_HEIGHT, 0);
            }
            _ => {
                let delta = dir.delta().y();
                let mut row = start_row;
                for col in start_col..(start_col + length) {
                    self.fall_anim[col as usize] = (row, BLOCK_HEIGHT, 0);
                    row += delta;
                }
            }
        }
    }

    fn animate_hilight(&mut self, enable: bool) {
        self.hilight_anim = if enable { 1 } else { 0 };
    }

    fn rect(&self) -> Rect { Rect::new(256, 56, 240, 240) }

    fn hilighted_coords(&self) -> HashSet<(i32, i32)> {
        if let Some(ref drag) = self.drag {
            drag.hilighted_coords()
        } else {
            HashSet::new()
        }
    }
}

impl Element<StarState, (i32, i32, WordDir, i32)> for LetterColumns {
    fn draw(&self, state: &StarState, canvas: &mut Canvas) {
        let hilighted = self.hilighted_coords();
        let rect = self.rect();
        let mut canvas = canvas.subcanvas(rect);
        for col in 0..state.num_columns() {
            for (row, &letter) in state.column_letters(col)
                                       .iter()
                                       .enumerate() {
                let row = row as i32;
                let sprite_idx = if self.hilight_anim > col &&
                                    self.hilight_anim <= 10 + col {
                    2
                } else if hilighted.contains(&(col, row)) {
                    1
                } else {
                    0
                };
                let (gap_row, gap, _) = self.fall_anim[col as usize];
                let gap = if row >= gap_row { gap } else { 0 };
                let pt = Point::new(col * BLOCK_WIDTH,
                                    rect.height() as i32 - gap -
                                    (1 + row) * BLOCK_HEIGHT);
                canvas.draw_sprite(&self.sprites[sprite_idx], pt);
                let pt = pt + Point::new(BLOCK_WIDTH / 2, BLOCK_HEIGHT - 3);
                canvas.draw_char(&self.font, Align::Center, pt, letter);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut StarState)
                    -> Action<(i32, i32, WordDir, i32)> {
        match event {
            &Event::ClockTick => {
                let mut redraw = false;
                for &mut (_, ref mut gap, ref mut speed) in
                    self.fall_anim.iter_mut() {
                    if *gap > 0 {
                        *gap = cmp::max(0, *gap + *speed);
                        *speed -= 1;
                        redraw = true;
                    }
                }
                if self.hilight_anim > 0 {
                    self.hilight_anim += 1;
                    if self.hilight_anim >= 20 {
                        self.hilight_anim = 0;
                    }
                    redraw = true;
                }
                Action::redraw_if(redraw)
            }
            &Event::MouseDown(pt) => {
                let rect = self.rect();
                if rect.contains(pt) {
                    let col = (pt.x() - rect.left()) / BLOCK_WIDTH;
                    let row = (rect.bottom() - pt.y()) / BLOCK_HEIGHT;
                    if row < state.column_letters(col).len() as i32 {
                        self.drag = Some(Drag::new(col, row));
                        return Action::redraw();
                    }
                }
                Action::ignore()
            }
            &Event::MouseDrag(pt) => {
                let rect = self.rect();
                if let Some(ref mut drag) = self.drag {
                    let from = rect.bottom_left() +
                               Point::new(drag.start_col * BLOCK_WIDTH +
                                          BLOCK_WIDTH / 2,
                                          -drag.start_row * BLOCK_HEIGHT -
                                          BLOCK_HEIGHT / 2);
                    Action::redraw_if(drag.set_delta(pt - from, state))
                } else {
                    Action::ignore()
                }
            }
            &Event::MouseUp => {
                if let Some(drag) = self.drag.take() {
                    Action::redraw().and_return(drag.normalize())
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct Drag {
    start_col: i32,
    start_row: i32,
    direction: Point,
    length: i32,
}

impl Drag {
    fn new(col: i32, row: i32) -> Drag {
        Drag {
            start_col: col,
            start_row: row,
            direction: Point::new(1, 0),
            length: 1,
        }
    }

    fn hilighted_coords(&self) -> HashSet<(i32, i32)> {
        let mut result = HashSet::new();
        let mut pt = Point::new(self.start_col, self.start_row);
        for _ in 0..self.length {
            result.insert((pt.x(), pt.y()));
            pt = pt + self.direction;
        }
        result
    }

    fn set_delta(&mut self, delta: Point, state: &StarState) -> bool {
        let mut redraw = false;
        let xabs = delta.x().abs();
        let yabs = delta.y().abs();
        if xabs < BLOCK_WIDTH + BLOCK_WIDTH / 2 &&
           yabs < BLOCK_HEIGHT + BLOCK_HEIGHT / 2 {
            let direction = if xabs > 2 * yabs {
                Point::new(delta.x().signum(), 0)
            } else if yabs > 2 * xabs {
                Point::new(0, -delta.y().signum())
            } else {
                Point::new(delta.x().signum(), -delta.y().signum())
            };
            if self.direction != direction {
                self.direction = direction;
                if self.length > 1 {
                    redraw = true;
                }
            }
        }
        let dx = delta.x() * self.direction.x();
        let dy = -delta.y() * self.direction.y();
        let dist = if dx.abs() > dy.abs() {
            (dx + dx.signum() * BLOCK_WIDTH / 2) / BLOCK_WIDTH
        } else {
            (dy + dy.signum() * BLOCK_HEIGHT / 2) / BLOCK_HEIGHT
        };
        if dist < 0 {
            self.direction = -self.direction;
            if self.length > 1 {
                redraw = true;
            }
        }
        let mut length = dist.abs() + 1;
        for offset in 1..length {
            let pt = Point::new(self.start_col, self.start_row) +
                     self.direction * offset;
            if pt.x() < 0 || pt.x() >= state.num_columns() || pt.y() < 0 ||
               pt.y() >= state.column_letters(pt.x()).len() as i32 {
                length = offset;
                break;
            }
        }
        if self.length != length {
            self.length = length;
            redraw = true;
        }
        redraw
    }

    fn normalize(&self) -> (i32, i32, WordDir, i32) {
        let col = self.start_col;
        let row = self.start_row;
        let len = self.length;
        let dist = len - 1;
        match (self.direction.x(), self.direction.y()) {
            (1, 0) => (col, row, WordDir::Horizontal, len),
            (1, 1) => (col, row, WordDir::DiagUp, len),
            (0, 1) => (col, row - dist, WordDir::Vertical, len),
            (-1, 1) => (col - dist, row + dist, WordDir::DiagDown, len),
            (-1, 0) => (col - dist, row, WordDir::Horizontal, len),
            (-1, -1) => (col - dist, row - dist, WordDir::DiagUp, len),
            (0, -1) => (col, row, WordDir::Vertical, len),
            (1, -1) => (col, row, WordDir::DiagDown, len),
            _ => panic!("invalid direction: {:?}", self.direction),
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to remove all 24 listed words from the grid.

Search across, down, and diagonally within the grid of letters
on the right for one of the words listed on the left, then $M{tap}{click}
and drag with $M{your finger}{the mouse} across the letters of that word to
remove them from the grid.  The remaining grid letters will
fall into place.  Continue this process until the list is empty.";

// ========================================================================= //
