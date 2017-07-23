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
use std::mem;

use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use save::Direction;
use save::plane::{PlaneGrid, PlaneObj};

// ========================================================================= //

pub enum PlaneCmd {
    Changed,
    PushUndo(Vec<(Point, Point)>),
}

// ========================================================================= //

const TILE_SIZE: u32 = 24;

pub struct PlaneGridView {
    left: i32,
    top: i32,
    obj_sprites: Vec<Sprite>,
    pipe_sprites: Vec<Sprite>,
    drag_from: Option<Point>,
    changes: Vec<(Point, Point)>,
}

impl PlaneGridView {
    pub fn new(resources: &mut Resources, left: i32, top: i32)
               -> PlaneGridView {
        PlaneGridView {
            left: left,
            top: top,
            obj_sprites: resources.get_sprites("plane/objects"),
            pipe_sprites: resources.get_sprites("plane/pipes"),
            drag_from: None,
            changes: Vec::new(),
        }
    }

    pub fn cancel_drag_and_clear_changes(&mut self) {
        self.drag_from = None;
        self.changes.clear();
    }

    fn rect(&self, grid: &PlaneGrid) -> Rect {
        Rect::new(self.left,
                  self.top,
                  grid.num_cols() * TILE_SIZE,
                  grid.num_rows() * TILE_SIZE)
    }

    fn pt_to_coords(&self, grid: &PlaneGrid, pt: Point) -> Option<Point> {
        let col = div_floor(pt.x() - self.left, TILE_SIZE as i32);
        let row = div_floor(pt.y() - self.top, TILE_SIZE as i32);
        let coords = Point::new(col, row);
        if grid.contains_coords(coords) {
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
            (Direction::West, Some(obj)) if obj.is_node() => 13,
            (Direction::West, _) => 0,
            (Direction::East, Some(PlaneObj::Cross)) => 11,
            (Direction::East, Some(obj)) if obj.is_node() => 15,
            (Direction::East, _) => 2,
            (Direction::South, Some(obj)) if obj.is_node() => 14,
            (Direction::South, _) => 1,
            (Direction::North, Some(obj)) if obj.is_node() => 16,
            (Direction::North, _) => 3,
        };
        let sprite = &self.pipe_sprites[sprite_index];
        canvas.draw_sprite(sprite, pos * TILE_SIZE as i32);
    }
}

impl Element<PlaneGrid, PlaneCmd> for PlaneGridView {
    fn draw(&self, grid: &PlaneGrid, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect(grid));
        canvas.clear((64, 64, 64));
        for row in 0..(grid.num_rows() as i32) {
            for col in 0..(grid.num_cols() as i32) {
                let coords = Point::new(col, row);
                if let Some(&obj) = grid.objects().get(&coords) {
                    let sprite_index = match obj {
                        PlaneObj::Wall => 0,
                        PlaneObj::Cross => 1,
                        PlaneObj::PurpleNode => 2,
                        PlaneObj::RedNode => 3,
                        PlaneObj::GreenNode => 4,
                        PlaneObj::BlueNode => 5,
                        PlaneObj::GrayNode => 6,
                    };
                    let sprite = &self.obj_sprites[sprite_index];
                    canvas.draw_sprite(sprite, coords * TILE_SIZE as i32);
                } else {
                    let pt = coords * TILE_SIZE as i32;
                    let rect = Rect::new(pt.x() + 1,
                                         pt.y() + 1,
                                         TILE_SIZE - 2,
                                         TILE_SIZE - 2);
                    canvas.draw_rect((72, 72, 72), rect);
                }
            }
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
                    -> Action<PlaneCmd> {
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
                            self.changes.push((coords1, coords2));
                            return Action::redraw()
                                .and_return(PlaneCmd::Changed);
                        }
                    }
                }
                Action::ignore()
            }
            &Event::MouseUp => {
                self.drag_from = None;
                if self.changes.is_empty() {
                    Action::ignore()
                } else {
                    let vec = mem::replace(&mut self.changes, Vec::new());
                    Action::redraw().and_return(PlaneCmd::PushUndo(vec))
                }
            }
            _ => Action::ignore(),
        }
    }
}

// ========================================================================= //
