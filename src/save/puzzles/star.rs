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

use std::ascii::AsciiExt;
use std::collections::BTreeSet;
use toml;

use gui::Point;
use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array, to_i32};

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
    (23, "Polaris",   2, 4, WordDir::Horizontal),
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
    found: BTreeSet<i32>,
    columns: Vec<Vec<char>>,
}

impl StarState {
    pub fn from_toml(mut table: toml::value::Table) -> StarState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let found = if access == Access::Solved {
            (0..(WORDS.len() as i32)).into_iter().collect()
        } else {
            let num_words = WORDS.len() as i32;
            pop_array(&mut table, FOUND_KEY)
                .into_iter()
                .map(to_i32)
                .filter(|&idx| 0 <= idx && idx < num_words)
                .collect()
        };

        let mut state = StarState {
            access: access,
            found: found,
            columns: Vec::new(),
        };
        state.regenerate_columns(); // TODO: don't panic if invalid
        state
    }

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
               row >= self.column_letters(col).len() as i32 {
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
        let mut insertions: Vec<(i32, usize)> =
            WORDS.iter()
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
    fn location(&self) -> Location { Location::StarCrossed }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { false }

    fn reset(&mut self) {
        self.found.clear();
        self.regenerate_columns();
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && !self.found.is_empty() {
            let found = self.found
                            .iter()
                            .map(|&idx| toml::Value::Integer(idx as i64))
                            .collect();
            table.insert(FOUND_KEY.to_string(), toml::Value::Array(found));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::WORDS;

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
    fn word_order() {
        let mut prev = "";
        for entry in WORDS {
            assert!(entry.1 > prev);
            prev = entry.1;
        }
    }

    // TODO: test that you can't get stuck by finding words in wrong order
}

// ========================================================================= //
