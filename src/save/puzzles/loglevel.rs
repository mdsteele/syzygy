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

#[derive(Default)]
pub struct LogLevelState {
    access: Access,
}

impl LogLevelState {
    pub fn from_toml(table: toml::Table) -> LogLevelState {
        LogLevelState { access: Access::from_toml(table.get(ACCESS_KEY)) }
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        toml::Value::Table(table)
    }

    pub fn visit(&mut self) { self.access.visit(); }

    pub fn reset(&mut self) {
        // TODO reset
    }

    pub fn replay(&mut self) {
        self.access = Access::Replay;
        // TODO replay
    }

    pub fn solve(&mut self) {
        self.access = Access::Solved;
        // TODO solve
    }
}

impl PuzzleState for LogLevelState {
    fn location(&self) -> Location { Location::LogLevel }

    fn access(&self) -> Access { self.access }

    fn can_reset(&self) -> bool { false } // TODO
}

// ========================================================================= //
