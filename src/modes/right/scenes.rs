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

const ELINSA: i32 = 2;
const YTTRIS: i32 = 1;

const DOOR_UPPER_L: i32 = -3;
const DOOR_UPPER_R: i32 = -4;
const DOOR_LOWER_L: i32 = -5;
const DOOR_LOWER_R: i32 = -6;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("the_ice_is_right"),
            Ast::Place(DOOR_UPPER_L, "tiles/caution_walls", 12, (504, 304)),
            Ast::Place(DOOR_UPPER_R, "tiles/caution_walls", 13, (520, 304)),
            Ast::Place(DOOR_LOWER_L, "tiles/caution_walls", 10, (504, 320)),
            Ast::Place(DOOR_LOWER_R, "tiles/caution_walls", 11, (520, 320)),
            Ast::Place(YTTRIS, "chars/yttris", 0, (440, 320)),
            Ast::Wait(0.5),
            Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 272)),
            Ast::Slide(ELINSA, (64, 272), false, false, 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ELINSA, (96, 288), 0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(YTTRIS, (440, 320), 0.5),
                ]),
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                              "Elinsa!  Oh, thank goodness\n\
                               you're here!  I had almost\n\
                               given up hope!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh.$r  So much for\n\
                       peace and quiet."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ELINSA, (128, 304), 0.5),
            Ast::Slide(ELINSA, (220, 304), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Let me guess, Yttris: you're stuck\n\
                       in here and the door won't open."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (420, 320), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "No, it's worse than that!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "We're $iboth$r  stuck in here\n\
                       and the door won't open!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Yes.  That is worse."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Also, it's $icold$r  in here!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Oh, is it?\n\
                       $iI hadn't noticed."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "What are you even\n\
                       doing in here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Hmm.  Let's see...I was on my\n\
                       way down to the engine room,\n\
                       thinking about polar bears..."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                          "...when suddenly I realized that we\n\
                           should all have $icake$r  to celebrate\n\
                           whenever we get the ship fixed, and\n\
                           so the next thing I knew, I was-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.4),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::E,
                          "Okay, stop.  That tells me\n\
                           everything I need to know."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Let's just get this storage\n\
                       section fixed, and worry\n\
                       about cake later."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                          "Yippee!"),
            ]),
            Ast::Loop(0, 2, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (420, 320), 0.5),
            ]))),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::E,
                          "Quietly, please."),
            ]),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NE,
                      "Hmph.  Maybe I should have just\n\
                       left Mezure and Yttris in here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NE,
                      "Just think how much\n\
                       work I could get done!"),
        ]),
    ];
    (ELINSA, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::NW,
                      "I hope we can still\n\
                       get cake later!"),
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
            Ast::Par(vec![
                Ast::Slide(DOOR_UPPER_L, (504, 288), true, false, 0.5),
                Ast::Slide(DOOR_UPPER_R, (520, 288), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_L, (504, 336), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_R, (520, 336), true, false, 0.5),
            ]),
            Ast::Slide(ELINSA, (592, 320), true, false, 1.0),
            Ast::Remove(ELINSA),
            Ast::Slide(YTTRIS, (592, 320), true, false, 0.75),
            Ast::Remove(YTTRIS),
            Ast::Par(vec![
                Ast::Slide(DOOR_UPPER_L, (504, 304), true, false, 0.5),
                Ast::Slide(DOOR_UPPER_R, (520, 304), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_L, (504, 320), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_R, (520, 320), true, false, 0.5),
            ]),
            Ast::Wait(1.0),
            Ast::Queue(1, 0),
            Ast::Wait(0.1),
            Ast::Queue(1, 1),
            Ast::Queue(1, 2),
            Ast::Queue(1, 3),
            Ast::Wait(0.1),
            Ast::Queue(1, 4),
            Ast::Queue(1, 5),
            Ast::Queue(1, 6),
            Ast::Wait(0.1),
            Ast::Queue(1, 7),
            Ast::Queue(1, 8),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
