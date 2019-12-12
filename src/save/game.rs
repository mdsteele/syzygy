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

use toml;

use crate::save::{AtticState, AutoState, BlackState, BlameState, BlindState,
           CubeState, DayState, DisconState, DotsState, DoubleState,
           FailureState, FictionState, FinaleState, GearsState, GroundState,
           HeadedState, HexState, IcyEmState, JogState, LaneState,
           LevelUpState, LineState, Location, LogLevelState, MeetState,
           MissedState, NoReturnState, OrderState, PasswordState, PovState,
           PrologState, PuzzleState, RightState, SauceState, ServesState,
           SimpleState, StarState, SyrupState, SyzygyState, TheYState,
           TreadState, WhatchaState, WreckedState};
use crate::save::util::{Tomlable, to_table};

// ========================================================================= //

const EVER_CLICKED_INFO_KEY: &str = "ever_clicked_info";
const LOCATION_KEY: &str = "location";

// ========================================================================= //

pub struct Game {
    pub location: Location,
    pub prolog: PrologState,
    pub a_light_in_the_attic: AtticState,
    pub autofac_tour: AutoState,
    pub black_and_blue: BlackState,
    pub column_as_icy_em: IcyEmState,
    pub connect_the_dots: DotsState,
    pub cross_sauce: SauceState,
    pub cross_the_line: LineState,
    pub cube_tangle: CubeState,
    pub disconnected: DisconState,
    pub double_cross: DoubleState,
    pub fact_or_fiction: FictionState,
    pub hex_spangled: HexState,
    pub ice_to_meet_you: MeetState,
    pub if_memory_serves: ServesState,
    pub jog_your_memory: JogState,
    pub level_headed: HeadedState,
    pub level_up: LevelUpState,
    pub light_syrup: SyrupState,
    pub log_level: LogLevelState,
    pub memory_lane: LaneState,
    pub missed_connections: MissedState,
    pub password_file: PasswordState,
    pub plane_and_simple: SimpleState,
    pub plane_as_day: DayState,
    pub point_of_no_return: NoReturnState,
    pub point_of_order: OrderState,
    pub point_of_view: PovState,
    pub shift_gears: GearsState,
    pub shift_the_blame: BlameState,
    pub shifting_ground: GroundState,
    pub star_crossed: StarState,
    pub system_failure: FailureState,
    pub system_syzygy: SyzygyState,
    pub the_ice_is_right: RightState,
    pub the_y_factor: TheYState,
    pub three_blind_ice: BlindState,
    pub tread_lightly: TreadState,
    pub whatcha_column: WhatchaState,
    pub wrecked_angle: WreckedState,
    pub finale: FinaleState,
    pub ever_clicked_info: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game::from_toml(toml::Value::Boolean(false));
        game.location = Location::Prolog;
        game
    }

    pub fn from_toml(value: toml::Value) -> Game {
        let mut table = to_table(value);
        let table_ref = &mut table;
        Game {
            location: Location::pop_from_table(table_ref, LOCATION_KEY),
            prolog: PrologState::pop_from_game_table(table_ref),
            a_light_in_the_attic: AtticState::pop_from_game_table(table_ref),
            autofac_tour: AutoState::pop_from_game_table(table_ref),
            black_and_blue: BlackState::pop_from_game_table(table_ref),
            column_as_icy_em: IcyEmState::pop_from_game_table(table_ref),
            connect_the_dots: DotsState::pop_from_game_table(table_ref),
            cross_sauce: SauceState::pop_from_game_table(table_ref),
            cross_the_line: LineState::pop_from_game_table(table_ref),
            cube_tangle: CubeState::pop_from_game_table(table_ref),
            disconnected: DisconState::pop_from_game_table(table_ref),
            double_cross: DoubleState::pop_from_game_table(table_ref),
            fact_or_fiction: FictionState::pop_from_game_table(table_ref),
            hex_spangled: HexState::pop_from_game_table(table_ref),
            ice_to_meet_you: MeetState::pop_from_game_table(table_ref),
            if_memory_serves: ServesState::pop_from_game_table(table_ref),
            jog_your_memory: JogState::pop_from_game_table(table_ref),
            level_headed: HeadedState::pop_from_game_table(table_ref),
            level_up: LevelUpState::pop_from_game_table(table_ref),
            light_syrup: SyrupState::pop_from_game_table(table_ref),
            log_level: LogLevelState::pop_from_game_table(table_ref),
            memory_lane: LaneState::pop_from_game_table(table_ref),
            missed_connections: MissedState::pop_from_game_table(table_ref),
            password_file: PasswordState::pop_from_game_table(table_ref),
            plane_and_simple: SimpleState::pop_from_game_table(table_ref),
            plane_as_day: DayState::pop_from_game_table(table_ref),
            point_of_no_return: NoReturnState::pop_from_game_table(table_ref),
            point_of_order: OrderState::pop_from_game_table(table_ref),
            point_of_view: PovState::pop_from_game_table(table_ref),
            shift_gears: GearsState::pop_from_game_table(table_ref),
            shift_the_blame: BlameState::pop_from_game_table(table_ref),
            shifting_ground: GroundState::pop_from_game_table(table_ref),
            star_crossed: StarState::pop_from_game_table(table_ref),
            system_failure: FailureState::pop_from_game_table(table_ref),
            system_syzygy: SyzygyState::pop_from_game_table(table_ref),
            the_ice_is_right: RightState::pop_from_game_table(table_ref),
            the_y_factor: TheYState::pop_from_game_table(table_ref),
            three_blind_ice: BlindState::pop_from_game_table(table_ref),
            tread_lightly: TreadState::pop_from_game_table(table_ref),
            whatcha_column: WhatchaState::pop_from_game_table(table_ref),
            wrecked_angle: WreckedState::pop_from_game_table(table_ref),
            finale: FinaleState::pop_from_game_table(table_ref),
            ever_clicked_info: bool::pop_from_table(table_ref,
                                                    EVER_CLICKED_INFO_KEY),
        }
    }

    pub fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
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

    pub fn puzzle_state(&self, loc: Location) -> &dyn PuzzleState {
        match loc {
            Location::Map => panic!("no PuzzleState for Map"),
            Location::Prolog => &self.prolog,
            Location::ALightInTheAttic => &self.a_light_in_the_attic,
            Location::AutofacTour => &self.autofac_tour,
            Location::BlackAndBlue => &self.black_and_blue,
            Location::ColumnAsIcyEm => &self.column_as_icy_em,
            Location::ConnectTheDots => &self.connect_the_dots,
            Location::CrossSauce => &self.cross_sauce,
            Location::CrossTheLine => &self.cross_the_line,
            Location::CubeTangle => &self.cube_tangle,
            Location::Disconnected => &self.disconnected,
            Location::DoubleCross => &self.double_cross,
            Location::FactOrFiction => &self.fact_or_fiction,
            Location::HexSpangled => &self.hex_spangled,
            Location::IceToMeetYou => &self.ice_to_meet_you,
            Location::IfMemoryServes => &self.if_memory_serves,
            Location::JogYourMemory => &self.jog_your_memory,
            Location::LevelHeaded => &self.level_headed,
            Location::LevelUp => &self.level_up,
            Location::LightSyrup => &self.light_syrup,
            Location::LogLevel => &self.log_level,
            Location::MemoryLane => &self.memory_lane,
            Location::MissedConnections => &self.missed_connections,
            Location::PasswordFile => &self.password_file,
            Location::PlaneAndSimple => &self.plane_and_simple,
            Location::PlaneAsDay => &self.plane_as_day,
            Location::PointOfNoReturn => &self.point_of_no_return,
            Location::PointOfOrder => &self.point_of_order,
            Location::PointOfView => &self.point_of_view,
            Location::ShiftGears => &self.shift_gears,
            Location::ShiftTheBlame => &self.shift_the_blame,
            Location::ShiftingGround => &self.shifting_ground,
            Location::StarCrossed => &self.star_crossed,
            Location::SystemFailure => &self.system_failure,
            Location::SystemSyzygy => &self.system_syzygy,
            Location::TheIceIsRight => &self.the_ice_is_right,
            Location::TheYFactor => &self.the_y_factor,
            Location::ThreeBlindIce => &self.three_blind_ice,
            Location::TreadLightly => &self.tread_lightly,
            Location::WhatchaColumn => &self.whatcha_column,
            Location::WreckedAngle => &self.wrecked_angle,
            Location::Finale => &self.finale,
        }
    }

    pub fn puzzle_state_mut(&mut self, loc: Location) -> &mut dyn PuzzleState {
        match loc {
            Location::Map => panic!("no PuzzleState for Map"),
            Location::Prolog => &mut self.prolog,
            Location::ALightInTheAttic => &mut self.a_light_in_the_attic,
            Location::AutofacTour => &mut self.autofac_tour,
            Location::BlackAndBlue => &mut self.black_and_blue,
            Location::ColumnAsIcyEm => &mut self.column_as_icy_em,
            Location::ConnectTheDots => &mut self.connect_the_dots,
            Location::CrossSauce => &mut self.cross_sauce,
            Location::CrossTheLine => &mut self.cross_the_line,
            Location::CubeTangle => &mut self.cube_tangle,
            Location::Disconnected => &mut self.disconnected,
            Location::DoubleCross => &mut self.double_cross,
            Location::FactOrFiction => &mut self.fact_or_fiction,
            Location::HexSpangled => &mut self.hex_spangled,
            Location::IceToMeetYou => &mut self.ice_to_meet_you,
            Location::IfMemoryServes => &mut self.if_memory_serves,
            Location::JogYourMemory => &mut self.jog_your_memory,
            Location::LevelHeaded => &mut self.level_headed,
            Location::LevelUp => &mut self.level_up,
            Location::LightSyrup => &mut self.light_syrup,
            Location::LogLevel => &mut self.log_level,
            Location::MemoryLane => &mut self.memory_lane,
            Location::MissedConnections => &mut self.missed_connections,
            Location::PasswordFile => &mut self.password_file,
            Location::PlaneAndSimple => &mut self.plane_and_simple,
            Location::PlaneAsDay => &mut self.plane_as_day,
            Location::PointOfNoReturn => &mut self.point_of_no_return,
            Location::PointOfOrder => &mut self.point_of_order,
            Location::PointOfView => &mut self.point_of_view,
            Location::ShiftGears => &mut self.shift_gears,
            Location::ShiftTheBlame => &mut self.shift_the_blame,
            Location::ShiftingGround => &mut self.shifting_ground,
            Location::StarCrossed => &mut self.star_crossed,
            Location::SystemFailure => &mut self.system_failure,
            Location::SystemSyzygy => &mut self.system_syzygy,
            Location::TheIceIsRight => &mut self.the_ice_is_right,
            Location::TheYFactor => &mut self.the_y_factor,
            Location::ThreeBlindIce => &mut self.three_blind_ice,
            Location::TreadLightly => &mut self.tread_lightly,
            Location::WhatchaColumn => &mut self.whatcha_column,
            Location::WreckedAngle => &mut self.wrecked_angle,
            Location::Finale => &mut self.finale,
        }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use crate::save::{Access, Location, PuzzleState};
    use super::Game;

    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.location, Location::Prolog);
        assert!(!game.ever_clicked_info);
        assert_eq!(game.prolog.access(), Access::Unvisited);
    }
}

// ========================================================================= //
