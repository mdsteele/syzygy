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

use std::ffi::CStr;
use std::io;
use std::os::raw::{c_char, c_uint};
use std::path::PathBuf;

// ========================================================================= //

pub fn get_default_save_file_path() -> io::Result<PathBuf> {
    let mut path = try!(get_default_save_dir_path());
    path.push("save_data.toml");
    Ok(path)
}

/// Returns the path of the directory (which may not yet exist) where the game
/// save file should normally be placed.
fn get_default_save_dir_path() -> io::Result<PathBuf> {
    let mut buffer = [0; 2048];
    let size = buffer.len() as c_uint;
    let error = unsafe {
        CStr::from_ptr(syzygy_save_dir(buffer.as_mut_ptr(), size))
    };
    let error = error.to_string_lossy().to_string();
    if !error.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, error));
    }
    let path = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    let path = try!(path.to_str().map_err(|err| {
        io::Error::new(io::ErrorKind::Other,
                       format!("failed to decode path: {}", err))
    }));
    Ok(PathBuf::from(path))
}

#[link(name = "syzygysys", kind = "static")]
extern "C" {
    /// Fills the buffer with the path to the directory (which may not exist
    /// yet) where we should store save files, and returns empty string on
    /// success; otherwise, returns an error message on failure.
    fn syzygy_save_dir(buffer: *mut c_char, size: c_uint) -> *const c_char;
}

// ========================================================================= //
