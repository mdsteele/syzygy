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

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use super::prefs::Prefs;
use toml;

// ========================================================================= //

pub struct SaveData {
    path: PathBuf,
    prefs: Prefs,
    unsaved: bool,
}

impl SaveData {
    pub fn new(path: PathBuf) -> SaveData {
        SaveData {
            path: path,
            prefs: Prefs::with_defaults(),
            unsaved: true,
        }
    }

    fn from_toml(path: PathBuf, table: &toml::Table) -> SaveData {
        let mut data = SaveData::new(path);
        data.unsaved = false;
        if let Some(prefs) = table.get(PREFS_KEY) {
            data.prefs = Prefs::from_toml(prefs);
        }
        data
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(PREFS_KEY.to_string(), self.prefs.to_toml());
        toml::Value::Table(table)
    }

    pub fn save_if_needed(&mut self) -> io::Result<()> {
        if self.unsaved {
            try!(fs::create_dir_all(self.path.parent().unwrap()));
            let mut file = try!(fs::File::create(&self.path));
            let string = self.to_toml().to_string();
            try!(file.write_all(string.as_bytes()));
            self.unsaved = false;
            if cfg!(debug_assertions) {
                println!("Saved game to disk.");
            }
        }
        Ok(())
    }

    fn load_from_disk(path: PathBuf) -> io::Result<SaveData> {
        let mut file = try!(fs::File::open(&path));
        let mut string = String::new();
        try!(file.read_to_string(&mut string));
        match toml::Parser::new(&string).parse() {
            Some(value) => Ok(SaveData::from_toml(path, &value)),
            None => {
                Err(io::Error::new(io::ErrorKind::InvalidData,
                                   "failed to parse toml"))
            }
        }
    }

    pub fn load_or_create(path: PathBuf) -> io::Result<SaveData> {
        if path.is_file() {
            SaveData::load_from_disk(path)
        } else {
            let mut data = SaveData::new(path);
            try!(data.save_if_needed());
            Ok(data)
        }
    }

    pub fn prefs(&self) -> &Prefs { &self.prefs }

    pub fn prefs_mut(&mut self) -> &mut Prefs {
        self.unsaved = true;
        &mut self.prefs
    }
}

const PREFS_KEY: &'static str = "prefs";

// ========================================================================= //
