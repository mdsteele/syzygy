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
use save::{Game, LevelUpState, PuzzleState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(i32, i32, char, char)>,
    crossword: CrosswordView,
    crossword_visible: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect,
               state: &LevelUpState)
               -> View {
        let mut core = {
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources, visible);
            PuzzleCore::new(resources, visible, state, intro, outro)
        };
        core.add_extra_scene(scenes::compile_elinsa_midscene(resources));
        core.add_extra_scene(scenes::compile_ugrent_midscene(resources));
        View {
            core: core,
            crossword: CrosswordView::new(resources,
                                          (392, 55),
                                          OFFSETS_CLUES,
                                          (392, 310)),
            crossword_visible: false,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.level_up;
        self.core.draw_back_layer(canvas);
        if self.crossword_visible {
            self.crossword.draw(state.crossword(), canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.level_up;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() && self.crossword_visible &&
            (event == &Event::ClockTick || !state.is_solved())
        {
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
        if !action.should_stop() {
            self.core.begin_character_scene_on_click(event);
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.level_up.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((row, index, chr, _)) = self.core.pop_undo() {
            game.level_up.crossword_mut().set_char(row, index, chr);
            self.crossword.reset_cursor();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((row, index, _, chr)) = self.core.pop_redo() {
            game.level_up.crossword_mut().set_char(row, index, chr);
            self.crossword.reset_cursor();
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.level_up.reset();
        self.crossword.reset_cursor();
    }

    fn solve(&mut self, game: &mut Game) {
        game.level_up.solve();
        self.crossword.reset_cursor();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for entry in self.core.drain_queue() {
            match entry {
                (0, 0) => self.crossword.animate_center_word(),
                (0, 1) => self.crossword.set_center_word_hilighted(true),
                (1, visible) => self.crossword_visible = visible != 0,
                _ => {}
            }
        }
    }
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const OFFSETS_CLUES: &[(i32, &str)] = &[
    (2, "a substance made of multiple elements"),
    (4, "perplexed; confounded"),
    (1, "to suddenly surprise or alarm someone"),
    (1, "one who settles a distant land"),
    (0, "a sweet, creamy pastry filling"),
    (0, "somewhat rare"),
    (2, "military officier just below a general"),
    (2, "rod used by an orchastra conductor"),
    (1, "a quantity beyond what is needed"),
    (3, "authoritative orders"),
];

const INFO_BOX_TEXT: &str = "\
Your goal is to fill in the crossword.

$M{Tap}{Click} on a box to select it, then type in the
word that matches the given clue, using the
$M{on-screen }{}keyboard.

If the word won't fit, you should at least take
a moment to consider its symbolism.

$M{Tap}{Click} on a character in the scene to hear their
words of wisdom.";

// ========================================================================= //
