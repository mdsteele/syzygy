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

use std::collections::VecDeque;
use toml;

use save::{Access, Location, PuzzleState};
use save::util::{ACCESS_KEY, pop_array, rotate_deque};

// ========================================================================= //

const TOKENS_KEY: &str = "tokens";

#[cfg_attr(rustfmt, rustfmt_skip)]
const INITIAL_TOKENS: &[u8] = &[
        0, 1, 2, 0,
       1,    2,    0,
     1, 2, 0, 1, 2, 0,
    1,    2,    0,    1,
     2, 0, 1, 2, 0, 1,
       2,    0,    1,
        2, 0, 1, 2,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const SOLVED_TOKENS: &[u8] = &[
        1, 1, 1, 1,
       0,    0,    0,
     2, 2, 2, 2, 2, 2,
    0,    0,    0,    0,
     1, 1, 1, 1, 1, 1,
       0,    0,    0,
        2, 2, 2, 2,
];

const WHEELS: &[[usize; 6]] = &[[0, 1, 5, 9, 8, 4],
                                [2, 3, 6, 11, 10, 5],
                                [7, 8, 14, 18, 17, 13],
                                [9, 10, 15, 20, 19, 14],
                                [11, 12, 16, 22, 21, 15],
                                [18, 19, 24, 27, 26, 23],
                                [20, 21, 25, 29, 28, 24]];

// ========================================================================= //

pub struct HexState {
    access: Access,
    tokens: Vec<u8>,
    is_initial: bool,
}

impl HexState {
    pub fn from_toml(mut table: toml::value::Table) -> HexState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let tokens = if access.is_solved() {
            SOLVED_TOKENS.to_vec()
        } else {
            let mut tokens: Vec<u8> = pop_array(&mut table, TOKENS_KEY)
                .iter()
                .filter_map(toml::Value::as_integer)
                .filter(|&token| 0 <= token && token < 3)
                .map(|token| token as u8)
                .collect();
            tokens.resize(INITIAL_TOKENS.len(), 0);
            let mut init_sorted = INITIAL_TOKENS.to_vec();
            init_sorted.sort();
            let mut tokens_sorted = tokens.clone();
            tokens_sorted.sort();
            if tokens_sorted != init_sorted {
                INITIAL_TOKENS.to_vec()
            } else {
                tokens
            }
        };
        let is_initial = (&tokens as &[u8]) == INITIAL_TOKENS;
        HexState {
            access: access,
            tokens: tokens,
            is_initial: is_initial,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.tokens = SOLVED_TOKENS.to_vec();
        self.is_initial = false;
    }

    pub fn tokens(&self) -> &Vec<u8> { &self.tokens }

    pub fn rotate_wheel_cw(&mut self, wheel: usize, by: i32) {
        debug_assert!(wheel < WHEELS.len());
        let wheel = &WHEELS[wheel];
        let mut tokens: VecDeque<u8> =
            wheel.iter().map(|&index| self.tokens[index]).collect();
        rotate_deque(&mut tokens, by);
        for (index, token) in tokens.into_iter().enumerate() {
            self.tokens[wheel[index]] = token;
        }
        self.is_initial = (&self.tokens as &[u8]) == INITIAL_TOKENS;
        if (&self.tokens as &[u8]) == SOLVED_TOKENS {
            self.access = Access::Solved;
        }
    }
}

impl PuzzleState for HexState {
    fn location(&self) -> Location { Location::HexSpangled }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.is_initial }

    fn reset(&mut self) {
        self.tokens = INITIAL_TOKENS.to_vec();
        self.is_initial = true;
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && !self.is_initial {
            let tokens = self.tokens
                             .iter()
                             .map(|&token| toml::Value::Integer(token as i64))
                             .collect();
            table.insert(TOKENS_KEY.to_string(), toml::Value::Array(tokens));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //
