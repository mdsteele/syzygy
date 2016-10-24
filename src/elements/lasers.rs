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
use std::collections::{HashMap, VecDeque};

use gui::{Action, Canvas, Element, Event, FRAME_DELAY_MILLIS, Point, Rect,
          Resources, Sprite};
use save::{Device, DeviceGrid, Direction, LaserColor};

// ========================================================================= //

const GRID_CELL_SIZE: i32 = 32;
const DRAG_MIN_MILLIS: u32 = 200;
const LASER_THICKNESS: i32 = 4;
const ANIM_SLOWDOWN: i32 = 5;

struct GridDrag {
    device: Device,
    dir: Direction,
    from_col: i32,
    from_row: i32,
    from_pt: Point,
    to_pt: Point,
    millis: u32,
}

pub struct LaserField {
    rect: Rect,
    gate_sprites: Vec<Sprite>,
    gem_sprites: Vec<Sprite>,
    sparks_sprites: Vec<Sprite>,
    wall_sprites: Vec<Sprite>,
    drag: Option<GridDrag>,
    lasers: HashMap<(Point, Direction), (LaserColor, i32)>,
    sparks: HashMap<(Point, Direction), i32>,
    anim_counter: i32,
}

impl LaserField {
    pub fn new(resources: &mut Resources, left: i32, top: i32,
               grid: &DeviceGrid)
               -> LaserField {
        let (num_cols, num_rows) = grid.size();
        let mut laser_field = LaserField {
            rect: Rect::new(left,
                            top,
                            (num_cols * GRID_CELL_SIZE) as u32,
                            (num_rows * GRID_CELL_SIZE) as u32),
            gate_sprites: resources.get_sprites("devices/gates"),
            gem_sprites: resources.get_sprites("devices/gems"),
            sparks_sprites: resources.get_sprites("devices/sparks"),
            wall_sprites: resources.get_sprites("devices/walls"),
            drag: None,
            lasers: HashMap::new(),
            sparks: HashMap::new(),
            anim_counter: 0,
        };
        laser_field.recalculate_lasers(grid);
        laser_field
    }

    fn draw_device_bg(&self, canvas: &mut Canvas, center: Point,
                      device: Device, dir: Direction) {
        match device {
            Device::Wall => {
                canvas.draw_sprite_centered(&self.wall_sprites[0], center);
            }
            Device::Channel => {
                canvas.draw_sprite_transformed(&self.wall_sprites[1],
                                               center,
                                               dir.degrees(),
                                               dir.is_vertical(),
                                               false);
            }
            Device::Emitter(color) => {
                canvas.draw_sprite_transformed(&self.wall_sprites[2],
                                               center,
                                               dir.degrees(),
                                               dir.is_vertical(),
                                               false);
                let i = match color {
                    LaserColor::Red => 0,
                    LaserColor::Green => 1,
                    LaserColor::Blue => 2,
                };
                canvas.draw_sprite_centered(&self.gem_sprites[i], center);
            }
            Device::Detector(color) => {
                canvas.draw_sprite_transformed(&self.wall_sprites[2],
                                               center,
                                               dir.degrees(),
                                               dir.is_vertical(),
                                               false);
                let i = match color {
                    LaserColor::Red => 3,
                    LaserColor::Green => 4,
                    LaserColor::Blue => 5,
                };
                canvas.draw_sprite_rotated(&self.gem_sprites[i],
                                           center,
                                           dir.degrees());
            }
            Device::Mirror => {
                canvas.draw_sprite_rotated(&self.gate_sprites[0],
                                           center,
                                           dir.degrees());
            }
        }
    }

    fn draw_device_fg(&self, canvas: &mut Canvas, center: Point,
                      device: Device, dir: Direction) {
        match device {
            Device::Mirror => {
                canvas.draw_sprite_rotated(&self.gate_sprites[1],
                                           center,
                                           dir.degrees());
            }
            _ => {}
        }
    }

    fn clear_lasers(&mut self) {
        self.lasers.clear();
        self.sparks.clear();
    }

    fn recalculate_lasers(&mut self, grid: &DeviceGrid) {
        self.clear_lasers();
        let (num_cols, num_rows) = grid.size();
        let mut queue: VecDeque<(Point, Direction, LaserColor)> =
            VecDeque::new();
        for row in 0..num_rows {
            for col in 0..num_cols {
                match grid.get(col, row) {
                    Some((Device::Emitter(color), dir)) => {
                        let coords = Point::new(col, row);
                        self.lasers.insert((coords, dir), (color, 10));
                        queue.push_back((coords, dir, color));
                    }
                    _ => {}
                }
            }
        }
        while let Some((coords, laser_dir, color)) = queue.pop_front() {
            let next = coords + laser_dir.delta();
            let anti_dir = laser_dir.opposite();
            if self.lasers.contains_key(&(next, anti_dir)) {
                if !self.sparks.contains_key(&(next, anti_dir)) {
                    self.sparks.insert((coords, laser_dir), 0);
                }
                continue;
            }
            match grid.get(next.x(), next.y()) {
                Some((Device::Wall, _)) |
                Some((Device::Emitter(_), _)) => {
                    self.sparks.insert((coords, laser_dir), 0);
                }
                Some((Device::Channel, ch_dir))
                    if !ch_dir.is_parallel_to(laser_dir) => {
                    self.sparks.insert((coords, laser_dir), 0);
                }
                Some((Device::Channel, _)) |
                None => {
                    let perp_dir = laser_dir.rotated_cw();
                    let mut dist = GRID_CELL_SIZE / 2;
                    if self.lasers.contains_key(&(next, perp_dir)) {
                        dist -= LASER_THICKNESS / 2;
                    }
                    self.lasers.insert((next, anti_dir), (color, dist));
                    self.lasers.insert((next, laser_dir), (color, dist));
                    queue.push_back((next, laser_dir, color));
                }
                Some((Device::Detector(det_color), det_dir)) => {
                    if det_dir == anti_dir {
                        self.lasers.insert((next, anti_dir), (color, 10));
                        if det_color != color {
                            self.sparks.insert((next, anti_dir), 10);
                        }
                    } else {
                        self.sparks.insert((coords, laser_dir), 0);
                    }
                }
                Some((Device::Mirror, mir_dir)) => {
                    let mut reflect_dir = match anti_dir {
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North,
                        Direction::North => Direction::West,
                    };
                    if mir_dir.is_vertical() {
                        reflect_dir = reflect_dir.opposite();
                    }
                    self.lasers.insert((next, anti_dir), (color, 15));
                    self.lasers.insert((next, reflect_dir), (color, 15));
                    queue.push_back((next, reflect_dir, color));
                }
            }
        }
    }
}

impl Element<DeviceGrid, ()> for LaserField {
    fn draw(&self, grid: &DeviceGrid, canvas: &mut Canvas) {
        let (num_cols, num_rows) = grid.size();
        {
            let mut canvas = canvas.subcanvas(self.rect);
            canvas.clear((64, 64, 64));
            for row in 0..num_rows {
                for col in 0..num_cols {
                    if let Some(ref drag) = self.drag {
                        if drag.from_pt != drag.to_pt &&
                           row == drag.from_row &&
                           col == drag.from_col {
                            continue;
                        }
                    }
                    if let Some((device, dir)) = grid.get(col, row) {
                        let pt = Point::new(col * GRID_CELL_SIZE +
                                            GRID_CELL_SIZE / 2,
                                            row * GRID_CELL_SIZE +
                                            GRID_CELL_SIZE / 2);
                        self.draw_device_bg(&mut canvas, pt, device, dir);
                    }
                }
            }
            for (&(coords, dir), &(laser_color, dist)) in self.lasers.iter() {
                let fill_color = match laser_color {
                    LaserColor::Red => (255, 64, 64),
                    LaserColor::Green => (64, 255, 64),
                    LaserColor::Blue => (64, 64, 255),
                };
                let mut fill_rect = match dir {
                    Direction::East => {
                        Rect::new(GRID_CELL_SIZE - dist,
                                  (GRID_CELL_SIZE - LASER_THICKNESS) / 2,
                                  dist as u32,
                                  LASER_THICKNESS as u32)
                    }
                    Direction::South => {
                        Rect::new((GRID_CELL_SIZE - LASER_THICKNESS) / 2,
                                  GRID_CELL_SIZE - dist,
                                  LASER_THICKNESS as u32,
                                  dist as u32)
                    }
                    Direction::West => {
                        Rect::new(0,
                                  (GRID_CELL_SIZE - LASER_THICKNESS) / 2,
                                  dist as u32,
                                  LASER_THICKNESS as u32)
                    }
                    Direction::North => {
                        Rect::new((GRID_CELL_SIZE - LASER_THICKNESS) / 2,
                                  0,
                                  LASER_THICKNESS as u32,
                                  dist as u32)
                    }
                };
                fill_rect.offset(coords.x() * GRID_CELL_SIZE,
                                 coords.y() * GRID_CELL_SIZE);
                canvas.fill_rect(fill_color, fill_rect);
            }
            for row in 0..num_rows {
                for col in 0..num_cols {
                    if let Some(ref drag) = self.drag {
                        if drag.from_pt != drag.to_pt &&
                           row == drag.from_row &&
                           col == drag.from_col {
                            continue;
                        }
                    }
                    if let Some((device, dir)) = grid.get(col, row) {
                        let pt = Point::new(col * GRID_CELL_SIZE +
                                            GRID_CELL_SIZE / 2,
                                            row * GRID_CELL_SIZE +
                                            GRID_CELL_SIZE / 2);
                        self.draw_device_fg(&mut canvas, pt, device, dir);
                    }
                }
            }
        }
        for (&(coords, dir), &dist) in self.sparks.iter() {
            let center = self.rect.top_left() +
                         dir.delta() * (GRID_CELL_SIZE / 2 - dist) +
                         Point::new(coords.x() * GRID_CELL_SIZE +
                                    GRID_CELL_SIZE / 2,
                                    coords.y() * GRID_CELL_SIZE +
                                    GRID_CELL_SIZE / 2);
            canvas.draw_sprite_transformed(&self.sparks_sprites[0],
                                           center,
                                           dir.degrees(),
                                           self.anim_counter < ANIM_SLOWDOWN,
                                           false);
        }
        if let Some(ref drag) = self.drag {
            if drag.from_pt != drag.to_pt {
                let center = self.rect.top_left() + drag.to_pt;
                self.draw_device_bg(canvas, center, drag.device, drag.dir);
                self.draw_device_fg(canvas, center, drag.device, drag.dir);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, grid: &mut DeviceGrid)
                    -> Action<()> {
        match event {
            &Event::ClockTick => {
                if let Some(ref mut drag) = self.drag {
                    drag.millis = cmp::min(drag.millis + FRAME_DELAY_MILLIS,
                                           DRAG_MIN_MILLIS);
                }
                self.anim_counter += 1;
                self.anim_counter %= 2 * ANIM_SLOWDOWN;
                if self.anim_counter % ANIM_SLOWDOWN == 0 &&
                   !self.sparks.is_empty() {
                    return Action::redraw();
                }
            }
            &Event::MouseDown(pt) => {
                if self.rect.contains(pt) {
                    let pt = pt - self.rect.top_left();
                    let col = pt.x() / GRID_CELL_SIZE;
                    let row = pt.y() / GRID_CELL_SIZE;
                    if let Some((device, dir)) = grid.get(col, row) {
                        if device.is_moveable() {
                            self.drag = Some(GridDrag {
                                device: device,
                                dir: dir,
                                from_col: col,
                                from_row: row,
                                from_pt: pt,
                                to_pt: pt,
                                millis: 0,
                            });
                        }
                    }
                }
            }
            &Event::MouseDrag(pt) => {
                if let Some(ref mut drag) = self.drag {
                    drag.to_pt = pt - self.rect.top_left();
                    self.lasers.clear();
                    self.sparks.clear();
                    return Action::redraw();
                }
            }
            &Event::MouseUp => {
                if let Some(drag) = self.drag.take() {
                    let to_col = drag.to_pt.x() / GRID_CELL_SIZE;
                    let to_row = drag.to_pt.y() / GRID_CELL_SIZE;
                    if drag.millis < DRAG_MIN_MILLIS {
                        if to_col == drag.from_col && to_row == drag.from_row {
                            grid.rotate(drag.from_col, drag.from_row);
                            self.recalculate_lasers(grid);
                        }
                    } else {
                        grid.move_to(drag.from_col,
                                     drag.from_row,
                                     to_col,
                                     to_row);
                        self.recalculate_lasers(grid);
                    }
                    return Action::redraw();
                }
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //
