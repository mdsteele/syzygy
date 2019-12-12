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
extern crate glob;
extern crate ico;
extern crate winres;

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

// ========================================================================= //

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.ends_with("-apple-darwin") {
        gcc::Build::new()
            .file("src/gui/loader/path_mac.m")
            .compile("syzygysys");
        println!("cargo:rustc-link-search=framework=/Library/Frameworks");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.11");
    } else if target.ends_with("-linux-gnu") {
        gcc::Build::new()
            .file("src/gui/loader/path_linux.c")
            .compile("syzygysys");
    } else if target.contains("-pc-windows-") {
        generate_rsrc_data_file().unwrap();
        let icon_path = generate_ico_file().unwrap();
        let mut res = winres::WindowsResource::new();
        res.set_icon(icon_path.to_str().unwrap());
        res.set_language(0x0409); // en_US
        res.compile().unwrap();
    } else {
        println!(
            "cargo:warning=System Syzygy doesn't currently support {}",
            target
        );
    }
}

// ========================================================================= //

const RSRC_GLOBS: &[&str] =
    &["data/backgrounds/*.bg", "data/fonts/*.ahf", "data/sprites/**/*.ahi"];

fn generate_rsrc_data_file() -> io::Result<()> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let dest_path = out_dir.join("rsrc_data.rs");
    let mut file = File::create(&dest_path)?;
    writeln!(file, "const RSRC_DATA: &[(&str, &[u8])] = &[")?;
    for rsrc_glob in RSRC_GLOBS.iter() {
        for path in glob::glob(rsrc_glob).unwrap() {
            let path: PathBuf = path.unwrap();
            let suffix = path.strip_prefix("data").unwrap();
            writeln!(
                file,
                "({:?}, include_bytes!(concat!(env!(\
                 \"CARGO_MANIFEST_DIR\"), '/', {:?}))),",
                suffix, path
            )?;
        }
    }
    writeln!(file, "];")?;
    Ok(())
}

// ========================================================================= //

const ICON_PNG_PATHS: &[&str] =
    &["data/icon/32x32.png", "data/icon/128x128.png"];

fn generate_ico_file() -> io::Result<PathBuf> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let ico_path = out_dir.join("icon.ico");
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for png_path in ICON_PNG_PATHS.iter() {
        let png_file = File::open(png_path)?;
        let image = ico::IconImage::read_png(png_file)?;
        icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);
    }
    let ico_file = File::create(&ico_path).unwrap();
    icon_dir.write(ico_file)?;
    Ok(ico_path)
}

// ========================================================================= //
