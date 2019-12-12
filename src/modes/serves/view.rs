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

use super::scenes;
use crate::elements::memory::{MemoryGridView, NextShapeView, FLIP_SLOWDOWN};
use crate::elements::{
    FadeStyle, ProgressBar, PuzzleCmd, PuzzleCore, PuzzleView,
};
use crate::gui::{Action, Canvas, Element, Event, Rect, Resources, Sound};
use crate::modes::SOLVED_INFO_TEXT;
use crate::save::{Direction, Game, PuzzleState, ServesState};

// ========================================================================= //

const REMOVE_DELAY: i32 = FLIP_SLOWDOWN * 5 + 20;
const REMOVE_SOUND_AT: i32 = 20 + FLIP_SLOWDOWN * 2;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    grid: MemoryGridView,
    next: NextShapeView,
    progress: ProgressBar,
    progress_adjust: u32,
    remove_countdown: i32,
    show_next: bool,
}

impl View {
    pub fn new(
        resources: &mut Resources,
        visible: Rect,
        state: &ServesState,
    ) -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_argony_midscene(resources));
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        View {
            core,
            grid: MemoryGridView::new(
                resources,
                "memory/serves",
                (256, 176),
                state.grid(),
            ),
            next: NextShapeView::new(resources, "memory/serves", (96, 208)),
            progress: ProgressBar::new(
                (104, 176),
                Direction::East,
                80,
                (191, 191, 0),
            ),
            progress_adjust: 0,
            remove_countdown: 0,
            show_next: false,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.if_memory_serves;
        self.core.draw_back_layer(canvas);
        if !state.is_solved() {
            let value = state.current_step() as u32 + self.progress_adjust;
            let maximum = state.total_num_steps() as u32;
            self.progress.draw(value, maximum, canvas);
        }
        self.grid.draw(state.grid(), canvas);
        self.core.draw_middle_layer(canvas);
        if self.show_next {
            self.next.draw(&state.next_shape(), canvas);
        }
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(
        &mut self,
        event: &Event,
        game: &mut Game,
    ) -> Action<PuzzleCmd> {
        let state = &mut game.if_memory_serves;
        let mut action = self.core.handle_event(event, state);
        if event == &Event::ClockTick && self.remove_countdown > 0 {
            self.remove_countdown -= 1;
            if self.remove_countdown == REMOVE_SOUND_AT {
                let symbol = self.grid.flip_symbol();
                let sound = if state.can_remove_symbol(symbol) {
                    self.progress_adjust = 1;
                    Sound::mid_puzzle_chime()
                } else {
                    Sound::talk_annoyed_hi()
                };
                action.merge(Action::redraw().and_play_sound(sound));
            }
            if self.remove_countdown == 0 {
                self.progress_adjust = 0;
                state.remove_symbol(self.grid.flip_symbol());
                self.grid.clear_flip();
                if state.is_solved() {
                    self.core.begin_outro_scene();
                    action = action.and_return(PuzzleCmd::Save);
                }
                action.also_redraw();
            }
        }
        if (!action.should_stop() && self.remove_countdown == 0)
            || event == &Event::ClockTick
        {
            let subaction =
                self.next.handle_event(event, &mut state.next_shape());
            if let Some(&pt) = subaction.value() {
                let (col, row) = self.grid.coords_for_point(pt);
                if let Some(symbol) = state.try_place_shape(col, row) {
                    action.also_play_sound(Sound::device_drop());
                    self.grid.place_symbol(symbol);
                }
            }
            action.merge(subaction.but_no_value());
        }
        if (!action.should_stop() && self.remove_countdown == 0)
            || event == &Event::ClockTick
        {
            let subaction = self.grid.handle_event(event, state.grid_mut());
            if let Some(&symbol) = subaction.value() {
                action.also_play_sound(Sound::device_rotate());
                self.grid.reveal_symbol(symbol);
                self.remove_countdown = REMOVE_DELAY;
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
        if game.if_memory_serves.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, _: &mut Game) {}

    fn redo(&mut self, _: &mut Game) {}

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.if_memory_serves.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        game.if_memory_serves.solve();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.show_next = value != 0;
            } else if kind == 1 {
                if value >= 0 && (value as usize) < LETTERS.len() {
                    let (col, row, letter) = LETTERS[value as usize];
                    self.grid.add_letter(col, row, letter);
                }
            }
        }
    }
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const LETTERS: &[(i32, i32, char)] = &[
    (1, 0, 'I'), (1, 1, 'N'), (1, 2, 'T'), (1, 3, 'E'),
    (3, 0, 'C'), (3, 1, 'O'), (3, 2, 'N'), (3, 3, 'S'),
    (5, 0, 'I'), (5, 1, 'N'), (5, 2, 'D'), (5, 3, 'E'),
];

const INFO_BOX_TEXT: &str = "\
Your goal is to place (and later remove) each group of tiles on
the grid.

When a group of tiles appears on the left, use $M{your finger}{the mouse} to
drag it onto the grid on the right.  The tiles will then flip over;
the backs of the tiles will be green.

Tiles will eventually turn from green to gray; once all tiles
with a given symbol are gray, they may be safely removed.
You can remove a group of tiles at any time by $M{tapp}{click}ing any of
the tiles on the grid that had that symbol.  However, if you
accidentally remove a tile that's still green, you will have to
start over.

$M{Tap}{Click} on a character in the scene to hear their words of wisdom.";

// ========================================================================= //
