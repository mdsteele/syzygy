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

use std::default::Default;
use toml;

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Location {
    Map,
    Prolog,
    ALightInTheAttic,
    BlackAndBlue,
    ConnectTheDots,
    CrossTheLine,
    CubeTangle,
    Disconnected,
    LevelUp,
    LightSyrup,
    LogLevel,
    MissedConnections,
    PasswordFile,
    ShiftingGround,
    SystemFailure,
    TreadLightly,
    WreckedAngle,
}

impl Location {
    pub fn all() -> &'static [Location] { ALL_LOCATIONS }

    pub fn name(self) -> &'static str {
        match self {
            Location::Map => "The Map",
            Location::Prolog => "Prolog",
            Location::ALightInTheAttic => "A Light in the Attic",
            Location::BlackAndBlue => "Black and Blue",
            Location::ConnectTheDots => "Connect the Dots",
            Location::CrossTheLine => "Cross the Line",
            Location::CubeTangle => "Cube Tangle",
            Location::Disconnected => "Disconnected",
            Location::LevelUp => "Level Up",
            Location::LightSyrup => "Light Syrup",
            Location::LogLevel => "Log Level",
            Location::MissedConnections => "Missed Connections",
            Location::PasswordFile => "Password File",
            Location::ShiftingGround => "Shifting Ground",
            Location::SystemFailure => "System Failure",
            Location::TreadLightly => "Tread Lightly",
            Location::WreckedAngle => "Wrecked Angle",
        }
    }

    pub fn next(self) -> Location {
        match self {
            Location::Map => Location::Map,
            Location::Prolog => Location::Disconnected,
            Location::ALightInTheAttic => Location::LightSyrup,
            Location::BlackAndBlue => Location::Map,
            Location::ConnectTheDots => Location::MissedConnections,
            Location::CrossTheLine => Location::Map,
            Location::CubeTangle => Location::Map,
            Location::Disconnected => Location::LogLevel,
            Location::LevelUp => Location::Map,
            Location::LightSyrup => Location::TreadLightly,
            Location::LogLevel => Location::SystemFailure,
            Location::MissedConnections => Location::Map,
            Location::PasswordFile => Location::Map,
            Location::ShiftingGround => Location::Map,
            Location::SystemFailure => Location::PasswordFile,
            Location::TreadLightly => Location::Map,
            Location::WreckedAngle => Location::ShiftingGround,
        }
    }

    pub fn prereqs(self) -> Vec<Location> {
        match self {
            Location::Map => vec![],
            Location::Prolog => vec![],
            Location::ALightInTheAttic => vec![Location::Prolog],
            Location::BlackAndBlue => vec![Location::Prolog],
            Location::ConnectTheDots => vec![Location::LogLevel],
            Location::CrossTheLine => vec![Location::Prolog],
            Location::CubeTangle => vec![Location::WreckedAngle],
            Location::Disconnected => vec![Location::Prolog],
            Location::LevelUp => vec![Location::MissedConnections],
            Location::LightSyrup => vec![Location::ALightInTheAttic],
            Location::LogLevel => vec![Location::Disconnected],
            Location::MissedConnections => vec![Location::ConnectTheDots],
            Location::PasswordFile => vec![Location::SystemFailure],
            Location::ShiftingGround => vec![Location::WreckedAngle],
            Location::SystemFailure => vec![Location::LogLevel],
            Location::TreadLightly => vec![Location::LightSyrup],
            Location::WreckedAngle => vec![Location::Prolog],
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Location::Map => "map",
            Location::Prolog => "prolog",
            Location::ALightInTheAttic => "a_light_in_the_attic",
            Location::BlackAndBlue => "black_and_blue",
            Location::ConnectTheDots => "connect_the_dots",
            Location::CrossTheLine => "cross_the_line",
            Location::CubeTangle => "cube_tangle",
            Location::Disconnected => "disconnected",
            Location::LevelUp => "level_up",
            Location::LightSyrup => "light_syrup",
            Location::LogLevel => "log_level",
            Location::MissedConnections => "missed_connections",
            Location::PasswordFile => "password_file",
            Location::ShiftingGround => "shifting_ground",
            Location::SystemFailure => "system_failure",
            Location::TreadLightly => "tread_lightly",
            Location::WreckedAngle => "wrecked_angle",
        }
    }

    pub fn from_toml(value: Option<&toml::Value>) -> Location {
        if let Some(string) = value.and_then(toml::Value::as_str) {
            for &location in Location::all() {
                if string == location.key() {
                    return location;
                }
            }
        }
        Default::default()
    }

    pub fn to_toml(self) -> toml::Value {
        toml::Value::String(self.key().to_string())
    }
}

impl Default for Location {
    fn default() -> Location { Location::Map }
}

const ALL_LOCATIONS: &'static [Location] = &[Location::Map,
                                             Location::Prolog,
                                             Location::ALightInTheAttic,
                                             Location::BlackAndBlue,
                                             Location::ConnectTheDots,
                                             Location::CrossTheLine,
                                             Location::CubeTangle,
                                             Location::Disconnected,
                                             Location::LevelUp,
                                             Location::LightSyrup,
                                             Location::LogLevel,
                                             Location::MissedConnections,
                                             Location::PasswordFile,
                                             Location::ShiftingGround,
                                             Location::SystemFailure,
                                             Location::TreadLightly,
                                             Location::WreckedAngle];

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::Location;

    #[test]
    fn toml_round_trip() {
        for original in Location::all() {
            let result = Location::from_toml(Some(&original.to_toml()));
            assert_eq!(result, *original);
        }
    }
}

// ========================================================================= //
