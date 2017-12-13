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

// ========================================================================= //

pub struct Prefs {
    fullscreen: bool,
}

impl Prefs {
    pub fn with_defaults() -> Prefs { Prefs { fullscreen: true } }

    pub fn from_toml(table: &toml::value::Table) -> Prefs {
        let mut prefs = Prefs::with_defaults();
        if let Some(fullscreen) = table
            .get(FULLSCREEN_KEY)
            .and_then(toml::Value::as_bool)
        {
            prefs.fullscreen = fullscreen;
        }
        prefs
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(FULLSCREEN_KEY.to_string(),
                     toml::Value::Boolean(self.fullscreen));
        toml::Value::Table(table)
    }

    pub fn fullscreen(&self) -> bool { self.fullscreen }

    #[allow(dead_code)]
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }
}

const FULLSCREEN_KEY: &str = "fullscreen";

// ========================================================================= //
