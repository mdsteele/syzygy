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

use save::{AtticState, BlackState, BlameState, CubeState, DayState,
           DisconState, DotsState, DoubleState, FailureState, FictionState,
           GearsState, GroundState, HeadedState, IcyEmState, JogState,
           LaneState, LevelUpState, LineState, Location, LogLevelState,
           MeetState, MissedState, OrderState, PasswordState, PrologState,
           PuzzleState, RightState, SauceState, ServesState, SimpleState,
           StarState, SyrupState, SyzygyState, TheYState, TreadState,
           VirtueState, WhatchaState, WreckedState};
use save::util::{pop_table, to_table};

// ========================================================================= //

const EVER_CLICKED_INFO_KEY: &str = "ever_clicked_info";
const LOCATION_KEY: &str = "location";

// ========================================================================= //

pub struct Game {
    pub location: Location,
    pub prolog: PrologState,
    pub a_light_in_the_attic: AtticState,
    pub black_and_blue: BlackState,
    pub column_as_icy_em: IcyEmState,
    pub connect_the_dots: DotsState,
    pub cross_sauce: SauceState,
    pub cross_the_line: LineState,
    pub cube_tangle: CubeState,
    pub disconnected: DisconState,
    pub double_cross: DoubleState,
    pub fact_or_fiction: FictionState,
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
    pub point_of_order: OrderState,
    pub shift_gears: GearsState,
    pub shift_the_blame: BlameState,
    pub shifting_ground: GroundState,
    pub star_crossed: StarState,
    pub system_failure: FailureState,
    pub system_syzygy: SyzygyState,
    pub the_ice_is_right: RightState,
    pub the_y_factor: TheYState,
    pub tread_lightly: TreadState,
    pub virtue_or_ice: VirtueState,
    pub whatcha_column: WhatchaState,
    pub wrecked_angle: WreckedState,
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
            location: Location::from_toml(table_ref.get(LOCATION_KEY)),
            prolog: PrologState::from_toml(
                pop_table(table_ref, Location::Prolog.key())),
            a_light_in_the_attic: AtticState::from_toml(
                pop_table(table_ref, Location::ALightInTheAttic.key())),
            black_and_blue: BlackState::from_toml(
                pop_table(table_ref, Location::BlackAndBlue.key())),
            column_as_icy_em: IcyEmState::from_toml(
                pop_table(table_ref, Location::ColumnAsIcyEm.key())),
            connect_the_dots: DotsState::from_toml(
                pop_table(table_ref, Location::ConnectTheDots.key())),
            cross_sauce: SauceState::from_toml(
                pop_table(table_ref, Location::CrossSauce.key())),
            cross_the_line: LineState::from_toml(
                pop_table(table_ref, Location::CrossTheLine.key())),
            cube_tangle: CubeState::from_toml(
                pop_table(table_ref, Location::CubeTangle.key())),
            disconnected: DisconState::from_toml(
                pop_table(table_ref, Location::Disconnected.key())),
            double_cross: DoubleState::from_toml(
                pop_table(table_ref, Location::DoubleCross.key())),
            fact_or_fiction: FictionState::from_toml(
                pop_table(table_ref, Location::FactOrFiction.key())),
            ice_to_meet_you: MeetState::from_toml(
                pop_table(table_ref, Location::IceToMeetYou.key())),
            if_memory_serves: ServesState::from_toml(
                pop_table(table_ref, Location::IfMemoryServes.key())),
            jog_your_memory: JogState::from_toml(
                pop_table(table_ref, Location::JogYourMemory.key())),
            level_headed: HeadedState::from_toml(
                pop_table(table_ref, Location::LevelHeaded.key())),
            level_up: LevelUpState::from_toml(
                pop_table(table_ref, Location::LevelUp.key())),
            light_syrup: SyrupState::from_toml(
                pop_table(table_ref, Location::LightSyrup.key())),
            log_level: LogLevelState::from_toml(
                pop_table(table_ref, Location::LogLevel.key())),
            memory_lane: LaneState::from_toml(
                pop_table(table_ref, Location::MemoryLane.key())),
            missed_connections: MissedState::from_toml(
                pop_table(table_ref, Location::MissedConnections.key())),
            password_file: PasswordState::from_toml(
                pop_table(table_ref, Location::PasswordFile.key())),
            plane_and_simple: SimpleState::from_toml(
                pop_table(table_ref, Location::PlaneAndSimple.key())),
            plane_as_day: DayState::from_toml(
                pop_table(table_ref, Location::PlaneAsDay.key())),
            point_of_order: OrderState::from_toml(
                pop_table(table_ref, Location::PointOfOrder.key())),
            shift_gears: GearsState::from_toml(
                pop_table(table_ref, Location::ShiftGears.key())),
            shift_the_blame: BlameState::from_toml(
                pop_table(table_ref, Location::ShiftTheBlame.key())),
            shifting_ground: GroundState::from_toml(
                pop_table(table_ref, Location::ShiftingGround.key())),
            star_crossed: StarState::from_toml(
                pop_table(table_ref, Location::StarCrossed.key())),
            system_failure: FailureState::from_toml(
                pop_table(table_ref, Location::SystemFailure.key())),
            system_syzygy: SyzygyState::from_toml(
                pop_table(table_ref, Location::SystemSyzygy.key())),
            the_ice_is_right: RightState::from_toml(
                pop_table(table_ref, Location::TheIceIsRight.key())),
            the_y_factor: TheYState::from_toml(
                pop_table(table_ref, Location::TheYFactor.key())),
            tread_lightly: TreadState::from_toml(
                pop_table(table_ref, Location::TreadLightly.key())),
            virtue_or_ice: VirtueState::from_toml(
                pop_table(table_ref, Location::VirtueOrIce.key())),
            whatcha_column: WhatchaState::from_toml(
                pop_table(table_ref, Location::WhatchaColumn.key())),
            wrecked_angle: WreckedState::from_toml(
                pop_table(table_ref, Location::WreckedAngle.key())),
            ever_clicked_info: table_ref.get(EVER_CLICKED_INFO_KEY)
                                        .and_then(toml::Value::as_bool)
                                        .unwrap_or(false),
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

