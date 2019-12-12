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

use crate::elements::{Ast, Scene, TalkPos, TalkStyle};
use crate::gui::{Resources, Sound};

// ========================================================================= //

const MEZURE: i32 = 0;
const UGRENT: i32 = 1;

const WEST_DOOR_UPPER: i32 = -4;
const WEST_DOOR_LOWER: i32 = -3;
const EAST_DOOR_UPPER: i32 = -2;
const EAST_DOOR_LOWER: i32 = -1;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("disconnected"),
            Ast::Place(WEST_DOOR_UPPER, "tiles/caution_walls", 5, (64, 288)),
            Ast::Place(WEST_DOOR_LOWER, "tiles/caution_walls", 4, (64, 304)),
            Ast::Place(EAST_DOOR_UPPER, "tiles/caution_walls", 5, (472, 288)),
            Ast::Place(EAST_DOOR_LOWER, "tiles/caution_walls", 4, (472, 304)),
            Ast::Wait(0.75),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 304)),
                    Ast::Slide(UGRENT, (346, 304), false, true, 1.5),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                              "All right, first task for you:"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.85),
                    Ast::Place(MEZURE, "chars/mezure", 0, (-16, 304)),
                    Ast::Slide(MEZURE, (302, 304), false, true, 1.5),
                ]),
                Ast::Seq(vec![
                    Ast::Par(vec![
                        Ast::Slide(WEST_DOOR_UPPER, (64, 272),
                                   false, false, 0.25),
                        Ast::Slide(WEST_DOOR_LOWER, (64, 320),
                                   false, false, 0.25),
                    ]),
                    Ast::Wait(1.0),
                    Ast::Par(vec![
                        Ast::Slide(WEST_DOOR_UPPER, (64, 288),
                                   false, false, 0.25),
                        Ast::Slide(WEST_DOOR_LOWER, (64, 304),
                                   false, false, 0.25),
                    ]),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Past here is the storage node\n\
                       where the system logs are kept."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Get back there and figure out\n\
                       what happened.  Then get to work\n\
                       on helping fix whatever's broken."),
        ]),
        Ast::Par(vec![
            Ast::Slide(UGRENT, (178, 304), true, true, 1.0),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Um, sure.  Sorry, where exactly\n\
                           is the logs storage node?"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (354, 304), true, true, 0.85),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "I'd better show you.  Follow me."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "It's right this way-"),
            Ast::Seq(vec![
                Ast::Slide(UGRENT, (448, 304), true, false, 0.5),
                Ast::Sound(Sound::character_collision()),
                Ast::Slide(UGRENT, (430, 304), false, true, 0.25),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW, "Um."),
            Ast::Seq(vec![
                Ast::Slide(UGRENT, (448, 304), true, false, 0.25),
                Ast::Sound(Sound::character_collision()),
                Ast::Slide(UGRENT, (438, 304), false, true, 0.25),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW, "Hmm."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "It's supposed to open automatically..."),
            Ast::Seq(vec![
                Ast::Slide(UGRENT, (448, 304), true, false, 0.25),
                Ast::Sound(Sound::character_collision()),
                Ast::Slide(UGRENT, (438, 304), false, true, 0.25),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (418, 304), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Change of plans.  Your first\n\
                       task is to fix this stupid door."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (196, 304), true, true, 0.85),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "I'll check in on you later.\n\
                       Meanwhile I need to sweep the\n\
                       perimeter and make sure there\n\
                       hasn't been a security breach."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(UGRENT, (-16, 304), true, false, 0.75),
                Ast::Remove(UGRENT),
                Ast::Wait(1.0),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Well."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.2),
                Ast::Par(vec![
                    Ast::Slide(WEST_DOOR_UPPER, (64, 272), false, false, 0.25),
                    Ast::Slide(WEST_DOOR_LOWER, (64, 320), false, false, 0.25),
                ]),
                Ast::Wait(0.4),
                Ast::Par(vec![
                    Ast::Slide(WEST_DOOR_UPPER, (64, 288), false, false, 0.25),
                    Ast::Slide(WEST_DOOR_LOWER, (64, 304), false, false, 0.25),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I...don't really know much\n\
                       about fixing doors."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (262, 304), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Maybe I should take a look\n\
                       inside this box up here?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "That seems safe."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1), // Make laser field visible.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Huh.  No wonder this\n\
                       thing isn't working."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I guess I'd better fix\n\
                       up these connections."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Maybe I'll start with the red\n\
                       laser.  That one seems easiest."),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Slide(EAST_DOOR_UPPER, (472, 272), true, false, 0.5),
                Ast::Slide(EAST_DOOR_LOWER, (472, 320), true, false, 0.5),
            ]),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "That's looking better."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Maybe now I can get a look at\n\
                       those system logs.  Gotta figure\n\
                       out what's going on around here."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 304), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Seq((0..11).map(|index| {
                Ast::Seq(vec![
                    Ast::Queue(1, index),
                    Ast::Wait(0.1),
                ])
            }).collect()),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
