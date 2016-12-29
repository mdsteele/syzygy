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
use super::puzzles::{AtticState, CubeState, DisconState, DotsState,
                     FailureState, GroundState, LevelUpState, LineState,
                     LogLevelState, MissedState, PasswordState, PrologState,
                     PuzzleState, TreadState, WreckedState};
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
    pub connect_the_dots: DotsState,
    pub cross_the_line: LineState,
    pub cube_tangle: CubeState,
    pub disconnected: DisconState,
    pub level_up: LevelUpState,
    pub log_level: LogLevelState,
    pub missed_connections: MissedState,
    pub password_file: PasswordState,
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
        let prolog = pop_table(&mut table, Location::Prolog.key());
        let attic = pop_table(&mut table, Location::ALightInTheAttic.key());
        let dots = pop_table(&mut table, Location::ConnectTheDots.key());
        let line = pop_table(&mut table, Location::CrossTheLine.key());
        let cube = pop_table(&mut table, Location::CubeTangle.key());
        let discon = pop_table(&mut table, Location::Disconnected.key());
        let levelup = pop_table(&mut table, Location::LevelUp.key());
        let loglevel = pop_table(&mut table, Location::LogLevel.key());
        let missed = pop_table(&mut table, Location::MissedConnections.key());
        let password = pop_table(&mut table, Location::PasswordFile.key());
        let ground = pop_table(&mut table, Location::ShiftingGround.key());
        let failure = pop_table(&mut table, Location::SystemFailure.key());
        let tread = pop_table(&mut table, Location::TreadLightly.key());
        let wrecked = pop_table(&mut table, Location::WreckedAngle.key());
        Game {
            location: Location::from_toml(table.get(LOCATION_KEY)),
            prolog: PrologState::from_toml(prolog),
            a_light_in_the_attic: AtticState::from_toml(attic),
            connect_the_dots: DotsState::from_toml(dots),
            cross_the_line: LineState::from_toml(line),
            cube_tangle: CubeState::from_toml(cube),
            disconnected: DisconState::from_toml(discon),
            level_up: LevelUpState::from_toml(levelup),
            log_level: LogLevelState::from_toml(loglevel),
            missed_connections: MissedState::from_toml(missed),
            password_file: PasswordState::from_toml(password),
            shifting_ground: GroundState::from_toml(ground),
            system_failure: FailureState::from_toml(failure),
            tread_lightly: TreadState::from_toml(tread),
            wrecked_angle: WreckedState::from_toml(wrecked),
            ever_clicked_info: table.get(EVER_CLICKED_INFO_KEY)
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

    pub fn puzzle_state(&self, location: Location) -> &PuzzleState {
        match location {
            Location::Map => panic!("no PuzzleState for Map"),
            Location::Prolog => &self.prolog,
            Location::ALightInTheAttic => &self.a_light_in_the_attic,
            Location::ConnectTheDots => &self.connect_the_dots,
            Location::CrossTheLine => &self.cross_the_line,
            Location::CubeTangle => &self.cube_tangle,
            Location::Disconnected => &self.disconnected,
            Location::LevelUp => &self.level_up,
            Location::LogLevel => &self.log_level,
            Location::MissedConnections => &self.missed_connections,
            Location::PasswordFile => &self.password_file,
            Location::ShiftingGround => &self.shifting_ground,
            Location::SystemFailure => &self.system_failure,
            Location::TreadLightly => &self.tread_lightly,
            Location::WreckedAngle => &self.wrecked_angle,
        }
    }
}

// ========================================================================= //
