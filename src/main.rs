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

extern crate ahi;
extern crate getopts;
extern crate sdl2;
extern crate toml;

mod gui;
mod save;
mod title;

use self::gui::{Event, Window};
use self::save::SaveData;
use std::path::PathBuf;

// ========================================================================= //

struct Flags {
    fullscreen: Option<bool>,
    save_file: Option<PathBuf>,
    window_size: Option<(u32, u32)>,
}

impl Flags {
    fn parse_or_exit() -> Flags {
        let args: Vec<String> = std::env::args().collect();
        let mut opts = getopts::Options::new();
        opts.optflag("h", "help", "print this help menu");
        opts.optopt("", "fullscreen", "override fullscreen setting", "BOOL");
        opts.optopt("", "save_file", "override save file path", "FILE");
        opts.optopt("", "window_size", "override window size", "WxH");
        let matches = opts.parse(&args[1..]).unwrap_or_else(|failure| {
            println!("Error: {:?}", failure);
            println!("Run with --help to see available flags.");
            std::process::exit(1);
        });
        if matches.opt_present("help") {
            let brief = format!("Usage: {} [options]", &args[0]);
            print!("{}", opts.usage(&brief));
            std::process::exit(0);
        }
        let fullscreen = matches.opt_str("fullscreen")
                                .and_then(|value| value.parse().ok());
        let save_file = matches.opt_str("save_file").map(PathBuf::from);
        let window_size = matches.opt_str("window_size").and_then(|value| {
            let pieces: Vec<&str> = value.split('x').collect();
            if pieces.len() != 2 {
                return None;
            }
            pieces[0].parse::<u32>().ok().and_then(|width| {
                pieces[1].parse::<u32>().ok().and_then(|height| {
                    return Some((width, height));
                })
            })
        });
        Flags {
            fullscreen: fullscreen,
            save_file: save_file,
            window_size: window_size,
        }
    }

    fn ideal_size(&self) -> (u32, u32) {
        self.window_size.unwrap_or((480, 320))
    }

    fn force_ideal(&self) -> bool { self.window_size.is_some() }

    fn fullscreen(&self, prefs: &save::Prefs) -> bool {
        self.fullscreen.unwrap_or(prefs.fullscreen())
    }

    fn save_file(&self) -> PathBuf {
        match self.save_file {
            Some(ref path) => path.clone(),
            None => save::get_default_save_file_path().unwrap(),
        }
    }
}

// ========================================================================= //

const FRAME_DELAY_MILLIS: u32 = 50;

fn main() {
    let flags = Flags::parse_or_exit();
    let mut save_data = SaveData::load_or_create(flags.save_file()).unwrap();
    let sdl_context = sdl2::init().unwrap();
    let event_subsystem = sdl_context.event().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let mut window = Window::new(&sdl_context,
                                 "System Syzygy",
                                 (576, 384),
                                 flags.ideal_size(),
                                 flags.force_ideal(),
                                 flags.fullscreen(save_data.prefs()));
    let _timer = {
        Event::register_clock_ticks(&event_subsystem);
        let callback = Box::new(|| {
            Event::push_clock_tick(&event_subsystem);
            FRAME_DELAY_MILLIS
        });
        timer_subsystem.add_timer(FRAME_DELAY_MILLIS, callback)
    };
    title::run_title_screen(&mut window, &mut save_data);
}

// ========================================================================= //
