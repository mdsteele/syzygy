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

use crate::elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView};
use crate::elements::column::ColumnsView;
use crate::gui::{Action, Canvas, Element, Event, Rect, Resources};
use crate::modes::SOLVED_INFO_TEXT;
use crate::save::{Game, IcyEmState, PuzzleState};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(usize, i32)>,
    columns: ColumnsView,
    show_columns: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &IcyEmState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_relyng_midscene(resources));
        View {
            core: core,
            columns: ColumnsView::new(resources, 180, 100, 80),
            show_columns: false,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.column_as_icy_em;
        self.core.draw_back_layer(canvas);
        if self.show_columns {
            self.columns.draw(state.columns(), canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.column_as_icy_em;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() && self.show_columns &&
            (event == &Event::ClockTick || !state.is_solved())
        {
            let subaction = self.columns
                .handle_event(event, state.columns_mut());
            if let Some(&(col, by)) = subaction.value() {
                state.rotate_column(col, by);
                if state.is_solved() {
                    self.core.begin_outro_scene();
                    action = action.and_return(PuzzleCmd::Save);
                } else {
                    self.core.push_undo((col, by));
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
        if game.column_as_icy_em.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((col, by)) = self.core.pop_undo() {
            self.columns.clear_drag();
            game.column_as_icy_em.rotate_column(col, -by);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((col, by)) = self.core.pop_redo() {
            self.columns.clear_drag();
            game.column_as_icy_em.rotate_column(col, by);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.columns.clear_drag();
        self.core.clear_undo_redo();
        game.column_as_icy_em.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        self.columns.clear_drag();
        game.column_as_icy_em.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.show_columns = value != 0;
            } else if kind == 1 && value >= 0 {
                self.columns
                    .set_hilight_color(value as usize, (255, 128, 255));
            } else if kind == 2 && value >= 0 {
                self.columns.clear_hilight_color(value as usize);
            }
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &str = "\
Your goal is to slide the columns of letters until the
highlighted letters form two words horizontally
across.  There is only one possible pair of words that
can be formed simultaneously.

Drag a column up or down with $M{your finger}{the mouse} to rotate
its letters.  Moving one column may also cause other
columns to move at the same time.

$M{Tap}{Click} on a character in the scene to hear their words
of wisdom.";

// ========================================================================= //
