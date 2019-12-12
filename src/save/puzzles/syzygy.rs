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

use std::collections::HashSet;
use toml;

use super::PuzzleState;
use crate::gui::Point;
use crate::save::column::Columns;
use crate::save::device::{Device, DeviceGrid};
use crate::save::ice::{Object, ObjectGrid, Symbol, Transform};
use crate::save::plane::{PlaneGrid, PlaneObj};
use crate::save::util::{
    pop_array, pop_table, to_table, Tomlable, ACCESS_KEY,
};
use crate::save::{Access, Direction, Location, MixedColor};

// ========================================================================= //

const STAGE_KEY: &str = "stage";
const YTTRIS_KEY: &str = "yttris";
const ARGONY_KEY: &str = "argony";
const ELINSA_KEY: &str = "elinsa";
const UGRENT_KEY: &str = "ugrent";
const RELYNG_LIGHTS_KEY: &str = "relyng_lights";
const RELYNG_NEXT_KEY: &str = "relyng_next";
const MEZURE_COLUMNS_KEY: &str = "mezure_columns";
const MEZURE_ICE_GRID_KEY: &str = "mezure_ice_grid";
const MEZURE_PIPES_KEY: &str = "mezure_pipes";

#[cfg_attr(rustfmt, rustfmt_skip)]
const YTTRIS_COLUMNS_SPEC: &[(&str, i32, i32, &[(usize, i32)])] = &[
    ("UNDO", -1, 3, &[(0, 1), (4,  2), (3,  3)]),
    ("OPEN", -1, 2, &[(1, 1), (3, -2), (4,  3)]),
    ("FILE", -1, 1, &[(2, 1), (0,  2), (5,  3)]),
    ("BOLT", -1, 0, &[(3, 1), (5, -2), (1, -3)]),
    ("PICK", -1, 1, &[(4, 1),          (0, -3)]),
    ("KEYS", -1, 3, &[(5, 1), (2, -2), (4,  3)]),
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const ELINSA_SOLVED_PIPES: &[&[(i32, i32)]] = &[
    &[(7, 1), (6, 1), (5, 1), (4, 1), (3, 1), (2, 1), (2, 2)],
    &[(7, 1), (7, 2), (7, 3), (7, 4), (6, 4), (5, 4), (5, 3)],
    &[(7, 1), (7, 0), (6, 0), (5, 0), (4, 0), (3, 0), (2, 0), (1, 0), (1, 1),
      (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (1, 5)],
    &[(2, 2), (2, 3), (3, 3), (4, 3), (5, 3)],
    &[(2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 2), (9, 3)],
    &[(2, 2), (1, 2), (1, 3), (1, 4), (1, 5)],
    &[(5, 3), (6, 3), (7, 3), (8, 3), (9, 3)],
    &[(9, 3), (9, 4), (8, 4), (8, 5), (7, 5), (6, 5), (5, 5), (4, 5), (3, 5),
      (2, 5), (1, 5)],
];

const RELYNG_NUM_COLS: i32 = 5;
const RELYNG_NUM_ROWS: i32 = 4;
const RELYNG_INIT_NEXT: char = '+';

#[cfg_attr(rustfmt, rustfmt_skip)]
const MEZURE_COLUMNS_SPEC: &[(&str, i32, i32, &[(usize, i32)])] = &[
    ("YTTRIS", -5, 0, &[(0, 1), (1, -1), (2, 1), (3, -1), (4, 1), (5, -1)]),
    ("ARGONY", -5, 0, &[(0, -1), (1, 1), (2, -1), (3, 1), (4, -1), (5, 1)]),
    ("ELINSA", -5, 5, &[(0, 1), (1, -1), (2, 1), (3, -1), (4, 1), (5, -1)]),
    ("UGRENT", -5, 0, &[(0, -1), (1, 1), (2, -1), (3, 1), (4, -1), (5, 1)]),
    ("RELYNG", -5, 2, &[(0, 1), (1, -1), (2, 1), (3, -1), (4, 1), (5, -1)]),
    ("MEZURE", -5, 1, &[(0, -1), (1, 1), (2, -1), (3, 1), (4, -1), (5, 1)]),
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const MEZURE_GRAY_NODES_EMITTERS: &[((i32, i32), (i32, i32), Direction)] = &[
    ((3, 2), (0, 6), Direction::North),
    ((3, 4), (1, 6), Direction::North),
    ((3, 6), (2, 6), Direction::North),
];

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SyzygyStage {
    Yttris,
    Argony,
    Elinsa,
    Ugrent,
    Relyng,
    Mezure,
}

impl SyzygyStage {
    pub fn first() -> SyzygyStage {
        SyzygyStage::Yttris
    }

    pub fn next(self) -> SyzygyStage {
        match self {
            SyzygyStage::Yttris => SyzygyStage::Argony,
            SyzygyStage::Argony => SyzygyStage::Elinsa,
            SyzygyStage::Elinsa => SyzygyStage::Ugrent,
            SyzygyStage::Ugrent => SyzygyStage::Relyng,
            SyzygyStage::Relyng => SyzygyStage::Mezure,
            SyzygyStage::Mezure => SyzygyStage::Mezure,
        }
    }
}

impl Tomlable for SyzygyStage {
    fn from_toml(value: toml::Value) -> SyzygyStage {
        if let Some(string) = value.as_str() {
            match string {
                "yttris" => return SyzygyStage::Yttris,
                "argony" => return SyzygyStage::Argony,
                "elinsa" => return SyzygyStage::Elinsa,
                "ugrent" => return SyzygyStage::Ugrent,
                "relyng" => return SyzygyStage::Relyng,
                "mezure" => return SyzygyStage::Mezure,
                _ => {}
            }
        }
        SyzygyStage::first()
    }

    fn to_toml(&self) -> toml::Value {
        let string = match *self {
            SyzygyStage::Yttris => "yttris",
            SyzygyStage::Argony => "argony",
            SyzygyStage::Elinsa => "elinsa",
            SyzygyStage::Ugrent => "ugrent",
            SyzygyStage::Relyng => "relyng",
            SyzygyStage::Mezure => "mezure",
        };
        toml::Value::String(string.to_string())
    }
}

// ========================================================================= //

pub struct SyzygyState {
    access: Access,
    stage: SyzygyStage,
    yttris: Columns,
    argony: ObjectGrid,
    elinsa: PlaneGrid,
    ugrent: DeviceGrid,
    relyng_lights: HashSet<i32>,
    relyng_next: char,
    mezure_columns: Columns,
    mezure_lights: [bool; 6],
    mezure_satisfied: [bool; 6],
    mezure_ice_grid: ObjectGrid,
    mezure_laser_grid: DeviceGrid,
    mezure_pipe_grid: PlaneGrid,
}

impl SyzygyState {
    fn argony_base_grid() -> ObjectGrid {
        let q_goal = Symbol::CyanQ(Transform::identity());
        let u_goal = Symbol::CyanU(Transform::identity());
        let a_goal = Symbol::CyanA(Transform::identity());
        let d_goal = Symbol::CyanQ(Transform::identity().flipped_vert());
        let mut grid = ObjectGrid::new(9, 4);
        grid.add_object(1, 0, Object::Wall);
        grid.add_object(4, 0, Object::Wall);
        grid.add_object(1, 1, Object::Rotator);
        grid.add_object(4, 1, Object::Reflector(false));
        grid.add_object(8, 1, Object::Wall);
        grid.add_object(1, 2, Object::Wall);
        grid.add_object(3, 2, Object::PushPop(Direction::West));
        grid.add_object(4, 2, Object::Wall);
        grid.add_object(1, 3, Object::Goal(q_goal));
        grid.add_object(3, 3, Object::Goal(u_goal));
        grid.add_object(4, 3, Object::Wall);
        grid.add_object(5, 3, Object::Goal(a_goal));
        grid.add_object(7, 3, Object::Goal(d_goal));
        grid
    }

    fn argony_initial_grid() -> ObjectGrid {
        let mut grid = SyzygyState::argony_base_grid();
        let q_trans = Transform::identity().rotated_cw().rotated_cw();
        let u_trans = Transform::identity().flipped_vert();
        let a_trans = Transform::identity().rotated_cw().rotated_cw();
        let d_trans = Transform::identity().flipped_vert();
        grid.add_ice_block(5, 0, Symbol::CyanQ(q_trans));
        grid.add_ice_block(6, 0, Symbol::CyanA(a_trans));
        grid.add_ice_block(7, 0, Symbol::CyanU(u_trans));
        grid.add_ice_block(8, 0, Symbol::CyanQ(d_trans));
        grid
    }

    fn argony_solved_grid() -> ObjectGrid {
        SyzygyState::argony_base_grid().solved()
    }

    fn elinsa_initial_grid() -> PlaneGrid {
        let mut grid = PlaneGrid::new(10, 6);
        grid.place_object(0, 0, PlaneObj::Wall);
        grid.place_object(7, 1, PlaneObj::BlueNode);
        grid.place_object(8, 1, PlaneObj::Wall);
        grid.place_object(2, 2, PlaneObj::PurpleNode);
        grid.place_object(7, 2, PlaneObj::Cross);
        grid.place_object(5, 3, PlaneObj::RedNode);
        grid.place_object(7, 3, PlaneObj::Cross);
        grid.place_object(9, 3, PlaneObj::BlueNode);
        grid.place_object(3, 4, PlaneObj::Wall);
        grid.place_object(1, 5, PlaneObj::RedNode);
        grid
    }

    fn ugrent_base_grid() -> DeviceGrid {
        let mut grid = DeviceGrid::new(7, 5);
        grid.set(0, 0, Device::Emitter(MixedColor::Red), Direction::South);
        grid.set(3, 0, Device::Detector(MixedColor::Red), Direction::East);
        grid.set(6, 2, Device::Detector(MixedColor::Green), Direction::West);
        grid.set(3, 3, Device::Wall, Direction::East);
        grid.set(0, 4, Device::Emitter(MixedColor::Green), Direction::North);
        grid.set(3, 4, Device::Detector(MixedColor::Blue), Direction::East);
        grid
    }

    fn ugrent_initial_grid() -> DeviceGrid {
        let mut grid = SyzygyState::ugrent_base_grid();
        grid.set(2, 0, Device::Mirror, Direction::East);
        grid.set(2, 1, Device::Mirror, Direction::South);
        grid.set(2, 2, Device::Mirror, Direction::East);
        grid.set(2, 3, Device::Mirror, Direction::South);
        grid.set(2, 4, Device::Mirror, Direction::East);
        grid.set(3, 1, Device::Mixer, Direction::East);
        grid.set(3, 2, Device::Mixer, Direction::East);
        grid.set(4, 1, Device::Splitter, Direction::East);
        grid.set(4, 2, Device::Splitter, Direction::East);
        grid.set(4, 3, Device::Splitter, Direction::East);
        grid.set(5, 0, Device::Mirror, Direction::South);
        grid.set(5, 1, Device::Mirror, Direction::East);
        grid.set(5, 2, Device::Mirror, Direction::South);
        grid.set(5, 3, Device::Mirror, Direction::East);
        grid.set(5, 4, Device::Mirror, Direction::South);
        grid
    }

    fn ugrent_solved_grid() -> DeviceGrid {
        let mut grid = SyzygyState::ugrent_base_grid();
        grid.set(1, 0, Device::Mirror, Direction::East);
        grid.set(2, 0, Device::Mirror, Direction::South);
        grid.set(5, 0, Device::Mirror, Direction::South);
        grid.set(0, 1, Device::Mirror, Direction::South);
        grid.set(1, 1, Device::Splitter, Direction::East);
        grid.set(2, 1, Device::Mirror, Direction::South);
        grid.set(5, 1, Device::Splitter, Direction::East);
        grid.set(1, 2, Device::Mixer, Direction::East);
        grid.set(4, 2, Device::Mirror, Direction::South);
        grid.set(5, 2, Device::Mixer, Direction::East);
        grid.set(0, 3, Device::Mirror, Direction::East);
        grid.set(1, 3, Device::Mirror, Direction::East);
        grid.set(4, 3, Device::Mirror, Direction::South);
        grid.set(5, 3, Device::Splitter, Direction::East);
        grid.set(5, 4, Device::Mirror, Direction::East);
        grid
    }

    fn mezure_initial_ice_grid() -> ObjectGrid {
        let mut grid = ObjectGrid::new(3, 6);
        grid.add_object(1, 2, Object::Wall);
        grid.add_object(1, 1, Object::Rotator);
        grid.add_ice_block(1, 0, Symbol::Mirror(true));
        grid.add_ice_block(0, 2, Symbol::Mirror(true));
        grid.add_ice_block(2, 2, Symbol::Mirror(true));
        grid.add_ice_block(1, 3, Symbol::Mirror(true));
        grid
    }

    fn mezure_initial_laser_grid() -> DeviceGrid {
        let mut grid = DeviceGrid::new(4, 7);
        for &(_, (col, row), dir) in MEZURE_GRAY_NODES_EMITTERS {
            let device = Device::Emitter(MixedColor::Black);
            grid.set(col, row, device, dir);
        }
        grid.set(1, 2, Device::Wall, Direction::East);
        grid.set(3, 0, Device::Detector(MixedColor::Cyan), Direction::West);
        grid.set(3, 1, Device::Detector(MixedColor::Magenta), Direction::West);
        grid.set(3, 2, Device::Detector(MixedColor::Yellow), Direction::West);
        grid.set(3, 3, Device::Detector(MixedColor::White), Direction::West);
        grid.set(3, 4, Device::Detector(MixedColor::Cyan), Direction::West);
        grid.set(3, 5, Device::Detector(MixedColor::Yellow), Direction::West);
        grid.set(3, 6, Device::Wall, Direction::East);
        grid
    }

    fn mezure_initial_pipe_grid() -> PlaneGrid {
        let mut grid = PlaneGrid::new(4, 9);
        grid.place_object(0, 0, PlaneObj::Wall);
        grid.place_object(1, 1, PlaneObj::RedNode);
        grid.place_object(1, 4, PlaneObj::GreenNode);
        grid.place_object(1, 7, PlaneObj::BlueNode);
        for &((col, row), _, _) in MEZURE_GRAY_NODES_EMITTERS {
            grid.place_object(col, row, PlaneObj::GrayNode);
        }
        grid
    }

    pub fn solve_stage(&mut self) {
        match self.stage {
            SyzygyStage::Yttris => {
                self.yttris.solve();
                self.advance_stage();
            }
            SyzygyStage::Argony => {
                self.argony = SyzygyState::argony_solved_grid();
                self.advance_stage();
            }
            SyzygyStage::Elinsa => {
                self.elinsa.remove_all_pipes();
                for pipe in ELINSA_SOLVED_PIPES {
                    let mut p1 = Point::new(pipe[0].0, pipe[0].1);
                    for i in 1..pipe.len() {
                        let p2 = Point::new(pipe[i].0, pipe[i].1);
                        self.elinsa.toggle_pipe(p1, p2);
                        p1 = p2;
                    }
                }
                debug_assert!(self.elinsa.all_nodes_are_connected());
                self.advance_stage();
            }
            SyzygyStage::Ugrent => {
                self.ugrent = SyzygyState::ugrent_solved_grid();
                self.advance_stage();
            }
            SyzygyStage::Relyng => {
                let num_lights = RELYNG_NUM_COLS * RELYNG_NUM_ROWS;
                self.relyng_lights = (0..num_lights).collect();
                self.advance_stage();
            }
            SyzygyStage::Mezure => {
                self.mezure_columns.solve();
                self.mezure_pipe_grid.remove_all_pipes();
                self.mezure_regenerate_laser_grid();
                self.access = Access::Solved;
            }
        }
    }

    pub fn stage(&self) -> SyzygyStage {
        self.stage
    }

    pub fn advance_stage(&mut self) {
        self.stage = self.stage.next();
    }

    pub fn yttris_columns(&self) -> &Columns {
        &self.yttris
    }

    pub fn yttris_columns_mut(&mut self) -> &mut Columns {
        &mut self.yttris
    }

    fn reset_yttris(&mut self) {
        self.yttris.reset()
    }

    pub fn argony_grid(&self) -> &ObjectGrid {
        &self.argony
    }

    pub fn argony_grid_mut(&mut self) -> &mut ObjectGrid {
        &mut self.argony
    }

    fn reset_argony(&mut self) {
        self.argony = SyzygyState::argony_initial_grid();
    }

    pub fn elinsa_grid(&self) -> &PlaneGrid {
        &self.elinsa
    }

    pub fn elinsa_grid_mut(&mut self) -> &mut PlaneGrid {
        &mut self.elinsa
    }

    fn reset_elinsa(&mut self) {
        self.elinsa.remove_all_pipes()
    }

    pub fn ugrent_grid(&self) -> &DeviceGrid {
        &self.ugrent
    }

    pub fn ugrent_grid_mut(&mut self) -> &mut DeviceGrid {
        &mut self.ugrent
    }

    fn reset_ugrent(&mut self) {
        self.ugrent = SyzygyState::ugrent_initial_grid();
    }

    pub fn relyng_is_lit(&self, (col, row): (i32, i32)) -> bool {
        debug_assert!(col >= 0 && col < RELYNG_NUM_COLS);
        debug_assert!(row >= 0 && row < RELYNG_NUM_ROWS);
        !self.relyng_lights.contains(&(row * RELYNG_NUM_COLS + col))
    }

    pub fn relyng_is_done(&self) -> bool {
        self.relyng_lights.len()
            == (RELYNG_NUM_COLS * RELYNG_NUM_ROWS) as usize
    }

    pub fn relyng_next_shape(&self) -> char {
        self.relyng_next
    }

    pub fn relyng_toggle(&mut self, (col, row): (i32, i32)) {
        self.relyng_toggle_shape(col, row);
        self.relyng_next = match self.relyng_next {
            '+' => 'N',
            'N' => 'X',
            'X' => 'Z',
            'Z' => '+',
            _ => unreachable!(),
        };
    }

    pub fn relyng_untoggle(&mut self, (col, row): (i32, i32)) {
        self.relyng_next = match self.relyng_next {
            '+' => 'Z',
            'N' => '+',
            'X' => 'N',
            'Z' => 'X',
            _ => unreachable!(),
        };
        self.relyng_toggle_shape(col, row);
    }

    fn relyng_toggle_shape(&mut self, col: i32, row: i32) {
        match self.relyng_next {
            '+' => {
                self.relyng_toggle_light(col, row);
                self.relyng_toggle_light(col + 1, row);
                self.relyng_toggle_light(col, row + 1);
                self.relyng_toggle_light(col - 1, row);
                self.relyng_toggle_light(col, row - 1);
            }
            'N' => {
                self.relyng_toggle_light(col, row);
                self.relyng_toggle_light(col - 1, row);
                self.relyng_toggle_light(col - 1, row + 1);
                self.relyng_toggle_light(col + 1, row);
                self.relyng_toggle_light(col + 1, row - 1);
            }
            'X' => {
                self.relyng_toggle_light(col, row);
                self.relyng_toggle_light(col - 1, row - 1);
                self.relyng_toggle_light(col + 1, row - 1);
                self.relyng_toggle_light(col - 1, row + 1);
                self.relyng_toggle_light(col + 1, row + 1);
            }
            'Z' => {
                self.relyng_toggle_light(col, row);
                self.relyng_toggle_light(col, row - 1);
                self.relyng_toggle_light(col - 1, row - 1);
                self.relyng_toggle_light(col, row + 1);
                self.relyng_toggle_light(col + 1, row + 1);
            }
            _ => unreachable!(),
        }
    }

    fn relyng_toggle_light(&mut self, col: i32, row: i32) {
        if (col >= 0 && col < RELYNG_NUM_COLS)
            && (row >= 0 && row < RELYNG_NUM_ROWS)
        {
            let index = row * RELYNG_NUM_COLS + col;
            if !self.relyng_lights.remove(&index) {
                self.relyng_lights.insert(index);
            }
        }
    }

    fn reset_relyng(&mut self) {
        self.relyng_lights.clear();
        self.relyng_next = RELYNG_INIT_NEXT;
    }

    pub fn mezure_lights(&self) -> &[bool; 6] {
        &self.mezure_lights
    }

    pub fn mezure_satisfied(&self) -> &[bool; 6] {
        &self.mezure_satisfied
    }

    pub fn mezure_columns(&self) -> &Columns {
        &self.mezure_columns
    }

    pub fn mezure_columns_mut(&mut self) -> &mut Columns {
        &mut self.mezure_columns
    }

    pub fn mezure_rotate_column(&mut self, col: usize, by: i32) {
        self.mezure_columns.rotate_column(col, by);
        if self.mezure_columns.is_solved() {
            self.access = Access::Solved;
        }
    }

    pub fn mezure_ice_grid(&self) -> &ObjectGrid {
        &self.mezure_ice_grid
    }

    pub fn mezure_ice_grid_mut(&mut self) -> &mut ObjectGrid {
        &mut self.mezure_ice_grid
    }

    pub fn mezure_laser_grid(&self) -> &DeviceGrid {
        &self.mezure_laser_grid
    }

    pub fn mezure_laser_grid_mut(&mut self) -> &mut DeviceGrid {
        &mut self.mezure_laser_grid
    }

    pub fn mezure_pipe_grid(&self) -> &PlaneGrid {
        &self.mezure_pipe_grid
    }

    pub fn mezure_pipe_grid_mut(&mut self) -> &mut PlaneGrid {
        &mut self.mezure_pipe_grid
    }

    fn reset_mezure(&mut self) {
        self.mezure_columns.reset();
        self.mezure_ice_grid = SyzygyState::mezure_initial_ice_grid();
        self.mezure_pipe_grid.remove_all_pipes();
        self.mezure_regenerate_laser_grid();
    }

    pub fn mezure_regenerate_laser_grid(&mut self) {
        self.mezure_laser_grid.clear_all_movable_objects();
        for (&coords, &symbol) in self.mezure_ice_grid.ice_blocks() {
            match symbol {
                Symbol::Mirror(mirrored) => {
                    let direction = if mirrored {
                        Direction::South
                    } else {
                        Direction::East
                    };
                    self.mezure_laser_grid.set(
                        coords.x(),
                        coords.y(),
                        Device::Mirror,
                        direction,
                    );
                }
                _ => {}
            }
        }
        let gray_nodes = self.mezure_pipe_grid.gray_node_colors();
        for &((gx, gy), (ex, ey), dir) in MEZURE_GRAY_NODES_EMITTERS {
            let color = *gray_nodes.get(&Point::new(gx, gy)).unwrap();
            self.mezure_laser_grid.set(ex, ey, Device::Emitter(color), dir);
        }
    }

    pub fn set_mezure_satisfied_detectors(
        &mut self,
        positions: HashSet<(i32, i32)>,
    ) {
        self.mezure_satisfied = [false; 6];
        for row in 0..6 {
            self.mezure_satisfied[row as usize] =
                positions.contains(&(3, row));
        }
        for index in 0..6 {
            self.mezure_lights[index] = !(self.mezure_satisfied[index]
                ^ self.mezure_satisfied[(index + 1) % 6]
                ^ self.mezure_satisfied[(index + 5) % 6]);
        }
        for index in 0..6 {
            let linkages = if self.mezure_lights[index] {
                let mut linkages = MEZURE_COLUMNS_SPEC[index].3.to_vec();
                linkages.retain(|&(col, _)| self.mezure_lights[col]);
                linkages
            } else {
                Vec::new()
            };
            self.mezure_columns.set_linkages(index, linkages);
        }
        if self.is_solved() {
            self.mezure_satisfied = [false; 6];
        }
    }
}

impl PuzzleState for SyzygyState {
    fn location() -> Location {
        Location::SystemSyzygy
    }

    fn access(&self) -> Access {
        self.access
    }

    fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    fn can_reset(&self) -> bool {
        match self.stage {
            SyzygyStage::Yttris => self.yttris.can_reset(),
            SyzygyStage::Argony => self.argony.is_modified(),
            SyzygyStage::Elinsa => !self.elinsa.pipes().is_empty(),
            SyzygyStage::Ugrent => self.ugrent.is_modified(),
            SyzygyStage::Relyng => !self.relyng_lights.is_empty(),
            SyzygyStage::Mezure => {
                self.mezure_columns.can_reset()
                    || self.mezure_ice_grid.is_modified()
                    || !self.mezure_pipe_grid.pipes().is_empty()
            }
        }
    }

    fn reset(&mut self) {
        match self.stage {
            SyzygyStage::Yttris => self.reset_yttris(),
            SyzygyStage::Argony => self.reset_argony(),
            SyzygyStage::Elinsa => self.reset_elinsa(),
            SyzygyStage::Ugrent => self.reset_ugrent(),
            SyzygyStage::Relyng => self.reset_relyng(),
            SyzygyStage::Mezure => self.reset_mezure(),
        }
    }

    fn replay(&mut self) {
        self.stage = SyzygyStage::first();
        self.reset_yttris();
        self.reset_argony();
        self.reset_elinsa();
        self.reset_ugrent();
        self.reset_relyng();
        self.reset_mezure();
        self.access = Access::BeginReplay;
    }
}

impl Tomlable for SyzygyState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(STAGE_KEY.to_string(), self.stage.to_toml());
            match self.stage {
                SyzygyStage::Yttris => {
                    if self.yttris.can_reset() {
                        table.insert(
                            YTTRIS_KEY.to_string(),
                            self.yttris.to_toml(),
                        );
                    }
                }
                SyzygyStage::Argony => {
                    if self.argony.is_modified() {
                        table.insert(
                            ARGONY_KEY.to_string(),
                            self.argony.to_toml(),
                        );
                    }
                }
                SyzygyStage::Elinsa => {
                    table.insert(
                        ELINSA_KEY.to_string(),
                        self.elinsa.pipes_to_toml(),
                    );
                }
                SyzygyStage::Ugrent => {
                    if self.ugrent.is_modified() {
                        table.insert(
                            UGRENT_KEY.to_string(),
                            self.ugrent.to_toml(),
                        );
                    }
                }
                SyzygyStage::Relyng => {
                    let lights = self
                        .relyng_lights
                        .iter()
                        .map(|&idx| toml::Value::Integer(idx as i64))
                        .collect();
                    table.insert(
                        RELYNG_LIGHTS_KEY.to_string(),
                        toml::Value::Array(lights),
                    );
                    let mut next = String::new();
                    next.push(self.relyng_next);
                    table.insert(
                        RELYNG_NEXT_KEY.to_string(),
                        toml::Value::String(next),
                    );
                }
                SyzygyStage::Mezure => {
                    if self.mezure_columns.can_reset() {
                        table.insert(
                            MEZURE_COLUMNS_KEY.to_string(),
                            self.mezure_columns.to_toml(),
                        );
                    }
                    if self.mezure_ice_grid.is_modified() {
                        table.insert(
                            MEZURE_ICE_GRID_KEY.to_string(),
                            self.mezure_ice_grid.to_toml(),
                        );
                    }
                    if !self.mezure_pipe_grid.pipes().is_empty() {
                        table.insert(
                            MEZURE_PIPES_KEY.to_string(),
                            self.mezure_pipe_grid.pipes_to_toml(),
                        );
                    }
                }
            }
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> SyzygyState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let stage = SyzygyStage::pop_from_table(&mut table, STAGE_KEY);
        let yttris = Columns::from_toml(
            YTTRIS_COLUMNS_SPEC,
            pop_array(&mut table, YTTRIS_KEY),
        );
        let argony = ObjectGrid::from_toml(
            pop_table(&mut table, ARGONY_KEY),
            &SyzygyState::argony_initial_grid(),
        );
        let mut elinsa = SyzygyState::elinsa_initial_grid();
        elinsa.set_pipes_from_toml(pop_array(&mut table, ELINSA_KEY));
        let ugrent = DeviceGrid::from_toml(
            pop_array(&mut table, UGRENT_KEY),
            &SyzygyState::ugrent_initial_grid(),
        );
        let relyng_next = match table
            .get(RELYNG_NEXT_KEY)
            .and_then(toml::Value::as_str)
            .unwrap_or("")
        {
            "+" => '+',
            "N" => 'N',
            "X" => 'X',
            "Z" => 'Z',
            _ => RELYNG_INIT_NEXT,
        };
        let relyng_lights = pop_array(&mut table, RELYNG_LIGHTS_KEY)
            .into_iter()
            .map(i32::from_toml)
            .filter(|&idx| 0 <= idx && idx < RELYNG_NUM_COLS * RELYNG_NUM_ROWS)
            .collect();
        let mezure_columns = Columns::from_toml(
            MEZURE_COLUMNS_SPEC,
            pop_array(&mut table, MEZURE_COLUMNS_KEY),
        );
        let mezure_ice_grid = ObjectGrid::from_toml(
            pop_table(&mut table, MEZURE_ICE_GRID_KEY),
            &SyzygyState::mezure_initial_ice_grid(),
        );
        let mut mezure_pipe_grid = SyzygyState::mezure_initial_pipe_grid();
        mezure_pipe_grid
            .set_pipes_from_toml(pop_array(&mut table, MEZURE_PIPES_KEY));
        let mut state = SyzygyState {
            access,
            stage,
            yttris,
            argony,
            elinsa,
            ugrent,
            relyng_next,
            relyng_lights,
            mezure_columns,
            mezure_lights: [true; 6],
            mezure_satisfied: [false; 6],
            mezure_ice_grid,
            mezure_laser_grid: SyzygyState::mezure_initial_laser_grid(),
            mezure_pipe_grid,
        };
        state.mezure_regenerate_laser_grid();
        state
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::SyzygyStage;
    use crate::save::util::Tomlable;

    const ALL_STAGES: &[SyzygyStage] = &[
        SyzygyStage::Yttris,
        SyzygyStage::Argony,
        SyzygyStage::Elinsa,
        SyzygyStage::Ugrent,
        SyzygyStage::Relyng,
        SyzygyStage::Mezure,
    ];

    #[test]
    fn stage_toml_round_trip() {
        for &original in ALL_STAGES {
            let result = SyzygyStage::from_toml(original.to_toml());
            assert_eq!(result, original);
        }
    }
}

// ========================================================================= //
