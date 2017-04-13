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

use num_integer::div_floor;

use elements::{PuzzleCmd, PuzzleCore, PuzzleView};
use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use modes::SOLVED_INFO_TEXT;
use save::{Direction, Game, PuzzleState, SimpleState};
use save::plane::{PlaneGrid, PlaneObj};
use super::scenes::{compile_intro_scene, compile_outro_scene};

// ========================================================================= //

pub struct View {
    core: PuzzleCore<()>,
    grid: PlaneGridView,
}

impl View {
    pub fn new(resources: &mut Resources, visible: Rect, state: &SimpleState)
               -> View {
        let intro = compile_intro_scene(resources);
        let outro = compile_outro_scene(resources);
        let core = PuzzleCore::new(resources, visible, state, intro, outro);
        let mut view = View {
            core: core,
            grid: PlaneGridView::new(resources, 100, 50),
        };
        view.drain_queue();
        view
    }

    fn drain_queue(&mut self) {
        for (_, _) in self.core.drain_queue() {
            // TODO drain queue
        }
    }
}

impl Element<Game, PuzzleCmd> for View {
    fn draw(&self, game: &Game, canvas: &mut Canvas) {
        let state = &game.plane_and_simple;
        self.core.draw_back_layer(canvas);
        self.grid.draw(state.grid(), canvas);
        self.core.draw_middle_layer(canvas);
        self.core.draw_front_layer(canvas, state);
    }

    fn handle_event(&mut self, event: &Event, game: &mut Game)
                    -> Action<PuzzleCmd> {
        let state = &mut game.plane_and_simple;
        let mut action = self.core.handle_event(event, state);
        self.drain_queue();
        if !action.should_stop() && !state.is_solved() {
            let subaction = self.grid.handle_event(event, state.grid_mut());
            if let Some(&()) = subaction.value() {
                if state.advance_stage_if_done() {
                    self.core.clear_undo_redo();
                    if state.is_solved() {
                        self.core.begin_outro_scene();
                    } else {
                        // TODO play sound and animate grid changes
                    }
                } else {
                    // TODO push undo
                }
            }
            action.merge(subaction.but_no_value());
        }
        action
    }
}

impl PuzzleView for View {
    fn info_text(&self, game: &Game) -> &'static str {
        if game.plane_and_simple.is_solved() {
            SOLVED_INFO_TEXT
        } else {
            INFO_BOX_TEXT
        }
    }

    fn undo(&mut self, _game: &mut Game) {
        if let Some(_) = self.core.pop_undo() {
            // TODO implement undo
        }
    }

    fn redo(&mut self, _game: &mut Game) {
        if let Some(_) = self.core.pop_redo() {
            // TODO implement redo
        }
    }

    fn reset(&mut self, game: &mut Game) {
        let state = &mut game.plane_and_simple;
        self.core.clear_undo_redo();
        state.reset();
    }

    fn solve(&mut self, game: &mut Game) {
        let state = &mut game.plane_and_simple;
        state.solve();
        self.core.begin_outro_scene();
        self.drain_queue();
    }
}

// ========================================================================= //

const TILE_SIZE: u32 = 24;

struct PlaneGridView {
    left: i32,
    top: i32,
    obj_sprites: Vec<Sprite>,
    pipe_sprites: Vec<Sprite>,
    drag_from: Option<Point>,
}

impl PlaneGridView {
    fn new(resources: &mut Resources, left: i32, top: i32) -> PlaneGridView {
        PlaneGridView {
            left: left,
            top: top,
            obj_sprites: resources.get_sprites("plane/objects"),
            pipe_sprites: resources.get_sprites("plane/pipes"),
            drag_from: None,
        }
    }

    fn rect(&self, grid: &PlaneGrid) -> Rect {
        Rect::new(self.left,
                  self.top,
                  grid.width() * TILE_SIZE,
                  grid.height() * TILE_SIZE)
    }

    fn pt_to_coords(&self, grid: &PlaneGrid, pt: Point) -> Option<Point> {
        let col = div_floor(pt.x() - self.left, TILE_SIZE as i32);
        let row = div_floor(pt.y() - self.top, TILE_SIZE as i32);
        let coords = Point::new(col, row);
        if grid.rect().contains(coords) {
            Some(coords)
        } else {
            None
        }
    }

