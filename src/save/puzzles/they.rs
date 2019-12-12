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
use std::collections::HashSet;
use toml;

use super::PuzzleState;
use crate::save::util::{to_table, Tomlable, ACCESS_KEY};
use crate::save::{Access, Location};

// ========================================================================= //

const SEQUENCE_KEY: &str = "sequence";

const INITIAL_LETTERS: &[char] = &['M', 'A', 'X', 'I', 'M', 'I', 'Z', 'E'];
const SOLVED_LETTERS: &[char] = &['C', 'R', 'E', 'A', 'T', 'I', 'V', 'E'];
const SOLVED_SEQUENCE: &[i8] = &[1, 0, 4, 5, 3, 2];

// ========================================================================= //

pub struct TheYState {
    access: Access,
    sequence: Vec<i8>,
    letters: Vec<char>,
}

impl TheYState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.sequence = SOLVED_SEQUENCE.to_vec();
        self.regenerate_letters_from_sequence();
        debug_assert_eq!(&self.letters as &[char], SOLVED_LETTERS);
    }

    pub fn sequence(&self) -> &Vec<i8> {
        &self.sequence
    }

    pub fn set_sequence(&mut self, sequence: Vec<i8>) {
        self.sequence = sequence;
        self.regenerate_letters_from_sequence();
    }

    pub fn letters(&self) -> &Vec<char> {
        &self.letters
    }

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

impl PuzzleState for TheYState {
    fn location() -> Location {
        Location::TheYFactor
    }

    fn access(&self) -> Access {
        self.access
    }

    fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    fn can_reset(&self) -> bool {
        !self.sequence.is_empty()
    }

    fn reset(&mut self) {
        self.sequence.clear();
        self.regenerate_letters_from_sequence();
    }
}

impl Tomlable for TheYState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && !self.sequence.is_empty() {
            let seq = self
                .sequence
                .iter()
                .map(|&idx| toml::Value::Integer(idx as i64))
                .collect();
            table.insert(SEQUENCE_KEY.to_string(), toml::Value::Array(seq));
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> TheYState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let sequence = if access.is_solved() {
            SOLVED_SEQUENCE.iter().cloned().collect()
        } else {
            let mut seq = Vec::<i8>::pop_from_table(&mut table, SEQUENCE_KEY);
            seq.retain(|&idx| 0 <= idx && idx < 6);
            let unique: HashSet<i8> = seq.iter().cloned().collect();
            if unique.len() != seq.len() {
                Vec::new()
            } else {
                seq
            }
        };
        let mut state = TheYState { access, sequence, letters: Vec::new() };
        state.regenerate_letters_from_sequence();
        state
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
            for letter in letters.iter_mut() {
                *letter = match *letter {
                    'A' => 'U',
                    'E' => 'A',
                    'I' => 'E',
                    'O' => 'I',
                    'U' => 'O',
                    ltr => ltr,
                }
            }
        }
        1 => {
            letters[0] = increment_letter(letters[0], 7);
            letters[1] = increment_letter(letters[1], 11);
        }
        2 => {
            letters.swap(1, 2);
            letters.swap(5, 6);
        }
        3 => {
            letters[2] = 'R';
        }
        4 => {
            for letter in letters.iter_mut() {
                *letter = match *letter {
                    'M' => 'C',
                    'D' => 'L',
                    'C' => 'X',
                    'L' => 'V',
                    'X' => 'I',
                    ltr => ltr,
                }
            }
        }
        5 => {
            let first_half: Vec<char> = letters.drain(0..4).collect();
            letters.extend_from_slice(&first_half);
        }
        _ => panic!("bad transformation index: {}", index),
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use super::{
        apply_transformation, TheYState, INITIAL_LETTERS, SEQUENCE_KEY,
        SOLVED_LETTERS, SOLVED_SEQUENCE,
    };
    use crate::save::util::{Tomlable, ACCESS_KEY};
    use crate::save::Access;

    #[test]
    fn transform_letters() {
        let mut letters = "FACETIOUSLY".chars().collect::<Vec<char>>();
        apply_transformation(&mut letters, 0);
        assert_eq!(letters, "FUCATEIOSLY".chars().collect::<Vec<char>>());
        apply_transformation(&mut letters, 4);
        assert_eq!(letters, "FUXATEIOSVY".chars().collect::<Vec<char>>());
        apply_transformation(&mut letters, 3);
        assert_eq!(letters, "FURATEIOSVY".chars().collect::<Vec<char>>());
        apply_transformation(&mut letters, 1);
        assert_eq!(letters, "MFRATEIOSVY".chars().collect::<Vec<char>>());

        let mut letters = "ABCDEFGH".chars().collect::<Vec<char>>();
        apply_transformation(&mut letters, 2);
        assert_eq!(letters, "ACBDEGFH".chars().collect::<Vec<char>>());
        apply_transformation(&mut letters, 5);
        assert_eq!(letters, "EGFHACBD".chars().collect::<Vec<char>>());
    }

    #[test]
    fn toml_round_trip() {
        let mut state = TheYState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.append(3);
        state.append(1);
        state.append(4);
        let letters = state.letters.clone();

        let state = TheYState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.sequence, vec![3, 1, 4]);
        assert_eq!(letters, state.letters);
    }

    #[test]
    fn from_empty_toml() {
        let state = TheYState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.sequence, vec![]);
        assert_eq!(state.letters, INITIAL_LETTERS.to_vec());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = TheYState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.sequence, SOLVED_SEQUENCE.to_vec());
        assert_eq!(state.letters, SOLVED_LETTERS.to_vec());
    }

    #[test]
    fn from_valid_sequence_toml() {
        let mut table = toml::value::Table::new();
        table.insert(
            SEQUENCE_KEY.to_string(),
            toml::Value::Array(vec![
                toml::Value::Integer(1),
                toml::Value::Integer(2),
                toml::Value::Integer(3),
            ]),
        );
        let state = TheYState::from_toml(toml::Value::Table(table));
        assert_eq!(state.sequence, vec![1, 2, 3]);
    }

    #[test]
    fn from_invalid_repeat_sequence_toml() {
        let mut table = toml::value::Table::new();
        table.insert(
            SEQUENCE_KEY.to_string(),
            toml::Value::Array(vec![toml::Value::Integer(1); 2]),
        );
        let state = TheYState::from_toml(toml::Value::Table(table));
        assert_eq!(state.sequence, vec![]);
        assert_eq!(state.letters, INITIAL_LETTERS.to_vec());
    }

    #[test]
    fn from_invalid_index_sequence_toml() {
        let mut table = toml::value::Table::new();
        table.insert(
            SEQUENCE_KEY.to_string(),
            toml::Value::Array(vec![toml::Value::Integer(6)]),
        );
        let state = TheYState::from_toml(toml::Value::Table(table));
        assert_eq!(state.sequence, vec![]);
        assert_eq!(state.letters, INITIAL_LETTERS.to_vec());
    }
}

// ========================================================================= //
