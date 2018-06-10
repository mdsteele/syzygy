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

extern crate gcc;

// ========================================================================= //

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.ends_with("-apple-darwin") {
        gcc::Build::new().file("src/gui/path_mac.m").compile("syzygysys");
        println!("cargo:rustc-link-search=framework=/Library/Frameworks");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.11");
    } else if target.ends_with("-linux-gnu") {
        gcc::Build::new().file("src/gui/path_linux.c").compile("syzygysys");
    } else {
        println!("cargo:warning=System Syzygy doesn't currently support {}",
                 target);
    }
}

// ========================================================================= //
