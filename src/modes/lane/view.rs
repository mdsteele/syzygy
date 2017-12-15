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

use elements::{Paragraph, ProgressBar, PuzzleCmd, PuzzleCore, PuzzleView};
use elements::memory::{FLIP_SLOWDOWN, MemoryGridView, NextShapeView};
use gui::{Action, Align, Canvas, Element, Event, Point, Rect, Resources,
          Sound, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Direction, Game, LaneState, PuzzleState};
use super::scenes;

// ========================================================================= //

const REMOVE_DELAY: i32 = FLIP_SLOWDOWN * 5 + 20;
const REMOVE_SOUND_AT: i32 = 20 + FLIP_SLOWDOWN * 2;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    grid: MemoryGridView,
    next: NextShapeView,
    free: FreeSymbolView,
    progress: ProgressBar,
    progress_adjust: u32,
    prompt: PromptView,
    remove_countdown: i32,
    show_next: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &LaneState)
               -> View {
        let mut core = {
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, intro, outro)
        };
        core.add_extra_scene(scenes::compile_argony_midscene(resources));
        core.add_extra_scene(scenes::compile_ugrent_midscene(resources));
        View {
            core: core,
            grid: MemoryGridView::new(resources,
                                      "memory/lane",
                                      (208, 64),
                                      state.grid()),
            next: NextShapeView::new(resources, "memory/lane", (96, 64)),
            free: FreeSymbolView::new(resources, (448, 112)),
            progress: ProgressBar::new((112, 176),
                                       Direction::East,
                                       80,
                                       (191, 191, 0)),
            progress_adjust: 0,
            prompt: PromptView::new(resources),
            remove_countdown: 0,
            show_next: false,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.memory_lane;
        self.core.draw_back_layer(canvas);
        if !state.is_solved() {
            let value = state.current_stage() as u32 + self.progress_adjust;
            let maximum = state.total_num_stages() as u32;
            self.progress.draw(value, maximum, canvas);
        }
        self.free.draw(state, canvas);
        self.grid.draw(state.grid(), canvas);
        if self.show_next && self.remove_countdown == 0 &&
            !self.next.is_dragging()
        {
            self.prompt.draw(state, canvas);
        }
        self.core.draw_middle_layer(canvas);
        if self.show_next {
            self.next.draw(&state.next_shape(), canvas);
        }
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.memory_lane;
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
                }
                action.also_redraw();
            }
        }
        if !action.should_stop() {
            let subaction = self.next
                .handle_event(event, &mut state.next_shape());
            if let Some(&pt) = subaction.value() {
                let (col, row) = self.grid.coords_for_point(pt);
                if let Some(symbol) = state.try_place_shape(col, row) {
                    action.also_play_sound(Sound::device_drop());
                    self.grid.place_symbol(symbol);
                }
            }
            action.merge(subaction.but_no_value());
        }
        if (!action.should_stop() && self.remove_countdown == 0) ||
            event == &Event::ClockTick
        {
            let subaction = self.grid.handle_event(event, state.grid_mut());
            if let Some(&symbol) = subaction.value() {
                if state.next_remove().is_some() {
                    if state.can_remove_symbol(symbol) {
                        state.decay_symbol_all(symbol);
                    }
                    action.also_play_sound(Sound::device_rotate());
                    self.grid.reveal_symbol(symbol);
                    self.remove_countdown = REMOVE_DELAY;
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
            let index = (symbol - 1) as usize * 2;
            canvas.draw_sprite(&self.symbol_sprites[index], self.top_left);
        }
    }
}

// ========================================================================= //

const PLACE_PROMPT: &str = "Drag the shape on the left onto the \
                            center grid.";
const REMOVE_PROMPT: &str = "$M{Tap}{Click} any tile in the center \
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

#[cfg_attr(rustfmt, rustfmt_skip)]
const LETTERS: &[(i32, i32, char)] = &[
    (0, 0, 'E'), (0, 1, 'N'), (0, 2, 'C'), (0, 3, 'E'),
    (2, 0, 'T'), (2, 1, 'I'), (2, 2, 'V'), (2, 3, 'E'),
    (4, 0, 'E'), (4, 1, 'N'), (4, 2, 'C'), (4, 3, 'E'),
];

const INFO_BOX_TEXT: &str = "\
Your goal is to place (and later remove) each group of tiles on
the grid.

When a group of tiles appears on the left, use $M{your finger}{the mouse} to
drag it onto the grid on the right.  The tiles will then flip over.

At certain points, you will be prompted to remove a group of
tiles from the grid that had a given symbol.  $M{Tap}{Click} any of the
tiles on the grid that had that symbol to remove all of them.
However, if you accidentally remove a tile with the wrong
symbol,  you will have to start over.

$M{Tap}{Click} on a character in the scene to hear their words of wisdom.";

// ========================================================================= //
