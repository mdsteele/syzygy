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
use std::cmp;

use gui::{Action, Canvas, Element, Event, Point, Rect, Resources, Sprite};
use gui::Sound;
use save::Direction;
use save::ice::{BlockSlide, Object, ObjectGrid, Transform};

// ========================================================================= //

const GRID_CELL_SIZE: i32 = 32;

const SWIPE_THRESHOLD: i32 = 15;

const SLIDE_START_SPEED: i32 = 5;
const SLIDE_MAX_SPEED: i32 = 45;
const SLIDE_ACCEL: i32 = 8;

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

struct SlideAnimation {
    slide_dir: Direction,
    to_coords: Point,
    remaining_dist: i32,
    speed: i32,
    pushed: Option<Point>,
    transform: Transform,
}

impl SlideAnimation {
    fn cell_dist(&self) -> i32 {
        (self.remaining_dist + GRID_CELL_SIZE - 1) / GRID_CELL_SIZE
    }
}

// ========================================================================= //

pub struct GridView {
    rect: Rect,
    obj_sprites: Vec<Sprite>,
    symbol_sprites: Vec<Sprite>,
    drag: Option<GridDrag>,
    animation: Option<SlideAnimation>,
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
            animation: None,
        }
    }

    pub fn is_animating(&self) -> bool { self.animation.is_some() }

    pub fn animate_slide(&mut self, slide: &BlockSlide) {
        self.drag = None;
        self.animation = Some(SlideAnimation {
            slide_dir: slide.direction(),
            to_coords: slide.to_coords(),
            remaining_dist: GRID_CELL_SIZE * slide.distance(),
            speed: SLIDE_START_SPEED,
            pushed: slide.pushed(),
            transform: slide.transform().inverse(),
        });
    }

    pub fn reset_animation(&mut self) {
        self.drag = None;
        self.animation = None;
    }

    fn cell_rect(&self, coords: Point) -> Rect {
        Rect::new(coords.x() * GRID_CELL_SIZE,
                  coords.y() * GRID_CELL_SIZE,
                  GRID_CELL_SIZE as u32,
                  GRID_CELL_SIZE as u32)
    }

    fn draw_push_pop(&self, coords: Point, direction: Direction,
                     canvas: &mut Canvas) {
        if let Some(ref anim) = self.animation {
            if let Some(pushed) = anim.pushed {
                if pushed == coords {
                    {
                        let rect = self.cell_rect(coords);
                        let mut canvas = canvas.subcanvas(rect);
                        let center = canvas.rect().center() -
                                     anim.slide_dir.delta() *
                                     anim.remaining_dist;
                        self.draw_push_pop_at(center, direction, &mut canvas);
                    }
                    {
                        let rect = self.cell_rect(anim.to_coords);
                        let mut canvas = canvas.subcanvas(rect);
                        let center =
                            canvas.rect().center() +
                            anim.slide_dir.delta() *
                            cmp::max(0, GRID_CELL_SIZE - anim.remaining_dist);
                        self.draw_push_pop_at(center,
                                              direction.opposite(),
                                              &mut canvas);
                    }
                    return;
                }
            }
        }
        let center = coords * GRID_CELL_SIZE +
                     Point::new(GRID_CELL_SIZE / 2, GRID_CELL_SIZE / 2);
        self.draw_push_pop_at(center, direction, canvas);
    }

    fn draw_push_pop_at(&self, center: Point, direction: Direction,
                        canvas: &mut Canvas) {
        canvas.draw_sprite_transformed(&self.obj_sprites[2],
                                       center,
                                       direction.degrees(),
                                       false,
                                       (direction == Direction::South ||
                                        direction == Direction::West));
    }

    pub fn draw_objects(&self, grid: &ObjectGrid, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect);
        for (&coords, &object) in grid.objects() {
            let center = coords * GRID_CELL_SIZE +
                         Point::new(GRID_CELL_SIZE / 2, GRID_CELL_SIZE / 2);
            match object {
                Object::Gap => {}
                Object::Wall => {
                    canvas.draw_sprite_centered(&self.obj_sprites[1], center);
                }
                Object::PushPop(direction) => {
                    self.draw_push_pop(coords, direction, &mut canvas);
                }
                Object::Rotator => {
                    canvas.draw_sprite_centered(&self.obj_sprites[3], center);
                }
                Object::Reflector(vertical) => {
                    canvas.draw_sprite_rotated(&self.obj_sprites[4],
                                               center,
                                               if vertical { 90 } else { 0 });
                }
                Object::Goal(symbol) => {
                    canvas.draw_sprite_transformed(
                        &self.symbol_sprites[2 * symbol.sprite_index() + 1],
                        center,
                        symbol.sprite_degrees(),
                        false,
                        symbol.sprite_mirrored());
                }
            }
        }
    }

    pub fn draw_ice_blocks(&self, grid: &ObjectGrid, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect);
        for (&coords, &symbol) in grid.ice_blocks() {
            let mut center = coords * GRID_CELL_SIZE +
                             Point::new(GRID_CELL_SIZE / 2,
                                        GRID_CELL_SIZE / 2);
            let mut symbol = symbol;
            if let Some(ref anim) = self.animation {
                if anim.to_coords == coords {
                    center = center -
                             anim.slide_dir.delta() * anim.remaining_dist;
                    symbol = symbol.transformed(anim.transform);
                }
            }
            let sprite = &self.symbol_sprites[symbol.sprite_index() * 2];
            canvas.draw_sprite_transformed(sprite,
                                           center,
                                           symbol.sprite_degrees(),
                                           false,
                                           symbol.sprite_mirrored());
            canvas.draw_sprite_centered(&self.obj_sprites[0], center);
        }
    }
}

