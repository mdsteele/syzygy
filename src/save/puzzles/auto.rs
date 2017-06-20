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

use save::{Access, Location};
use save::util::{ACCESS_KEY, pop_array};
use super::PuzzleState;

// ========================================================================= //

const SEQUENCE_KEY: &str = "sequence";

const INITIAL_LETTERS: &[char] = &['E', 'D', 'U', 'C', 'A', 'T', 'I', 'O',
                                   'N'];
const SOLVED_LETTERS: &[char] = &['S', 'E', 'C', 'U', 'R', 'I', 'T', 'Y'];
const SOLVED_SEQUENCE: &[i8] = &[3, 0, 1, 4, 2];

// ========================================================================= //

pub struct AutoState {
    access: Access,
    sequence: Vec<i8>,
    letters: Vec<char>,
}

impl AutoState {
    pub fn from_toml(mut table: toml::value::Table) -> AutoState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let sequence = if access.is_solved() {
            SOLVED_SEQUENCE.iter().cloned().collect()
        } else {
            let seq: Vec<i8> = pop_array(&mut table, SEQUENCE_KEY)
                .iter()
                .filter_map(toml::Value::as_integer)
                .filter(|&idx| 0 <= idx && idx < 5)
                .map(|idx| idx as i8)
                .collect();
            let unique: HashSet<i8> = seq.iter().cloned().collect();
            if unique.len() != seq.len() {
                Vec::new()
            } else {
                seq
            }
        };
        let mut state = AutoState {
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

impl PuzzleState for AutoState {
    fn location(&self) -> Location { Location::AutofacTour }

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
            let len = letters.len();
            letters[len - 2] = increment_letter(letters[len - 2], 7);
            letters[len - 1] = increment_letter(letters[len - 1], 12);
        }
        1 => {
            for letter in letters.iter_mut() {
                *letter = match *letter {
                    'B' => 'Z',
                    'C' => 'B',
                    'D' => 'C',
                    'F' => 'D',
                    'G' => 'F',
                    'H' => 'G',
                    'J' => 'H',
                    'K' => 'J',
                    'L' => 'K',
                    'M' => 'L',
                    'N' => 'M',
                    'P' => 'N',
                    'Q' => 'P',
                    'R' => 'Q',
                    'S' => 'R',
                    'T' => 'S',
                    'V' => 'T',
                    'W' => 'V',
                    'X' => 'W',
                    'Y' => 'X',
                    'Z' => 'Y',
                    ltr => ltr,
                }
            }
        }
        2 => {
            letters.remove(4);
        }
        3 => {
            let mut sorted: Vec<(char, usize)> =
                letters.iter().enumerate().map(|(i, &c)| (c, i)).collect();
            sorted.sort();
            let (chr, idx) = sorted[sorted.len() - 2];
            letters.remove(idx);
            letters.insert(0, chr);
        }
        4 => {
            letters[5] = 'R';
        }
        _ => panic!("bad transformation index: {}", index),
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use save::{Access, PuzzleState};
    use save::util::{ACCESS_KEY, to_table};
    use super::{AutoState, INITIAL_LETTERS, SEQUENCE_KEY, SOLVED_LETTERS,
                SOLVED_SEQUENCE, apply_transformation};

    #[test]
    fn transform_letters() {
        let mut letters = "FACETIOUSLY".chars().collect::<Vec<char>>();
        apply_transformation(&mut letters, 1);
        assert_eq!(letters, "DABESIOURKX".chars().collect::<Vec<char>>());
        apply_transformation(&mut letters, 4);
        assert_eq!(letters, "DABESROURKX".chars().collect::<Vec<char>>());
        apply_transformation(&mut letters, 0);
        assert_eq!(letters, "DABESROURRJ".chars().collect::<Vec<char>>());
        apply_transformation(&mut letters, 3);
        assert_eq!(letters, "SDABEROURRJ".chars().collect::<Vec<char>>());

        let mut letters = "ABCDEFGHI".chars().collect::<Vec<char>>();
        apply_transformation(&mut letters, 2);
        assert_eq!(letters, "ABCDFGHI".chars().collect::<Vec<char>>());
    }

    #[test]
    fn toml_round_trip() {
        let mut state = AutoState::from_toml(toml::value::Table::new());
        state.access = Access::Replaying;
        state.append(3);
        state.append(1);
        state.append(4);
        let letters = state.letters.clone();

        let state = AutoState::from_toml(to_table(state.to_toml()));
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.sequence, vec![3, 1, 4]);
        assert_eq!(letters, state.letters);
    }

    #[test]
    fn from_empty_toml() {
        let state = AutoState::from_toml(toml::value::Table::new());
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.sequence, vec![]);
        assert_eq!(state.letters, INITIAL_LETTERS.to_vec());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = AutoState::from_toml(table);
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.sequence, SOLVED_SEQUENCE.to_vec());
        assert_eq!(state.letters, SOLVED_LETTERS.to_vec());
    }

    #[test]
    fn from_valid_sequence_toml() {
        let mut table = toml::value::Table::new();
        table.insert(SEQUENCE_KEY.to_string(),
                     toml::Value::Array(vec![toml::Value::Integer(1),
                                             toml::Value::Integer(2),
                                             toml::Value::Integer(3)]));
        let state = AutoState::from_toml(table);
        assert_eq!(state.sequence, vec![1, 2, 3]);
    }

    #[test]
    fn from_invalid_repeat_sequence_toml() {
        let mut table = toml::value::Table::new();
        table.insert(SEQUENCE_KEY.to_string(),
                     toml::Value::Array(vec![toml::Value::Integer(1); 2]));
        let state = AutoState::from_toml(table);
        assert_eq!(state.sequence, vec![]);
        assert_eq!(state.letters, INITIAL_LETTERS.to_vec());
    }

    #[test]
    fn from_invalid_index_sequence_toml() {
        let mut table = toml::value::Table::new();
        table.insert(SEQUENCE_KEY.to_string(),
                     toml::Value::Array(vec![toml::Value::Integer(5)]));
        let state = AutoState::from_toml(table);
        assert_eq!(state.sequence, vec![]);
        assert_eq!(state.letters, INITIAL_LETTERS.to_vec());
    }
}

// ========================================================================= //
