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

use save::Access;
use super::super::util::ACCESS_KEY;

// ========================================================================= //

#[derive(Default)]
pub struct PrologState {
    access: Access,
}

impl PrologState {
    pub fn from_toml(table: toml::Table) -> PrologState {
        PrologState { access: Access::from_toml(table.get(ACCESS_KEY)) }
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        toml::Value::Table(table)
    }

    pub fn access(&self) -> Access { self.access }

    pub fn is_visited(&self) -> bool { self.access.is_visited() }

    pub fn visit(&mut self) { self.access.visit(); }

    pub fn is_solved(&self) -> bool { self.access == Access::Solved }

    pub fn mark_solved(&mut self) { self.access = Access::Solved; }
}

// ========================================================================= //
