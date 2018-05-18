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

use gui::Point;
use save::{Access, Location};
use save::util::{ACCESS_KEY, Tomlable, to_table};
use super::PuzzleState;

// ========================================================================= //

const FOUND_KEY: &str = "found";

#[cfg_attr(rustfmt, rustfmt_skip)]
const WORDS: &[(i32, &str, i32, i32, WordDir)] = &[
    ( 7, "Altair",    4, 3, WordDir::Horizontal),
    (13, "Canis",     5, 4, WordDir::Horizontal),
    (21, "Cetus",     0, 8, WordDir::DiagDown),
    (11, "Corvus",    0, 6, WordDir::DiagDown),
    (20, "Cygnus",    3, 5, WordDir::Horizontal),
    (16, "Deneb",     2, 2, WordDir::Vertical),
    ( 0, "Elnath",    3, 0, WordDir::Horizontal),
    ( 4, "Fomalhaut", 1, 1, WordDir::Horizontal),
    (14, "Fornax",    1, 3, WordDir::Horizontal),
    ( 2, "Gemini",    0, 0, WordDir::Horizontal),
    ( 8, "Indus",     4, 1, WordDir::Vertical),
    ( 1, "Leo",       5, 2, WordDir::DiagDown),
    (18, "Libra",     5, 7, WordDir::DiagDown),
    ( 5, "Lyra",      9, 0, WordDir::Vertical),
    (10, "Mirzam",    0, 1, WordDir::Vertical),
    (12, "Norma",     5, 0, WordDir::DiagUp),
    (19, "Pavo",      8, 4, WordDir::Vertical),
    (23, "Polaris",   2, 5, WordDir::Horizontal),
    (17, "Procyon",   1, 5, WordDir::Horizontal),
    (22, "Regulus",   0, 7, WordDir::Horizontal),
    ( 6, "Rigel",     5, 5, WordDir::DiagDown),
    ( 9, "Spica",     0, 2, WordDir::Horizontal),
    ( 3, "Ursa",      0, 0, WordDir::DiagUp),
    (15, "Vega",      1, 2, WordDir::Horizontal),
];

const FINAL_WORD: &str = "SKEPTICISM";

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordDir {
    DiagUp,
    Horizontal,
    DiagDown,
    Vertical,
}

impl WordDir {
    pub fn delta(self) -> Point {
        match self {
            WordDir::DiagUp => Point::new(1, 1),
            WordDir::Horizontal => Point::new(1, 0),
            WordDir::DiagDown => Point::new(1, -1),
            WordDir::Vertical => Point::new(0, -1),
        }
    }
}

// ========================================================================= //

pub struct StarState {
    access: Access,
    found: HashSet<i32>,
    columns: Vec<Vec<char>>,
}

impl StarState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        for index in 0..self.num_words() {
            self.found.insert(index);
        }
        self.regenerate_columns();
    }

    pub fn num_words(&self) -> i32 { WORDS.len() as i32 }

    pub fn word(&self, index: i32) -> &str {
        debug_assert!(index >= 0 && index < WORDS.len() as i32);
        WORDS[index as usize].1
    }

    pub fn word_is_found(&self, index: i32) -> bool {
        self.found.contains(&index)
    }

    pub fn num_columns(&self) -> i32 { self.columns.len() as i32 }

    pub fn column_letters(&self, index: i32) -> &[char] {
        debug_assert!(index >= 0 && index < self.columns.len() as i32);
        &self.columns[index as usize]
    }

    pub fn try_remove_word(&mut self, col: i32, row: i32, dir: WordDir,
                           length: i32)
                           -> bool {
        let mut pt = Point::new(col, row);
        let mut word = String::new();
        for _ in 0..length {
            let col = pt.x();
            let row = pt.y();
            if col < 0 || row < 0 || col >= self.num_columns() ||
                row >= self.column_letters(col).len() as i32
            {
                return false;
            }
            word.push(self.columns[col as usize][row as usize]);
            pt = pt + dir.delta();
        }
        for (index, entry) in WORDS.iter().enumerate() {
            if word == entry.1.to_ascii_uppercase() {
                self.found.insert(index as i32);
                self.regenerate_columns();
                if self.found.len() == WORDS.len() {
                    self.access = Access::Solved;
                }
                return true;
            }
        }
        false
    }

    fn regenerate_columns(&mut self) {
        let mut insertions: Vec<(i32, usize)> = WORDS
            .iter()
            .enumerate()
            .map(|(index, &tuple)| (tuple.0, index))
            .filter(|&(_, index)| !self.found.contains(&(index as i32)))
            .collect();
        insertions.sort();
        self.columns = FINAL_WORD.chars().map(|ch| vec![ch]).collect();
        for (_, index) in insertions.into_iter() {
            let (_, word, col, row, dir) = WORDS[index];
            let mut pt = Point::new(col, row);
            for chr in word.chars() {
                let chr = chr.to_ascii_uppercase();
                assert!(pt.x() >= 0 && pt.y() >= 0);
                self.columns[pt.x() as usize].insert(pt.y() as usize, chr);
                if dir != WordDir::Vertical {
                    pt = pt + dir.delta();
                }
            }
        }
    }
}

