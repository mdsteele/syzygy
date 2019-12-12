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

use super::util::Tomlable;

// ========================================================================= //

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ValidChars {
    Letters,
    LettersAndNumbers,
    LettersAndSymbols,
}

impl ValidChars {
    pub fn contains(self, chr: char) -> bool {
        match chr {
            'A'..='Z' | ' ' => true,
            '0'..='9' => self == ValidChars::LettersAndNumbers,
            '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => {
                self == ValidChars::LettersAndSymbols
            }
            _ => false,
        }
    }
}

// ========================================================================= //

pub struct CrosswordState {
    words: Vec<Vec<char>>,
    valid: ValidChars,
    is_initial: bool,
}

impl CrosswordState {
    pub fn new(valid: ValidChars, words: &[&str]) -> CrosswordState {
        let words = words.iter().map(|word| word.chars().collect()).collect();
        let is_initial = all_spaces(&words);
        CrosswordState {
            words: words,
            valid: valid,
            is_initial: is_initial,
        }
    }

    pub fn from_toml(array: toml::value::Array, valid: ValidChars,
                     solved: &[&str])
                     -> CrosswordState {
        let mut words: Vec<Vec<char>> =
            Vec::<String>::from_toml(toml::Value::Array(array))
                .into_iter()
                .map(|word| word.chars().collect())
                .collect();
        words.resize(solved.len(), Vec::new());
        for (row, word) in words.iter_mut().enumerate() {
            word.resize(solved[row].len(), ' ');
            for chr in word.iter_mut() {
                if !valid.contains(*chr) {
                    *chr = ' ';
                }
            }
        }
        let is_initial = all_spaces(&words);
        CrosswordState {
            words: words,
            valid: valid,
            is_initial: is_initial,
        }
    }

    pub fn to_toml(&self) -> toml::Value {
        let words = self.words
            .iter()
            .map(|chars| chars.iter().cloned().collect())
            .map(toml::Value::String)
            .collect();
        toml::Value::Array(words)
    }

    pub fn can_reset(&self) -> bool { !self.is_initial }

    pub fn reset(&mut self) {
        for word in self.words.iter_mut() {
            for chr in word.iter_mut() {
                *chr = ' ';
            }
        }
        self.is_initial = true;
    }

    pub fn valid_chars(&self) -> ValidChars { self.valid }

    pub fn words(&self) -> &Vec<Vec<char>> { &self.words }

    pub fn words_are(&self, words: &[&str]) -> bool {
        let words: Vec<Vec<char>> =
            words.iter().map(|word| word.chars().collect()).collect();
        self.words == words
    }

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
        self.is_initial = all_spaces(&self.words);
    }
}

// ========================================================================= //

fn all_spaces(words: &Vec<Vec<char>>) -> bool {
    words.iter().all(|word| word.iter().all(|chr| *chr == ' '))
}

// ========================================================================= //
