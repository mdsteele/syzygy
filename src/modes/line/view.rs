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

use std::rc::Rc;

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect, Resources};
use modes::SOLVED_INFO_TEXT;
use save::{Game, LineState, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    grid1: LetterGrid,
    grid2: LetterGrid,
    answers: AnswersDisplay,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &LineState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            grid1: LetterGrid::new(resources, 80, 48, false),
            grid2: LetterGrid::new(resources, 320, 48, true),
            answers: AnswersDisplay::new(resources, 168, 272),
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
        let state = &game.cross_the_line;
        self.core.draw_back_layer(canvas);
        self.grid1.draw(state, canvas);
        self.grid2.draw(state, canvas);
        self.answers.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.cross_the_line;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() {
            let mut subaction = self.grid1.handle_event(event, state);
            if !subaction.should_stop() {
                subaction.merge(self.grid2.handle_event(event, state));
            }
            if let Some(&()) = subaction.value() {
                if let Some(index1) = self.grid1.selected {
                    if let Some(index2) = self.grid2.selected {
                        // TODO: short delay first
                        self.grid1.selected = None;
                        self.grid2.selected = None;
                        // TODO: play sound based on pick_chars result
                        state.pick_chars(index1, index2);
                        if state.is_solved() {
                            self.core.begin_outro_scene();
                        }
                    }
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            let subaction = self.answers.handle_event(event, state);
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.cross_the_line.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, _: &mut Game) {}

    fn redo(&mut self, _: &mut Game) {}

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.cross_the_line.reset();
    }

    fn replay(&mut self, game: &mut Game) {
        game.cross_the_line.replay();
        self.core.replay();
        self.drain_queue();
    }

    fn solve(&mut self, game: &mut Game) {
        game.cross_the_line.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const BOX_USIZE: u32 = 24;
const BOX_SIZE: i32 = BOX_USIZE as i32;
const MAX_GRID_WIDTH: u32 = 176;
const MAX_GRID_HEIGHT: u32 = 144;

struct LetterGrid {
    left: i32,
    top: i32,
    is_grid_2: bool,
    font: Rc<Font>,
    selected: Option<usize>,
}

impl LetterGrid {
    fn new(resources: &mut Resources, left: i32, top: i32, is_grid_2: bool)
           -> LetterGrid {
        LetterGrid {
            left: left,
            top: top,
            is_grid_2: is_grid_2,
            font: resources.get_font("block"),
            selected: None,
        }
    }

    fn max_rect(&self) -> Rect {
        Rect::new(self.left, self.top, MAX_GRID_WIDTH, MAX_GRID_HEIGHT)
    }

    fn grid_rect(&self, num_cols: i32, num_chars: usize) -> Rect {
        let num_rows = (num_chars as i32 + num_cols - 1) / num_cols;
        let width = num_cols * BOX_SIZE;
        let height = num_rows * BOX_SIZE;
        let left = self.left + (MAX_GRID_WIDTH as i32 - width) / 2;
        let top = self.top + (MAX_GRID_HEIGHT as i32 - height) / 2;
        Rect::new(left, top, width as u32, height as u32)
    }
}

impl Element<LineState, ()> for LetterGrid {
    fn draw(&self, state: &LineState, canvas: &mut Canvas) {
        let num_cols = state.num_cols();
        let grid = if self.is_grid_2 {
            state.grid2()
        } else {
            state.grid1()
        };
        let grid_rect = self.grid_rect(num_cols, grid.len());
        let mut col = 0;
        let mut row = 0;
        for (index, &chr) in grid.iter().enumerate() {
            let box_left = grid_rect.left() + col * BOX_SIZE;
            let box_top = grid_rect.top() + row * BOX_SIZE;
            if self.selected == Some(index) {
                let rect = Rect::new(box_left, box_top, BOX_USIZE, BOX_USIZE);
                canvas.fill_rect((255, 255, 128), rect);
            }
            let pt = Point::new(box_left + BOX_SIZE / 2,
                                box_top + BOX_SIZE - 3);
            canvas.draw_char(&self.font, Align::Center, pt, chr);
            col += 1;
            if col >= num_cols {
                col = 0;
                row += 1;
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut LineState)
                    -> Action<()> {
        match event {
            &Event::MouseDown(pt) => {
                let num_cols = state.num_cols();
                let num_chars = if self.is_grid_2 {
                    state.grid2().len()
                } else {
                    state.grid1().len()
                };
                let rect = self.grid_rect(num_cols, num_chars);
                let mut new_selected = self.selected;
                if rect.contains(pt) {
                    let col = (pt.x() - rect.left()) / BOX_SIZE;
                    let row = (pt.y() - rect.top()) / BOX_SIZE;
                    let index = (row * num_cols + col) as usize;
                    if index >= num_chars || self.selected == Some(index) {
                        new_selected = None;
                    } else {
                        new_selected = Some(index);
                    }
                } else if self.max_rect().contains(pt) {
                    new_selected = None;
                }
                if new_selected != self.selected {
                    self.selected = new_selected;
                    return Action::redraw().and_return(());
                }
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //

struct AnswersDisplay {
    left: i32,
    top: i32,
    font: Rc<Font>,
}


impl AnswersDisplay {
    fn new(resources: &mut Resources, left: i32, top: i32) -> AnswersDisplay {
        AnswersDisplay {
            left: left,
            top: top,
            font: resources.get_font("block"),
        }
    }
}

impl Element<LineState, ()> for AnswersDisplay {
    fn draw(&self, state: &LineState, canvas: &mut Canvas) {
        // TODO: When solved, display only a certain subset of the letters.
        for stage in 0..state.current_stage() {
            let (chr1, chr2) = state.stage_letters(stage);
            let cx = self.left + stage * BOX_SIZE + BOX_SIZE / 2;
            let pt1 = Point::new(cx, self.top + BOX_SIZE - 3);
            let pt2 = Point::new(cx, self.top + 2 * BOX_SIZE - 3);
            canvas.draw_char(&self.font, Align::Center, pt1, chr1);
            canvas.draw_char(&self.font, Align::Center, pt2, chr2);
        }
    }

    fn handle_event(&mut self, event: &Event, _state: &mut LineState)
                    -> Action<()> {
        match event {
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to find the discrepancy between the
two upper grids.

Each of the two upper grids contains a character
that does not appear in the other.  $M{Tap}{Click} each of
those two characters to proceed.  If you choose
incorrectly, the grids will rescramble and you
will have to try again.";

// ========================================================================= //
