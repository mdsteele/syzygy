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

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Direction, Game, PuzzleState, CubeState};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<(Direction, i32, i32)>,
    grid: CubeGrid,
    solution: SolutionDisplay,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &CubeState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            grid: CubeGrid::new(resources, 232, 72),
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
        let state = &game.cube_tangle;
        self.core.draw_back_layer(canvas);
        self.solution.draw(state, canvas);
        self.grid.draw(state, canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.cube_tangle;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() {
            let subaction = self.grid.handle_event(event, state);
            if let Some(&(dir, rank, by)) = subaction.value() {
                if state.is_solved() {
                    self.core.begin_outro_scene();
                    self.drain_queue();
                } else {
                    self.core.push_undo((dir, rank, by));
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
        if game.cube_tangle.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, game: &mut Game) {
        if let Some((dir, rank, by)) = self.core.pop_undo() {
            game.cube_tangle.rotate_cubes(dir, rank, -by);
        }
    }

    fn redo(&mut self, game: &mut Game) {
        if let Some((dir, rank, by)) = self.core.pop_redo() {
            game.cube_tangle.rotate_cubes(dir, rank, by);
        }
    }

    fn reset(&mut self, game: &mut Game) {
        self.core.clear_undo_redo();
        game.cube_tangle.reset();
    }

    fn replay(&mut self, game: &mut Game) {
        game.cube_tangle.replay();
        self.core.replay();
        self.solution.set_index(-1);
        self.drain_queue();
    }

    fn solve(&mut self, game: &mut Game) {
        game.cube_tangle.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const CUBE_USIZE: u32 = 32;
const CUBE_SIZE: i32 = CUBE_USIZE as i32;

struct Drag {
    from: Point,
    to: Point,
    accum: Option<(Direction, i32, i32)>,
}

impl Drag {
    pub fn new(start: Point) -> Drag {
        Drag {
            from: start,
            to: start,
            accum: None,
        }
    }

    pub fn accum(self) -> Option<(Direction, i32, i32)> { self.accum }

    pub fn set_to(&mut self, to: Point) -> Option<(Direction, i32, i32)> {
        self.to = to;
        let (dir, rank, dist) = self.dir_rank_dist();
        if dist > CUBE_SIZE / 2 {
            let by = 1 + (dist - CUBE_SIZE / 2) / CUBE_SIZE;
            self.from = self.from + dir.delta() * (by * CUBE_SIZE);
            if let Some((accum_dir, accum_rank, ref mut accum_by)) =
                   self.accum {
                assert_eq!(dir.is_vertical(), accum_dir.is_vertical());
                assert_eq!(rank, accum_rank);
                if dir == accum_dir {
                    *accum_by += by;
                } else {
                    *accum_by -= by;
                }
            } else {
                self.accum = Some((dir, rank, by));
            }
            Some((dir, rank, by))
        } else {
            None
        }
    }

    fn dir_rank_dist(&self) -> (Direction, i32, i32) {
        let delta = self.to - self.from;
        let vertical = match self.accum {
            Some((dir, _, _)) => dir.is_vertical(),
            None => delta.x().abs() <= delta.y().abs(),
        };
        let (dir, dist) = if vertical {
            if delta.y() >= 0 {
                (Direction::South, delta.y())
            } else {
                (Direction::North, -delta.y())
            }
        } else {
            if delta.x() >= 0 {
                (Direction::East, delta.x())
            } else {
                (Direction::West, -delta.x())
            }
        };
        let rank = if dir.is_vertical() {
            self.from.x() / CUBE_SIZE
        } else {
            self.from.y() / CUBE_SIZE
        };
        (dir, rank, dist)
    }

    pub fn tilt_dir_for(&self, col: i32, row: i32) -> Option<Direction> {
        let (dir, rank, dist) = self.dir_rank_dist();
        let for_rank = if dir.is_vertical() {
            col
        } else {
            row
        };
        if rank == for_rank && dist >= CUBE_SIZE / 4 {
            Some(dir)
        } else {
            None
        }
    }
}

// ========================================================================= //

struct CubeGrid {
    left: i32,
    top: i32,
    cubes: Vec<Sprite>,
    faces: Vec<Sprite>,
    drag: Option<Drag>,
}

impl CubeGrid {
    fn new(resources: &mut Resources, left: i32, top: i32) -> CubeGrid {
        CubeGrid {
            left: left,
            top: top,
            cubes: resources.get_sprites("tangle/cubes"),
            faces: resources.get_sprites("tangle/faces"),
            drag: None,
        }
    }

    fn rect(&self) -> Rect {
        Rect::new(self.left, self.top, 4 * CUBE_USIZE, 4 * CUBE_USIZE)
    }

    fn tilt_dir_for(&self, col: i32, row: i32) -> Option<Direction> {
        if let Some(ref drag) = self.drag {
            drag.tilt_dir_for(col, row)
        } else {
            None
        }
    }
}

impl Element<CubeState, (Direction, i32, i32)> for CubeGrid {
    fn draw(&self, state: &CubeState, canvas: &mut Canvas) {
        for row in 0..4 {
            for col in 0..4 {
                let pt = Point::new(self.left + col * CUBE_SIZE,
                                    self.top + row * CUBE_SIZE);
                let (fr, rt, bt) = state.faces_at(col, row);
                if let Some(dir) = self.tilt_dir_for(col, row) {
                    if dir.is_vertical() {
                        canvas.draw_sprite_transposed(&self.cubes[1], pt);
                        let (tp, bt) = if dir == Direction::South {
                            (5 - bt, fr)
                        } else {
                            (fr, bt)
                        };
                        canvas.draw_sprite_transposed(&self.faces[12 + bt],
                                                      pt);
                        canvas.draw_sprite_transposed(&self.faces[18 + tp],
                                                      pt);
                        canvas.draw_sprite_transposed(&self.faces[24 + rt],
                                                      pt);
                    } else {
                        canvas.draw_sprite(&self.cubes[1], pt);
                        let (lt, rt) = if dir == Direction::East {
                            (5 - rt, fr)
                        } else {
                            (fr, rt)
                        };
                        canvas.draw_sprite(&self.faces[12 + rt], pt);
                        canvas.draw_sprite(&self.faces[18 + lt], pt);
                        canvas.draw_sprite(&self.faces[24 + bt], pt);
                    }
                } else {
                    canvas.draw_sprite(&self.cubes[0], pt);
                    canvas.draw_sprite(&self.faces[fr], pt);
                    canvas.draw_sprite(&self.faces[rt + 6], pt);
                    canvas.draw_sprite_transposed(&self.faces[bt + 6], pt);
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event, state: &mut CubeState)
                    -> Action<(Direction, i32, i32)> {
        let rect = self.rect();
        match event {
            &Event::MouseDown(pt) if !state.is_solved() => {
                if rect.contains(pt) {
                    let rel_pt = pt - rect.top_left();
                    self.drag = Some(Drag::new(rel_pt));
                }
                Action::ignore()
            }
            &Event::MouseDrag(pt) => {
                if let Some(mut drag) = self.drag.take() {
                    if let Some((dir, rank, by)) =
                           drag.set_to(pt - rect.top_left()) {
                        state.rotate_cubes(dir, rank, by);
                        if state.is_solved() {
                            return Action::redraw().and_return(drag.accum()
                                                                   .unwrap());
                        }
                    }
                    self.drag = Some(drag);
                    Action::redraw()
                } else {
                    Action::ignore()
                }
            }
            &Event::MouseUp => {
                if let Some(drag) = self.drag.take() {
                    if let Some(cmd) = drag.accum() {
                        Action::redraw().and_return(cmd)
                    } else {
                        Action::redraw()
                    }
                } else {
                    Action::ignore()
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const SOLUTION_LEFT: i32 = 96;
const SOLUTION_TOP: i32 = 128;

struct SolutionDisplay {
    sprites: Vec<Sprite>,
    index: usize,
    anim: usize,
}

impl SolutionDisplay {
    fn new(resources: &mut Resources) -> SolutionDisplay {
        SolutionDisplay {
            sprites: resources.get_sprites("tangle/solution"),
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

impl Element<CubeState, PuzzleCmd> for SolutionDisplay {
    fn draw(&self, _state: &CubeState, canvas: &mut Canvas) {
        let index = if self.anim > 0 {
            ((self.anim / 2) % 3) + 2
        } else {
            self.index
        };
        canvas.draw_sprite(&self.sprites[index],
                           Point::new(SOLUTION_LEFT, SOLUTION_TOP));
    }

    fn handle_event(&mut self, event: &Event, _state: &mut CubeState)
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
Your goal is to arrange the front faces of the cubes in
the large grid in the middle into the pattern shown on
the small grid on the left.

Drag a cube on the large grid up, down, left, or right
to rotate that whole row or column.";

// ========================================================================= //
