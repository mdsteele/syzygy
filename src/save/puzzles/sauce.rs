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
use std::collections::HashSet;
use toml;

use super::PuzzleState;
use crate::save::util::{to_table, Tomlable, ACCESS_KEY};
use crate::save::{Access, Location};

// ========================================================================= //

const CURRENT_KEY: &str = "current";
const DONE_KEY: &str = "done";

#[cfg_attr(rustfmt, rustfmt_skip)]
const WORD_CLUES: &[(&str, &str)] = &[
    ("TOUGH BLUFF", "a difficult deception"),
    ("ROSE TOES", "blush-colored foot fingers"),
    ("BOOZE DUES", "liquor membership fees"),
    ("FLOUR TOWER", "a silo for ground wheat"),
    ("FAUX SNOW", "fake frozen precipitation"),
    ("BASS PLACE", "a location for low notes"),
    ("GNU QUEUE", "wildebeests waiting in a line"),
    ("FOUR MORE", "an additional quartet"),
    ("MAZE DAYS", "the era of the labyrinth"),
    ("BRICK CLIQUE", "an in-group for clay blocks"),
    ("TRITE FIGHT", "an unoriginal battle"),
    ("THROUGH STEW", "from one side of the meat soup to the other"),
    ("BUYS FLIES", "purchases insects"),
    ("THROWN STONE", "a hurled rock"),
    ("MAIN REIGN", "the primary period of royal rule"),
    ("STEAK BRAKE", "a beef decelerator"),
    ("WAX TACKS", "small nails of honeycomb material"),
    ("WHOLE BOWL", "an entire basin"),
    ("CREEK PIQUE", "a tributary irritation"),
    ("ONE SUN", "a single star"),
    ("GOOSE TRUCE", "a ceasefire between waterbirds"),
    ("HIGH EYE", "a raised visual organ"),
    ("JEWEL TOOL", "a gemstone utensil"),
    ("GNAWED ROD", "a chewed pole"),
    ("PARTIAL MARSHAL", "a biased parade leader"),
    ("SCORED BOARD", "a notched wooden plank"),
    ("FENCE TENTS", "to sell stolen cloth shelters"),
    ("STRAY SLEIGH", "an out-of-place horse-drawn sled"),
    ("TRUCKER SUCCOR", "assistance for teamsters"),
    ("BRIE QUAY", "a wharf for soft cheese"),
];

// ========================================================================= //

pub struct SauceState {
    access: Access,
    done: HashSet<i32>,
    current: i32,
}

impl SauceState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.done = (0..(WORD_CLUES.len() as i32)).collect();
        self.current = 0;
    }

    pub fn total_num_clues(&self) -> u32 {
        WORD_CLUES.len() as u32
    }

    pub fn num_clues_done(&self) -> u32 {
        self.done.len() as u32
    }

    pub fn current_clue(&self) -> &'static str {
        debug_assert!(
            self.current >= 0 && self.current < WORD_CLUES.len() as i32
        );
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
                prefix.push(' ');
                if chars.peek() == Some(&' ') {
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

impl PuzzleState for SauceState {
    fn location() -> Location {
        Location::CrossSauce
    }

    fn access(&self) -> Access {
        self.access
    }

    fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    fn can_reset(&self) -> bool {
        false
    }

    fn reset(&mut self) {
        self.done.clear();
        self.current = 0;
    }
}

impl Tomlable for SauceState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.access.is_solved() {
            table.insert(
                CURRENT_KEY.to_string(),
                toml::Value::Integer(self.current as i64),
            );
            let mut done: Vec<i32> = self.done.iter().cloned().collect();
            done.sort();
            let done = done
                .into_iter()
                .map(|idx| toml::Value::Integer(idx as i64))
                .collect();
            table.insert(DONE_KEY.to_string(), toml::Value::Array(done));
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> SauceState {
        let mut table = to_table(value);
        let num_clues = WORD_CLUES.len() as i32;
        let mut access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let done: HashSet<i32> = if access == Access::Solved {
            (0..num_clues).collect()
        } else {
            let mut set = HashSet::<i32>::pop_from_table(&mut table, DONE_KEY);
            set.retain(|&idx| 0 <= idx && idx < num_clues);
            set
        };
        if done.len() == WORD_CLUES.len() {
            access = Access::Solved;
        }
        let current = if access.is_solved() {
            0
        } else {
            min(
                max(0, i32::pop_from_table(&mut table, CURRENT_KEY)),
                num_clues - 1,
            )
        };
        let mut state = SauceState { access, done, current };
        if !state.is_solved() && state.done.contains(&state.current) {
            state.go_next();
        }
        state
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use super::{SauceState, CURRENT_KEY, DONE_KEY, WORD_CLUES};
    use crate::save::util::{Tomlable, ACCESS_KEY};
    use crate::save::Access;

    #[test]
    fn toml_round_trip() {
        let mut state = SauceState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.done.insert(3);
        state.done.insert(1);
        state.done.insert(4);
        state.current = 7;

        let state = SauceState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.done, vec![1, 3, 4].iter().cloned().collect());
        assert_eq!(state.current, 7);
    }

    #[test]
    fn from_empty_toml() {
        let state = SauceState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert!(state.done.is_empty());
        assert_eq!(state.current, 0);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = SauceState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.done, (0..(WORD_CLUES.len() as i32)).collect());
        assert_eq!(state.current, 0);
    }

    #[test]
    fn from_invalid_current_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        table.insert(
            CURRENT_KEY.to_string(),
            toml::Value::Integer(WORD_CLUES.len() as i64),
        );

        let state = SauceState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Unsolved);
        assert!(state.done.is_empty());
        assert_eq!(state.current, (WORD_CLUES.len() as i32) - 1);
    }

    #[test]
    fn from_invalid_done_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        let done = vec![-1, 0, 5, WORD_CLUES.len() as i64];
        table.insert(
            DONE_KEY.to_string(),
            toml::Value::Array(
                done.into_iter().map(toml::Value::Integer).collect(),
            ),
        );

        let state = SauceState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Unsolved);
        assert_eq!(state.done, vec![0, 5].iter().cloned().collect());
    }

    #[test]
    fn from_current_already_done_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        table.insert(CURRENT_KEY.to_string(), toml::Value::Integer(2));
        table.insert(
            DONE_KEY.to_string(),
            toml::Value::Array(vec![
                toml::Value::Integer(1),
                toml::Value::Integer(2),
                toml::Value::Integer(3),
            ]),
        );

        let state = SauceState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Unsolved);
        assert_eq!(state.done, vec![1, 2, 3].iter().cloned().collect());
        assert_eq!(state.current, 4);
    }

    #[test]
    fn from_everything_done_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        table.insert(CURRENT_KEY.to_string(), toml::Value::Integer(5));
        table.insert(
            DONE_KEY.to_string(),
            toml::Value::Array(
                (0..(WORD_CLUES.len() as i64))
                    .map(toml::Value::Integer)
                    .collect(),
            ),
        );

        let state = SauceState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.done, (0..(WORD_CLUES.len() as i32)).collect());
        assert_eq!(state.current, 0);
    }
}

// ========================================================================= //
