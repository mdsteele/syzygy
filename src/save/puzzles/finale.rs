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
use super::super::util::ACCESS_KEY;

// ========================================================================= //

pub struct FinaleState {
    access: Access,
}

impl FinaleState {
    pub fn from_toml(table: toml::value::Table) -> FinaleState {
        FinaleState { access: Access::from_toml(table.get(ACCESS_KEY)) }
    }
}

impl PuzzleState for FinaleState {
    fn location(&self) -> Location { Location::Finale }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    // This is called when the intro scene finishes.  Instead of marking the
    // puzzle visited like normal, for the Finale we just mark it solved.
    fn visit(&mut self) { self.access = Access::Solved; }

    fn can_reset(&self) -> bool { false }

    fn reset(&mut self) {}

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        toml::Value::Table(table)
    }
}

// ========================================================================= //