    fn draw_pipe_tip(&self, grid: &PlaneGrid, pos: Point, dir: Direction,
                     canvas: &mut Canvas) {
        let obj = grid.objects().get(&pos).cloned();
        let sprite_index = match (dir, obj) {
            (Direction::West, Some(PlaneObj::Cross)) => 10,
            (Direction::West, Some(PlaneObj::Node)) => 13,
            (Direction::West, _) => 0,
            (Direction::East, Some(PlaneObj::Cross)) => 11,
            (Direction::East, Some(PlaneObj::Node)) => 15,
            (Direction::East, _) => 2,
            (Direction::South, Some(PlaneObj::Node)) => 14,
            (Direction::South, _) => 1,
            (Direction::North, Some(PlaneObj::Node)) => 16,
            (Direction::North, _) => 3,
        };
        let sprite = &self.pipe_sprites[sprite_index];
        canvas.draw_sprite(sprite, pos * TILE_SIZE as i32);
    }
}

impl Element<PlaneGrid, ()> for PlaneGridView {
    fn draw(&self, grid: &PlaneGrid, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect(grid));
        canvas.clear((64, 64, 64));
        for (&coords, &obj) in grid.objects() {
            let sprite_index = match obj {
                PlaneObj::Wall => 0,
                PlaneObj::Cross => 1,
                PlaneObj::Node => 2,
            };
            let sprite = &self.obj_sprites[sprite_index];
            canvas.draw_sprite(sprite, coords * TILE_SIZE as i32);
        }
        for pipe in grid.pipes() {
            debug_assert!(pipe.len() >= 2);
            let mut start = pipe[0];
            let mut next = pipe[1];
            let mut dir = Direction::from_delta(next - start);
            self.draw_pipe_tip(grid, start, dir, &mut canvas);
            for index in 2..pipe.len() {
                start = next;
                next = pipe[index];
                let prev_dir = dir;
                dir = Direction::from_delta(next - start);
                let sprite_index = match (prev_dir, dir) {
                    (Direction::East, Direction::North) => 6,
                    (Direction::East, Direction::South) => 7,
                    (Direction::West, Direction::North) => 5,
                    (Direction::West, Direction::South) => 4,
                    (Direction::East, _) |
                    (Direction::West, _) => {
                        let obj = grid.objects().get(&start).cloned();
                        if obj == Some(PlaneObj::Cross) { 12 } else { 8 }
                    }
                    (Direction::North, Direction::East) => 4,
                    (Direction::North, Direction::West) => 7,
                    (Direction::South, Direction::East) => 5,
                    (Direction::South, Direction::West) => 6,
                    (Direction::North, _) |
                    (Direction::South, _) => 9,
                };
                let sprite = &self.pipe_sprites[sprite_index];
                canvas.draw_sprite(sprite, start * TILE_SIZE as i32);
            }
            dir = Direction::from_delta(start - next);
            self.draw_pipe_tip(grid, next, dir, &mut canvas);
        }
    }

    fn handle_event(&mut self, event: &Event, grid: &mut PlaneGrid)
                    -> Action<()> {
        match event {
            &Event::MouseDown(pt) if self.rect(grid).contains(pt) => {
                self.drag_from = self.pt_to_coords(grid, pt);
                Action::ignore().and_stop()
            }
            &Event::MouseDrag(pt) => {
                if let Some(coords1) = self.drag_from {
                    if let Some(coords2) = self.pt_to_coords(grid, pt) {
                        self.drag_from = Some(coords2);
                        if grid.toggle_pipe(coords1, coords2) {
                            return Action::redraw().and_return(());
                        }
                    }
                }
                Action::ignore()
            }
            &Event::MouseUp => {
                self.drag_from = None;
                Action::ignore()
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //

const INFO_BOX_TEXT: &'static str = "\
Your goal is to connect each purple node to each other
purple node.

Drag across the grid with $M{your finger}{the mouse} to create or
remove pipes between the nodes.";

// ========================================================================= //
