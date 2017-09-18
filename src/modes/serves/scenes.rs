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
const MEZURE: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("if_memory_serves"),
            Ast::Place(ARGONY, "chars/argony", 0, (438, 128)),
            Ast::Wait(0.75),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 128)),
            Ast::Slide(MEZURE, (198, 128), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Ah, you're just\n\
                       in time, child."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Hello again, Ms. Argony.\n\
                       What am I just in time for?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "For helping me fix\n\
                       these memory banks."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Ugrent and I already fixed the\n\
                       forward banks.  Would you believe\n\
                       that these ones are broken too?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Considering what I've experienced\n\
                       with $ievery part$r  of this ship so far,\n\
                       I'd be shocked if they weren't."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Yes, well."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Admittedly, this ship...has not\n\
                       always been as robust over the\n\
                       years as she could have been."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "But she hasn't given up yet,\n\
                       so neither should we.  You\n\
                       should treat her with respect."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Oh.  Okay, um, I'm sorry for\n\
                       doubting you, Ms. Ship."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Now then, let's you and I get\n\
                       these memory banks reset."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (260, 128), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Sure, I'd be happy to help.\n\
                       What's wrong with them?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Pretty much everything."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Unlike the last batch, this one's\n\
                       reference counter is totally shot.\n\
                       So we're on our own for knowing\n\
                       when to deallocate each block."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "I'm sorry, I don't-\n\
                       What does that-\n\
                       ...What?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "$iSigh.$r  Don't they teach\n\
                       young ones about low-level\n\
                       memory allocation anymore?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Tell you what, just help\n\
                       me remember where we\n\
                       put each block."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1), // Show next shape.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_argony_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "The sequence of allocations and\n\
                       deallocations is the same every\n\
                       time, but the tiles don't always\n\
                       turn gray in the same order."),
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::SE,
                      "Is it cheating to just\n\
                       use a pencil and paper?"),
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
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE, "Much better."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (-16, 128), true, false, 0.5),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Queue(1, 0),
            Ast::Queue(1, 1),
            Ast::Queue(1, 2),
            Ast::Queue(1, 3),
            Ast::Wait(0.5),
            Ast::Queue(1, 4),
            Ast::Queue(1, 5),
            Ast::Queue(1, 6),
            Ast::Queue(1, 7),
            Ast::Wait(0.5),
            Ast::Queue(1, 8),
            Ast::Queue(1, 9),
            Ast::Queue(1, 10),
            Ast::Queue(1, 11),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
