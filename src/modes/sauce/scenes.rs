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

const ARGONY: i32 = 1;
const UGRENT: i32 = 2;
const YTTRIS: i32 = 3;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("cross_sauce"),
            Ast::Place(UGRENT, "chars/ugrent", 0, (436, 240)),
            Ast::Queue(1, 1),  // Display "RHYME TIME!".
            Ast::Wait(1.0),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 240)),
            Ast::Slide(YTTRIS, (188, 240), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Ooh, rhymes!  Are\n\
                       we doing poetry?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 0),  // Clear display.
            Ast::Wait(0.5),
            Ast::Queue(1, 2),  // Display "THYME CLIMB".
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "I'm inspecting this\n\
                       security checkpoint.\n\
                       It's delicate work."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Can I help?\n\
                       I love poetry!"),
            Ast::Loop(0, 2, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (188, 240), 0.5),
            ]))),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "No!  This isn't\n\
                       poetry, Yttris!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Queue(1, 0),  // Clear display.
                Ast::Wait(0.5),
                Ast::Queue(1, 3),  // Display "SUBLIME ENZYME".
            ]),
            Ast::Seq(vec![
                Ast::Place(ARGONY, "chars/argony", 0, (-16, 240)),
                Ast::Slide(ARGONY, (144, 240), false, true, 1.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                          "Oh, let her\n\
                           help, Ugrent."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Slide(YTTRIS, (244, 240), true, true, 0.75),
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Yeah!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "With respect, Lady Argony,\n\
                       I am $itrying$r  to keep us all\n\
                       safe, and this inspection\n\
                       needs to be done $ijust so."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Doing things ``just so'' is not\n\
                       exactly Yttris' strong suit."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "All facts I am well aware\n\
                       of, Ugrent.  And you don't\n\
                       need to call me ``Lady.''"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "But in my experience, having an\n\
                       extra set of eyes and a fresh\n\
                       perspective on a problem usually\n\
                       leads to a better result."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "$L* Plus, in case you *\n      * haven't heard, *\n\
                       * I'm great at finding *\n      * rhyming words! *"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "...Please don't undercut\n\
                       my argument, Yttris."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 0),  // Clear display.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Fine, if you insist..."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1),  // Show clues.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Queue(0, 0),  // Hide clues.
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW, "Much better."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (592, 240), true, false, 0.75),
            Ast::Remove(UGRENT),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
