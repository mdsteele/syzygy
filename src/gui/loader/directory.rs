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

use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use super::path::resource_data_root_dir;

// ========================================================================= //

pub struct ResourceLoader {
    root_dir: PathBuf,
}

impl ResourceLoader {
    pub fn new() -> ResourceLoader {
        let root_dir = resource_data_root_dir();
        if cfg!(debug_assertions) {
            println!("resource_data_root_dir: {:?}", root_dir);
        }
        ResourceLoader { root_dir }
    }

    pub fn load(&self, path: &Path) -> io::Result<ResourceFile> {
        Ok(BufReader::new(File::open(self.root_dir.join(path))?))
    }
}

pub type ResourceFile = BufReader<File>;

// ========================================================================= //
