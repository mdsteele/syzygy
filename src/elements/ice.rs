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

use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use save::Direction;
use save::ice::{BlockSlide, Object, ObjectGrid};

// ========================================================================= //

const GRID_CELL_SIZE: i32 = 32;
const SWIPE_THRESHOLD: i32 = 15;

// ========================================================================= //

enum GridSwipe {
    Nothing,
    Reset,
    Swipe(Point, Direction),
}

struct GridDrag {
    coords: Point,
    from_mouse_pt: Point,
}

impl GridDrag {
    fn new(coords: Point, mouse_pt: Point) -> GridDrag {
        GridDrag {
            coords: coords,
            from_mouse_pt: mouse_pt,
        }
    }

    fn swipe(&self, to: Point) -> GridSwipe {
        let delta = to - self.from_mouse_pt;
        if delta.x().abs() < SWIPE_THRESHOLD {
            if delta.y().abs() < SWIPE_THRESHOLD {
                GridSwipe::Nothing
            } else {
                if delta.y() >= 2 * SWIPE_THRESHOLD {
                    GridSwipe::Swipe(self.coords, Direction::South)
                } else if delta.y() <= -2 * SWIPE_THRESHOLD {
                    GridSwipe::Swipe(self.coords, Direction::North)
                } else {
                    GridSwipe::Nothing
                }
            }
        } else {
            if delta.y().abs() < SWIPE_THRESHOLD {
                if delta.x() >= 2 * SWIPE_THRESHOLD {
                    GridSwipe::Swipe(self.coords, Direction::East)
                } else if delta.x() <= -2 * SWIPE_THRESHOLD {
                    GridSwipe::Swipe(self.coords, Direction::West)
                } else {
                    GridSwipe::Nothing
                }
            } else {
                GridSwipe::Reset
            }
        }
    }
}

// ========================================================================= //

pub struct GridView {
    rect: Rect,
    obj_sprites: Vec<Sprite>,
    symbol_sprites: Vec<Sprite>,
    drag: Option<GridDrag>,
}

impl GridView {
    pub fn new(resources: &mut Resources, left: i32, top: i32,
               grid: &ObjectGrid)
               -> GridView {
        let (num_cols, num_rows) = grid.size();
        GridView {
            rect: Rect::new(left,
                            top,
                            (num_cols * GRID_CELL_SIZE) as u32,
                            (num_rows * GRID_CELL_SIZE) as u32),
            obj_sprites: resources.get_sprites("ice/objects"),
            symbol_sprites: resources.get_sprites("ice/symbols"),
            drag: None,
        }
    }

    pub fn animate_slide(&mut self, _slide: &BlockSlide) {
        // TODO
    }
}

impl Element<ObjectGrid, (Point, Direction)> for GridView {
    fn draw(&self, grid: &ObjectGrid, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect);
        canvas.clear((64, 64, 96));
        for (&coords, &object) in grid.objects() {
            let center = coords * GRID_CELL_SIZE +
                         Point::new(GRID_CELL_SIZE / 2, GRID_CELL_SIZE / 2);
            match object {
                Object::Wall => {
                    canvas.draw_sprite_centered(&self.obj_sprites[1], center);
                }
                Object::PushPop(direction) => {
                    canvas.draw_sprite_transformed(&self.obj_sprites[2],
                                                   center,
                                                   direction.degrees(),
                                                   false,
                                                   (direction ==
                                                    Direction::South ||
                                                    direction ==
                                                    Direction::West));
                }
                Object::Rotator => {
                    canvas.draw_sprite_centered(&self.obj_sprites[3], center);
                }
                Object::Goal(direction, symbol) => {
                    canvas.draw_sprite_rotated(
                        &self.symbol_sprites[2 * symbol + 1],
                        center,
                        direction.degrees());
                }
            }
        }
        for (&coords, &(direction, symbol)) in grid.ice_blocks() {
            let center = coords * GRID_CELL_SIZE +
                         Point::new(GRID_CELL_SIZE / 2, GRID_CELL_SIZE / 2);
            canvas.draw_sprite_rotated(&self.symbol_sprites[symbol * 2],
                                       center,
                                       direction.degrees());
            canvas.draw_sprite_centered(&self.obj_sprites[0], center);
        }
    }

    fn handle_event(&mut self, event: &Event, grid: &mut ObjectGrid)
                    -> Action<(Point, Direction)> {
        match event {
            &Event::MouseDown(pt) if self.rect.contains(pt) => {
                let col = div_floor(pt.x() - self.rect.left(), GRID_CELL_SIZE);
                let row = div_floor(pt.y() - self.rect.top(), GRID_CELL_SIZE);
                let coords = Point::new(col, row);
                if grid.ice_blocks().contains_key(&coords) {
                    self.drag = Some(GridDrag::new(coords, pt));
                    // TODO: play sound
                    return Action::ignore().and_stop();
                }
            }
            &Event::MouseDrag(pt) => {
                let swipe = if let Some(ref drag) = self.drag {
                    drag.swipe(pt)
                } else {
                    GridSwipe::Nothing
                };
                match swipe {
                    GridSwipe::Nothing => {}
                    GridSwipe::Reset => self.drag = None,
                    GridSwipe::Swipe(coords, dir) => {
                        self.drag = None;
                        // TODO: play sound
                        return Action::redraw().and_return((coords, dir));
                    }
                }
            }
            &Event::MouseUp => {
                self.drag = None;
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //
