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
use std::collections::BTreeSet;
use toml;

use save::{Access, Location};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array, to_i32};

// ========================================================================= //

const CURRENT_KEY: &str = "current";
const DONE_KEY: &str = "done";

#[cfg_attr(rustfmt, rustfmt_skip)]
const WORD_CLUES: &[(&str, &str)] = &[
    ("LEFT RIGHT", "remaining / correct"),
    ("MAJOR MINOR", "a military officer / a child"),
    ("HOT COLD", "popular / a viral infection"),
    ("DARK LIGHT", "morbid / easy to lift"),
    ("KIND MEAN", "a type or variety / an average"),
    ("HIGH LOW", "euphoric / to moo"),
    ("UP DOWN", "out of bed / feathers"),
    ("EVEN ODD", "balanced / unusual"),
    ("FAST SLOW", "to abstain from food / dim-witted"),
    ("TIGHT LOOSE", "miserly / at large"),
    ("RISE FALL", "a hill / autumn"),
    ("LIVING DEAD", "a sufficient income / absolutely"),
    ("HARD SOFT", "alcoholic / quiet"),
    ("BACK FRONT", "to support / to pay in advance"),
    ("IN OUT", "fashionable / three strikes"),
    ("COMMON RARE", "a public park / barely cooked"),
    ("WELL ILL", "a waterhole / a misfortune"),
    ("FIRST LAST", "foremost / to endure"),
    ("BLACK WHITE", "served without milk / part of the eye"),
    ("FRESH STALE", "impudent / hackneyed"),
    ("SINK FLOAT", "a wash basin / a parade display"),
    ("BOTTOM TOP", "Shakespearean donkey / a spinning toy"),
    ("OPEN CLOSE", "frank / nearby"),
    ("BREAK FIX", "an interruption / to rig"),
    ("SMOOTH ROUGH", "charming / outside the fairway"),
    ("SOW REAP", "a female pig / to obtain"),
    ("HIT MISS", "a successful pop song / a young woman"),
    ("PLAIN FANCY", "a flat grassy area / to desire"),
    ("LIFT DROP", "an elevator / an iota of water"),
    ("WORK PLAY", "a literary composition / a theatrical performance"),
];

// ========================================================================= //

pub struct DoubleState {
    access: Access,
    done: BTreeSet<i32>,
    current: i32,
}

impl DoubleState {
    pub fn from_toml(mut table: toml::value::Table) -> DoubleState {
        let num_clues = WORD_CLUES.len() as i32;
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let current = min(max(0,
                              table.remove(CURRENT_KEY)
                                   .map(to_i32)
                                   .unwrap_or(0)),
                          num_clues - 1);
        let done = if access == Access::Solved {
            (0..num_clues).collect()
        } else {
            pop_array(&mut table, DONE_KEY)
                .into_iter()
                .map(to_i32)
                .filter(|&idx| 0 <= idx && idx < num_clues)
                .collect()
        };
        DoubleState {
            access: access,
            done: done,
            current: current,
        }
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.done = (0..(WORD_CLUES.len() as i32)).collect();
        self.current = 0;
    }

    pub fn total_num_clues(&self) -> u32 { WORD_CLUES.len() as u32 }

    pub fn num_clues_done(&self) -> u32 { self.done.len() as u32 }

    pub fn current_clue(&self) -> &str {
        debug_assert!(self.current >= 0 &&
                      self.current < WORD_CLUES.len() as i32);
        WORD_CLUES[self.current as usize].1
    }

    pub fn go_next(&mut self) {
        let num_clues = WORD_CLUES.len() as i32;
        let mut next = (self.current + 1) % num_clues;
        while next != self.current && self.done.contains(&next) {
            next = (next + 1) % num_clues;
        }
        self.current = next;
    }

    pub fn go_prev(&mut self) {
        let mut prev = self.current - 1;
        while prev != self.current {
            if prev < 0 {
                prev = WORD_CLUES.len() as i32 - 1;
            }
            if !self.done.contains(&prev) {
                break;
            }
            prev -= 1;
        }
        self.current = prev;
    }

    pub fn try_text(&mut self, text: &str) -> (String, bool, bool) {
        let mut prefix = String::new();
        let mut chars = text.chars().peekable();
        for chr in WORD_CLUES[self.current as usize].0.chars() {
            if chr == ' ' {
                prefix.push('/');
                if chars.peek() == Some(&'/') {
                    chars.next();
                }
            } else {
                if let Some(next) = chars.next() {
                    if next == chr {
                        prefix.push(chr);
                    } else {
                        return (prefix, true, false);
                    }
                } else {
                    return (prefix, false, false);
                }
            }
        }
        self.done.insert(self.current);
        if self.done.len() == WORD_CLUES.len() {
            self.access = Access::Solved;
        }
        (prefix, false, true)
    }
}

impl PuzzleState for DoubleState {
    fn location(&self) -> Location { Location::DoubleCross }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { false }

    fn reset(&mut self) {
        self.done.clear();
        self.current = 0;
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.access.is_solved() {
            table.insert(CURRENT_KEY.to_string(),
                         toml::Value::Integer(self.current as i64));
            let done = self.done
                           .iter()
                           .map(|&idx| toml::Value::Integer(idx as i64))
                           .collect();
            table.insert(DONE_KEY.to_string(), toml::Value::Array(done));
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //
