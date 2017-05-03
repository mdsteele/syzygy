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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Location {
    Map,
    Prolog,
    ALightInTheAttic,
    BlackAndBlue,
    ConnectTheDots,
    CrossSauce,
    CrossTheLine,
    CubeTangle,
    Disconnected,
    DoubleCross,
    FactOrFiction,
    IfMemoryServes,
    JogYourMemory,
    LevelUp,
    LightSyrup,
    LogLevel,
    MemoryLane,
    MissedConnections,
    PasswordFile,
    PlaneAndSimple,
    PlaneAsDay,
    ShiftGears,
    ShiftTheBlame,
    ShiftingGround,
    StarCrossed,
    SystemFailure,
    SystemSyzygy,
    TheYFactor,
    TreadLightly,
    WhatchaColumn,
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
            Location::CrossSauce => "Cross Sauce",
            Location::CrossTheLine => "Cross the Line",
            Location::CubeTangle => "Cube Tangle",
            Location::Disconnected => "Disconnected",
            Location::DoubleCross => "Double-Cross",
            Location::FactOrFiction => "Fact or Fiction",
            Location::IfMemoryServes => "If Memory Serves",
            Location::JogYourMemory => "Jog Your Memory",
            Location::LevelUp => "Level Up",
            Location::LightSyrup => "Light Syrup",
            Location::LogLevel => "Log Level",
            Location::MemoryLane => "Memory Lane",
            Location::MissedConnections => "Missed Connections",
            Location::PasswordFile => "Password File",
            Location::PlaneAndSimple => "Plane and Simple",
            Location::PlaneAsDay => "Plane as Day",
            Location::ShiftGears => "Shift Gears",
            Location::ShiftTheBlame => "Shift the Blame",
            Location::ShiftingGround => "Shifting Ground",
            Location::StarCrossed => "Star-Crossed",
            Location::SystemFailure => "System Failure",
            Location::SystemSyzygy => "System Syzygy",
            Location::TheYFactor => "The Y Factor",
            Location::TreadLightly => "Tread Lightly",
            Location::WhatchaColumn => "Whatcha Column",
            Location::WreckedAngle => "Wrecked Angle",
        }
    }

    pub fn next(self) -> Location {
        match self {
            Location::Map => Location::Map,
            Location::Prolog => Location::Disconnected,
            Location::ALightInTheAttic => Location::LightSyrup,
            Location::BlackAndBlue => Location::ShiftTheBlame,
            Location::ConnectTheDots => Location::MissedConnections,
            Location::CrossSauce => Location::StarCrossed,
            Location::CrossTheLine => Location::DoubleCross,
            Location::CubeTangle => Location::Map,
            Location::Disconnected => Location::LogLevel,
            Location::DoubleCross => Location::CrossSauce,
            Location::FactOrFiction => Location::Map,
            Location::IfMemoryServes => Location::JogYourMemory,
            Location::JogYourMemory => Location::Map,
            Location::LevelUp => Location::Map,
            Location::LightSyrup => Location::TreadLightly,
            Location::LogLevel => Location::SystemFailure,
            Location::MemoryLane => Location::JogYourMemory,
            Location::MissedConnections => Location::Map,
            Location::PasswordFile => Location::SystemSyzygy,
            Location::PlaneAndSimple => Location::PlaneAsDay,
            Location::PlaneAsDay => Location::Map,
            Location::ShiftGears => Location::Map,
            Location::ShiftTheBlame => Location::Map,
            Location::ShiftingGround => Location::Map,
            Location::StarCrossed => Location::Map,
            Location::SystemFailure => Location::PasswordFile,
            Location::SystemSyzygy => Location::Map,
            Location::TheYFactor => Location::FactOrFiction,
            Location::TreadLightly => Location::Map,
            Location::WhatchaColumn => Location::Map,
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
            Location::CrossSauce => vec![Location::DoubleCross],
            Location::CrossTheLine => vec![Location::Prolog],
            Location::CubeTangle => vec![Location::WreckedAngle],
            Location::Disconnected => vec![Location::Prolog],
            Location::DoubleCross => vec![Location::CrossTheLine],
            Location::FactOrFiction => vec![Location::TheYFactor],
            Location::IfMemoryServes => vec![Location::MemoryLane],
            Location::JogYourMemory => vec![Location::IfMemoryServes],
            Location::LevelUp => vec![Location::MissedConnections],
            Location::LightSyrup => vec![Location::ALightInTheAttic],
            Location::LogLevel => vec![Location::Disconnected],
            Location::MemoryLane => vec![Location::CubeTangle],
            Location::MissedConnections => vec![Location::ConnectTheDots],
            Location::PasswordFile => vec![Location::SystemFailure],
            Location::PlaneAndSimple => vec![Location::StarCrossed],
            Location::PlaneAsDay => vec![Location::PlaneAndSimple],
            Location::ShiftGears => vec![Location::ShiftingGround],
            Location::ShiftTheBlame => vec![Location::BlackAndBlue],
            Location::ShiftingGround => vec![Location::WreckedAngle],
            Location::StarCrossed => vec![Location::CrossSauce],
            Location::SystemFailure => vec![Location::LogLevel],
            Location::SystemSyzygy => vec![Location::PasswordFile],
            Location::TheYFactor => vec![Location::Prolog],
            Location::TreadLightly => vec![Location::LightSyrup],
            Location::WhatchaColumn => vec![Location::MissedConnections],
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
            Location::CrossSauce => "cross_sauce",
            Location::CrossTheLine => "cross_the_line",
            Location::CubeTangle => "cube_tangle",
            Location::Disconnected => "disconnected",
            Location::DoubleCross => "double_cross",
            Location::FactOrFiction => "fact_or_fiction",
            Location::IfMemoryServes => "if_memory_serves",
            Location::JogYourMemory => "jog_your_memory",
            Location::LevelUp => "level_up",
            Location::LightSyrup => "light_syrup",
            Location::LogLevel => "log_level",
            Location::MemoryLane => "memory_lane",
            Location::MissedConnections => "missed_connections",
            Location::PasswordFile => "password_file",
            Location::PlaneAndSimple => "plane_and_simple",
            Location::PlaneAsDay => "plane_as_day",
            Location::ShiftGears => "shift_gears",
            Location::ShiftTheBlame => "shift_the_blame",
            Location::ShiftingGround => "shifting_ground",
            Location::StarCrossed => "star_crossed",
            Location::SystemFailure => "system_failure",
            Location::SystemSyzygy => "system_syzygy",
            Location::TheYFactor => "the_y_factor",
            Location::TreadLightly => "tread_lightly",
            Location::WhatchaColumn => "whatcha_column",
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

const ALL_LOCATIONS: &[Location] = &[Location::Map,
                                     Location::Prolog,
                                     Location::ALightInTheAttic,
                                     Location::BlackAndBlue,
                                     Location::ConnectTheDots,
                                     Location::CrossSauce,
                                     Location::CrossTheLine,
                                     Location::CubeTangle,
                                     Location::Disconnected,
                                     Location::DoubleCross,
                                     Location::FactOrFiction,
                                     Location::IfMemoryServes,
                                     Location::JogYourMemory,
                                     Location::LevelUp,
                                     Location::LightSyrup,
                                     Location::LogLevel,
                                     Location::MemoryLane,
                                     Location::MissedConnections,
                                     Location::PasswordFile,
                                     Location::PlaneAndSimple,
                                     Location::PlaneAsDay,
                                     Location::ShiftGears,
                                     Location::ShiftTheBlame,
                                     Location::ShiftingGround,
                                     Location::StarCrossed,
                                     Location::SystemFailure,
                                     Location::SystemSyzygy,
                                     Location::TreadLightly,
                                     Location::TheYFactor,
                                     Location::WhatchaColumn,
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
