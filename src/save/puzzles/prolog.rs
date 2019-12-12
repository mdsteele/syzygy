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

use crate::save::{Access, Location};
use crate::save::util::{ACCESS_KEY, Tomlable, to_table};
use super::PuzzleState;

// ========================================================================= //

pub struct PrologState {
    access: Access,
}

impl PuzzleState for PrologState {
    fn location() -> Location { Location::Prolog }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    // This is called when the intro scene finishes.  Instead of marking the
    // puzzle visited like normal, for the Prolog we just mark it solved.
    fn visit(&mut self) { self.access = Access::Solved; }

    fn can_reset(&self) -> bool { false }

    fn reset(&mut self) {}
}

impl Tomlable for PrologState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> PrologState {
        let mut table = to_table(value);
        PrologState { access: Access::pop_from_table(&mut table, ACCESS_KEY) }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use crate::save::Access;
    use crate::save::util::{ACCESS_KEY, Tomlable};
    use super::PrologState;

    #[test]
    fn toml_round_trip() {
        let mut state = PrologState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;

        let state = PrologState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
    }

    #[test]
    fn from_empty_toml() {
        let state = PrologState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = PrologState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
    }
}

// ========================================================================= //
