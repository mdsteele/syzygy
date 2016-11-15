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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Align, Canvas, Element, Event, Font, Point, Rect,
          Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Direction, Game, PuzzleState, WreckedState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(Direction, i32)>,
    grid: WreckedGrid,
    solution: SolutionDisplay,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &WreckedState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources, visible);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            grid: WreckedGrid::new(resources, 84, 132),
            solution: SolutionDisplay::new(resources),
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for (_, index) in self.core.drain_queue() {
            self.solution.set_index(index);
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.wrecked_angle;
        self.core.draw_back_layer(canvas);
        self.solution.draw(state, canvas);
        self.grid.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.wrecked_angle;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() {
            let subaction = self.grid.handle_event(event, state);
            if let Some(&(dir, rank)) = subaction.value() {
                state.shift_tiles(dir, rank);
                if state.is_solved() {
                    if cfg!(debug_assertions) {
                        println!("Puzzle solved, beginning outro.");
                    }
                    self.core.begin_outro_scene();
                    self.drain_queue();
                    self.core.clear_undo_redo();
                } else {
                    self.core.push_undo((dir, rank));
                }
            }
            action.merge(subaction.but_no_value());
        }
        if !action.should_stop() {
            action.merge(self.solution.handle_event(event, state));
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.wrecked_angle.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((dir, rank)) = self.core.pop_undo() {
            game.wrecked_angle.shift_tiles(dir.opposite(), rank);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((dir, rank)) = self.core.pop_redo() {
            game.wrecked_angle.shift_tiles(dir, rank);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.wrecked_angle.reset();
    }

    fn replay(&mut self, game: &mut Game) {
        game.wrecked_angle.replay();
        self.core.replay();
        self.drain_queue();
    }

    fn solve(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.wrecked_angle.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const LARGE_TILE_SIZE: u32 = 24;

struct Drag {
    from: Point,
    to: Point,
}

impl Drag {
    fn new(start: Point) -> Drag {
        Drag {
            from: start,
            to: start,
        }
    }

    fn dir_rank_dist(&self) -> (Direction, i32, i32) {
        let delta = self.to - self.from;
        let dir = if delta.x().abs() > delta.y().abs() {
            if delta.x() >= 0 {
                Direction::East
            } else {
                Direction::West
            }
        } else {
            if delta.y() >= 0 {
                Direction::South
            } else {
                Direction::North
            }
        };
        let rank = if dir.is_vertical() {
            self.from.x() / LARGE_TILE_SIZE as i32
        } else {
            self.from.y() / LARGE_TILE_SIZE as i32
        };
        let dist = cmp::min(20, cmp::max(delta.x().abs(), delta.y().abs()));
        (dir, rank, dist)
    }

    fn offset_for(&self, col: i32, row: i32) -> Point {
        let (dir, rank, dist) = self.dir_rank_dist();
        let for_rank = if dir.is_vertical() {
            col
        } else {
            row
        };
        if rank == for_rank {
            dir.delta() * dist
        } else {
            Point::new(0, 0)
        }
    }

    fn command(self) -> Option<(Direction, i32)> {
        let (dir, rank, dist) = self.dir_rank_dist();
        if dist > LARGE_TILE_SIZE as i32 / 2 {
            Some((dir, rank))
        } else {
            None
        }
    }
}

// ========================================================================= //

struct WreckedGrid {
    left: i32,
    top: i32,
    tile_sprites: Vec<Sprite>,
    drag: Option<Drag>,
}

impl WreckedGrid {
    fn new(resources: &mut Resources, left: i32, top: i32) -> WreckedGrid {
        WreckedGrid {
            left: left,
            top: top,
            tile_sprites: resources.get_sprites("wrecked/large"),
            drag: None,
        }
    }

    fn rect(&self) -> Rect {
        Rect::new(self.left,
                  self.top,
                  9 * LARGE_TILE_SIZE,
                  7 * LARGE_TILE_SIZE)
    }
}

impl Element<WreckedState, (Direction, i32)> for WreckedGrid {
    fn draw(&self, state: &WreckedState, canvas: &mut Canvas) {
        canvas.fill_rect((15, 20, 15),
                         Rect::new(self.left,
                                   self.top,
                                   9 * LARGE_TILE_SIZE,
                                   7 * LARGE_TILE_SIZE));
        for row in 0..7 {
            let top = self.top + row * LARGE_TILE_SIZE as i32;
            for col in 0..9 {
                let left = self.left + col * LARGE_TILE_SIZE as i32;
                let mut pt = Point::new(left, top);
                if let Some(ref drag) = self.drag {
                    pt = pt + drag.offset_for(col, row);
                }
                if let Some(index) = state.tile_at(col, row) {
                    canvas.draw_sprite(&self.tile_sprites[index], pt);
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut WreckedState)
                    -> Action<(Direction, i32)> {
        let rect = self.rect();
        match event {
            &Event::MouseDown(pt) if !state.is_solved() => {
                if rect.contains(pt) {
                    let rel_pt = pt - rect.top_left();
                    let col = rel_pt.x() / LARGE_TILE_SIZE as i32;
                    let row = rel_pt.y() / LARGE_TILE_SIZE as i32;
                    if state.tile_at(col, row).is_some() {
                        self.drag = Some(Drag::new(rel_pt));
                    }
                }
                Action::ignore()
            }
            &Event::MouseDrag(pt) => {
                if let Some(ref mut drag) = self.drag {
                    drag.to = pt - rect.top_left();
                    Action::redraw()
                } else {
                    Action::ignore()
                }
            }
            &Event::MouseUp => {
                if let Some(cmd) = self.drag.take().and_then(Drag::command) {
                    Action::redraw().and_return(cmd)
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const SOLUTION_LEFT: i32 = 452;
const SOLUTION_TOP: i32 = 211;

struct SolutionDisplay {
    font: Rc<Font>,
    sprites: Vec<Sprite>,
    index: usize,
    anim: usize,
}

impl SolutionDisplay {
    fn new(resources: &mut Resources) -> SolutionDisplay {
        SolutionDisplay {
            font: resources.get_font("roman"),
            sprites: resources.get_sprites("wrecked/solution"),
            index: 0,
            anim: 0,
        }
    }

    fn set_index(&mut self, index: i32) {
        if index >= 0 {
            self.index = index as usize;
            self.anim = 12;
        } else {
            self.index = (-index - 1) as usize;
            self.anim = 0;
        }
    }
}

impl Element<WreckedState, PuzzleCmd> for SolutionDisplay {
    fn draw(&self, state: &WreckedState, canvas: &mut Canvas) {
        let index = if self.anim > 0 {
            ((self.anim / 2) % 3) + 3
        } else {
            self.index
        };
        canvas.draw_sprite(&self.sprites[index],
                           Point::new(SOLUTION_LEFT, SOLUTION_TOP));
        if index == 0 {
            canvas.draw_text(&self.font,
                             Align::Center,
                             Point::new(SOLUTION_LEFT + 28,
                                        SOLUTION_TOP + 18),
                             "Status:");
            let status = if state.is_solved() {
                "Fixed, sorta."
            } else {
                "BORKEN"
            };
            canvas.draw_text(&self.font,
                             Align::Center,
                             Point::new(SOLUTION_LEFT + 28,
                                        SOLUTION_TOP + 32),
                             status);
        }
    }

    fn handle_event(&mut self, event: &Event, _state: &mut WreckedState)
                    -> Action<PuzzleCmd> {
        match event {
            &Event::ClockTick => {
                if self.anim > 0 {
                    self.anim -= 1;
                    Action::redraw()
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to arrange the large grid on the left into
the pattern shown on the small grid on the right.

Drag a tile on the large grid up, down, left, or right
to shift that whole row or column by one.";

// ========================================================================= //
