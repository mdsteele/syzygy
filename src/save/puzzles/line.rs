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

use rand::{self, Rng, SeedableRng};
use toml;

use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array, to_i32, to_u32};

// ========================================================================= //

const SEED_KEY: &'static str = "seed";
const STAGE_KEY: &'static str = "stage";

#[cfg_attr(rustfmt, rustfmt_skip)]
const GRIDS: &'static [(i32, char, char, &'static str)] = &[
    (4, 'K', 'N', "CDGHMOTWXYZ"), // 4x3 = 12
    (5, '9', '6', "BEFILORSZ12378"), // 5x3 = 15
    (4, 'A', 'V', "CDGJKPQRUWY4560"), // 4x4 = 16
    (6, 'I', 'T', "AEFHKLMNVWXZ12679"), // 6x3 = 18
    (5, 'M', 'W', "BCDGJOPQRSU23456890"), // 5x4 = 20
    (6, '5', 'S', "ABCEFGIMOQRTVWZ01234678"), // 6x4 = 24
    (5, 'F', 'E', "BCDGHIKLMPRSWXYZ23456890"), // 5x5 = 25
    (6, '`', '\'', "!@#$%^&*()-_=+[]{}\\|;:\"<>~./?"), // 6x5 = 30
    (6, 'O', '0', "ABCDEFGHIJKNPQSUVWYZ1235689#$(};,-~"), // 6x6 = 36
    (7, 'P', 'R', "ABCDEFGHIJLMNOQSTUVWXYZ0123456789{)[]:;<`"), // 7x6 = 42
];

// ========================================================================= //

#[derive(Default)]
pub struct LineState {
    access: Access,
    stage: i32,
    seed: [u32; 8],
    grid1: Vec<char>,
    grid2: Vec<char>,
}

impl LineState {
    pub fn from_toml(mut table: toml::Table) -> LineState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let stage = if access == Access::Solved {
            GRIDS.len() as i32
        } else {
            table.remove(STAGE_KEY).map(to_i32).unwrap_or(0)
        };
        let mut seed = [0; 8];
        if access != Access::Solved {
            let mut index = 0;
            for value in pop_array(&mut table, SEED_KEY)
                             .into_iter()
                             .map(to_u32) {
                seed[index] = value;
                index += 1;
                if index >= 8 {
                    break;
                }
            }
            while index < 8 {
                seed[index] = rand::random();
                index += 1;
            }
        }
        let mut state = LineState {
            access: access,
            stage: stage,
            seed: seed,
            grid1: Vec::new(),
            grid2: Vec::new(),
        };
        state.update_grids();
        state
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.stage = GRIDS.len() as i32;
        self.update_grids();
    }

    pub fn current_stage(&self) -> i32 { self.stage }

    pub fn stage_letters(&self, stage: i32) -> (char, char) {
        assert!(stage >= 0);
        assert!(stage < GRIDS.len() as i32);
        let entry = &GRIDS[stage as usize];
        (entry.1, entry.2)
    }

    pub fn grid1(&self) -> &[char] { &self.grid1 }

    pub fn grid2(&self) -> &[char] { &self.grid2 }

    pub fn num_cols(&self) -> i32 {
        debug_assert!(self.stage >= 0);
        if self.stage < GRIDS.len() as i32 {
            GRIDS[self.stage as usize].0
        } else {
            1
        }
    }

    pub fn pick_chars(&mut self, index1: usize, index2: usize) -> bool {
        debug_assert!(self.stage >= 0);
        assert!(self.stage < GRIDS.len() as i32);
        assert!(index1 < self.grid1.len());
        assert!(index2 < self.grid2.len());
        let entry = &GRIDS[self.stage as usize];
        if self.grid1[index1] == entry.1 && self.grid2[index2] == entry.2 {
            self.stage += 1;
            self.reseed();
            if self.stage >= GRIDS.len() as i32 {
                self.access = Access::Solved;
            }
            true
        } else {
            self.reseed();
            false
        }
    }

    pub fn reseed(&mut self) {
        for value in self.seed.iter_mut() {
            *value = rand::random();
        }
        self.update_grids();
    }

    fn update_grids(&mut self) {
        debug_assert!(self.stage >= 0);
        if self.stage >= GRIDS.len() as i32 {
            self.grid1.clear();
            self.grid2.clear();
            return;
        }
        let entry = &GRIDS[self.stage as usize];
        self.grid1 = entry.3.chars().collect();
        self.grid2 = self.grid1.clone();
        self.grid1.push(entry.1);
        self.grid2.push(entry.2);
        let mut rng = rand::chacha::ChaChaRng::from_seed(&self.seed);
        rng.shuffle(&mut self.grid1);
        rng.shuffle(&mut self.grid2);
    }
}

impl PuzzleState for LineState {
    fn location(&self) -> Location { Location::CrossTheLine }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { false }

    fn reset(&mut self) {
        self.stage = 0;
        self.reseed();
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(STAGE_KEY.to_string(),
                         toml::Value::Integer(self.stage as i64));
            table.insert(SEED_KEY.to_string(),
                         toml::Value::Array(self.seed
                                                .iter()
                                                .map(|&value| value as i64)
                                                .map(toml::Value::Integer)
                                                .collect()));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //
