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
            Ast::SetBg("black_and_blue_1"),
            Ast::Wait(1.0),
            Ast::Place(MEZURE, "chars/mezure", 0, (336, 400)),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (320, 368), 0.75),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (288, 336), 0.5),
            Ast::Wait(0.25),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (256, 304), 0.5),
            Ast::Slide(MEZURE, (230, 304), false, true, 0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "$iHuff...puff..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "$iWhew$r...that Argony...she's\n\
                       suprisingly fast on her\n\
                       feet for someone her age."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I think we're...back\n\
                       in the bio-dome?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (304, 306), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Now, which way\n\
                       did she go?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 304)),
            Ast::Slide(YTTRIS, (90, 304), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Oh, hey there, Mezure!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (240, 304), true, true, 0.6),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Hi, Yttris.  Did you\n\
                       see Argony pass through\n\
                       here just now?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "I think I saw her heading\n\
                       for the upper levels?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Thanks.  Which way do\n\
                       I go to get up there?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Hmm...good question!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "I think I know a\n\
                       way.  Follow me!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (-16, 304), true, false, 0.6),
            Ast::Remove(YTTRIS),
            Ast::Slide(MEZURE, (-16, 304), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(0.5),
            Ast::SetBg("black_and_blue_2"),
            Ast::Queue(0, 1), // Show tree.
            Ast::Wait(0.25),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(YTTRIS, "chars/yttris", 0, (592, 304)),
                    Ast::Slide(YTTRIS, (560, 304), false, false, 0.1),
                    Ast::Slide(YTTRIS, (544, 288), false, false, 0.1),
                    Ast::Slide(YTTRIS, (464, 288), false, false, 0.4),
                    Ast::Slide(YTTRIS, (448, 304), false, false, 0.1),
                    Ast::Slide(YTTRIS, (416, 304), false, false, 0.15),
                    Ast::Slide(YTTRIS, (400, 320), false, false, 0.15),
                    Ast::Slide(YTTRIS, (218, 320), false, true, 1.2),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Place(MEZURE, "chars/mezure", 0, (592, 304)),
                    Ast::Slide(MEZURE, (560, 304), false, false, 0.12),
                    Ast::Slide(MEZURE, (544, 288), false, false, 0.12),
                    Ast::Slide(MEZURE, (464, 288), false, false, 0.5),
                    Ast::Slide(MEZURE, (448, 304), false, false, 0.15),
                    Ast::Slide(MEZURE, (432, 304), false, true, 0.15),
                ]),
            ]),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "We just need to reach\n\
                       that girder up above."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "We can climb the\n\
                       tree to get there!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "I hate to break it to\n\
                       you, Yttris, but..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "...the tree isn't\n\
                       tall enough."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Oh.  Huh."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "I guess we could wait for\n\
                       it to grow some more?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Yttris!  I don't have time\n\
                       to stand here and wait for\n\
                       this tree to double in age!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "There's $igot$r  to be\n\
                       another way up there."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "You don't give up\n\
                       easily, do you?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Well then!  Time to put my\n\
                       horticultural skills to good\n\
                       use.  We'll just prune this tree\n\
                       to shape so we can climb it!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "How is pruning a tree going\n\
                       to make it any taller?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "This is no ordinary tree.\n\
                       Watch and learn!"),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Slide(MEZURE, (592, 320), true, false, 1.0),
            Ast::Remove(MEZURE),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
