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
use super::game::Game;
use super::prefs::Prefs;
use toml;

use super::util::to_table;

// ========================================================================= //

const GAME_KEY: &str = "game";
const PREFS_KEY: &str = "prefs";

// ========================================================================= //

pub struct SaveData {
    path: PathBuf,
    prefs: Prefs,
    game: Option<Game>,
}

impl SaveData {
    pub fn new(path: PathBuf) -> SaveData {
        SaveData {
            path: path,
            prefs: Prefs::with_defaults(),
            game: None,
        }
    }

    fn from_toml(path: PathBuf, mut table: toml::value::Table) -> SaveData {
        let mut data = SaveData::new(path);
        if let Some(prefs) = table.get(PREFS_KEY)
                                  .and_then(toml::Value::as_table) {
            data.prefs = Prefs::from_toml(prefs);
        }
        if let Some(game) = table.remove(GAME_KEY) {
            data.game = Some(Game::from_toml(game));
        }
        data
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(PREFS_KEY.to_string(), self.prefs.to_toml());
        if let Some(ref game) = self.game {
            table.insert(GAME_KEY.to_string(), game.to_toml());
        }
        toml::Value::Table(table)
    }

    pub fn save_to_disk(&mut self) -> io::Result<()> {
        try!(fs::create_dir_all(self.path.parent().unwrap()));
        let mut file = try!(fs::File::create(&self.path));
        let string = self.to_toml().to_string();
        try!(file.write_all(string.as_bytes()));
        if cfg!(debug_assertions) {
            println!("Saved game to disk.");
        }
        Ok(())
    }

    fn load_from_disk(path: PathBuf) -> io::Result<SaveData> {
        let mut file = try!(fs::File::open(&path));
        let mut string = String::new();
        try!(file.read_to_string(&mut string));
        match string.parse::<toml::Value>() {
            Ok(value) => Ok(SaveData::from_toml(path, to_table(value))),
            Err(_) => {
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
            try!(data.save_to_disk());
            Ok(data)
        }
    }

    pub fn prefs(&self) -> &Prefs { &self.prefs }

    pub fn prefs_mut(&mut self) -> &mut Prefs { &mut self.prefs }

    pub fn game(&self) -> Option<&Game> {
        match self.game {
            Some(ref game) => Some(game),
            None => None,
        }
    }

    pub fn game_mut(&mut self) -> Option<&mut Game> {
        match self.game {
            Some(ref mut game) => Some(game),
            None => None,
        }
    }

    pub fn start_new_game(&mut self) -> &mut Game {
        self.game = Some(Game::new());
        self.game.as_mut().unwrap()
    }

    pub fn erase_game(&mut self) { self.game = None; }
}

// ========================================================================= //
