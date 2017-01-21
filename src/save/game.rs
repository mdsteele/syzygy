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

use super::location::Location;
use super::puzzles::{AtticState, BlackState, BlameState, CubeState,
                     DisconState, DotsState, FailureState, GearsState,
                     GroundState, LevelUpState, LineState, LogLevelState,
                     MissedState, PasswordState, PrologState, PuzzleState,
                     SyrupState, TreadState, WreckedState};
use super::util::{pop_table, to_table};

// ========================================================================= //

const EVER_CLICKED_INFO_KEY: &'static str = "ever_clicked_info";
const LOCATION_KEY: &'static str = "location";

// ========================================================================= //

#[derive(Default)]
pub struct Game {
    pub location: Location,
    pub prolog: PrologState,
    pub a_light_in_the_attic: AtticState,
    pub black_and_blue: BlackState,
    pub connect_the_dots: DotsState,
    pub cross_the_line: LineState,
    pub cube_tangle: CubeState,
    pub disconnected: DisconState,
    pub level_up: LevelUpState,
    pub light_syrup: SyrupState,
    pub log_level: LogLevelState,
    pub missed_connections: MissedState,
    pub password_file: PasswordState,
    pub shift_gears: GearsState,
    pub shift_the_blame: BlameState,
    pub shifting_ground: GroundState,
    pub system_failure: FailureState,
    pub tread_lightly: TreadState,
    pub wrecked_angle: WreckedState,
    pub ever_clicked_info: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut game: Game = Default::default();
        game.location = Location::Prolog;
        game
    }

    pub fn from_toml(value: toml::Value) -> Game {
        let mut table = to_table(value);
        let table_ref = &mut table;
        Game {
            location: Location::from_toml(table_ref.get(LOCATION_KEY)),
            prolog: PrologState::from_toml(
                pop_table(table_ref, Location::Prolog.key())),
            a_light_in_the_attic: AtticState::from_toml(
                pop_table(table_ref, Location::ALightInTheAttic.key())),
            black_and_blue: BlackState::from_toml(
                pop_table(table_ref, Location::BlackAndBlue.key())),
            connect_the_dots: DotsState::from_toml(
                pop_table(table_ref, Location::ConnectTheDots.key())),
            cross_the_line: LineState::from_toml(
                pop_table(table_ref, Location::CrossTheLine.key())),
            cube_tangle: CubeState::from_toml(
                pop_table(table_ref, Location::CubeTangle.key())),
            disconnected: DisconState::from_toml(
                pop_table(table_ref, Location::Disconnected.key())),
            level_up: LevelUpState::from_toml(
                pop_table(table_ref, Location::LevelUp.key())),
            light_syrup: SyrupState::from_toml(
                pop_table(table_ref, Location::LightSyrup.key())),
            log_level: LogLevelState::from_toml(
                pop_table(table_ref, Location::LogLevel.key())),
            missed_connections: MissedState::from_toml(
                pop_table(table_ref, Location::MissedConnections.key())),
            password_file: PasswordState::from_toml(
                pop_table(table_ref, Location::PasswordFile.key())),
            shift_gears: GearsState::from_toml(
                pop_table(table_ref, Location::ShiftGears.key())),
            shift_the_blame: BlameState::from_toml(
                pop_table(table_ref, Location::ShiftTheBlame.key())),
            shifting_ground: GroundState::from_toml(
                pop_table(table_ref, Location::ShiftingGround.key())),
            system_failure: FailureState::from_toml(
                pop_table(table_ref, Location::SystemFailure.key())),
            tread_lightly: TreadState::from_toml(
                pop_table(table_ref, Location::TreadLightly.key())),
            wrecked_angle: WreckedState::from_toml(
                pop_table(table_ref, Location::WreckedAngle.key())),
            ever_clicked_info: table_ref.get(EVER_CLICKED_INFO_KEY)
                                        .and_then(toml::Value::as_bool)
                                        .unwrap_or(false),
        }
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::Table::new();
        table.insert(LOCATION_KEY.to_string(), self.location.to_toml());
        for &location in Location::all() {
            if location != Location::Map {
                let puzzle_state = self.puzzle_state(location);
                if puzzle_state.has_been_visited() {
                    table.insert(location.key().to_string(),
                                 puzzle_state.to_toml());
                }
            }
        }
        table.insert(EVER_CLICKED_INFO_KEY.to_string(),
                     toml::Value::Boolean(self.ever_clicked_info));
        toml::Value::Table(table)
    }

    pub fn is_unlocked(&self, location: Location) -> bool {
        location.prereqs().iter().all(|&prereq| self.has_been_solved(prereq))
    }

    pub fn has_been_solved(&self, location: Location) -> bool {
        self.puzzle_state(location).has_been_solved()
    }

    pub fn puzzle_state(&self, loc: Location) -> &PuzzleState {
        match loc {
            Location::Map => panic!("no PuzzleState for Map"),
            Location::Prolog => &self.prolog,
            Location::ALightInTheAttic => &self.a_light_in_the_attic,
            Location::BlackAndBlue => &self.black_and_blue,
            Location::ConnectTheDots => &self.connect_the_dots,
            Location::CrossTheLine => &self.cross_the_line,
            Location::CubeTangle => &self.cube_tangle,
            Location::Disconnected => &self.disconnected,
            Location::LevelUp => &self.level_up,
            Location::LightSyrup => &self.light_syrup,
            Location::LogLevel => &self.log_level,
            Location::MissedConnections => &self.missed_connections,
            Location::PasswordFile => &self.password_file,
            Location::ShiftGears => &self.shift_gears,
            Location::ShiftTheBlame => &self.shift_the_blame,
            Location::ShiftingGround => &self.shifting_ground,
            Location::SystemFailure => &self.system_failure,
            Location::TreadLightly => &self.tread_lightly,
            Location::WreckedAngle => &self.wrecked_angle,
        }
    }

    pub fn puzzle_state_mut(&mut self, loc: Location) -> &mut PuzzleState {
        match loc {
            Location::Map => panic!("no PuzzleState for Map"),
            Location::Prolog => &mut self.prolog,
            Location::ALightInTheAttic => &mut self.a_light_in_the_attic,
            Location::BlackAndBlue => &mut self.black_and_blue,
            Location::ConnectTheDots => &mut self.connect_the_dots,
            Location::CrossTheLine => &mut self.cross_the_line,
            Location::CubeTangle => &mut self.cube_tangle,
            Location::Disconnected => &mut self.disconnected,
            Location::LevelUp => &mut self.level_up,
            Location::LightSyrup => &mut self.light_syrup,
            Location::LogLevel => &mut self.log_level,
            Location::MissedConnections => &mut self.missed_connections,
            Location::PasswordFile => &mut self.password_file,
            Location::ShiftGears => &mut self.shift_gears,
            Location::ShiftTheBlame => &mut self.shift_the_blame,
            Location::ShiftingGround => &mut self.shifting_ground,
            Location::SystemFailure => &mut self.system_failure,
            Location::TreadLightly => &mut self.tread_lightly,
            Location::WreckedAngle => &mut self.wrecked_angle,
        }
    }
}

// ========================================================================= //
