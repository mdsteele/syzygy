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

mod elements;
mod gui;
mod modes;
mod save;

use std::path::PathBuf;

use self::gui::{Event, Window, FRAME_DELAY_MILLIS};
use self::modes::Mode;
use self::save::{Location, SaveData};

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
        opts.optflagopt(
            "",
            "fullscreen",
            "override fullscreen setting",
            "BOOL",
        );
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
        let fullscreen = matches
            .opt_default("fullscreen", "true")
            .and_then(|value| value.parse().ok());
        let save_file = matches.opt_str("save_file").map(PathBuf::from);
        let window_size = matches.opt_str("window_size").and_then(|value| {
            match &value as &str {
                "full" => Some((576, 384)),
                "small" => Some((480, 320)),
                "tall" => Some((480, 384)),
                "wide" => Some((576, 320)),
                _ => {
                    let pieces: Vec<&str> = value.split('x').collect();
                    if pieces.len() != 2 {
                        return None;
                    }
                    pieces[0].parse::<u32>().ok().and_then(|width| {
                        pieces[1].parse::<u32>().ok().and_then(|height| {
                            return Some((width, height));
                        })
                    })
                }
            }
        });
        Flags { fullscreen, save_file, window_size }
    }

    fn ideal_size(&self) -> (u32, u32) {
        self.window_size.unwrap_or((480, 320))
    }

    fn force_ideal(&self) -> bool {
        self.window_size.is_some()
    }

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

fn main() {
    let flags = Flags::parse_or_exit();
    let mut save_data = SaveData::load_or_create(flags.save_file()).unwrap();
    let sdl_context = sdl2::init().unwrap();
    let event_subsystem = sdl_context.event().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let mut window = Window::new(
        &sdl_context,
        "System Syzygy",
        (576, 384),
        flags.ideal_size(),
        flags.force_ideal(),
        flags.fullscreen(save_data.prefs()),
    );
    let _timer = {
        Event::register_clock_ticks(&event_subsystem);
        let callback = Box::new(|| {
            Event::push_clock_tick(&event_subsystem);
            FRAME_DELAY_MILLIS
        });
        timer_subsystem.add_timer(FRAME_DELAY_MILLIS, callback)
    };
    let mut mode = Mode::Title;
    loop {
        mode = match mode {
            Mode::Title => {
                modes::run_title_screen(&mut window, &mut save_data)
            }
            Mode::Location(loc) => {
                save_data.game_mut().location = loc;
                match loc {
                    Location::Map => {
                        modes::run_map_screen(&mut window, &mut save_data)
                    }
                    Location::Prolog => {
                        modes::run_prolog(&mut window, &mut save_data)
                    }
                    Location::ALightInTheAttic => {
                        modes::run_a_light_in_the_attic(
                            &mut window,
                            &mut save_data,
                        )
                    }
                    Location::AutofacTour => {
                        modes::run_autofac_tour(&mut window, &mut save_data)
                    }
                    Location::BlackAndBlue => {
                        modes::run_black_and_blue(&mut window, &mut save_data)
                    }
                    Location::ColumnAsIcyEm => modes::run_column_as_icy_em(
                        &mut window,
                        &mut save_data,
                    ),
                    Location::ConnectTheDots => modes::run_connect_the_dots(
                        &mut window,
                        &mut save_data,
                    ),
                    Location::CrossSauce => {
                        modes::run_cross_sauce(&mut window, &mut save_data)
                    }
                    Location::CrossTheLine => {
                        modes::run_cross_the_line(&mut window, &mut save_data)
                    }
                    Location::CubeTangle => {
                        modes::run_cube_tangle(&mut window, &mut save_data)
                    }
                    Location::Disconnected => {
                        modes::run_disconnected(&mut window, &mut save_data)
                    }
                    Location::DoubleCross => {
                        modes::run_double_cross(&mut window, &mut save_data)
                    }
                    Location::FactOrFiction => {
                        modes::run_fact_or_fiction(&mut window, &mut save_data)
                    }
                    Location::HexSpangled => {
                        modes::run_hex_spangled(&mut window, &mut save_data)
                    }
                    Location::IceToMeetYou => {
                        modes::run_ice_to_meet_you(&mut window, &mut save_data)
                    }
                    Location::IfMemoryServes => modes::run_if_memory_serves(
                        &mut window,
                        &mut save_data,
                    ),
                    Location::JogYourMemory => {
                        modes::run_jog_your_memory(&mut window, &mut save_data)
                    }
                    Location::LevelHeaded => {
                        modes::run_level_headed(&mut window, &mut save_data)
                    }
                    Location::LevelUp => {
                        modes::run_level_up(&mut window, &mut save_data)
                    }
                    Location::LightSyrup => {
                        modes::run_light_syrup(&mut window, &mut save_data)
                    }
                    Location::LogLevel => {
                        modes::run_log_level(&mut window, &mut save_data)
                    }
                    Location::MemoryLane => {
                        modes::run_memory_lane(&mut window, &mut save_data)
                    }
                    Location::MissedConnections => {
                        modes::run_missed_connections(
                            &mut window,
                            &mut save_data,
                        )
                    }
                    Location::PasswordFile => {
                        modes::run_password_file(&mut window, &mut save_data)
                    }
                    Location::PlaneAndSimple => modes::run_plane_and_simple(
                        &mut window,
                        &mut save_data,
                    ),
                    Location::PlaneAsDay => {
                        modes::run_plane_as_day(&mut window, &mut save_data)
                    }
                    Location::PointOfNoReturn => {
                        modes::run_point_of_no_return(
                            &mut window,
                            &mut save_data,
                        )
                    }
                    Location::PointOfOrder => {
                        modes::run_point_of_order(&mut window, &mut save_data)
                    }
                    Location::PointOfView => {
                        modes::run_point_of_view(&mut window, &mut save_data)
                    }
                    Location::ShiftGears => {
                        modes::run_shift_gears(&mut window, &mut save_data)
                    }
                    Location::ShiftTheBlame => {
                        modes::run_shift_the_blame(&mut window, &mut save_data)
                    }
                    Location::ShiftingGround => {
                        modes::run_shifting_ground(&mut window, &mut save_data)
                    }
                    Location::StarCrossed => {
                        modes::run_star_crossed(&mut window, &mut save_data)
                    }
                    Location::SystemFailure => {
                        modes::run_system_failure(&mut window, &mut save_data)
                    }
                    Location::SystemSyzygy => {
                        modes::run_system_syzygy(&mut window, &mut save_data)
                    }
                    Location::TheIceIsRight => modes::run_the_ice_is_right(
                        &mut window,
                        &mut save_data,
                    ),
                    Location::TheYFactor => {
                        modes::run_the_y_factor(&mut window, &mut save_data)
                    }
                    Location::ThreeBlindIce => {
                        modes::run_three_blind_ice(&mut window, &mut save_data)
                    }
                    Location::TreadLightly => {
                        modes::run_tread_lightly(&mut window, &mut save_data)
                    }
                    Location::WhatchaColumn => {
                        modes::run_whatcha_column(&mut window, &mut save_data)
                    }
                    Location::WreckedAngle => {
                        modes::run_wrecked_angle(&mut window, &mut save_data)
                    }
                    Location::Finale => {
                        modes::run_finale(&mut window, &mut save_data)
                    }
                }
            }
            Mode::Quit => break,
        };
    }
    if let Err(error) = save_data.save_to_disk() {
        println!("Failed to save game: {}", error);
    }
}

// ========================================================================= //
