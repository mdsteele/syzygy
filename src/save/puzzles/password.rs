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

use save::{Access, CrosswordState, Location, ValidChars};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array, to_i32};

// ========================================================================= //

const VALID: ValidChars = ValidChars::Letters;

const ACTIVE_SLOT_KEY: &'static str = "slot";
const ELINSA_KEY: &'static str = "elisna";
const ARGONY_KEY: &'static str = "argony";
const TEZURE_KEY: &'static str = "tezure";
const YTTRIS_KEY: &'static str = "yttris";
const UGRENT_KEY: &'static str = "ugrent";
const RELYNG_KEY: &'static str = "relyng";
const SLIDERS_KEY: &'static str = "sliders";

const ELINSA_WORDS: &'static [&'static str] = &["ENGINEERING",
                                                "INTELLIGENCE",
                                                "IMPATIENT",
                                                "INDEPENDENCE",
                                                "RESOURCEFUL",
                                                "SARCASM"];
const ARGONY_WORDS: &'static [&'static str] = &["VENERATED",
                                                "PERSPECTIVE",
                                                "KNOWLEDGE",
                                                "HISTORY",
                                                "EXPERIENCE",
                                                "ELDERLY"];
const TEZURE_WORDS: &'static [&'static str] = &["DETERMINED",
                                                "TEAMWORK",
                                                "ORGANIZED",
                                                "DUTIFUL",
                                                "JUNIOR",
                                                "LEADERSHIP"];
const YTTRIS_WORDS: &'static [&'static str] = &["ENERGY",
                                                "EMOTION",
                                                "SPONTANEOUS",
                                                "FEARFUL",
                                                "CREATIVE",
                                                "ARTISTIC"];
const UGRENT_WORDS: &'static [&'static str] = &["CAUTION",
                                                "GRUFF",
                                                "PROTECTIVE",
                                                "SECURITY",
                                                "CONSERVATIVE",
                                                "METICULOUS"];
const RELYNG_WORDS: &'static [&'static str] = &["REALISM",
                                                "SKEPTICISM",
                                                "STEALTH",
                                                "SECRECY",
                                                "DUBIOUSNESS",
                                                "INVESTIGATION"];

const INIT_SLIDERS: [i32; 6] = [-3, -1, -4, -1, -4, -2];
const SOLVED_SLIDERS: [i32; 6] = [-4, -5, -2, 0, -1, -3];

// ========================================================================= //

pub struct PasswordState {
    access: Access,
    active_slot: i32,
    crosswords: [(bool, CrosswordState); 6],
    sliders: [i32; 6],
}

impl PasswordState {
    pub fn from_toml(mut table: toml::value::Table) -> PasswordState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let active_slot = max(0,
                              min(5,
                                  table.remove(ACTIVE_SLOT_KEY)
                                       .map(to_i32)
                                       .unwrap_or(0)));
        let sliders = if access == Access::Solved {
            SOLVED_SLIDERS
        } else {
            let mut sliders = INIT_SLIDERS;
            for (index, offset) in pop_array(&mut table, SLIDERS_KEY)
                                       .iter()
                                       .filter_map(toml::Value::as_integer)
                                       .filter(|&off| -5 <= off && off <= 0)
                                       .map(|off| off as i32)
                                       .enumerate() {
                if index >= sliders.len() {
                    break;
                }
                sliders[index] = offset;
            }
            sliders
        };
        PasswordState {
            access: access,
            active_slot: active_slot,
            crosswords: [load(&mut table, access, ELINSA_KEY, ELINSA_WORDS),
                         load(&mut table, access, ARGONY_KEY, ARGONY_WORDS),
                         load(&mut table, access, TEZURE_KEY, TEZURE_WORDS),
                         load(&mut table, access, YTTRIS_KEY, YTTRIS_WORDS),
                         load(&mut table, access, UGRENT_KEY, UGRENT_WORDS),
                         load(&mut table, access, RELYNG_KEY, RELYNG_WORDS)],
            sliders: sliders,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.crosswords = [(true, CrosswordState::new(VALID, ELINSA_WORDS)),
                           (true, CrosswordState::new(VALID, ARGONY_WORDS)),
                           (true, CrosswordState::new(VALID, TEZURE_WORDS)),
                           (true, CrosswordState::new(VALID, YTTRIS_WORDS)),
                           (true, CrosswordState::new(VALID, UGRENT_WORDS)),
                           (true, CrosswordState::new(VALID, RELYNG_WORDS))];
        self.sliders = SOLVED_SLIDERS;
    }

    pub fn active_slot(&self) -> i32 { self.active_slot }

    pub fn set_active_slot(&mut self, slot: i32) {
        assert!(0 <= slot && slot < 6);
        self.active_slot = slot;
    }

    pub fn crossword(&self, slot: i32) -> &CrosswordState {
        assert!(slot >= 0 && slot < 6);
        &self.crosswords[slot as usize].1
    }

    pub fn crossword_mut(&mut self, slot: i32) -> &mut CrosswordState {
        assert!(slot >= 0 && slot < 6);
        &mut self.crosswords[slot as usize].1
    }

    pub fn check_crossword(&mut self, slot: i32) -> bool {
        let words = match slot {
            0 => ELINSA_WORDS,
            1 => ARGONY_WORDS,
            2 => TEZURE_WORDS,
            3 => YTTRIS_WORDS,
            4 => UGRENT_WORDS,
            5 => RELYNG_WORDS,
            _ => panic!("bad slot: {}", slot),
        };
        let done = self.crosswords[slot as usize].1.words_are(words);
        self.crosswords[slot as usize].0 = done;
        done
    }

    pub fn crossword_is_done(&self, slot: i32) -> bool {
        assert!(slot >= 0 && slot < 6);
        self.crosswords[slot as usize].0
    }

    pub fn all_crosswords_done(&self) -> bool {
        (0..6).all(|slot| self.crossword_is_done(slot))
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

impl Default for PasswordState {
    fn default() -> PasswordState {
        PasswordState {
            access: Default::default(),
            active_slot: 0,
            crosswords: [(false, CrosswordState::blank(VALID, ELINSA_WORDS)),
                         (false, CrosswordState::blank(VALID, ARGONY_WORDS)),
                         (false, CrosswordState::blank(VALID, TEZURE_WORDS)),
                         (false, CrosswordState::blank(VALID, YTTRIS_WORDS)),
                         (false, CrosswordState::blank(VALID, UGRENT_WORDS)),
                         (false, CrosswordState::blank(VALID, RELYNG_WORDS))],
            sliders: INIT_SLIDERS,
        }
    }
}

impl PuzzleState for PasswordState {
    fn location(&self) -> Location { Location::PasswordFile }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn allow_reset_for_undo_redo(&self) -> bool { false }

    fn can_reset(&self) -> bool {
        if self.all_crosswords_done() {
            self.sliders != INIT_SLIDERS
        } else {
            let (done, ref cross) = self.crosswords[self.active_slot as usize];
            !done && cross.can_reset()
        }
    }

    fn reset(&mut self) {
        if self.all_crosswords_done() {
            self.sliders = INIT_SLIDERS;
        } else {
            self.crosswords[self.active_slot as usize].0 = false;
            self.crosswords[self.active_slot as usize].1.reset();
        }
    }

    fn replay(&mut self) {
        self.active_slot = 0;
        for &mut (ref mut done, ref mut crossword) in &mut self.crosswords {
            *done = false;
            crossword.reset();
        }
        self.sliders = INIT_SLIDERS;
        self.access = Access::BeginReplay;
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(ACTIVE_SLOT_KEY.to_string(),
                         toml::Value::Integer(self.active_slot as i64));
            table.insert(ELINSA_KEY.to_string(),
                         self.crosswords[0].1.to_toml());
            table.insert(ARGONY_KEY.to_string(),
                         self.crosswords[1].1.to_toml());
            table.insert(TEZURE_KEY.to_string(),
                         self.crosswords[2].1.to_toml());
            table.insert(YTTRIS_KEY.to_string(),
                         self.crosswords[3].1.to_toml());
            table.insert(UGRENT_KEY.to_string(),
                         self.crosswords[4].1.to_toml());
            table.insert(RELYNG_KEY.to_string(),
                         self.crosswords[5].1.to_toml());
            table.insert(SLIDERS_KEY.to_string(),
                         toml::Value::Array(self.sliders
                                                .iter()
                                                .map(|&off| off as i64)
                                                .map(toml::Value::Integer)
                                                .collect()));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //

fn load(table: &mut toml::value::Table, access: Access, key: &str,
        solved_words: &[&str])
        -> (bool, CrosswordState) {
    if access == Access::Solved {
        (true, CrosswordState::new(VALID, solved_words))
    } else {
        let crossword = CrosswordState::from_toml(pop_array(table, key),
                                                  VALID,
                                                  solved_words);
        (crossword.words_are(solved_words), crossword)
    }
}

// ========================================================================= //