impl Element<ObjectGrid, (Point, Direction)> for GridView {
    fn draw(&self, grid: &ObjectGrid, canvas: &mut Canvas) {
        let (cols, rows) = grid.size();
        let objects = grid.objects();
        for row in 0..rows {
            for col in 0..cols {
                match objects.get(&Point::new(col, row)) {
                    Some(&Object::Gap) |
                    Some(&Object::Wall) => {}
                    _ => {
                        let rect =
                            Rect::new(self.rect.left() + col * GRID_CELL_SIZE,
                                      self.rect.top() + row * GRID_CELL_SIZE,
                                      GRID_CELL_SIZE as u32,
                                      GRID_CELL_SIZE as u32);
                        canvas.fill_rect((64, 64, 96), rect);
                    }
                }
            }
        }
        self.draw_objects(grid, canvas);
        self.draw_ice_blocks(grid, canvas);
    }

    fn handle_event(&mut self, event: &Event, grid: &mut ObjectGrid)
                    -> Action<(Point, Direction)> {
        match event {
            &Event::ClockTick => {
                if let Some(mut anim) = self.animation.take() {
                    let old_dist = anim.cell_dist();
                    anim.remaining_dist -= anim.speed;
                    anim.speed = cmp::min(anim.speed + SLIDE_ACCEL,
                                          SLIDE_MAX_SPEED);
                    let mut action = Action::redraw();
                    if anim.remaining_dist > 0 {
                        let new_dist = anim.cell_dist();
                        if new_dist != old_dist {
                            let coords = anim.to_coords -
                                         anim.slide_dir.delta() * new_dist;
                            match grid.objects().get(&coords) {
                                Some(&Object::Rotator) => {
                                    // TODO: play sound
                                    anim.transform = anim.transform
                                                         .rotated_cw()
                                }
                                Some(&Object::Reflector(false)) => {
                                    // TODO: play sound
                                    anim.transform = anim.transform
                                                         .flipped_horz()
                                }
                                Some(&Object::Reflector(true)) => {
                                    // TODO: play sound
                                    anim.transform = anim.transform
                                                         .flipped_vert()
                                }
                                _ => {}
                            }
                        }
                        self.animation = Some(anim);
                    } else {
                        action.also_play_sound(Sound::device_rotate());
                    }
                    return action;
                }
            }
            &Event::MouseDown(pt) if self.animation.is_none() => {
                let col = div_floor(pt.x() - self.rect.left(), GRID_CELL_SIZE);
                let row = div_floor(pt.y() - self.rect.top(), GRID_CELL_SIZE);
                let coords = Point::new(col, row);
                if grid.ice_blocks().contains_key(&coords) {
                    self.drag = Some(GridDrag::new(coords, pt));
                    // TODO: play "grab" sound
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
                        // TODO: play "slide" sound
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
