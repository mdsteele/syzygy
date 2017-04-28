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

use std::char;
use toml;

use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array};

// ========================================================================= //

const SEQUENCE_KEY: &str = "sequence";

const INITIAL_LETTERS: &[char] = &['F', 'A', 'N', 'T', 'A', 'S', 'Y'];
const SOLVED_LETTERS: &[char] = &['H', 'I', 'S', 'T', 'O', 'R', 'Y'];
const SOLVED_SEQUENCE: &[i8] = &[3, 0, 2, 5, 4, 1];

// ========================================================================= //

pub struct FictionState {
    access: Access,
    sequence: Vec<i8>,
    letters: Vec<char>,
}

impl FictionState {
    pub fn from_toml(mut table: toml::value::Table) -> FictionState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let sequence = if access.is_solved() {
            SOLVED_SEQUENCE.iter().cloned().collect()
        } else {
            pop_array(&mut table, SEQUENCE_KEY)
                .iter()
                .filter_map(toml::Value::as_integer)
                .filter(|&idx| 0 <= idx && idx < 6)
                .map(|idx| idx as i8)
                .collect()
        };
        let mut state = FictionState {
            access: access,
            sequence: sequence,
            letters: Vec::new(),
        };
        state.regenerate_letters_from_sequence();
        state
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.sequence = SOLVED_SEQUENCE.to_vec();
        self.regenerate_letters_from_sequence();
        debug_assert_eq!(&self.letters as &[char], SOLVED_LETTERS);
    }

    pub fn sequence(&self) -> &Vec<i8> { &self.sequence }

    pub fn set_sequence(&mut self, sequence: Vec<i8>) {
        self.sequence = sequence;
        self.regenerate_letters_from_sequence();
    }

    pub fn letters(&self) -> &Vec<char> { &self.letters }

    pub fn has_used(&self, index: i8) -> bool {
        self.sequence.contains(&index)
    }

    pub fn append(&mut self, index: i8) {
        assert!(index >= 0 && index < 6);
        assert!(!self.has_used(index));
        self.sequence.push(index);
        apply_transformation(&mut self.letters, index);
        if &self.letters as &[char] == SOLVED_LETTERS {
            self.access = Access::Solved;
        }
    }

    fn regenerate_letters_from_sequence(&mut self) {
        self.letters = INITIAL_LETTERS.to_vec();
        for &index in &self.sequence {
            apply_transformation(&mut self.letters, index);
        }
    }
}

impl PuzzleState for FictionState {
    fn location(&self) -> Location { Location::FactOrFiction }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.sequence.is_empty() }

    fn reset(&mut self) {
        self.sequence.clear();
        self.regenerate_letters_from_sequence();
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && !self.sequence.is_empty() {
            let seq = self.sequence
                          .iter()
                          .map(|&idx| toml::Value::Integer(idx as i64))
                          .collect();
            table.insert(SEQUENCE_KEY.to_string(), toml::Value::Array(seq));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //

fn increment_letter(letter: char, by: u32) -> char {
    debug_assert!(letter >= 'A' && letter <= 'Z');
    char::from_u32((letter as u32 - 'A' as u32 + by) % 26 + 'A' as u32)
        .unwrap()
}

fn apply_transformation(letters: &mut Vec<char>, index: i8) {
    match index {
        0 => {
            letters[0..7].sort();
        }
        1 => {
            letters[0..3].sort();
            letters[4..7].sort();
        }
        2 => {
            letters[0] = increment_letter(letters[0], 7);
            letters[1] = increment_letter(letters[1], 11);
        }
        3 => {
            letters[2] = 'R';
        }
        4 => {
            letters[0] = increment_letter(letters[0], 3);
            letters[6] = increment_letter(letters[6], 3);
        }
        5 => {
            let old = letters.clone();
            letters[0] = old[2];
            letters[1] = old[0];
            letters[2] = old[4];
            letters[3] = old[5];
            letters[4] = old[6];
            letters[5] = old[3];
            letters[6] = old[1];
        }
        _ => panic!("bad transformation index: {}", index),
    }
}

// ========================================================================= //
