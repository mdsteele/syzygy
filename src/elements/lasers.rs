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
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use crate::gui::{Action, Align, Canvas, Element, Event, FRAME_DELAY_MILLIS, Font,
          Point, Rect, Resources, Sound, Sprite};
use crate::save::{Direction, MixedColor};
use crate::save::device::{Device, DeviceGrid};

// ========================================================================= //

const GRID_CELL_SIZE: i32 = 32;
const ROTATE_MAX_MILLIS: u32 = 200;
const LASER_THICKNESS: i32 = 4;
const ANIM_SLOWDOWN: i32 = 5;

// ========================================================================= //

#[derive(Clone, Copy)]
pub enum LaserCmd {
    Moved(i32, i32, i32, i32),
    Rotated(i32, i32),
}

// ========================================================================= //

struct GridDrag {
    device: Device,
    dir: Direction,
    from_col: i32,
    from_row: i32,
    from_pt: Point,
    to_pt: Point,
    millis: u32,
    moved: bool,
}

// ========================================================================= //

pub struct LaserField {
    rect: Rect,
    gate_sprites: Vec<Sprite>,
    gem_sprites: Vec<Sprite>,
    sparks_sprites: Vec<Sprite>,
    wall_sprites: Vec<Sprite>,
    drag: Option<GridDrag>,
    lasers: HashMap<(Point, Direction), (MixedColor, i32)>,
    sparks: HashMap<(Point, Direction), i32>,
    anim_counter: i32,
    font: Rc<Font>,
    letters: HashMap<(i32, i32), char>,
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
            font: resources.get_font("block"),
            letters: HashMap::new(),
        };
        laser_field.recalculate_lasers(grid);
        laser_field
    }

    pub fn add_letter(&mut self, coords: (i32, i32), letter: char) {
        self.letters.insert(coords, letter);
    }

    pub fn satisfied_detector_positions(&self, grid: &DeviceGrid)
                                        -> HashSet<(i32, i32)> {
        let mut positions = HashSet::new();
        let (num_cols, num_rows) = grid.size();
        for row in 0..num_rows {
            for col in 0..num_cols {
                match grid.get(col, row) {
                    Some((Device::Detector(color), dir)) => {
                        let coords = Point::new(col, row);
                        match self.lasers.get(&(coords, dir)) {
                            Some(&(laser, _)) if laser == color => {
                                positions.insert((col, row));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        positions
    }

    pub fn all_detectors_satisfied(&self, grid: &DeviceGrid) -> bool {
        let (num_cols, num_rows) = grid.size();
        for row in 0..num_rows {
            for col in 0..num_cols {
                match grid.get(col, row) {
                    Some((Device::Detector(color), dir)) => {
                        let coords = Point::new(col, row);
                        match self.lasers.get(&(coords, dir)) {
                            Some(&(laser, _)) if laser == color => {}
                            _ => return false,
                        }
                    }
                    _ => {}
                }
            }
        }
        true
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
            Device::CrossChannel => {
                canvas.draw_sprite_centered(&self.wall_sprites[2], center);
            }
            Device::Emitter(color) => {
                canvas.draw_sprite_transformed(&self.wall_sprites[3],
                                               center,
                                               -dir.degrees(),
                                               dir.is_vertical(),
                                               false);
                let index = color_index(color);
                canvas.draw_sprite_centered(&self.gem_sprites[index], center);
            }
            Device::Detector(color) => {
                canvas.draw_sprite_transformed(&self.wall_sprites[3],
                                               center,
                                               -dir.degrees(),
                                               dir.is_vertical(),
                                               false);
                let index = color_index(color) + 8;
                canvas.draw_sprite_rotated(&self.gem_sprites[index],
                                           center,
                                           dir.degrees());
            }
            Device::Mirror => {
                canvas.draw_sprite_rotated(&self.gate_sprites[0],
                                           center,
                                           dir.degrees());
            }
            Device::Splitter => {
                canvas.draw_sprite_rotated(&self.gate_sprites[2],
                                           center,
                                           dir.degrees());
            }
            Device::Mixer => {
                canvas.draw_sprite_rotated(&self.gate_sprites[3],
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

    pub fn clear_lasers(&mut self) {
        self.lasers.clear();
        self.sparks.clear();
        self.drag = None;
    }

    pub fn recalculate_lasers(&mut self, grid: &DeviceGrid) {
        self.clear_lasers();
        let (num_cols, num_rows) = grid.size();
        let mut queue: VecDeque<(Point, Direction, MixedColor)> =
            VecDeque::new();
        for row in 0..num_rows {
            for col in 0..num_cols {
                match grid.get(col, row) {
                    Some((Device::Emitter(color), dir)) => {
                        if color != MixedColor::Black {
                            let coords = Point::new(col, row);
                            self.lasers.insert((coords, dir), (color, 10));
                            queue.push_back((coords, dir, color));
                        }
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
                Some((Device::CrossChannel, _)) |
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
                Some((Device::Splitter, split_dir)) => {
                    if split_dir == laser_dir {
                        self.lasers.insert((next, anti_dir), (color, 6));
                        let left_dir = laser_dir.rotated_ccw();
                        let right_dir = laser_dir.rotated_cw();
                        self.lasers.insert((next, left_dir), (color, 6));
                        self.lasers.insert((next, right_dir), (color, 6));
                        self.sparks.remove(&(next, left_dir));
                        self.sparks.remove(&(next, right_dir));
                        queue.push_back((next, left_dir, color));
                        queue.push_back((next, right_dir, color));
                    } else if split_dir == anti_dir {
                        self.lasers.insert((next, anti_dir), (color, 3));
                        self.sparks.insert((next, anti_dir), 3);
                    } else {
                        self.lasers.insert((next, anti_dir), (color, 6));
                        self.sparks.insert((next, anti_dir), 6);
                    }
                }
                Some((Device::Mixer, mixer_dir)) => {
                    if mixer_dir == laser_dir {
                        self.lasers.insert((next, anti_dir), (color, 1));
                        self.sparks.insert((next, anti_dir), 1);
                    } else if mixer_dir == anti_dir {
                        self.lasers.insert((next, anti_dir), (color, 3));
                        self.sparks.insert((next, anti_dir), 3);
                    } else {
                        self.lasers.insert((next, anti_dir), (color, 3));
                        if let Some(&(other, _)) =
                            self.lasers.get(&(next, laser_dir))
                        {
                            let output = mixer_output(color, other);
                            self.lasers.insert((next, mixer_dir), (output, 3));
                            self.sparks.remove(&(next, mixer_dir));
                            queue.push_back((next, mixer_dir, output));
                        }
                    }
                }
            }
        }
    }

    pub fn draw_immovables(&self, grid: &DeviceGrid, canvas: &mut Canvas) {
        let (num_cols, num_rows) = grid.size();
        let mut canvas = canvas.subcanvas(self.rect);
        canvas.clear((64, 64, 64));
        for row in 0..num_rows {
            for col in 0..num_cols {
                if let Some((device, dir)) = grid.get(col, row) {
                    if !device.is_moveable() {
                        let pt = Point::new(col * GRID_CELL_SIZE +
                                                GRID_CELL_SIZE / 2,
                                            row * GRID_CELL_SIZE +
                                                GRID_CELL_SIZE / 2);
                        self.draw_device_bg(&mut canvas, pt, device, dir);
                    }
                }
            }
        }
    }

    fn draw_movables_bg(&self, grid: &DeviceGrid, canvas: &mut Canvas) {
        let (num_cols, num_rows) = grid.size();
        let mut canvas = canvas.subcanvas(self.rect);
        for row in 0..num_rows {
            for col in 0..num_cols {
                if let Some(ref drag) = self.drag {
                    if drag.from_pt != drag.to_pt && row == drag.from_row &&
                        col == drag.from_col
                    {
                        continue;
                    }
                }
                if let Some((device, dir)) = grid.get(col, row) {
                    if device.is_moveable() {
                        let pt = Point::new(col * GRID_CELL_SIZE +
                                                GRID_CELL_SIZE / 2,
                                            row * GRID_CELL_SIZE +
                                                GRID_CELL_SIZE / 2);
                        self.draw_device_bg(&mut canvas, pt, device, dir);
                    }
                }
            }
        }
    }

    fn draw_movables_fg(&self, grid: &DeviceGrid, canvas: &mut Canvas) {
        let (num_cols, num_rows) = grid.size();
        let mut canvas = canvas.subcanvas(self.rect);
        for row in 0..num_rows {
            for col in 0..num_cols {
                if let Some(ref drag) = self.drag {
                    if drag.from_pt != drag.to_pt && row == drag.from_row &&
                        col == drag.from_col
                    {
                        continue;
                    }
                }
                if let Some((device, dir)) = grid.get(col, row) {
                    if device.is_moveable() {
                        let pt = Point::new(col * GRID_CELL_SIZE +
                                                GRID_CELL_SIZE / 2,
                                            row * GRID_CELL_SIZE +
                                                GRID_CELL_SIZE / 2);
                        self.draw_device_fg(&mut canvas, pt, device, dir);
                    }
                }
            }
        }
    }

    pub fn draw_lasers(&self, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect);
        for (&(coords, dir), &(laser_color, dist)) in self.lasers.iter() {
            let fill_color = match laser_color {
                MixedColor::Black => unreachable!(),
                MixedColor::Red => (255, 64, 64),
                MixedColor::Green => (64, 255, 64),
                MixedColor::Yellow => (255, 255, 64),
                MixedColor::Blue => (64, 64, 255),
                MixedColor::Magenta => (255, 64, 255),
                MixedColor::Cyan => (64, 255, 255),
                MixedColor::White => (255, 255, 255),
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
    }

    pub fn draw_sparks(&self, canvas: &mut Canvas) {
        for (&(coords, dir), &dist) in self.sparks.iter() {
            let center = self.rect.top_left() +
                dir.delta() * (GRID_CELL_SIZE / 2 - dist) +
                Point::new(coords.x() * GRID_CELL_SIZE + GRID_CELL_SIZE / 2,
                           coords.y() * GRID_CELL_SIZE + GRID_CELL_SIZE / 2);
            canvas.draw_sprite_transformed(&self.sparks_sprites[0],
                                           center,
                                           dir.degrees(),
                                           self.anim_counter < ANIM_SLOWDOWN,
                                           false);
        }
    }
}

impl Element<DeviceGrid, LaserCmd> for LaserField {
    fn draw(&self, grid: &DeviceGrid, canvas: &mut Canvas) {
        self.draw_immovables(grid, canvas);
        for (&(col, row), &letter) in self.letters.iter() {
            let pt = Point::new(self.rect.left() + col * GRID_CELL_SIZE +
                                    GRID_CELL_SIZE / 2,
                                self.rect.top() + row * GRID_CELL_SIZE +
                                    GRID_CELL_SIZE / 2 +
                                    9);
            canvas.draw_char(&self.font, Align::Center, pt, letter);
        }
        self.draw_movables_bg(grid, canvas);
        self.draw_lasers(canvas);
        self.draw_movables_fg(grid, canvas);
        self.draw_sparks(canvas);
        if let Some(ref drag) = self.drag {
            if drag.from_pt != drag.to_pt {
                let center = self.rect.top_left() + drag.to_pt;
                self.draw_device_bg(canvas, center, drag.device, drag.dir);
                self.draw_device_fg(canvas, center, drag.device, drag.dir);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, grid: &mut DeviceGrid)
                    -> Action<LaserCmd> {
        match event {
            &Event::ClockTick => {
                if let Some(ref mut drag) = self.drag {
                    drag.millis = cmp::min(drag.millis + FRAME_DELAY_MILLIS,
                                           ROTATE_MAX_MILLIS + 1);
                }
                self.anim_counter += 1;
                self.anim_counter %= 2 * ANIM_SLOWDOWN;
                if self.anim_counter % ANIM_SLOWDOWN == 0 &&
                    !self.sparks.is_empty()
                {
                    return Action::redraw();
                }
            }
            &Event::MouseDown(pt) => {
                if self.rect.contains_point(pt) {
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
                                                 moved: false,
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
                    let mut action = Action::redraw();
                    if !drag.moved {
                        drag.moved = true;
                        action.also_play_sound(Sound::device_pickup());
                    }
                    return action;
                }
            }
            &Event::MouseUp => {
                if let Some(drag) = self.drag.take() {
                    let to_col = drag.to_pt.x() / GRID_CELL_SIZE;
                    let to_row = drag.to_pt.y() / GRID_CELL_SIZE;
                    return if to_col == drag.from_col &&
                        to_row == drag.from_row
                    {
                        if drag.millis <= ROTATE_MAX_MILLIS {
                            grid.rotate(drag.from_col, drag.from_row);
                            self.recalculate_lasers(grid);
                            Action::redraw()
                                .and_play_sound(Sound::device_rotate())
                                .and_return(LaserCmd::Rotated(drag.from_col,
                                                              drag.from_row))
                        } else {
                            self.recalculate_lasers(grid);
                            Action::redraw()
                        }
                    } else {
                        let success = grid.move_to(drag.from_col,
                                                   drag.from_row,
                                                   to_col,
                                                   to_row);
                        self.recalculate_lasers(grid);
                        if success {
                            Action::redraw()
                                .and_play_sound(Sound::device_drop())
                                .and_return(LaserCmd::Moved(drag.from_col,
                                                            drag.from_row,
                                                            to_col,
                                                            to_row))
                        } else {
                            Action::redraw()
                        }
                    };
                }
            }
            _ => {}
        }
        Action::ignore()
    }
}

// ========================================================================= //

pub struct DangerSign {
    font1: Rc<Font>,
    font2: Rc<Font>,
    string1: String,
    string2: String,
    rect: Rect,
}

impl DangerSign {
    pub fn new(resources: &mut Resources, (left, top): (i32, i32),
               string1: &str, string2: &str)
               -> DangerSign {
        DangerSign {
            font1: resources.get_font("danger"),
            font2: resources.get_font("tiny"),
            string1: string1.to_string(),
            string2: string2.to_string(),
            rect: Rect::new(left, top, 80, 32),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        let mut canvas = canvas.subcanvas(self.rect);
        canvas.draw_text(&self.font1,
                         Align::Center,
                         Point::new(40, 16),
                         &self.string1);
        canvas.draw_text(&self.font2,
                         Align::Center,
                         Point::new(40, 26),
                         &self.string2);
    }
}

// ========================================================================= //

fn color_index(color: MixedColor) -> usize {
    match color {
        MixedColor::Black => 0,
        MixedColor::Red => 1,
        MixedColor::Green => 2,
        MixedColor::Yellow => 3,
        MixedColor::Blue => 4,
        MixedColor::Magenta => 5,
        MixedColor::Cyan => 6,
        MixedColor::White => 7,
    }
}

fn mixer_output(color1: MixedColor, color2: MixedColor) -> MixedColor {
    let red = (color1.has_red() && color2.has_red()) ||
        (color1.has_green() && color2.has_blue()) ||
        (color1.has_blue() && color2.has_green());
    let green = (color1.has_green() && color2.has_green()) ||
        (color1.has_red() && color2.has_blue()) ||
        (color1.has_blue() && color2.has_red());
    let blue = (color1.has_blue() && color2.has_blue()) ||
        (color1.has_red() && color2.has_green()) ||
        (color1.has_green() && color2.has_red());
    MixedColor::from_rgb(red, green, blue)
}

// ========================================================================= //
