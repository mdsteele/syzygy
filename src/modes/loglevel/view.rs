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
use save::{Game, LogLevelState, PuzzleState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32, char, char)>,
    crossword: CrosswordView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &LogLevelState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            crossword: CrosswordView::new(resources, 364, 56, OFFSETS_CLUES),
        };
        view.drain_queue();
        view
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

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.log_level;
        self.core.draw_back_layer(canvas);
        self.crossword.draw(state.crossword(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.log_level;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
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
                    self.core.clear_undo_redo();
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
        if game.log_level.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((row, index, chr, _)) = self.core.pop_undo() {
            game.log_level.crossword_mut().set_char(row, index, chr);
            self.crossword.reset_cursor();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((row, index, _, chr)) = self.core.pop_redo() {
            game.log_level.crossword_mut().set_char(row, index, chr);
            self.crossword.reset_cursor();
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.log_level.reset();
        self.crossword.reset_cursor();
    }

    fn replay(&mut self, game: &mut Game) {
        game.log_level.replay();
        self.crossword.reset_cursor();
        self.crossword.set_center_word_hilighted(false);
        self.core.replay();
        self.drain_queue();
    }

    fn solve(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.log_level.solve();
        self.crossword.reset_cursor();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const OFFSETS_CLUES: &'static [(i32, &'static str)] = &[
    (5, "relaxed or casual in manner or dress"),
    (1, "in Spanish, ``la'' rather than ``el''"),
    (1, "an animated TV show"),
    (1, "very good; marvelous"),
    (1, "crime-solving science"),
    (2, "to make holes in"),
    (6, "coincidental; serendipitous"),
    (1, "the study of matter and energy"),
    (2, "an ingredient in tonic water"),
    (0, "to separate words with symbols"),
];

const INFO_BOX_TEXT: &'static str = "\
Your goal is to fill in the crossword.

$M{Tap}{Click} on a box to select it, then type in the
word that matches the given clue, using the
$M{on-screen }{}keyboard.

If the word won't fit, you may need to fudge
the numbers.";

// ========================================================================= //
