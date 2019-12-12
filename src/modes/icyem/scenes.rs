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

const ARGONY: i32 = 1;
const ARGONY_BG: i32 = -5;
const MEZURE: i32 = 2;
const MEZURE_BG: i32 = -6;
const RELYNG: i32 = -7;

const DOOR_UPPER_L: i32 = -1;
const DOOR_UPPER_R: i32 = -2;
const DOOR_LOWER_L: i32 = -3;
const DOOR_LOWER_R: i32 = -4;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Par(vec![
            Ast::SetBg("column_as_icy_em_1"),
            Ast::Place(DOOR_UPPER_L, "tiles/caution_walls", 12, (456, 288)),
            Ast::Place(DOOR_UPPER_R, "tiles/caution_walls", 13, (472, 288)),
            Ast::Place(DOOR_LOWER_L, "tiles/caution_walls", 10, (456, 304)),
            Ast::Place(DOOR_LOWER_R, "tiles/caution_walls", 11, (472, 304)),
            Ast::Seq(vec![
                Ast::Place(ARGONY, "chars/argony", 0, (270, 304)),
                Ast::Slide(ARGONY, (415, 304), false, true, 2.0),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Place(MEZURE, "chars/mezure", 0, (-16, 256)),
                Ast::Slide(MEZURE, (96, 256), false, true, 1.0),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(MEZURE, (130, 272), 0.5),
                Ast::Slide(MEZURE, (160, 272), false, false, 0.25),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(MEZURE, (194, 288), 0.5),
                Ast::Slide(MEZURE, (224, 288), false, true, 0.35),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "So who's this person\n\
                           you're looking for?"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Someone who knows how\n\
                       to find out what's been\n\
                       going on around here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Now, be a dear and help\n\
                       me get this door open."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (256, 304), 0.5),
            Ast::Slide(MEZURE, (295, 304), false, true, 0.4),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Are you, uh, sure\n\
                       that's a good idea?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "The last time I went\n\
                       into that icebox, I\n\
                       got locked inside."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "That's because you didn't\n\
                       know what you were doing.\n\
                       I, on the other hand, know\n\
                       $iexactly$r  what I'm doing."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Which is what?  Walking\n\
                       into an industrial freezer\n\
                       with a known history of\n\
                       trapping people inside?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Correct.\n\
                       Chop-chop, now!"),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh."),
            Ast::Slide(MEZURE, (432, 304), true, true, 1.25),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Slide(ARGONY, (390, 304), true, true, 0.35),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.25),
            Ast::Slide(MEZURE, (422, 304), false, true, 0.15),
            Ast::Slide(MEZURE, (438, 304), true, false, 0.15),
            Ast::Sound(Sound::character_collision()),
            Ast::Par(vec![
                Ast::Slide(MEZURE, (432, 304), false, true, 0.15),
                Ast::Slide(DOOR_UPPER_L, (456, 272), true, false, 0.5),
                Ast::Slide(DOOR_UPPER_R, (472, 272), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_L, (456, 320), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_R, (472, 320), true, false, 0.5),
            ]),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Ah, thank you, child."),
        ]),
        Ast::Seq(vec![
            Ast::Swap(ARGONY, ARGONY_BG),
            Ast::Slide(ARGONY_BG, (592, 304), true, false, 1.0),
            Ast::Remove(ARGONY_BG),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "W- Wait for me!"),
        ]),
        Ast::Seq(vec![
            Ast::Swap(MEZURE, MEZURE_BG),
            Ast::Slide(MEZURE_BG, (592, 304), true, false, 0.75),
            Ast::Remove(MEZURE_BG),
            Ast::Wait(0.75),
            Ast::Par(vec![
                Ast::Slide(DOOR_UPPER_L, (456, 288), true, false, 0.5),
                Ast::Slide(DOOR_UPPER_R, (472, 288), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_L, (456, 304), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_R, (472, 304), true, false, 0.5),
            ]),
            Ast::Wait(1.0),
            Ast::Remove(DOOR_UPPER_L),
            Ast::Remove(DOOR_UPPER_R),
            Ast::Remove(DOOR_LOWER_L),
            Ast::Remove(DOOR_LOWER_R),
            Ast::SetBg("column_as_icy_em_2"),
            Ast::Queue(0, 1), // Show columns.
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(ARGONY, "chars/argony", 0, (-16, 304)),
                    Ast::Slide(ARGONY, (400, 304), false, true, 2.5),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(1.0),
                    Ast::Place(MEZURE, "chars/mezure", 0, (-16, 304)),
                    Ast::Slide(MEZURE, (240, 304), false, true, 1.0),
                ]),
            ]),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Come along\n\
                       now, child."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "But...what about\n\
                       this thing?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "What about it?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "It looks broken.\n\
                       Shouldn't we fix it?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "No.  We can\n\
                       deal with it later."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "But-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                          "Time's a wastin'!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.6),
                Ast::Slide(ARGONY, (592, 304), true, false, 1.0),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(ARGONY),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Again with\n\
                       the ``$isigh.$r''"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 304), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(1.5),
            Ast::Place(RELYNG, "chars/relyng", 0, (300, 336)),
            Ast::Slide(RELYNG, (300, 320), true, true, 0.5),
            Ast::Wait(0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (300, 304), 0.5),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                      "Heh.  Well then.  Guess\n\
                       I'll just have to fix\n\
                       this thing myself."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                      "Just need to fix one\n\
                       word, and the other\n\
                       one is inevitable."),
        ]),
    ];
    (RELYNG, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Seq((0..7).map(|index| Ast::Seq(vec![
                Ast::Wait(0.075),
                Ast::Queue(1, index),
                Ast::Queue(1, 13 - index),
            ])).collect()),
            Ast::Wait(0.25),
            Ast::Seq((0..7).map(|index| Ast::Seq(vec![
                Ast::Wait(0.075),
                Ast::Queue(2, index),
                Ast::Queue(2, 13 - index),
            ])).collect()),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                      "Yeah, that sounds\n\
                       about right."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                      "I'd say there are a\n\
                       few $itoo many$r  secrets\n\
                       around here right now."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                      "...and not enough of\n\
                       them are mine."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(RELYNG, (592, 304), true, false, 1.0),
            Ast::Remove(RELYNG),
            Ast::Wait(0.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
