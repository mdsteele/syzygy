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

extern crate getopts;
extern crate sdl2;

mod gui;
mod title;

use gui::Element;

// ========================================================================= //

struct Flags {
    fullscreen: Option<bool>,
}

impl Flags {
    fn parse_or_exit() -> Flags {
        let args: Vec<String> = std::env::args().collect();
        let mut opts = getopts::Options::new();
        opts.optflag("h", "help", "print this help menu");
        opts.optopt("", "fullscreen", "override fullscreen setting", "BOOL");
        let matches = opts.parse(&args[1..]).unwrap_or_else(|failure| {
            println!("Error: {:?}", failure);
            println!("Run with --help to see available flags.");
            std::process::exit(1);
        });
        if matches.opt_present("help") {
            let brief = format!("Usage: {} [options]", &args[0]);
            print!("{}", opts.usage(&brief));
            std::process::exit(1);
        }
        let fullscreen = matches.opt_str("fullscreen")
                                .and_then(|value| value.parse().ok());
        Flags { fullscreen: fullscreen }
    }

    fn fullscreen(&self) -> bool {
        self.fullscreen.unwrap_or(false)
    }
}

// ========================================================================= //

const FRAME_DELAY_MILLIS: u32 = 50;

fn main() {
    let flags = Flags::parse_or_exit();
    let sdl_context = sdl2::init().unwrap();
    let event_subsystem = sdl_context.event().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let mut window = gui::Window::new(&sdl_context,
                                      "System Syzygy",
                                      (576, 384),
                                      (480, 320),
                                      flags.fullscreen());
    let _timer = {
        gui::Event::register_clock_ticks(&event_subsystem);
        let callback = Box::new(|| {
            gui::Event::push_clock_tick(&event_subsystem);
            FRAME_DELAY_MILLIS
        });
        timer_subsystem.add_timer(FRAME_DELAY_MILLIS, callback)
    };
    let mut state = ();
    let mut view = title::TitleView::new();
    window.render(&state, &view);
    let mut event_queue = gui::EventQueue::new(&sdl_context);
    loop {
        let action = match event_queue.next() {
            gui::Event::Quit => break,
            event => view.handle_event(&event, &mut state),
        };
        if action.should_redraw() {
            window.render(&state, &view);
        }
    }
}

// ========================================================================= //