impl PuzzleState for StarState {
    fn location() -> Location { Location::StarCrossed }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { false }

    fn reset(&mut self) {
        self.found.clear();
        self.regenerate_columns();
    }
}

impl Tomlable for StarState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && !self.found.is_empty() {
            table.insert(FOUND_KEY.to_string(), self.found.to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> StarState {
        let mut table = to_table(value);
        let mut access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let found: HashSet<i32> = if access == Access::Solved {
            (0..(WORDS.len() as i32)).into_iter().collect()
        } else {
            let num_words = WORDS.len() as i32;
            let found = Vec::<i32>::pop_from_table(&mut table, FOUND_KEY);
            found.into_iter().filter(|&i| 0 <= i && i < num_words).collect()
        };
        if found.len() == WORDS.len() {
            access = Access::Solved;
        }
        let mut state = StarState {
            access: access,
            found: found,
            columns: Vec::new(),
        };
        state.regenerate_columns(); // TODO: don't panic if invalid
        state
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};
    use toml;

    use save::{Access, PuzzleState};
    use save::util::{ACCESS_KEY, Tomlable};
    use super::{FINAL_WORD, FOUND_KEY, StarState, WORDS, WordDir};

    #[test]
    fn toml_round_trip() {
        let mut state = StarState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        assert!(state.try_remove_word(2, 5, WordDir::Horizontal, 7));
        assert_eq!(state.found.len(), 1);
        let found = state.found.clone();
        let columns = state.columns.clone();

        let state = StarState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.found, found);
        assert_eq!(state.columns, columns);
    }

    #[test]
    fn from_empty_toml() {
        let state = StarState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert!(state.found.is_empty());
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = StarState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.found.len(), WORDS.len());
        assert_eq!(state.columns,
                   FINAL_WORD
                       .chars()
                       .map(|chr| vec![chr])
                       .collect::<Vec<Vec<char>>>());
    }

    #[test]
    fn from_already_found_everything_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        let found: HashSet<i32> =
            (0..(WORDS.len() as i32)).into_iter().collect();
        table.insert(FOUND_KEY.to_string(), found.to_toml());

        let state = StarState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.found, found);
        assert_eq!(state.columns,
                   FINAL_WORD
                       .chars()
                       .map(|chr| vec![chr])
                       .collect::<Vec<Vec<char>>>());
    }

    #[test]
    fn word_insertion_indicies() {
        let mut indicies = BTreeSet::new();
        for entry in WORDS {
            indicies.insert(entry.0);
        }
        let indicies: Vec<i32> = indicies.into_iter().collect();
        let expected: Vec<i32> = (0..(WORDS.len() as i32)).collect();
        assert_eq!(indicies, expected);
    }

    #[test]
    fn words_are_ordered_alphabetically() {
        let mut prev = "";
        for entry in WORDS {
            assert!(entry.1 > prev);
            prev = entry.1;
        }
    }

    #[test]
    fn correct_removal_order_works() {
        let mut state = StarState::from_toml(toml::Value::Boolean(false));
        let mut words = WORDS.to_vec();
        words.sort_by_key(|entry| -entry.0);
        for (_, word, col, mut row, dir) in words {
            if dir == WordDir::Vertical {
                row += word.len() as i32 - 1;
            }
            assert!(!state.is_solved());
            assert!(state.try_remove_word(col, row, dir, word.len() as i32),
                    "Could not remove {} from ({}, {})",
                    word,
                    col,
                    row);
        }
        assert!(state.is_solved());
    }

    #[test]
    fn words_can_only_be_found_in_order() {
        let mut state = StarState::from_toml(toml::Value::Boolean(false));
        let mut words = WORDS.to_vec();
        words.sort_by_key(|entry| -entry.0);
        let num_columns = FINAL_WORD.len() as i32;
        let all_dirs = [
            WordDir::DiagUp,
            WordDir::Horizontal,
            WordDir::DiagDown,
            WordDir::Vertical,
        ];
        for i in 0..words.len() {
            let (_, correct_word, correct_col, mut correct_row, correct_dir) =
                words[i];
            let correct_length = correct_word.len() as i32;
            if correct_dir == WordDir::Vertical {
                correct_row += correct_length - 1;
            }
            let correct_tuple =
                (correct_col, correct_row, correct_dir, correct_length);
            // First, try removing every word except the correct one.
            for col in 0..num_columns {
                for row in 0..(state.columns[col as usize].len() as i32) {
                    for &dir in all_dirs.iter() {
                        for length in 3..10 {
                            if (col, row, dir, length) == correct_tuple {
                                continue;
                            }
                            assert!(!state.try_remove_word(col, row, dir,
                                                           length),
                                    "Removed ({}, {}, {:?}, {}) on step {}",
                                    col, row, dir, length, i);
                            assert!(!state.is_solved());
                        }
                    }
                }
            }
            // Now remove the correct word.
            assert!(state.try_remove_word(correct_col,
                                          correct_row,
                                          correct_dir,
                                          correct_length),
                    "Could not remove {} from ({}, {})",
                    correct_word,
                    correct_col,
                    correct_row);
        }
        assert!(state.is_solved());
    }
}

// ========================================================================= //
