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

use app_dirs::{self, AppDataType, AppDirsError, AppInfo};
use std::io;
use std::path::PathBuf;

// ========================================================================= //

const APP_INFO: AppInfo = AppInfo {
    name: "System Syzygy",
    author: "mdsteele",
};

pub fn get_default_save_file_path() -> io::Result<PathBuf> {
    let mut path = match app_dirs::app_root(AppDataType::UserData,
                                            &APP_INFO) {
        Ok(path) => path,
        Err(AppDirsError::Io(error)) => return Err(error),
        Err(AppDirsError::NotSupported) => {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      "app directory not supported"));
        }
        Err(AppDirsError::InvalidAppInfo) => {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      "app info invalid"));
        }
    };
    path.push("save_data.toml");
    Ok(path)
}

// ========================================================================= //
