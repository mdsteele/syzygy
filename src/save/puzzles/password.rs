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

use std::cmp::{max, min};
use toml;

use super::PuzzleState;
use crate::save::util::{pop_array, to_table, Tomlable, ACCESS_KEY};
use crate::save::{Access, CrosswordState, Location, ValidChars};

// ========================================================================= //

const VALID: ValidChars = ValidChars::Letters;

const ACTIVE_INDEX_KEY: &str = "index";
const ELINSA_KEY: &str = "elisna";
const ARGONY_KEY: &str = "argony";
const MEZURE_KEY: &str = "mezure";
const YTTRIS_KEY: &str = "yttris";
const UGRENT_KEY: &str = "ugrent";
const RELYNG_KEY: &str = "relyng";
const SLIDERS_KEY: &str = "sliders";

const ELINSA_WORDS: &[&str] = &[
    "ENGINEERING",
    "INTELLIGENCE",
    "IMPATIENT",
    "INDEPENDENCE",
    "RESOURCEFUL",
    "SARCASM",
];
const ARGONY_WORDS: &[&str] = &[
    "VENERATED",
    "PERSPECTIVE",
    "KNOWLEDGE",
    "HISTORY",
    "EXPERIENCE",
    "ELDERLY",
];
const MEZURE_WORDS: &[&str] = &[
    "DETERMINED",
    "TEAMWORK",
    "ORGANIZED",
    "DUTIFUL",
    "JUNIOR",
    "LEADERSHIP",
];
const YTTRIS_WORDS: &[&str] =
    &["ENERGY", "EMOTION", "SPONTANEOUS", "FEARFUL", "CREATIVE", "ARTISTIC"];
const UGRENT_WORDS: &[&str] = &[
    "CAUTION",
    "GRUFF",
    "PROTECTIVE",
    "SECURITY",
    "CONSERVATIVE",
    "METICULOUS",
];
const RELYNG_WORDS: &[&str] = &[
    "REALISM",
    "SKEPTICISM",
    "STEALTH",
    "SECRECY",
    "DUBIOUSNESS",
    "INVESTIGATION",
];

const INIT_SLIDERS: [i32; 6] = [-3, -1, -4, -1, -4, -2];
const SOLVED_SLIDERS: [i32; 6] = [-4, -5, -2, 0, -1, -3];

// ========================================================================= //

pub struct PasswordState {
    access: Access,
    active_index: usize,
    crosswords: [(bool, CrosswordState); 6],
    sliders: [i32; 6],
}

impl PasswordState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.solve_all_crosswords();
        self.sliders = SOLVED_SLIDERS;
    }

    pub fn solve_all_crosswords(&mut self) {
        self.crosswords = [
            (true, CrosswordState::new(VALID, ELINSA_WORDS)),
            (true, CrosswordState::new(VALID, ARGONY_WORDS)),
            (true, CrosswordState::new(VALID, MEZURE_WORDS)),
            (true, CrosswordState::new(VALID, YTTRIS_WORDS)),
            (true, CrosswordState::new(VALID, UGRENT_WORDS)),
            (true, CrosswordState::new(VALID, RELYNG_WORDS)),
        ];
    }

    pub fn active_index(&self) -> usize {
        self.active_index
    }

    pub fn set_active_index(&mut self, index: usize) {
        debug_assert!(index < 6);
        self.active_index = index;
    }

    pub fn crossword(&self, index: usize) -> &CrosswordState {
        debug_assert!(index < 6);
        &self.crosswords[index].1
    }

    pub fn crossword_mut(&mut self, index: usize) -> &mut CrosswordState {
        debug_assert!(index < 6);
        &mut self.crosswords[index].1
    }

    pub fn check_crossword(&mut self, index: usize) -> bool {
        let words = match index {
            0 => ELINSA_WORDS,
            1 => ARGONY_WORDS,
            2 => MEZURE_WORDS,
            3 => YTTRIS_WORDS,
            4 => UGRENT_WORDS,
            5 => RELYNG_WORDS,
            _ => panic!("bad index: {}", index),
        };
        let done = self.crosswords[index].1.words_are(words);
        self.crosswords[index].0 = done;
        done
    }

    pub fn crossword_is_done(&self, index: usize) -> bool {
        debug_assert!(index < 6);
        self.crosswords[index].0
    }

    pub fn all_crosswords_done(&self) -> bool {
        (0..6).all(|index| self.crossword_is_done(index))
    }

    pub fn get_slider_offset(&self, col: i32) -> i32 {
        assert!(col >= 0 && col < self.sliders.len() as i32);
        self.sliders[col as usize]
    }

    pub fn set_slider_offset(&mut self, col: i32, offset: i32) {
        assert!(col >= 0 && col < self.sliders.len() as i32);
        assert!(offset >= -5 && offset <= 0);
        self.sliders[col as usize] = offset;
        if self.sliders == SOLVED_SLIDERS {
            self.access = Access::Solved;
        }
    }
}

impl PuzzleState for PasswordState {
    fn location() -> Location {
        Location::PasswordFile
    }

    fn access(&self) -> Access {
        self.access
    }

    fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    fn allow_reset_for_undo_redo(&self) -> bool {
        false
    }

    fn can_reset(&self) -> bool {
        if self.all_crosswords_done() {
            self.sliders != INIT_SLIDERS
        } else {
            let (done, ref cross) = self.crosswords[self.active_index];
            !done && cross.can_reset()
        }
    }

    fn reset(&mut self) {
        if self.all_crosswords_done() {
            self.sliders = INIT_SLIDERS;
        } else {
            self.crosswords[self.active_index].0 = false;
            self.crosswords[self.active_index].1.reset();
        }
    }

    fn replay(&mut self) {
        self.active_index = 0;
        for &mut (ref mut done, ref mut crossword) in &mut self.crosswords {
            *done = false;
            crossword.reset();
        }
        self.sliders = INIT_SLIDERS;
        self.access = Access::BeginReplay;
    }
}

impl Tomlable for PasswordState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(
                ACTIVE_INDEX_KEY.to_string(),
                toml::Value::Integer(self.active_index as i64),
            );
            table.insert(
                ELINSA_KEY.to_string(),
                self.crosswords[0].1.to_toml(),
            );
            table.insert(
                ARGONY_KEY.to_string(),
                self.crosswords[1].1.to_toml(),
            );
            table.insert(
                MEZURE_KEY.to_string(),
                self.crosswords[2].1.to_toml(),
            );
            table.insert(
                YTTRIS_KEY.to_string(),
                self.crosswords[3].1.to_toml(),
            );
            table.insert(
                UGRENT_KEY.to_string(),
                self.crosswords[4].1.to_toml(),
            );
            table.insert(
                RELYNG_KEY.to_string(),
                self.crosswords[5].1.to_toml(),
            );
            table.insert(
                SLIDERS_KEY.to_string(),
                toml::Value::Array(
                    self.sliders
                        .iter()
                        .map(|&off| off as i64)
                        .map(toml::Value::Integer)
                        .collect(),
                ),
            );
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> PasswordState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let active_index =
            max(0, min(5, i32::pop_from_table(&mut table, ACTIVE_INDEX_KEY)))
                as usize;
        let sliders = if access == Access::Solved {
            SOLVED_SLIDERS
        } else {
            let mut sliders = INIT_SLIDERS;
            for (index, offset) in pop_array(&mut table, SLIDERS_KEY)
                .iter()
                .filter_map(toml::Value::as_integer)
                .filter(|&off| -5 <= off && off <= 0)
                .map(|off| off as i32)
                .enumerate()
            {
                if index >= sliders.len() {
                    break;
                }
                sliders[index] = offset;
            }
            sliders
        };
        PasswordState {
            access,
            active_index,
            crosswords: [
                load(&mut table, access, ELINSA_KEY, ELINSA_WORDS),
                load(&mut table, access, ARGONY_KEY, ARGONY_WORDS),
                load(&mut table, access, MEZURE_KEY, MEZURE_WORDS),
                load(&mut table, access, YTTRIS_KEY, YTTRIS_WORDS),
                load(&mut table, access, UGRENT_KEY, UGRENT_WORDS),
                load(&mut table, access, RELYNG_KEY, RELYNG_WORDS),
            ],
            sliders,
        }
    }
}

// ========================================================================= //

fn load(
    table: &mut toml::value::Table,
    access: Access,
    key: &str,
    solved_words: &[&str],
) -> (bool, CrosswordState) {
    if access == Access::Solved {
        (true, CrosswordState::new(VALID, solved_words))
    } else {
        let crossword = CrosswordState::from_toml(
            pop_array(table, key),
            VALID,
            solved_words,
        );
        (crossword.words_are(solved_words), crossword)
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use super::{PasswordState, INIT_SLIDERS, SOLVED_SLIDERS};
    use crate::save::util::{Tomlable, ACCESS_KEY};
    use crate::save::Access;

    #[test]
    fn toml_round_trip_crosswords() {
        let mut state = PasswordState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.crossword_mut(0).set_char(2, 0, 'H');
        state.crossword_mut(0).set_char(2, 1, 'E');
        state.crossword_mut(0).set_char(2, 2, 'L');
        state.crossword_mut(0).set_char(2, 3, 'L');
        state.crossword_mut(0).set_char(2, 4, 'O');
        state.crossword_mut(1).set_char(0, 0, 'W');
        state.crossword_mut(1).set_char(0, 1, 'O');
        state.crossword_mut(1).set_char(0, 2, 'R');
        state.crossword_mut(1).set_char(0, 3, 'L');
        state.crossword_mut(1).set_char(0, 4, 'D');
        state.set_active_index(2);

        let state = PasswordState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.active_index, 2);
        assert!(state.crossword(0).can_reset());
        assert_eq!(
            state.crossword(0).words()[2],
            vec!['H', 'E', 'L', 'L', 'O', ' ', ' ', ' ', ' ']
        );
        assert!(state.crossword(1).can_reset());
        assert_eq!(
            state.crossword(1).words()[0],
            vec!['W', 'O', 'R', 'L', 'D', ' ', ' ', ' ', ' ']
        );
        assert!(!state.crossword(2).can_reset());
    }

    #[test]
    fn toml_round_trip_sliders() {
        let mut state = PasswordState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.solve_all_crosswords();
        state.sliders = [0, -1, -2, -3, -4, -5];

        let state = PasswordState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.sliders, [0, -1, -2, -3, -4, -5]);
    }

    #[test]
    fn from_empty_toml() {
        let state = PasswordState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.active_index, 0);
        assert!(state.crosswords.iter().all(|&(done, _)| !done));
        assert!(state
            .crosswords
            .iter()
            .all(|&(_, ref crossword)| !crossword.can_reset()));
        assert_eq!(state.sliders, INIT_SLIDERS);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = PasswordState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert!(state.crosswords.iter().all(|&(done, _)| done));
        assert_eq!(state.sliders, SOLVED_SLIDERS);
    }
}

// ========================================================================= //
