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
use gui::Resources;

// ========================================================================= //

pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Place(1, "Elinsa", (348, 304)),
            Ast::Wait(1.0),
            Ast::Talk(1, TalkStyle::Thought, TalkPos::NW,
                      "Ugh.  Stupid piece of junk.  Why\n\
                       am I even bothering fixing this?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Place(0, "Tezure", (-16, 80)),
                Ast::Slide(0, (140, 80), true, true, 1.0),
                Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                          "Oh, hi down there!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Talk(1, TalkStyle::Thought, TalkPos::NW, "Oh, great."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$CI'm Tezure, the new administrator\n\
                       process.  Who are you?"),
        ]),
        Ast::Seq(vec![
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                      "$CElinsa, chief engineer.\n\
                       And I'm extremely busy."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$COh.  What's that thing you're\n\
                       working on?  It looks...broken."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                      "$CYes $ithank you$r, how\n\
                       observant of you."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                      "$CIt's a tri-state quantum ion\n\
                       transmission power alignment grid,\n\
                       for calibrating our attitude thrusters.\n\
                       At the moment, it happens to also be\n\
                       1) shot to hell, and therefore\n\
                       2) a complete waste of my time.\n\
                       Any other questions?"),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$CUm, any way I can help?  I'm\n\
                       supposed to be helping."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                      "$COh, $isure$r.  Why don't you\n\
                       run down to the supply depot and\n\
                       fetch me a jar of elbow grease?\n\
                       That would be a $ihuge$r help."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$CCome on, don't give me that.\n\
                       I wasn't born yesterday."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "...actually, technically I was born today."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$CAnyway, if you don't want my help,\n\
                       just say so.  There's no need to be rude."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                      "$CYou want to help?  Fine.  I'll\n\
                       tell you how you can help."),
        ]),
        Ast::Par(vec![
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                      "$CYou can fix this for me.  I've got\n\
                       more important things to be doing."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(1, (592, 304), true, false, 0.5),
            Ast::Wait(0.25),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$COkay.  Yeah!  I can totally\n\
                       figure this out."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "Er, how will I know when it's fixed?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(1, (480, 304), false, true, 0.5),
            Ast::Par(vec![
                Ast::Talk(1, TalkStyle::Normal, TalkPos::W,
                          "$C$iSigh.$r  See that big thing?\n\
                           You need to make it look like this."),
                Ast::Jump(1, (480, 304), 0.5),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Queue(0, 1),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE, "Um, okay."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$CBut...this one has a bunch\n\
                       of giant holes in it."),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Talk(1, TalkStyle::Normal, TalkPos::W,
                          "Ugh, fine, make it look like this, then."),
                Ast::Seq(vec![
                    Ast::Jump(1, (480, 304), 0.5),
                    Ast::Slide(1, (592, 304), true, false, 0.5),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Queue(0, 2),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, -3),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "Sure, how hard could this be?"),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Queue(0, 0),
            Ast::Wait(0.75),
            Ast::Queue(0, -1),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "Hey Elinsa, I did it!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(1, (432, 304), false, true, 0.5),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                      "Huh?  You fixed it??"),
        ]),
        Ast::Seq(vec![
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                      "$CI'm...impressed, actually.\n\
                       This really is a big help.\n\
                       Sorry for being a jerk earlier."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$CIt's okay.  I know you're probably\n\
                       under a lot of stress."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "$CWell, is there anything else\n\
                       I can try to help fix?"),
        ]),
        Ast::Seq(vec![
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE,
                      "Er, like, maybe that support beam?"),
        ]),
        Ast::Seq(vec![
            Ast::Jump(1, (432, 416), 0.75),
            Ast::Wait(0.25),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW, "%#$$@&!!"),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
