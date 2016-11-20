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

use toml;

use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array, to_string};

// ========================================================================= //

const WORDS_KEY: &'static str = "words";

const SOLVED_WORDS: &'static [&'static str] = &["IN4MAL", "FEMI9", "CAR2N",
                                                "1DERFUL", "4EN6", "PER48",
                                                "42ITOUS", "PHY6", "QUI9",
                                                "PUNC28"];

// ========================================================================= //

pub struct LogLevelState {
    access: Access,
    words: Vec<Vec<char>>,
}

impl LogLevelState {
    pub fn from_toml(mut table: toml::Table) -> LogLevelState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let words = if access == Access::Solved {
            LogLevelState::solved_words()
        } else {
            let mut words: Vec<Vec<char>> = pop_array(&mut table, WORDS_KEY)
                                                .into_iter()
                                                .map(to_string)
                                                .map(|word| {
                                                    word.chars().collect()
                                                })
                                                .collect();
            words.resize(SOLVED_WORDS.len(), Vec::new());
            for (row, word) in words.iter_mut().enumerate() {
                word.resize(SOLVED_WORDS[row].len(), ' ');
                for chr in word.iter_mut() {
                    if !LogLevelState::is_valid_char(*chr) {
                        *chr = ' ';
                    }
                }
            }
            words
        };
        LogLevelState {
            access: access,
            words: words,
        }
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            let words = self.words
                            .iter()
                            .map(|chars| chars.iter().cloned().collect())
                            .map(toml::Value::String)
                            .collect();
            table.insert(WORDS_KEY.to_string(), toml::Value::Array(words));
        }
        toml::Value::Table(table)
    }

    pub fn visit(&mut self) { self.access.visit(); }

    pub fn reset(&mut self) { self.words = LogLevelState::initial_words(); }

    pub fn replay(&mut self) {
        self.access = Access::Replay;
        self.reset();
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.words = LogLevelState::solved_words();
    }

    pub fn words(&self) -> &Vec<Vec<char>> { &self.words }

    pub fn get_char(&self, row: i32, index: i32) -> char {
        assert!(row >= 0 && row < self.words.len() as i32);
        let word = &self.words[row as usize];
        assert!(index >= 0 && index < word.len() as i32);
        word[index as usize]
    }

    pub fn set_char(&mut self, row: i32, index: i32, chr: char) {
        {
            assert!(row >= 0 && row < self.words.len() as i32);
            let word = &mut self.words[row as usize];
            assert!(index >= 0 && index < word.len() as i32);
            word[index as usize] = chr;
        }
        if self.words == LogLevelState::solved_words() {
            self.access = Access::Solved;
        }
    }

    pub fn is_valid_char(chr: char) -> bool {
        match chr {
            'A'...'Z' | '0'...'9' | ' ' => true,
            _ => false,
        }
    }

    fn initial_words() -> Vec<Vec<char>> {
        SOLVED_WORDS.iter()
                    .map(|word| word.chars().map(|_| ' ').collect())
                    .collect()
    }

    fn solved_words() -> Vec<Vec<char>> {
        SOLVED_WORDS.iter().map(|word| word.chars().collect()).collect()
    }
}

impl Default for LogLevelState {
    fn default() -> LogLevelState {
        LogLevelState {
            access: Default::default(),
            words: LogLevelState::initial_words(),
        }
    }
}

impl PuzzleState for LogLevelState {
    fn location(&self) -> Location { Location::LogLevel }

    fn access(&self) -> Access { self.access }

    fn can_reset(&self) -> bool { false } // TODO
}

// ========================================================================= //
