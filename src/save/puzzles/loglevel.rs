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

use save::{Access, CrosswordState, Location, ValidChars};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array};

// ========================================================================= //

const WORDS_KEY: &str = "words";

const SOLVED_WORDS: &[&str] = &["IN4MAL", "FEMI9", "CAR2N", "1DERFUL",
                                "4EN6", "PER48", "42ITOUS", "PHY6", "QUI9",
                                "PUNC28"];

const VALID_CHARS: ValidChars = ValidChars::LettersAndNumbers;

// ========================================================================= //

pub struct LogLevelState {
    access: Access,
    words: CrosswordState,
}

impl LogLevelState {
    pub fn from_toml(mut table: toml::value::Table) -> LogLevelState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let words = if access == Access::Solved {
            CrosswordState::new(VALID_CHARS, SOLVED_WORDS)
        } else {
            CrosswordState::from_toml(pop_array(&mut table, WORDS_KEY),
                                      VALID_CHARS,
                                      SOLVED_WORDS)
        };
        LogLevelState {
            access: access,
            words: words,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.words = CrosswordState::new(VALID_CHARS, SOLVED_WORDS);
    }

    pub fn check_if_solved(&mut self) {
        if self.words.words_are(SOLVED_WORDS) {
            self.access = Access::Solved;
        }
    }

    pub fn crossword(&self) -> &CrosswordState { &self.words }

    pub fn crossword_mut(&mut self) -> &mut CrosswordState { &mut self.words }
}

impl PuzzleState for LogLevelState {
    fn location(&self) -> Location { Location::LogLevel }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.words.can_reset() }

    fn reset(&mut self) { self.words.reset(); }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(WORDS_KEY.to_string(), self.words.to_toml());
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //
