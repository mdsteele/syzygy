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

use gui::Rect;
use save::{Access, Location};
use save::plane::{PlaneGrid, PlaneObj};
use super::PuzzleState;
use super::super::util::{ACCESS_KEY, pop_array};

// ========================================================================= //

const STAGE_KEY: &'static str = "stage";
// const YTTRIS_KEY: &'static str = "yttris";
// const ARGONY_KEY: &'static str = "argony";
const ELINSA_KEY: &'static str = "elinsa";
// const UGRENT_KEY: &'static str = "ugrent";
// const RELYNG_KEY: &'static str = "relyng";
// const MEZURE_KEY: &'static str = "mezure";

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyzygyStage {
    Yttris,
    Argony,
    Elinsa,
    Ugrent,
    Relyng,
    Mezure,
}

impl SyzygyStage {
    pub fn first() -> SyzygyStage { SyzygyStage::Yttris }

    pub fn from_toml(value: Option<&toml::Value>) -> SyzygyStage {
        if let Some(string) = value.and_then(toml::Value::as_str) {
            match string {
                "yttris" => return SyzygyStage::Yttris,
                "argony" => return SyzygyStage::Argony,
                "elinsa" => return SyzygyStage::Elinsa,
                "ugrent" => return SyzygyStage::Ugrent,
                "relyng" => return SyzygyStage::Relyng,
                "mezure" => return SyzygyStage::Mezure,
                _ => {}
            }
        }
        SyzygyStage::first()
    }

    pub fn to_toml(self) -> toml::Value {
        let string = match self {
            SyzygyStage::Yttris => "yttris",
            SyzygyStage::Argony => "argony",
            SyzygyStage::Elinsa => "elinsa",
            SyzygyStage::Ugrent => "ugrent",
            SyzygyStage::Relyng => "relyng",
            SyzygyStage::Mezure => "mezure",
        };
        toml::Value::String(string.to_string())
    }
}

// ========================================================================= //

pub struct SyzygyState {
    access: Access,
    stage: SyzygyStage,
    elinsa: PlaneGrid,
}

impl SyzygyState {
    fn elinsa_initial_grid() -> PlaneGrid {
        let mut grid = PlaneGrid::new(Rect::new(0, 0, 12, 6));
        grid.place_object(0, 0, PlaneObj::Wall);
        grid.place_object(9, 1, PlaneObj::BlueNode);
        grid.place_object(10, 1, PlaneObj::Wall);
        grid.place_object(2, 2, PlaneObj::PurpleNode);
        grid.place_object(9, 2, PlaneObj::Cross);
        grid.place_object(5, 3, PlaneObj::RedNode);
        grid.place_object(9, 3, PlaneObj::Cross);
        grid.place_object(11, 3, PlaneObj::BlueNode);
        grid.place_object(3, 4, PlaneObj::Wall);
        grid.place_object(1, 5, PlaneObj::RedNode);
        grid
    }

    pub fn from_toml(mut table: toml::value::Table) -> SyzygyState {
        let access = Access::from_toml(table.get(ACCESS_KEY));
        let stage = SyzygyStage::from_toml(table.get(STAGE_KEY));
        let mut elinsa = SyzygyState::elinsa_initial_grid();
        elinsa.set_pipes_from_toml(pop_array(&mut table, ELINSA_KEY));
        SyzygyState {
            access: access,
            stage: stage,
            elinsa: elinsa,
        }
    }

    pub fn solve(&mut self) { self.access = Access::Solved; }

    pub fn stage(&self) -> SyzygyStage { self.stage }

    pub fn advance_stage_if_done(&mut self) -> bool {
        match self.stage {
            SyzygyStage::Elinsa => {
                if self.elinsa.all_nodes_are_connected() {
                    self.stage = SyzygyStage::Ugrent;
                    return true;
                }
            }
            _ => {} // TODO
        }
        false
    }

    pub fn elinsa_grid(&self) -> &PlaneGrid { &self.elinsa }

    pub fn elinsa_grid_mut(&mut self) -> &mut PlaneGrid { &mut self.elinsa }
}

impl PuzzleState for SyzygyState {
    fn location(&self) -> Location { Location::SystemSyzygy }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool {
        match self.stage {
            SyzygyStage::Elinsa => !self.elinsa.pipes().is_empty(),
            _ => false, // TODO
        }
    }

    fn reset(&mut self) {
        match self.stage {
            SyzygyStage::Elinsa => self.elinsa.remove_all_pipes(),
            _ => {} // TODO
        }
    }

    fn replay(&mut self) {
        self.stage = SyzygyStage::first();
        self.elinsa.remove_all_pipes();
        // TODO others
        self.access = Access::BeginReplay;
    }

    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(STAGE_KEY.to_string(), self.stage.to_toml());
            table.insert(ELINSA_KEY.to_string(), self.elinsa.pipes_to_toml());
        }
        toml::Value::Table(table)
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::SyzygyStage;

    const ALL_STAGES: &'static [SyzygyStage] = &[SyzygyStage::Yttris,
                                                 SyzygyStage::Argony,
                                                 SyzygyStage::Elinsa,
                                                 SyzygyStage::Ugrent,
                                                 SyzygyStage::Relyng,
                                                 SyzygyStage::Mezure];

    #[test]
    fn stage_toml_round_trip() {
        for &original in ALL_STAGES {
            let result = SyzygyStage::from_toml(Some(&original.to_toml()));
            assert_eq!(result, original);
        }
    }
}

// ========================================================================= //
