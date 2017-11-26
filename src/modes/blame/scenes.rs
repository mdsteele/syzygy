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

use elements::{Ast, Scene, TalkPos, TalkStyle};
use gui::{Resources, Sound};

// ========================================================================= //

pub const MEZURE: i32 = 1;
pub const YTTRIS: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("shift_the_blame"),
            Ast::Wait(0.5),
            Ast::Place(MEZURE, "chars/mezure", 0, (592, 288)),
            Ast::Slide(MEZURE, (496, 288), false, false, 0.5),
            Ast::Slide(MEZURE, (480, 304), false, false, 0.15),
            Ast::Slide(MEZURE, (432, 304), false, false, 0.3),
            Ast::Slide(MEZURE, (416, 320), false, false, 0.15),
            Ast::Slide(MEZURE, (325, 320), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Hmm, looks like I still\n\
                       have farther up to go."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (200, 320), true, true, 0.85),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Now, what's all\n\
                       this malarkey?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(YTTRIS, "chars/yttris", 0, (592, 288)),
            Ast::Slide(YTTRIS, (496, 288), false, false, 0.5),
            Ast::Slide(YTTRIS, (480, 304), false, false, 0.15),
            Ast::Slide(YTTRIS, (450, 304), false, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Sorry, got here as\n\
                       fast as I could."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Those are maintenance platforms!\n\
                       I've seen Elinsa use those to climb\n\
                       up; you could probably do the same."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (220, 320), true, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Wait, how did you get up\n\
                       here?  I thought you were\n\
                       afraid to climb the tree?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Huh?  Oh, I just\n\
                       took the stairs."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "You took the...what!?  Yttris,\n\
                       why didn't you tell me there\n\
                       were stairs leading up here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "You didn't ask!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I $isaid$r,  ``There's\n\
                       got to be another...''"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh$r...okay, Yttris.\n\
                       I need to get up to the\n\
                       level above here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Okay!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "So tell me, Yttris: are\n\
                       there stairs leading\n\
                       up there from here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Nah, I don't\n\
                       think so."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "$iFine.$r  I guess I'll\n\
                       just have to do this\n\
                       the hard way."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(MEZURE, (416, 320), true, false, 1.0),
                Ast::Slide(MEZURE, (432, 304), false, false, 0.15),
                Ast::Slide(MEZURE, (448, 304), false, true, 0.2),
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "I'm going to get up\n\
                           there if it kills me.\n\
                           My job demands it!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.65),
                Ast::Slide(YTTRIS, (478, 304), true, true, 0.3),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Good luck,\n\
                       Mezure!"),
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
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::Auto,
                      "Yttris, are you sure\n\
                       this is the way that\n\
                       Argony went?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Yeah.  I mean, I think\n\
                       so.  I mean, probably?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::Auto,
                      "...Great."),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::NW,
                      "I wonder if I should\n\
                       have mentioned the\n\
                       freight elevator?"),
        ]),
    ];
    (YTTRIS, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Got there!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Time to go see where\n\
                       Argony ran off to."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 64), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Wow, that Mezure\n\
                       sure did a good job!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Wait(0.2),
                Ast::Queue(0, 8),
                Ast::Queue(1, 7),
                Ast::Queue(2, 6),
                Ast::Queue(3, 5),
                Ast::Queue(4, 6),
                Ast::Queue(5, 7),
                Ast::Queue(6, 8),
                Ast::Wait(1.5),
            ]),
            Ast::Seq(vec![
                Ast::Slide(YTTRIS, (445, 304), true, true, 0.3),
                Ast::Wait(0.9),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                          "Probably would've\n\
                           been easier to just\n\
                           do this, though."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (384, 280), 0.65),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (352, 248), 0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(YTTRIS, (320, 216), 0.5),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Sound(Sound::platform_shift(1)),
                    Ast::Queue(2, 7),
                ]),
            ]),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (288, 184), 0.5),
            Ast::Sound(Sound::platform_shift(1)),
            Ast::Queue(2, 6),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (320, 152), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (352, 120), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (384, 88), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (416, 64), 0.5),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "w00t!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Slide(YTTRIS, (406, 64), true, true, 0.2),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "You know, it's a\n\
                       pretty nice view\n\
                       from up here!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "...Maybe heights\n\
                       aren't all that bad."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Sorry for doubting\n\
                       you, heights!"),
            Ast::Slide(YTTRIS, (592, 64), true, false, 0.75),
        ]),
        Ast::Seq(vec![
            Ast::Remove(YTTRIS),
            Ast::Wait(0.35),
            Ast::Queue(-2, 0), // Move all platforms to final position.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
