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

use elements::{CrosswordView, PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Canvas, Element, Event, Rect, Resources};
use modes::SOLVED_INFO_TEXT;
use save::{Game, HeadedState, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32, char, char)>,
    crossword: CrosswordView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &HeadedState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        View {
            core: core,
            crossword: CrosswordView::new(resources,
                                          (427, 76),
                                          OFFSETS_CLUES,
                                          (416, 310)),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.level_headed;
        self.core.draw_back_layer(canvas);
        self.crossword.draw(state.crossword(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.level_headed;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() &&
           (event == &Event::ClockTick || !state.is_solved()) {
            let subaction = self.crossword
                                .handle_event(event, state.crossword_mut());
            if let Some(&(row, index, chr)) = subaction.value() {
                let old_chr = state.crossword().get_char(row, index);
                state.crossword_mut().set_char(row, index, chr);
                state.check_if_solved();
                if state.is_solved() {
                    self.crossword.reset_cursor();
                    self.core.begin_outro_scene();
                } else {
                    self.core.push_undo((row, index, old_chr, chr));
                }
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.level_headed.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((row, index, chr, _)) = self.core.pop_undo() {
            game.level_headed.crossword_mut().set_char(row, index, chr);
            self.crossword.reset_cursor();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((row, index, _, chr)) = self.core.pop_redo() {
            game.level_headed.crossword_mut().set_char(row, index, chr);
            self.crossword.reset_cursor();
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.level_headed.reset();
        self.crossword.reset_cursor();
    }

    fn solve(&mut self, game: &mut Game) {
        game.level_headed.solve();
        self.crossword.reset_cursor();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for entry in self.core.drain_queue() {
            match entry {
                (0, 0) => self.crossword.animate_center_word(),
                (0, 1) => self.crossword.set_center_word_hilighted(true),
                _ => {}
            }
        }
    }
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const OFFSETS_CLUES: &[(i32, &str)] = &[
    (3, "a honey factory"),
    (2, "an illicit escapade"),
    (1, "the fifth noble gas"),
    (2, "to bleach"),
    (3, "a senior figure"),
    (2, "a white pool sphere"),
    (3, "overjoyed; rapturous"),
    (2, "the ocean floor"),
    (3, "an organic lens cap"),
];

const INFO_BOX_TEXT: &str = "\
Your goal is to fill in the crossword.

$M{Tap}{Click} on a box to select it, then type in the
word that matches the given clue, using the
$M{on-screen }{}keyboard.

If the word won't fit, you may need to turn
the problem on its head.";

// ========================================================================= //
