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

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("memory_lane"),
            Ast::Place(ARGONY, "chars/argony", 0, (404, 272)),
            Ast::Wait(0.75),
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 272)),
            Ast::Slide(UGRENT, (180, 272), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Well, look who's here?\n\
                       No doubt inspecting every\n\
                       nook and cranny while the\n\
                       rest of us work on repairs?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Good morning, Lady Argony.\n\
                       I am indeed on my security\n\
                       rounds, but I would be more\n\
                       than happy to assist you."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "I'm just teasing, Ugrent.\n\
                       And why do you always insist\n\
                       on calling me ``Lady?''  I've\n\
                       told you a hundred times\n\
                       it's unnecessary."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "It's a title befitting\n\
                       of your station."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Yeah, that and a dime will\n\
                       get you two nickels.  I don't\n\
                       have a ``station,'' Ugrent.\n\
                       I've just been around for\n\
                       a long time, that's all."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Be that as it may.\n\
                       May I ask what you're\n\
                       working on here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "$iSigh.$r  Always so formal\n\
                       with me, always so short\n\
                       with everyone else."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Since you asked: these memory\n\
                       banks are shot.  So I'm putting\n\
                       my ``station'' to good use by\n\
                       reinitializing them."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "You...you're re-entering\n\
                       all the data $iby heart!?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "What?  No, dummy!  I'm just\n\
                       fixing the memory allocator.\n\
                       I doubt anyone else here\n\
                       remembers how it works."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "But I'll admit that my\n\
                       short-term memory isn't\n\
                       what it once was, so be a\n\
                       gentleman and help me out."),
        ]),
        Ast::Queue(0, 1), // Show next shape and prompt.
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_argony_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "It's not just remembering\n\
                       where the symbols are.  We\n\
                       have to learn the sequence\n\
                       so we can plan ahead where\n\
                       each shape will fit."),
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(UGRENT, TalkStyle::Thought, TalkPos::NE,
                      "How $idoes$r this\n\
                       thing work?"),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE, "Much better."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (-16, 272), true, false, 0.5),
            Ast::Remove(UGRENT),
            Ast::Slide(ARGONY, (592, 272), true, false, 0.5),
            Ast::Remove(ARGONY),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
