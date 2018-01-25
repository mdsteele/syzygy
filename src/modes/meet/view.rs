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
use gui::{Action, Canvas, Element, Event, Rect, Resources};
use modes::SOLVED_INFO_TEXT;
use save::{Game, MeetState, PuzzleState};
use save::ice::BlockSlide;
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<BlockSlide>,
    grid: GridView,
    grid_visible: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &MeetState)
               -> View {
        let mut core = {
            let fade = (FadeStyle::LeftToRight, FadeStyle::LeftToRight);
            let intro = scenes::compile_intro_scene(resources, visible);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        core.add_extra_scene(scenes::compile_elinsa_midscene(resources));
        core.add_extra_scene(scenes::compile_mezure_midscene(resources));
        View {
            core: core,
            grid: GridView::new(resources, 96, 48, state.grid()),
            grid_visible: true,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.ice_to_meet_you;
        self.core.draw_back_layer(canvas);
        if self.grid_visible {
            self.grid.draw(state.grid(), canvas);
        }
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.ice_to_meet_you;
        let mut action = self.core.handle_event(event, state);
        if self.grid_visible && !action.should_stop() {
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
        if game.ice_to_meet_you.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some(slide) = self.core.pop_undo() {
            game.ice_to_meet_you.grid_mut().undo_slide(&slide);
            self.grid.reset_animation();
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some(slide) = self.core.pop_redo() {
            game.ice_to_meet_you.grid_mut().redo_slide(&slide);
            self.grid.reset_animation();
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.ice_to_meet_you.reset();
        self.grid.reset_animation();
    }

    fn solve(&mut self, game: &mut Game) {
        game.ice_to_meet_you.solve();
        self.grid.reset_animation();
        self.core.begin_outro_scene();
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                self.grid_visible = value != 0;
            } else if kind == 1 {
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
    ((1, 2), 'E'),
    ((3, 0), 'G'),
    ((2, 2), 'N'),
    ((3, 1), 'N'),
    ((3, 2), 'I'),
    ((4, 2), 'G'),
    ((3, 3), 'R'),
    ((5, 2), 'N'),
    ((3, 4), 'E'),
    ((6, 2), 'E'),
];

const INFO_BOX_TEXT: &str = "\
Your goal is to slide the blocks of ice until each one
covers its matching symbol on the grid.

Drag one of the ice blocks up, down, left, or right with
$M{your finger}{the mouse} to slide it in that direction.

$M{Tap}{Click} on a character in the scene to hear their words
of wisdom.";

// ========================================================================= //
