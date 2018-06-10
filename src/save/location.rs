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

use save::util::Tomlable;

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Location {
    Map,
    Prolog,
    ALightInTheAttic,
    AutofacTour,
    BlackAndBlue,
    ColumnAsIcyEm,
    ConnectTheDots,
    CrossSauce,
    CrossTheLine,
    CubeTangle,
    Disconnected,
    DoubleCross,
    FactOrFiction,
    HexSpangled,
    IceToMeetYou,
    IfMemoryServes,
    JogYourMemory,
    LevelHeaded,
    LevelUp,
    LightSyrup,
    LogLevel,
    MemoryLane,
    MissedConnections,
    PasswordFile,
    PlaneAndSimple,
    PlaneAsDay,
    PointOfNoReturn,
    PointOfOrder,
    PointOfView,
    ShiftGears,
    ShiftTheBlame,
    ShiftingGround,
    StarCrossed,
    SystemFailure,
    SystemSyzygy,
    TheIceIsRight,
    TheYFactor,
    ThreeBlindIce,
    TreadLightly,
    WhatchaColumn,
    WreckedAngle,
    Finale,
}

impl Location {
    pub fn all() -> &'static [Location] { ALL_LOCATIONS }

    pub fn name(self) -> &'static str {
        match self {
            Location::Map => "The System",
            Location::Prolog => "Prolog",
            Location::ALightInTheAttic => "A Light in the Attic",
            Location::AutofacTour => "Autofac Tour",
            Location::BlackAndBlue => "Black and Blue",
            Location::ColumnAsIcyEm => "Column as Icy 'Em",
            Location::ConnectTheDots => "Connect the Dots",
            Location::CrossSauce => "Cross Sauce",
            Location::CrossTheLine => "Cross the Line",
            Location::CubeTangle => "Cube Tangle",
            Location::Disconnected => "Disconnected",
            Location::DoubleCross => "Double-Cross",
            Location::FactOrFiction => "Fact or Fiction",
            Location::HexSpangled => "Hex-Spangled",
            Location::IceToMeetYou => "Ice to Meet You",
            Location::IfMemoryServes => "If Memory Serves",
            Location::JogYourMemory => "Jog Your Memory",
            Location::LevelHeaded => "Level-Headed",
            Location::LevelUp => "Level Up",
            Location::LightSyrup => "Light Syrup",
            Location::LogLevel => "Log Level",
            Location::MemoryLane => "Memory Lane",
            Location::MissedConnections => "Missed Connections",
            Location::PasswordFile => "Password File",
            Location::PlaneAndSimple => "Plane and Simple",
            Location::PlaneAsDay => "Plane as Day",
            Location::PointOfNoReturn => "Point of No Return",
            Location::PointOfOrder => "Point of Order",
            Location::PointOfView => "Point of View",
            Location::ShiftGears => "Shift Gears",
            Location::ShiftTheBlame => "Shift the Blame",
            Location::ShiftingGround => "Shifting Ground",
            Location::StarCrossed => "Star-Crossed",
            Location::SystemFailure => "System Failure",
            Location::SystemSyzygy => "System Syzygy",
            Location::TheIceIsRight => "The Ice is Right",
            Location::TheYFactor => "The Y Factor",
            Location::ThreeBlindIce => "Three Blind Ice",
            Location::TreadLightly => "Tread Lightly",
            Location::WhatchaColumn => "Whatcha Column",
            Location::WreckedAngle => "Wrecked Angle",
            Location::Finale => "Finale",
        }
    }

    pub fn next(self) -> Location {
        match self {
            Location::Map => Location::Map,
            Location::Prolog => Location::Disconnected,
            Location::ALightInTheAttic => Location::LightSyrup,
            Location::AutofacTour => Location::ColumnAsIcyEm,
            Location::BlackAndBlue => Location::ShiftTheBlame,
            Location::ColumnAsIcyEm => Location::BlackAndBlue,
            Location::ConnectTheDots => Location::MissedConnections,
            Location::CrossSauce => Location::ShiftGears,
            Location::CrossTheLine => Location::PlaneAndSimple,
            Location::CubeTangle => Location::Map,
            Location::Disconnected => Location::LogLevel,
            Location::DoubleCross => Location::WhatchaColumn,
            Location::FactOrFiction => Location::Map,
            Location::HexSpangled => Location::Map,
            Location::IceToMeetYou => Location::TheIceIsRight,
            Location::IfMemoryServes => Location::IceToMeetYou,
            Location::JogYourMemory => Location::Map,
            Location::LevelHeaded => Location::Map,
            Location::LevelUp => Location::ThreeBlindIce,
            Location::LightSyrup => Location::TreadLightly,
            Location::LogLevel => Location::SystemFailure,
            Location::MemoryLane => Location::MissedConnections,
            Location::MissedConnections => Location::IfMemoryServes,
            Location::PasswordFile => Location::SystemSyzygy,
            Location::PlaneAndSimple => Location::MemoryLane,
            Location::PlaneAsDay => Location::LevelUp,
            Location::PointOfNoReturn => Location::JogYourMemory,
            Location::PointOfOrder => Location::Map,
            Location::PointOfView => Location::CrossSauce,
            Location::ShiftGears => Location::PointOfNoReturn,
            Location::ShiftTheBlame => Location::PointOfOrder,
            Location::ShiftingGround => Location::CubeTangle,
            Location::StarCrossed => Location::PlaneAsDay,
            Location::SystemFailure => Location::PasswordFile,
            Location::SystemSyzygy => Location::Finale,
            Location::TheIceIsRight => Location::LevelUp,
            Location::TheYFactor => Location::HexSpangled,
            Location::ThreeBlindIce => Location::Map,
            Location::TreadLightly => Location::DoubleCross,
            Location::WhatchaColumn => Location::AutofacTour,
            Location::WreckedAngle => Location::ShiftingGround,
            Location::Finale => Location::Map,
        }
    }

    pub fn prereqs(self) -> Vec<Location> {
        match self {
            Location::Map => vec![],
            Location::Prolog => vec![],
            Location::ALightInTheAttic => vec![Location::Prolog],
            Location::AutofacTour => {
                vec![Location::HexSpangled, Location::WhatchaColumn]
            }
            Location::BlackAndBlue => vec![Location::ColumnAsIcyEm],
            Location::ColumnAsIcyEm => {
                vec![Location::AutofacTour, Location::IceToMeetYou]
            }
            Location::ConnectTheDots => vec![Location::LogLevel],
            Location::CrossSauce => vec![Location::PointOfView],
            Location::CrossTheLine => vec![Location::Prolog],
            Location::CubeTangle => vec![Location::ShiftingGround],
            Location::Disconnected => vec![Location::Prolog],
            Location::DoubleCross => vec![Location::TreadLightly],
            Location::FactOrFiction => {
                vec![
                    Location::CubeTangle,
                    Location::HexSpangled,
                    Location::WhatchaColumn,
                ]
            }
            Location::HexSpangled => vec![Location::TheYFactor],
            Location::IceToMeetYou => vec![Location::IfMemoryServes],
            Location::IfMemoryServes => {
                vec![Location::MissedConnections, Location::WhatchaColumn]
            }
            Location::JogYourMemory => vec![Location::PointOfNoReturn],
            Location::LevelHeaded => vec![Location::ColumnAsIcyEm],
            Location::LevelUp => {
                vec![Location::PlaneAsDay, Location::TheIceIsRight]
            }
            Location::LightSyrup => vec![Location::ALightInTheAttic],
            Location::LogLevel => vec![Location::Disconnected],
            Location::MemoryLane => vec![Location::PlaneAndSimple],
            Location::MissedConnections => {
                vec![
                    Location::ConnectTheDots,
                    Location::CubeTangle,
                    Location::MemoryLane,
                ]
            }
            Location::PasswordFile => vec![Location::SystemFailure],
            Location::PlaneAndSimple => vec![Location::CrossTheLine],
            Location::PlaneAsDay => vec![Location::StarCrossed],
            Location::PointOfNoReturn => {
                vec![Location::IfMemoryServes, Location::ShiftGears]
            }
            Location::PointOfOrder => vec![Location::ShiftTheBlame],
            Location::PointOfView => vec![Location::ShiftingGround],
            Location::ShiftGears => vec![Location::CrossSauce],
            Location::ShiftTheBlame => vec![Location::BlackAndBlue],
            Location::ShiftingGround => vec![Location::WreckedAngle],
            Location::StarCrossed => vec![Location::IfMemoryServes],
            Location::SystemFailure => vec![Location::LogLevel],
            Location::SystemSyzygy => vec![Location::PasswordFile],
            Location::TheIceIsRight => vec![Location::IceToMeetYou],
            Location::TheYFactor => vec![Location::Prolog],
            Location::ThreeBlindIce => vec![Location::LevelUp],
            Location::TreadLightly => vec![Location::LightSyrup],
            Location::WhatchaColumn => vec![Location::DoubleCross],
            Location::WreckedAngle => vec![Location::Prolog],
            Location::Finale => vec![Location::SystemSyzygy],
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Location::Map => "map",
            Location::Prolog => "prolog",
            Location::ALightInTheAttic => "a_light_in_the_attic",
            Location::AutofacTour => "autofac_tour",
            Location::BlackAndBlue => "black_and_blue",
            Location::ColumnAsIcyEm => "column_as_icy_em",
            Location::ConnectTheDots => "connect_the_dots",
            Location::CrossSauce => "cross_sauce",
            Location::CrossTheLine => "cross_the_line",
            Location::CubeTangle => "cube_tangle",
            Location::Disconnected => "disconnected",
            Location::DoubleCross => "double_cross",
            Location::FactOrFiction => "fact_or_fiction",
            Location::HexSpangled => "hex_spangled",
            Location::IceToMeetYou => "ice_to_meet_you",
            Location::IfMemoryServes => "if_memory_serves",
            Location::JogYourMemory => "jog_your_memory",
            Location::LevelHeaded => "level_headed",
            Location::LevelUp => "level_up",
            Location::LightSyrup => "light_syrup",
            Location::LogLevel => "log_level",
            Location::MemoryLane => "memory_lane",
            Location::MissedConnections => "missed_connections",
            Location::PasswordFile => "password_file",
            Location::PlaneAndSimple => "plane_and_simple",
            Location::PlaneAsDay => "plane_as_day",
            Location::PointOfNoReturn => "point_of_no_return",
            Location::PointOfOrder => "point_of_order",
            Location::PointOfView => "point_of_view",
            Location::ShiftGears => "shift_gears",
            Location::ShiftTheBlame => "shift_the_blame",
            Location::ShiftingGround => "shifting_ground",
            Location::StarCrossed => "star_crossed",
            Location::SystemFailure => "system_failure",
            Location::SystemSyzygy => "system_syzygy",
            Location::TheIceIsRight => "the_ice_is_right",
            Location::TheYFactor => "the_y_factor",
            Location::ThreeBlindIce => "three_blind_ice",
            Location::TreadLightly => "tread_lightly",
            Location::WhatchaColumn => "whatcha_column",
            Location::WreckedAngle => "wrecked_angle",
            Location::Finale => "finale",
        }
    }
}

