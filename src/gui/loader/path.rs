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

use std::env;
use std::ffi::{CStr, OsStr};
use std::os::raw::{c_char, c_uint};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

// ========================================================================= //

pub fn resource_data_root_dir() -> PathBuf {
    if env::var("CARGO").is_ok() {
        return PathBuf::from("./data");
    }
    let mut buffer = [0; 2048];
    let size = buffer.len() as c_uint;
    let error =
        unsafe { CStr::from_ptr(syzygy_rsrc_root(buffer.as_mut_ptr(), size)) };
    let error = error.to_string_lossy();
    if !error.is_empty() {
        panic!("syzygy_rsrc_root error: {}", error);
    }
    let path = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    PathBuf::from(OsStr::from_bytes(path.to_bytes())).join("data")
}

#[link(name = "syzygysys", kind = "static")]
extern "C" {
    /// Fills the buffer with the path to the directory where resource data is
    /// stored, and returns empty string on success; otherwise, returns an
    /// error message on failure.
    fn syzygy_rsrc_root(buffer: *mut c_char, size: c_uint) -> *const c_char;
}

// ========================================================================= //
