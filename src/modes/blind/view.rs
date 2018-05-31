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

use elements::{FadeStyle, PuzzleCmd, PuzzleCore, PuzzleView};
use elements::ice::GridView;
use gui::{Action, Canvas, Element, Event, Rect, Resources, Sound};
use modes::SOLVED_INFO_TEXT;
use save::{BlindState, Game, PuzzleState};
use save::ice::BlockSlide;
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<BlockSlide>,
    grid: GridView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &BlindState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::RightToLeft, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_elinsa_midscene(resources));
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        core.add_extra_scene(scenes::compile_ugrent_midscene(resources));
        View {
            core: core,
            grid: GridView::new(resources, 80, 64, state.grid()),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.three_blind_ice;
        self.core.draw_back_layer(canvas);
        self.grid.draw(state.grid(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.three_blind_ice;
        let mut action = self.core.handle_event(event, state);
        if !action.should_stop() &&
            (event == &Event::ClockTick || !state.is_solved())
        {
            let subaction = self.grid.handle_event(event, state.grid_mut());
            if let Some(&(coords, dir)) = subaction.value() {
                if let Some(slide) = state.slide_ice_block(coords, dir) {
                    action.also_play_sound(Sound::device_slide());
                    self.grid.animate_slide(&slide);
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                        action = action.and_return(PuzzleCmd::Save);
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
        if game.three_blind_ice.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(slide) = self.core.pop_undo() {
            game.three_blind_ice.grid_mut().undo_slide(&slide);
            self.grid.reset_animation();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(slide) = self.core.pop_redo() {
            game.three_blind_ice.grid_mut().redo_slide(&slide);
            self.grid.reset_animation();
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.three_blind_ice.reset();
        self.grid.reset_animation();
    }

    fn solve(&mut self, game: &mut Game) {
        game.three_blind_ice.solve();
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
$M{your finger}{the mouse} to slide it in that direction.

$M{Tap}{Click} on a character in the scene to hear their words
of wisdom.";

// ========================================================================= //