    pub fn puzzle_state(&self, loc: Location) -> &PuzzleState {
        match loc {
            Location::Map => panic!("no PuzzleState for Map"),
            Location::Prolog => &self.prolog,
            Location::ALightInTheAttic => &self.a_light_in_the_attic,
            Location::BlackAndBlue => &self.black_and_blue,
            Location::ColumnAsIcyEm => &self.column_as_icy_em,
            Location::ConnectTheDots => &self.connect_the_dots,
            Location::CrossSauce => &self.cross_sauce,
            Location::CrossTheLine => &self.cross_the_line,
            Location::CubeTangle => &self.cube_tangle,
            Location::Disconnected => &self.disconnected,
            Location::DoubleCross => &self.double_cross,
            Location::FactOrFiction => &self.fact_or_fiction,
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
            Location::PointOfOrder => &self.point_of_order,
            Location::ShiftGears => &self.shift_gears,
            Location::ShiftTheBlame => &self.shift_the_blame,
            Location::ShiftingGround => &self.shifting_ground,
            Location::StarCrossed => &self.star_crossed,
            Location::SystemFailure => &self.system_failure,
            Location::SystemSyzygy => &self.system_syzygy,
            Location::TheIceIsRight => &self.the_ice_is_right,
            Location::TheYFactor => &self.the_y_factor,
            Location::TreadLightly => &self.tread_lightly,
            Location::VirtueOrIce => &self.virtue_or_ice,
            Location::WhatchaColumn => &self.whatcha_column,
            Location::WreckedAngle => &self.wrecked_angle,
        }
    }

    pub fn puzzle_state_mut(&mut self, loc: Location) -> &mut PuzzleState {
        match loc {
            Location::Map => panic!("no PuzzleState for Map"),
            Location::Prolog => &mut self.prolog,
            Location::ALightInTheAttic => &mut self.a_light_in_the_attic,
            Location::BlackAndBlue => &mut self.black_and_blue,
            Location::ColumnAsIcyEm => &mut self.column_as_icy_em,
            Location::ConnectTheDots => &mut self.connect_the_dots,
            Location::CrossSauce => &mut self.cross_sauce,
            Location::CrossTheLine => &mut self.cross_the_line,
            Location::CubeTangle => &mut self.cube_tangle,
            Location::Disconnected => &mut self.disconnected,
            Location::DoubleCross => &mut self.double_cross,
            Location::FactOrFiction => &mut self.fact_or_fiction,
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
            Location::PointOfOrder => &mut self.point_of_order,
            Location::ShiftGears => &mut self.shift_gears,
            Location::ShiftTheBlame => &mut self.shift_the_blame,
            Location::ShiftingGround => &mut self.shifting_ground,
            Location::StarCrossed => &mut self.star_crossed,
            Location::SystemFailure => &mut self.system_failure,
            Location::SystemSyzygy => &mut self.system_syzygy,
            Location::TheIceIsRight => &mut self.the_ice_is_right,
            Location::TheYFactor => &mut self.the_y_factor,
            Location::TreadLightly => &mut self.tread_lightly,
            Location::VirtueOrIce => &mut self.virtue_or_ice,
            Location::WhatchaColumn => &mut self.whatcha_column,
            Location::WreckedAngle => &mut self.wrecked_angle,
        }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use save::{Access, Location, PuzzleState};
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
