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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use elements::ice::GridView;
use gui::{Action, Canvas, Element, Event, Rect, Resources};
use modes::SOLVED_INFO_TEXT;
use save::{Game, PuzzleState, VirtueState};
use save::ice::BlockSlide;
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<BlockSlide>,
    grid: GridView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &VirtueState)
               -> View {
        let core = {
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, intro, outro)
        };
        View {
            core: core,
            grid: GridView::new(resources, 80, 64, state.grid()),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.virtue_or_ice;
        self.core.draw_back_layer(canvas);
        self.grid.draw(state.grid(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.virtue_or_ice;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() {
            let subaction = self.grid.handle_event(event, state.grid_mut());
            if let Some(&(coords, dir)) = subaction.value() {
                if let Some(slide) = state.slide_ice_block(coords, dir) {
                    self.grid.animate_slide(&slide);
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                    } else {
                        self.core.push_undo(slide);
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
        if game.virtue_or_ice.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(slide) = self.core.pop_undo() {
            game.virtue_or_ice.grid_mut().undo_slide(&slide);
            self.grid.reset_animation();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(slide) = self.core.pop_redo() {
            game.virtue_or_ice.grid_mut().redo_slide(&slide);
            self.grid.reset_animation();
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.virtue_or_ice.reset();
        self.grid.reset_animation();
    }

    fn solve(&mut self, game: &mut Game) {
        game.virtue_or_ice.solve();
        self.grid.reset_animation();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 1 {
                if value >= 0 && (value as usize) < LETTERS.len() {
                    let (coords, chr) = LETTERS[value as usize];
                    self.grid.add_letter(coords, chr);
                }
            }
        }
    }
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const LETTERS: &[((i32, i32), char)] = &[
    // "PROTECTIVE":
    ((0, 5), 'C'),
    ((0, 4), 'E'),
    ((0, 6), 'T'),
    ((1, 4), 'V'),
    ((1, 6), 'O'),
    ((2, 4), 'I'),
    ((2, 6), 'R'),
    ((3, 6), 'P'),
    // "RESOURCEFUL":
    ((3, 1), 'R'),
    ((2, 1), 'C'),
    ((4, 1), 'U'),
    ((4, 0), 'L'),
    ((4, 2), 'F'),
    ((1, 1), 'E'),
    ((1, 0), 'S'),
    ((1, 2), 'R'),
    ((5, 1), 'O'),
];

const INFO_BOX_TEXT: &str = "\
Your goal is to slide the blocks of ice until each one
covers its matching symbol on the grid, in the same
orientation and chirality.

Drag one of the ice blocks up, down, left, or right with
$M{your finger}{the mouse} to slide it in that direction.";

// ========================================================================= //
