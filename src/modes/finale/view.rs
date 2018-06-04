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

use std::rc::Rc;

use elements::{CrosswordView, FadeStyle, MovingStars, PuzzleCmd, PuzzleCore,
               PuzzleView, Scene};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sound, Sprite};
use modes::syzygy::Atlatl;
use save::{CrosswordState, FinaleState, Game, PuzzleState, ValidChars};
use super::scenes;

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    stars_space: MovingStars,
    sun_sprites: Vec<Sprite>,
    xanadu3_sprites: Vec<Sprite>,
    xanadu4_sprites: Vec<Sprite>,
    planets_visible: bool,
    atlatl: Atlatl,
    atlatl_visible: bool,
    atlatl_beam: AtlatlBeam,
    title_credit: TitleCredit,
    crossword_state: CrosswordState,
    crossword_view: CrosswordView,
    crossword_visible: bool,
    letter_columns: LetterColumns,
    letter_columns_visible: bool,
    special_thanks: SpecialThanks,
    the_end: TheEnd,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &FinaleState)
               -> View {
        let core = {
            let fade = (FadeStyle::BottomToTop, FadeStyle::TopToBottom);
            let intro = scenes::compile_scene(resources);
            let outro = Scene::empty();
            PuzzleCore::new(resources, visible, state, fade, intro, outro)
        };
        View {
            core: core,
            stars_space: MovingStars::new(0, 0, 576, 384),
            sun_sprites: resources.get_sprites("title/sun"),
            xanadu3_sprites: resources.get_sprites("title/xanadu3"),
            xanadu4_sprites: resources.get_sprites("title/xanadu4"),
            planets_visible: false,
            atlatl: Atlatl::new(resources),
            atlatl_visible: false,
            atlatl_beam: AtlatlBeam::new(),
            title_credit: TitleCredit::new(resources),
            crossword_state:
                CrosswordState::new(ValidChars::LettersAndSymbols,
                                    CROSSWORD_WORDS),
            crossword_view: CrosswordView::new(resources,
                                               (364, 56),
                                               CROSSWORD_OFFSETS,
                                               (364, 304)),
            crossword_visible: false,
            letter_columns: LetterColumns::new(resources),
            letter_columns_visible: false,
            special_thanks: SpecialThanks::new(resources),
            the_end: TheEnd::new(resources),
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.finale;
        self.core.draw_back_layer(canvas);
        self.stars_space.draw(canvas);
        if self.planets_visible {
            canvas.fill_rect((255, 255, 255), Rect::new(0, 0, 64, 64));
            canvas.draw_sprite(&self.sun_sprites[0], Point::new(64, 0));
            canvas.draw_sprite(&self.sun_sprites[1], Point::new(64, 64));
            canvas.draw_sprite(&self.sun_sprites[2], Point::new(0, 64));
            canvas.draw_sprite_centered(&self.xanadu3_sprites[0],
                                        Point::new(288, 225));
            canvas.draw_sprite_centered(&self.xanadu4_sprites[0],
                                        Point::new(421, 166));
        }
        self.title_credit.draw(canvas);
        if self.crossword_visible {
            self.crossword_view.draw(&self.crossword_state, canvas);
        }
        if self.letter_columns_visible {
            self.letter_columns.draw(&(), canvas);
        }
        self.special_thanks.draw(canvas);
        self.the_end.draw(canvas);
        self.core.draw_middle_layer(canvas);
        if self.atlatl_visible {
            self.atlatl.draw(&(), canvas);
        }
        self.atlatl_beam.draw(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.finale;
        let mut action = self.core.handle_event(event, state);
        if event == &Event::ClockTick {
            if self.stars_space.tick_animation() {
                action.also_redraw();
            }
            if self.atlatl_beam.tick_animation() {
                action.also_redraw();
            }
        }
        if !action.should_stop() || event == &Event::ClockTick {
            let subaction = self.atlatl.handle_event(event, &mut ());
            action.merge(subaction.but_no_value());
        }
        if event == &Event::ClockTick {
            let subaction =
                self.crossword_view
                    .handle_event(event, &mut self.crossword_state);
            action.merge(subaction.but_no_value());
            let subaction = self.letter_columns.handle_event(event, &mut ());
            action.merge(subaction.but_no_value());
        }
        if state.is_solved() {
            self.core.begin_outro_scene();
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, _game: &Game) -> &'static str { INFO_BOX_TEXT }

    fn undo(&mut self, _game: &mut Game) {}

    fn redo(&mut self, _game: &mut Game) {}

    fn reset(&mut self, _game: &mut Game) {}

    fn solve(&mut self, _game: &mut Game) {}

    fn drain_queue(&mut self) {
        for (kind, value) in self.core.drain_queue() {
            match kind {
                1 => self.stars_space.set_visible(value != 0),
                2 => self.planets_visible = value != 0,
                3 => self.atlatl_visible = value != 0,
                4 => self.atlatl.set_all_indicators(value != 0),
                5 => {
                    if value == 1 {
                        self.atlatl_beam.turn_on(258, 258, 197, true);
                    } else if value == 2 {
                        self.atlatl_beam.turn_on(576, 576, 193, true);
                    } else if value == 3 {
                        self.atlatl_beam.turn_on(576, 275, 183, false);
                    } else {
                        self.atlatl_beam.turn_off();
                    }
                }
                6 => self.crossword_visible = value != 0,
                7 => self.crossword_view.animate_center_word(),
                8 => self.letter_columns_visible = value != 0,
                9 => {
                    self.letter_columns.animate_fall(0, 2, 20);
                    self.letter_columns.animate_fall(1, 0, 24);
                    self.letter_columns.animate_fall(2, 0, 28);
                    self.letter_columns.animate_fall(3, 1, 32);
                    self.letter_columns.animate_fall(4, 0, 36);
                    self.letter_columns.animate_fall(5, 0, 40);
                    self.letter_columns.animate_fall(6, 0, 44);
                    self.letter_columns.animate_fall(7, 1, 48);
                    self.letter_columns.animate_fall(8, 3, 52);
                    self.letter_columns.animate_fall(9, 2, 56);
                }
                10 => self.letter_columns.stop_animation(),
                11 => self.title_credit.display = value,
                12 => self.special_thanks.display = value,
                13 => self.the_end.display = value,
                _ => {}
            }
        }
    }
}

// ========================================================================= //

const BEAM_SPEED: u32 = 32; // pixels/frame
const BEAM_THICKNESS: u32 = 3;
const BEAM_DRIFT_SLOWDOWN: i32 = 6; // frames/pixel

struct AtlatlBeam {
    start: i32,
    length: u32,
    max_length: u32,
    anim: u32,
    y_pos: i32,
    drift_timer: i32,
}

impl AtlatlBeam {
    fn new() -> AtlatlBeam {
        AtlatlBeam {
            start: 0,
            length: 0,
            max_length: 0,
            anim: 0,
            y_pos: 0,
            drift_timer: 0,
        }
    }

    fn turn_on(&mut self, start: i32, max_length: u32, y_pos: i32,
               drift: bool) {
        self.start = start;
        self.length = 0;
        self.max_length = max_length;
        self.anim = 0;
        self.y_pos = y_pos;
        self.drift_timer = if drift { 0 } else { -1 };
    }

    fn turn_off(&mut self) {
        self.length = 0;
        self.max_length = 0;
    }

    fn draw(&self, canvas: &mut Canvas) {
        if self.length > 0 {
            let color = (if self.anim != 0 { 255 } else { 128 },
                         if self.anim != 1 { 255 } else { 128 },
                         if self.anim != 2 { 255 } else { 128 });
            let rect = Rect::new(self.start - (self.length as i32),
                                 self.y_pos - (BEAM_THICKNESS / 2) as i32,
                                 self.length,
                                 BEAM_THICKNESS);
            canvas.fill_rect(color, rect);
        }
    }

    fn tick_animation(&mut self) -> bool {
        if self.max_length == 0 {
            return false;
        }
        self.length = (self.length + BEAM_SPEED).min(self.max_length);
        self.anim = (self.anim + 1) % 3;
        if self.drift_timer >= 0 {
            self.drift_timer += 1;
            if self.drift_timer >= BEAM_DRIFT_SLOWDOWN {
                self.drift_timer = 0;
                self.y_pos -= 1;
            }
        }
        true
    }
}

// ========================================================================= //

pub struct TitleCredit {
    font1: Rc<Font>,
    font2: Rc<Font>,
    display: i32,
}

impl TitleCredit {
    fn new(resources: &mut Resources) -> TitleCredit {
        TitleCredit {
            font1: resources.get_font("block"),
            font2: resources.get_font("system"),
            display: 0,
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        if self.display >= 1 {
            canvas.draw_text(&self.font1,
                             Align::Center,
                             Point::new(408, 150),
                             "SYSTEM");
        }
        if self.display >= 2 {
            canvas.draw_text(&self.font1,
                             Align::Center,
                             Point::new(408, 182),
                             "SYZYGY");
        }
        if self.display >= 3 {
            canvas.draw_text(&self.font2,
                             Align::Center,
                             Point::new(408, 240),
                             "a game by mdsteele");
        }
    }
}

// ========================================================================= //

const BLOCK_WIDTH: i32 = 24;
const BLOCK_HEIGHT: i32 = 24;

struct LetterColumns {
    font: Rc<Font>,
    sprites: Vec<Sprite>,
    fall_anim: [(i32, i32, i32); 10],
}

impl LetterColumns {
    fn new(resources: &mut Resources) -> LetterColumns {
        LetterColumns {
            font: resources.get_font("block"),
            sprites: resources.get_sprites("cross/star"),
            fall_anim: [(0, 0, 0); 10],
        }
    }

    fn animate_fall(&mut self, col: i32, row: i32, length: i32) {
        self.fall_anim[col as usize] = (row, length * BLOCK_HEIGHT, 0);
    }

    fn stop_animation(&mut self) { self.fall_anim = [(0, 0, 0); 10]; }

    fn rect(&self) -> Rect { Rect::new(256, 56, 240, 240) }
}

impl Element<(), ()> for LetterColumns {
    fn draw(&self, _: &(), canvas: &mut Canvas) {
        let rect = self.rect();
        let mut canvas = canvas.subcanvas(rect);
        for col in 0..10 {
            for (row, &letter) in COLUMN_LETTERS[col].iter().enumerate() {
                let row = row as i32;
                let (gap_row, gap, _) = self.fall_anim[col];
                let gap = if row >= gap_row { gap } else { 0 };
                let pt = Point::new((col as i32) * BLOCK_WIDTH,
                                    rect.height() as i32 - gap -
                                        (1 + row) * BLOCK_HEIGHT);
                canvas.draw_sprite(&self.sprites[0], pt);
                let pt = pt + Point::new(BLOCK_WIDTH / 2, BLOCK_HEIGHT - 3);
                canvas.draw_char(&self.font, Align::Center, pt, letter);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, _: &mut ()) -> Action<()> {
        let mut action = Action::ignore();
        match event {
            &Event::ClockTick => {
                for &mut (_, ref mut gap, ref mut speed) in
                    self.fall_anim.iter_mut()
                {
                    if *gap > 0 {
                        *gap = (*gap - *speed).max(0);
                        *speed += 1;
                        action.also_redraw();
                        if *gap == 0 {
                            action.also_play_sound(Sound::device_rotate());
                        }
                    }
                }
            }
            _ => {}
        }
        action
    }
}

// ========================================================================= //

pub struct SpecialThanks {
    font1: Rc<Font>,
    font2: Rc<Font>,
    display: i32,
}

impl SpecialThanks {
    fn new(resources: &mut Resources) -> SpecialThanks {
        SpecialThanks {
            font1: resources.get_font("system"),
            font2: resources.get_font("block"),
            display: 0,
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        if self.display >= 1 {
            canvas.draw_text(&self.font1,
                             Align::Center,
                             Point::new(288, 140),
                             "...and many thanks for extreme patience from:");
        }
        if self.display >= 3 {
            canvas.draw_text(&self.font2,
                             Align::Left,
                             Point::new(133, 281),
                             "HOLLY");
        }
        if self.display >= 2 {
            canvas.draw_text(&self.font2,
                             Align::Center,
                             Point::new(288, 281),
                             "STEPH");
        }
        if self.display >= 4 {
            canvas.draw_text(&self.font2,
                             Align::Right,
                             Point::new(443, 281),
                             "EMILY");
        }
    }
}

// ========================================================================= //

pub struct TheEnd {
    font: Rc<Font>,
    display: i32,
}

impl TheEnd {
    fn new(resources: &mut Resources) -> TheEnd {
        TheEnd {
            font: resources.get_font("system"),
            display: 0,
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        if self.display >= 1 {
            canvas.draw_text(&self.font,
                             Align::Right,
                             Point::new(248, 214),
                             "T  H  E");
            canvas.draw_text(&self.font,
                             Align::Left,
                             Point::new(328, 214),
                             "E  N  D");
        }
    }
}

// ========================================================================= //

const CROSSWORD_WORDS: &[&str] = &[
    "KIPP",
    "JULIE",
    "BARBARA",
    "GUY",
    "T",
    "DAVE&ERIN",
    "CHRIS",
    "STEPH",
    "CAROLINE",
    "BEN",
    "G",
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const CROSSWORD_OFFSETS: &[(i32, &str)] = &[
    (2, ""),
    (2, ""),
    (4, ""),
    (2, ""),
    (0, ""),
    (5, ""),
    (4, ""),
    (1, ""),
    (5, ""),
    (2, ""),
    (0, ""),
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const COLUMN_LETTERS: [&[char]; 10] = [
    &[' ', ' ', 'A', ' ', 'A', ' ', ' ', 'C', ' ', 'A'],
    &['T', ' ', 'N', ' ', 'N', 'A', ' ', 'L', ' ', 'P'],
    &['O', 'C', 'D', ' ', 'D', 'N', ' ', 'I', ' ', 'O'],
    &[' ', 'O', ' ', 'P', 'R', 'D', 'J', 'F', ' ', 'L'],
    &['Y', 'U', ' ', 'L', 'E', ' ', 'O', 'F', ' ', 'O'],
    &['O', 'R', 'O', 'O', 'W', ' ', 'H', ' ', ' ', 'G'],
    &['U', 'S', 'F', 'T', ' ', ' ', 'N', ' ', ' ', 'I'],
    &['.', 'E', ' ', 'K', ' ', 'T', 'S', ' ', ' ', 'E'],
    &['.', ' ', ' ', 'I', ' ', 'O', 'O', ' ', 'T', 'S'],
    &['.', ' ', ' ', 'N', ' ', ' ', 'N', ' ', 'O', ' '],
];

pub const INFO_BOX_TEXT: &str = "\
Return to the map to select another scene.";

// ========================================================================= //
