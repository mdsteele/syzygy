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

use super::PuzzleState;
use crate::save::util::{pop_array, to_table, Tomlable, ACCESS_KEY};
use crate::save::{Access, CrosswordState, Location, ValidChars};

// ========================================================================= //

const WORDS_KEY: &str = "words";

const SOLVED_WORDS: &[&str] = &[
    "COM#", "NON+ED", "*TLE", ":IST", "CU*D", "UN,N", ":EL", "B@ON", "SUR+",
    ",NDS",
];

const VALID_CHARS: ValidChars = ValidChars::LettersAndSymbols;

// ========================================================================= //

pub struct LevelUpState {
    access: Access,
    words: CrosswordState,
}

impl LevelUpState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.words = CrosswordState::new(VALID_CHARS, SOLVED_WORDS);
    }

    pub fn check_if_solved(&mut self) {
        if self.words.words_are(SOLVED_WORDS) {
            self.access = Access::Solved;
        }
    }

    pub fn crossword(&self) -> &CrosswordState {
        &self.words
    }

    pub fn crossword_mut(&mut self) -> &mut CrosswordState {
        &mut self.words
    }
}

impl PuzzleState for LevelUpState {
    fn location() -> Location {
        Location::LevelUp
    }

    fn access(&self) -> Access {
        self.access
    }

    fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    fn can_reset(&self) -> bool {
        self.words.can_reset()
    }

    fn reset(&mut self) {
        self.words.reset();
    }
}

impl Tomlable for LevelUpState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() && self.can_reset() {
            table.insert(WORDS_KEY.to_string(), self.words.to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> LevelUpState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let words = if access == Access::Solved {
            CrosswordState::new(VALID_CHARS, SOLVED_WORDS)
        } else {
            CrosswordState::from_toml(
                pop_array(&mut table, WORDS_KEY),
                VALID_CHARS,
                SOLVED_WORDS,
            )
        };
        LevelUpState { access, words }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use super::{LevelUpState, SOLVED_WORDS};
    use crate::save::util::{Tomlable, ACCESS_KEY};
    use crate::save::Access;

    #[test]
    fn toml_round_trip() {
        let mut state = LevelUpState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.crossword_mut().set_char(1, 0, 'H');
        state.crossword_mut().set_char(1, 1, 'E');
        state.crossword_mut().set_char(1, 2, 'L');
        state.crossword_mut().set_char(1, 3, 'L');
        state.crossword_mut().set_char(1, 4, 'O');
        state.crossword_mut().set_char(1, 5, '!');

        let state = LevelUpState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert!(state.words.can_reset());
        assert_eq!(state.words.words()[1], vec!['H', 'E', 'L', 'L', 'O', '!']);
    }

    #[test]
    fn from_empty_toml() {
        let state = LevelUpState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert!(!state.words.can_reset());
        assert_eq!(state.words.words()[1], vec![' ', ' ', ' ', ' ', ' ', ' ']);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = LevelUpState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert!(state.words.words_are(SOLVED_WORDS));
    }
}

// ========================================================================= //
