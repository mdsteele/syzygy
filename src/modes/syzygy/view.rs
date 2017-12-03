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

use std::cmp;
use std::rc::Rc;

use elements::{self, PuzzleCmd, PuzzleCore, PuzzleView};
use elements::column::ColumnsView;
use elements::lasers::{LaserCmd, LaserField};
use elements::plane::{PlaneCmd, PlaneGridView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{self, Game, PuzzleState, SyzygyStage, SyzygyState};
use super::mezure::{MezureCmd, MezureView};
use super::relyng::LightsGrid;
use super::scenes;

// ========================================================================= //

#[derive(Clone)]
enum UndoRedo {
    Yttris(usize, i32),
    Argony(save::ice::BlockSlide),
    Elinsa(Vec<(Point, Point)>),
    Ugrent(LaserCmd),
    Relyng((i32, i32)),
    Mezure(MezureCmd),
}

// ========================================================================= //

const MAX_REVEAL: i32 = 144;
const REVEAL_SPEED: i32 = 4;

pub struct View {
    core: PuzzleCore<UndoRedo>,
    progress: SyzygyProgress,
    atlatl: Atlatl,
    yttris: ColumnsView,
    argony: elements::ice::GridView,
    elinsa: PlaneGridView,
    ugrent: LaserField,
    relyng: LightsGrid,
    mezure: MezureView,
    should_reveal: bool,
    reveal_amount: i32,
    stage: SyzygyStage,
    should_advance: bool,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect,
               state: &mut SyzygyState)
               -> View {
        let mut core = {
            let intro = scenes::compile_intro_scene(resources);
            let outro = scenes::compile_outro_scene(resources);
            PuzzleCore::new(resources, visible, state, intro, outro)
        };
        core.add_extra_scene(scenes::compile_post_yttris_scene(resources));
        core.add_extra_scene(scenes::compile_post_argony_scene(resources));
        core.add_extra_scene(scenes::compile_post_elinsa_scene(resources));
        core.add_extra_scene(scenes::compile_post_ugrent_scene(resources));
        core.add_extra_scene(scenes::compile_post_relyng_scene(resources));
        if !state.is_solved() {
            if state.stage() > SyzygyStage::Yttris {
                core.skip_extra_scene(scenes::POST_YTTRIS_SCENE);
            }
            if state.stage() > SyzygyStage::Argony {
                core.skip_extra_scene(scenes::POST_ARGONY_SCENE);
            }
            if state.stage() > SyzygyStage::Elinsa {
                core.skip_extra_scene(scenes::POST_ELINSA_SCENE);
            }
            if state.stage() > SyzygyStage::Ugrent {
                core.skip_extra_scene(scenes::POST_UGRENT_SCENE);
            }
            if state.stage() > SyzygyStage::Relyng {
                core.skip_extra_scene(scenes::POST_RELYNG_SCENE);
            }
        }
        View {
            core: core,
            progress: SyzygyProgress::new(resources, 320, 288),
            atlatl: Atlatl::new(resources),
            yttris: ColumnsView::new(resources, 196, 168, 0),
            argony: elements::ice::GridView::new(resources,
                                                 144,
                                                 128,
                                                 state.argony_grid()),
            elinsa: PlaneGridView::new(resources, 168, 120),
            ugrent: LaserField::new(resources, 176, 112, state.ugrent_grid()),
            relyng: LightsGrid::new(resources, 168, 128, state),
            mezure: MezureView::new(resources, state),
            should_reveal: false,
            reveal_amount: 0,
            stage: state.stage(),
            should_advance: false,
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.system_syzygy;
        self.core.draw_back_layer(canvas);
        self.progress.draw(&(), canvas);
        self.core.draw_middle_layer(canvas);
        self.atlatl.draw(&(), canvas);
        if self.reveal_amount > 0 {
            let clip = if self.reveal_amount >= MAX_REVEAL {
                canvas.rect()
            } else {
                Rect::new(0,
                          192 - self.reveal_amount,
                          canvas.width(),
                          2 * self.reveal_amount as u32)
            };
            let mut canvas = canvas.clipped(clip);
            match self.stage {
                SyzygyStage::Yttris => {
                    self.yttris.draw(state.yttris_columns(), &mut canvas);
                }
                SyzygyStage::Argony => {
                    self.argony.draw(state.argony_grid(), &mut canvas);
                }
                SyzygyStage::Elinsa => {
                    self.elinsa.draw(state.elinsa_grid(), &mut canvas);
                }
                SyzygyStage::Ugrent => {
                    self.ugrent.draw(state.ugrent_grid(), &mut canvas);
                }
                SyzygyStage::Relyng => self.relyng.draw(state, &mut canvas),
                SyzygyStage::Mezure => self.mezure.draw(state, &mut canvas),
            }
        }
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.system_syzygy;
        let mut action = self.core.handle_event(event, state);
        if event == &Event::ClockTick {
            if self.should_reveal && self.reveal_amount < MAX_REVEAL {
                self.reveal_amount =
                    cmp::min(MAX_REVEAL, self.reveal_amount + REVEAL_SPEED);
                action.also_redraw();
            } else if !self.should_reveal && self.reveal_amount > 0 {
                self.reveal_amount =
                    cmp::max(0, self.reveal_amount - REVEAL_SPEED);
                action.also_redraw();
            }
            if self.should_advance {
                self.stage = state.stage();
                self.should_advance = false;
                action.also_redraw();
            }
        }
        if !action.should_stop() || event == &Event::ClockTick {
            let subaction = self.progress.handle_event(event, &mut ());
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() || event == &Event::ClockTick {
            let subaction = self.atlatl.handle_event(event, &mut ());
            action.merge(subaction.but_no_value());
        }
        if self.should_reveal && self.reveal_amount >= MAX_REVEAL &&
            !action.should_stop() && !state.is_solved()
        {
            match self.stage {
                SyzygyStage::Yttris => {
                    let subaction =
                        self.yttris
                            .handle_event(event, state.yttris_columns_mut());
                    if let Some(&(col, by)) = subaction.value() {
                        state.yttris_columns_mut().rotate_column(col, by);
                        if state.yttris_columns().is_solved() {
                            self.core.clear_undo_redo();
                            state.advance_stage();
                            self.core
                                .begin_extra_scene(scenes::POST_YTTRIS_SCENE);
                        } else {
                            self.core.push_undo(UndoRedo::Yttris(col, by));
                        }
                    }
                    action.merge(subaction.but_no_value());
                }
                SyzygyStage::Argony => {
                    let subaction =
                        self.argony
                            .handle_event(event, state.argony_grid_mut());
                    if let Some(&(coords, dir)) = subaction.value() {
                        if let Some(slide) =
                            state
                                .argony_grid_mut()
                                .slide_ice_block(coords, dir)
                        {
                            self.argony.animate_slide(&slide);
                            if state.argony_grid().all_blocks_on_goals() {
                                self.core.clear_undo_redo();
                                state.advance_stage();
                                self.core.begin_extra_scene(
                                    scenes::POST_ARGONY_SCENE);
                            } else {
                                self.core.push_undo(UndoRedo::Argony(slide));
                            }
                        }
                    }
                    action.merge(subaction.but_no_value());
                }
                SyzygyStage::Elinsa => {
                    let mut subaction =
                        self.elinsa
                            .handle_event(event, state.elinsa_grid_mut());
                    match subaction.take_value() {
                        Some(PlaneCmd::Changed) => {
                            if state.elinsa_grid().all_nodes_are_connected() {
                                self.core.clear_undo_redo();
                                self.elinsa.cancel_drag_and_clear_changes();
                                state.advance_stage();
                                self.core.begin_extra_scene(
                                    scenes::POST_ELINSA_SCENE);
                            }
                        }
                        Some(PlaneCmd::PushUndo(changes)) => {
                            self.core.push_undo(UndoRedo::Elinsa(changes));
                        }
                        None => {}
                    }
                    action.merge(subaction.but_no_value());
                }
                SyzygyStage::Ugrent => {
                    let subaction =
                        self.ugrent
                            .handle_event(event, state.ugrent_grid_mut());
                    if let Some(&cmd) = subaction.value() {
                        if self.ugrent
                            .all_detectors_satisfied(state.ugrent_grid())
                        {
                            self.core.clear_undo_redo();
                            state.advance_stage();
                            self.core
                                .begin_extra_scene(scenes::POST_UGRENT_SCENE);
                        } else {
                            self.core.push_undo(UndoRedo::Ugrent(cmd));
                        }
                    }
                    action.merge(subaction.but_no_value());
                }
                SyzygyStage::Relyng => {
                    let subaction = self.relyng.handle_event(event, state);
                    if let Some(&pos) = subaction.value() {
                        state.relyng_toggle(pos);
                        if state.relyng_is_done() {
                            self.core.clear_undo_redo();
                            state.advance_stage();
                            self.core
                                .begin_extra_scene(scenes::POST_RELYNG_SCENE);
                        } else {
                            self.core.push_undo(UndoRedo::Relyng(pos));
                        }
                    }
                    action.merge(subaction.but_no_value());
                }
                SyzygyStage::Mezure => {
                    let mut subaction = self.mezure.handle_event(event, state);
                    if let Some(cmd) = subaction.take_value() {
                        // TODO: detect when solved
                        self.core.push_undo(UndoRedo::Mezure(cmd));
                    }
                    action.merge(subaction.but_no_value());
                }
            }
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.system_syzygy.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            match game.system_syzygy.stage() {
                SyzygyStage::Yttris => YTTRIS_INFO_BOX_TEXT,
                SyzygyStage::Argony => ARGONY_INFO_BOX_TEXT,
                SyzygyStage::Elinsa => ELINSA_INFO_BOX_TEXT,
                SyzygyStage::Ugrent => UGRENT_INFO_BOX_TEXT,
                SyzygyStage::Relyng => RELYNG_INFO_BOX_TEXT,
                SyzygyStage::Mezure => MEZURE_INFO_BOX_TEXT,
            }
        }
    }

    fn undo(&mut self, game: &mut Game) {
        let state = &mut game.system_syzygy;
        match self.core.pop_undo() {
            Some(UndoRedo::Yttris(col, by)) => {
                state.yttris_columns_mut().rotate_column(col, -by);
            }
            Some(UndoRedo::Argony(slide)) => {
                state.argony_grid_mut().undo_slide(&slide);
                self.argony.reset_animation();
            }
            Some(UndoRedo::Elinsa(changes)) => {
                for (coords1, coords2) in changes.into_iter().rev() {
                    state.elinsa_grid_mut().toggle_pipe(coords1, coords2);
                }
            }
            Some(UndoRedo::Ugrent(cmd)) => {
                let grid = state.ugrent_grid_mut();
                match cmd {
                    LaserCmd::Moved(col1, row1, col2, row2) => {
                        grid.move_to(col2, row2, col1, row1);
                    }
                    LaserCmd::Rotated(col, row) => {
                        grid.unrotate(col, row);
                    }
                }
                self.ugrent.recalculate_lasers(grid);
            }
            Some(UndoRedo::Relyng(pos)) => state.relyng_untoggle(pos),
            Some(UndoRedo::Mezure(MezureCmd::Pipes(changes))) => {
                for (coords1, coords2) in changes.into_iter().rev() {
                    state.mezure_pipe_grid_mut().toggle_pipe(coords1, coords2);
                }
                state.mezure_regenerate_laser_grid();
                self.mezure.refresh(state);
            }
            Some(UndoRedo::Mezure(MezureCmd::IceBlocks(slide))) => {
                state.mezure_ice_grid_mut().undo_slide(&slide);
                state.mezure_regenerate_laser_grid();
                self.mezure.refresh(state);
            }
            Some(UndoRedo::Mezure(MezureCmd::Columns(col, by))) => {
                state.mezure_columns_mut().rotate_column(col, -by);
            }
            None => {}
        }
    }

    fn redo(&mut self, game: &mut Game) {
        let state = &mut game.system_syzygy;
        match self.core.pop_redo() {
            Some(UndoRedo::Yttris(col, by)) => {
                state.yttris_columns_mut().rotate_column(col, by);
            }
            Some(UndoRedo::Argony(slide)) => {
                state.argony_grid_mut().redo_slide(&slide);
                self.argony.reset_animation();
            }
            Some(UndoRedo::Elinsa(changes)) => {
                for (coords1, coords2) in changes.into_iter() {
                    state.elinsa_grid_mut().toggle_pipe(coords1, coords2);
                }
            }
            Some(UndoRedo::Ugrent(cmd)) => {
                let grid = state.ugrent_grid_mut();
                match cmd {
                    LaserCmd::Moved(col1, row1, col2, row2) => {
                        grid.move_to(col1, row1, col2, row2);
                    }
                    LaserCmd::Rotated(col, row) => {
                        grid.unrotate(col, row);
                    }
                }
                self.ugrent.recalculate_lasers(grid);
            }
            Some(UndoRedo::Relyng(pos)) => state.relyng_toggle(pos),
            Some(UndoRedo::Mezure(MezureCmd::Pipes(changes))) => {
                for (coords1, coords2) in changes.into_iter() {
                    state.mezure_pipe_grid_mut().toggle_pipe(coords1, coords2);
                }
                state.mezure_regenerate_laser_grid();
                self.mezure.refresh(state);
            }
            Some(UndoRedo::Mezure(MezureCmd::IceBlocks(slide))) => {
                state.mezure_ice_grid_mut().redo_slide(&slide);
                state.mezure_regenerate_laser_grid();
                self.mezure.refresh(state);
            }
            Some(UndoRedo::Mezure(MezureCmd::Columns(col, by))) => {
                state.mezure_columns_mut().rotate_column(col, by);
            }
            None => {}
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.system_syzygy;
        self.core.clear_undo_redo();
        state.reset();
        self.ugrent.recalculate_lasers(state.ugrent_grid());
        self.mezure.refresh(state);
    }

    fn solve(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        let state = &mut game.system_syzygy;
        let stage = state.stage();
        state.solve_stage();
        match stage {
            SyzygyStage::Yttris => {
                self.core.begin_extra_scene(scenes::POST_YTTRIS_SCENE);
            }
            SyzygyStage::Argony => {
                self.core.begin_extra_scene(scenes::POST_ARGONY_SCENE);
            }
            SyzygyStage::Elinsa => {
                self.core.begin_extra_scene(scenes::POST_ELINSA_SCENE);
            }
            SyzygyStage::Ugrent => {
                self.ugrent.recalculate_lasers(state.ugrent_grid());
                self.core.begin_extra_scene(scenes::POST_UGRENT_SCENE);
            }
            SyzygyStage::Relyng => {
                self.core.begin_extra_scene(scenes::POST_RELYNG_SCENE);
            }
            SyzygyStage::Mezure => {
                self.core.begin_outro_scene();
            }
        }
    }

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            if kind == 0 {
                if value < 0 {
                    self.reveal_amount =
                        if self.should_reveal { MAX_REVEAL } else { 0 };
                } else {
                    self.should_reveal = value != 0;
                }
            } else if kind == 1 {
                if value == -2 {
                    self.progress.finish_animation();
                } else if value == -1 {
                    self.progress.start_display();
                } else if value >= 0 && value <= 6 {
                    self.progress.set_progress(value as usize);
                }
            } else if kind == 2 {
                self.atlatl.animate(value);
            } else if kind == 3 {
                self.should_advance = value != 0;
            } else if kind == 4 && value >= 0 {
                self.mezure.hilight_column_red(value as usize);
            } else if kind == 5 && value >= 0 {
                self.mezure.hilight_column_dark(value as usize);
            } else if kind == 6 && value >= 0 {
                self.yttris.set_hilight_color(value as usize, (255, 128, 255));
            } else if kind == 7 && value >= 0 {
                self.yttris.clear_hilight_color(value as usize);
            }
        }
    }
}

// ========================================================================= //

const BRIGHTNESS_SPEED: i32 = 26;
const ANIM_DISPLAY_FRAMES: i32 = 2;

enum ProgressAnim {
    Display(i32),
}

struct SyzygyProgress {
    left: i32,
    top: i32,
    font: Rc<Font>,
    num_chars: usize,
    brightness: [i32; 6],
    goal_brightness: [i32; 6],
    animation: Option<ProgressAnim>,
}

impl SyzygyProgress {
    fn new(resources: &mut Resources, left: i32, top: i32) -> SyzygyProgress {
        SyzygyProgress {
            left: left,
            top: top,
            font: resources.get_font("block"),
            num_chars: 0,
            brightness: [0; 6],
            goal_brightness: [0; 6],
            animation: None,
        }
    }

    fn rect(&self) -> Rect { Rect::new(self.left, self.top, 192, 32) }

    fn start_display(&mut self) {
        self.num_chars = 1;
        self.brightness[0] = 255;
        self.goal_brightness = [0; 6];
        self.animation = Some(ProgressAnim::Display(ANIM_DISPLAY_FRAMES));
    }

    fn set_progress(&mut self, progress: usize) {
        for (index, goal) in self.goal_brightness.iter_mut().enumerate() {
            *goal = if index < progress { 200 } else { 0 };
        }
    }

    fn finish_animation(&mut self) {
        match self.animation.take() {
            Some(ProgressAnim::Display(_)) => {
                self.num_chars = 6;
            }
            None => {}
        }
        self.brightness = self.goal_brightness;
    }
}

impl Element<(), ()> for SyzygyProgress {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect());
        for (index, &brightness) in self.brightness.iter().enumerate() {
            debug_assert!(brightness >= 0 && brightness <= 255);
            let color = (brightness as u8, 0, 0);
            canvas.fill_rect(color, Rect::new(32 * (index as i32), 0, 32, 32));
        }
        for (index, chr) in "SYZYGY".chars().enumerate() {
            if index >= self.num_chars {
                break;
            }
            let pt = Point::new(16 + 32 * (index as i32), 25);
            canvas.draw_char(&self.font, Align::Center, pt, chr);
        }
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<()> {
        match event {
            &Event::ClockTick => {
                let mut redraw = false;
                match self.animation.take() {
                    Some(ProgressAnim::Display(frames)) => {
                        let frames = frames - 1;
                        if frames > 0 {
                            self.animation =
                                Some(ProgressAnim::Display(frames));
                        } else {
                            self.brightness[self.num_chars] = 255;
                            self.num_chars += 1;
                            redraw = true;
                            if self.num_chars < 6 {
                                let frames = ANIM_DISPLAY_FRAMES;
                                self.animation =
                                    Some(ProgressAnim::Display(frames));
                            }
                        }
                    }
                    None => {}
                }
                for index in 0..6 {
                    let current = self.brightness[index];
                    let goal = self.goal_brightness[index];
                    if current < goal {
                        self.brightness[index] =
                            cmp::min(goal, current + BRIGHTNESS_SPEED);
                        redraw = true;
                    } else if current > goal {
                        self.brightness[index] =
                            cmp::max(goal, current - BRIGHTNESS_SPEED);
                        redraw = true;
                    }
                }
                Action::redraw_if(redraw)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

struct Atlatl {
    atlatl_sprites: Vec<Sprite>,
    indicator_sprites: Vec<Sprite>,
    sparkle_sprites: Vec<Sprite>,
    indicator_anim: [i32; 6],
}

impl Atlatl {
    fn new(resources: &mut Resources) -> Atlatl {
        Atlatl {
            atlatl_sprites: resources.get_sprites("syzygy/atlatl"),
            indicator_sprites: resources.get_sprites("syzygy/indicator"),
            sparkle_sprites: resources.get_sprites("syzygy/sparkle"),
            indicator_anim: [0; 6],
        }
    }

    fn animate(&mut self, col: i32) {
        if col >= 0 && col < 6 {
            self.indicator_anim[col as usize] = 1;
        } else {
            for anim in self.indicator_anim.iter_mut() {
                if *anim > 0 {
                    *anim = 9;
                }
            }
        }
    }
}

impl Element<(), ()> for Atlatl {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        canvas.draw_sprite(&self.atlatl_sprites[0], Point::new(128, 160));
        canvas.draw_sprite(&self.atlatl_sprites[1], Point::new(128, 192));
        canvas.draw_sprite(&self.atlatl_sprites[2], Point::new(160, 160));
        canvas.draw_sprite(&self.atlatl_sprites[3], Point::new(160, 192));
        canvas.draw_sprite(&self.atlatl_sprites[4], Point::new(192, 176));
        for col in 0..6 {
            let left = 224 + 32 * col;
            canvas.draw_sprite(&self.atlatl_sprites[5], Point::new(left, 176));
        }
        canvas.draw_sprite(&self.atlatl_sprites[6], Point::new(416, 176));
        for col in 0..6 {
            let pt = Point::new(224 + 38 * col, 192);
            let anim = self.indicator_anim[col as usize];
            let index = if anim > 4 { 1 } else { 0 };
            canvas.draw_sprite_centered(&self.indicator_sprites[index], pt);
            if anim > 0 && anim < 9 {
                let index = (anim - 1) as usize;
                canvas.draw_sprite_centered(&self.sparkle_sprites[index], pt);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<()> {
        match event {
            &Event::ClockTick => {
                let mut redraw = false;
                for anim in self.indicator_anim.iter_mut() {
                    if *anim > 0 && *anim < 9 {
                        *anim += 1;
                        redraw = true;
                    }
                }
                Action::redraw_if(redraw)
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const YTTRIS_INFO_BOX_TEXT: &str = "\
Your goal is to slide the columns of letters until the
hilighted letters form a word horizontally across.
There is only one possible word that can be formed.

Drag a column up or down with $M{your finger}{the mouse} to rotate
its letters.  Moving one column may also cause other
columns to move at the same time.";

const ARGONY_INFO_BOX_TEXT: &str = "\
Your goal is to slide the blocks of ice until each one
covers its matching symbol on the grid, in the same
orientation and chirality.

Drag one of the ice blocks up, down, left, or right with
$M{your finger}{the mouse} to slide it in that direction.";

const ELINSA_INFO_BOX_TEXT: &str = "\
Your goal is to connect each red node to each blue
node.  The purple node counts as both red and blue.

Drag across the grid with $M{your finger}{the mouse} to create or
remove pipes between the nodes.";

const UGRENT_INFO_BOX_TEXT: &str = "\
Your goal is to activate each detector on the right with
the appropriate color of laser.

Drag mirrors and other objects with $M{your finger}{the mouse} to
move their positions in the grid.  $M{Tap}{Click} objects to rotate
them.";

const RELYNG_INFO_BOX_TEXT: &str = "\
Your goal is to turn all twenty lights OFF.

$M{Tap}{Click} one of the lights to toggle that light and some
of the adjacent lights.  The pattern of adjancent lights
toggled will change after each move.";

const MEZURE_INFO_BOX_TEXT: &str = "\
Your goal is to form the final, missing word.";

// ========================================================================= //
