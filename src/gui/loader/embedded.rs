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

use std::io;
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/rsrc_data.rs"));

// ========================================================================= //

pub struct ResourceLoader {}

impl ResourceLoader {
    pub fn new() -> ResourceLoader {
        if cfg!(debug_assertions) {
            println!("using embedded resource data");
        }
        ResourceLoader {}
    }

    pub fn load(&self, path: &Path) -> io::Result<ResourceFile> {
        if let Some(string) = path.to_str() {
            if let Ok(index) =
                RSRC_DATA.binary_search_by_key(&string, |entry| entry.0)
            {
                return Ok(RSRC_DATA[index].1);
            }
        }
        let msg = format!("no such embedded resource file: {:?}", path);
        Err(io::Error::new(io::ErrorKind::NotFound, msg))
    }
}

pub type ResourceFile = &'static [u8];

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::RSRC_DATA;

    #[test]
    fn rsrc_data_is_sorted() {
        let mut prev = "";
        for entry in RSRC_DATA {
            let path: &str = entry.0;
            assert!(path > prev);
            prev = path;
        }
    }
}

// ========================================================================= //