impl Tomlable for Location {
    fn from_toml(value: toml::Value) -> Location {
        if let Some(string) = value.as_str() {
            for &location in Location::all() {
                if string == location.key() {
                    return location;
                }
            }
        }
        Default::default()
    }

    fn to_toml(&self) -> toml::Value {
        toml::Value::String(self.key().to_string())
    }
}

impl Default for Location {
    fn default() -> Location { Location::Map }
}

const ALL_LOCATIONS: &[Location] = &[
    Location::Map,
    Location::Prolog,
    Location::ALightInTheAttic,
    Location::AutofacTour,
    Location::BlackAndBlue,
    Location::ColumnAsIcyEm,
    Location::ConnectTheDots,
    Location::CrossSauce,
    Location::CrossTheLine,
    Location::CubeTangle,
    Location::Disconnected,
    Location::DoubleCross,
    Location::FactOrFiction,
    Location::HexSpangled,
    Location::IceToMeetYou,
    Location::IfMemoryServes,
    Location::JogYourMemory,
    Location::LevelHeaded,
    Location::LevelUp,
    Location::LightSyrup,
    Location::LogLevel,
    Location::MemoryLane,
    Location::MissedConnections,
    Location::PasswordFile,
    Location::PlaneAndSimple,
    Location::PlaneAsDay,
    Location::PointOfNoReturn,
    Location::PointOfOrder,
    Location::PointOfView,
    Location::ShiftGears,
    Location::ShiftTheBlame,
    Location::ShiftingGround,
    Location::StarCrossed,
    Location::SystemFailure,
    Location::SystemSyzygy,
    Location::TheIceIsRight,
    Location::TheYFactor,
    Location::ThreeBlindIce,
    Location::TreadLightly,
    Location::WhatchaColumn,
    Location::WreckedAngle,
    Location::Finale,
];

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use save::util::Tomlable;
    use super::Location;

    #[test]
    fn toml_round_trip() {
        for original in Location::all() {
            let result = Location::from_toml(original.to_toml());
            assert_eq!(result, *original);
        }
    }

    #[test]
    fn come_from_prereq() {
        for from in Location::all() {
            let to = from.next();
            if to != Location::Map {
                let prereqs = to.prereqs();
                assert!(prereqs.contains(&from),
                        "{:?} leads to {:?}, but isn't one of its prereqs \
                         ({:?})",
                        from,
                        to,
                        prereqs);
            }
        }
    }

    #[test]
    fn leads_to_only_dependent() {
        let mut dependents: HashMap<Location, Vec<Location>> = HashMap::new();
        for &location in Location::all() {
            for prereq in location.prereqs().into_iter() {
                dependents
                    .entry(prereq)
                    .or_insert_with(Vec::new)
                    .push(location);
            }
        }
        for (&location, direct_dependents) in dependents.iter() {
            if direct_dependents.len() == 1 {
                let dependent = direct_dependents.first().cloned().unwrap();
                let next = location.next();
                assert_eq!(dependent,
                           next,
                           "{:?} leads to {:?}, but it should lead to {:?}",
                           location,
                           next,
                           dependent);
            }
        }
    }

    #[test]
    fn transitive_dependencies() {
        let num_locations = Location::all().len();
        let mut deps_map: HashMap<Location, HashSet<Location>> =
            HashMap::new();
        while deps_map.len() < num_locations {
            let mut progress = false;
            for &location in Location::all() {
                if deps_map.contains_key(&location) {
                    continue;
                }
                if !location
                    .prereqs()
                    .into_iter()
                    .all(|req| deps_map.contains_key(&req))
                {
                    continue;
                }
                let mut loc_deps: HashSet<Location> = HashSet::new();
                for req in location.prereqs().into_iter() {
                    for &req_dep in deps_map.get(&req).unwrap().iter() {
                        loc_deps.insert(req_dep);
                    }
                    loc_deps.insert(req);
                }
                deps_map.insert(location, loc_deps);
                progress = true;
                break;
            }
            if !progress {
                panic!("Location dependency cycle.");
            }
        }
        let precedes = |loc1: Location, loc2: Location| {
            deps_map.get(&loc2).unwrap().contains(&loc1)
        };
        // "Column" puzzles:
        assert!(precedes(Location::WhatchaColumn, Location::ColumnAsIcyEm));
        // "Connect" puzzles:
        assert!(precedes(Location::Disconnected, Location::ConnectTheDots));
        assert!(precedes(Location::ConnectTheDots,
                         Location::MissedConnections));
        // "Factor" puzzles:
        assert!(precedes(Location::TheYFactor, Location::AutofacTour));
        assert!(precedes(Location::TheYFactor, Location::FactOrFiction));
        // "Ice" puzzles:
        assert!(precedes(Location::IceToMeetYou, Location::TheIceIsRight));
        assert!(precedes(Location::TheIceIsRight, Location::ThreeBlindIce));
        // "Memory" puzzles:
        assert!(precedes(Location::MemoryLane, Location::IfMemoryServes));
        assert!(precedes(Location::IfMemoryServes, Location::JogYourMemory));
        // "Plane" puzzles:
        assert!(precedes(Location::PlaneAndSimple, Location::PlaneAsDay));
        // "Shift" puzzles:
        assert!(precedes(Location::ShiftingGround, Location::ShiftGears));
        assert!(precedes(Location::ShiftingGround, Location::ShiftTheBlame));
    }
}

// ========================================================================= //
