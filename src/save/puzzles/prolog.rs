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

pub struct PrologState {
    access: Access,
}

impl PrologState {
    pub fn from_toml(table: toml::value::Table) -> PrologState {
        PrologState { access: Access::from_toml(table.get(ACCESS_KEY)) }
    }

    pub fn mark_solved(&mut self) { self.access = Access::Solved; }
}

impl PuzzleState for PrologState {
    fn location(&self) -> Location { Location::Prolog }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { false }

    fn reset(&mut self) {}

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        toml::Value::Table(table)
    }
}

// ========================================================================= //
