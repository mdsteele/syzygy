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
                       forward set.  Would you believe\n\
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
                      "Now then, let's you and me get\n\
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
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Nicely done, child.\n\
                       I appreciate the help."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "You're very welcome!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "So tell me, Mezure.  How\n\
                       is being ship's administrator\n\
                       going for you so far?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Oh, uh, it's fine, I guess."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Mostly, it consists of people\n\
                       telling me to fix things.\n\
                       But I do want to help out."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Them telling $iyou$r  to fix things?\n\
                       Mezure, dear child, I think\n\
                       perhaps you have it backwards."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "You are the $iadministrator$r, not a\n\
                       lackey.  Your job is to manage this\n\
                       motley crew and get our efforts\n\
                       organized, so we're not all\n\
                       working past each other."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "You're supposed to help us forge\n\
                       order out of this chaos, and if\n\
                       that means $iyou$r  need to give some\n\
                       orders, that's your prerogative."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "But Ugrent said-"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Ugrent is a stuffed shirt, and he\n\
                       doesn't actually outrank you, so\n\
                       don't let him push you around.\n\
                       If you need to tell $ihim$r  what to\n\
                       do too, you should feel free."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "But...but $iyou$r  ordered\n\
                       me to help you with these\n\
                       memory banks just now!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Age before beauty, dear."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (458, 128), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Now, I need to be going.\n\
                       Remember what I told you;\n\
                       get out there and whip\n\
                       this crew into shape!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(ARGONY, (592, 128), true, false, 1.0),
                Ast::Remove(ARGONY),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                          "$iSigh.$r  Yes, ma'am."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "$iMe$r  tell $ithem$r  what to do?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (360, 128), true, true, 1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Somehow, I doubt that\n\
                       Elinsa would take that\n\
                       well, coming from me."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 128), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Sound(Sound::transform_step(1)),
            Ast::Queue(1, 0),
            Ast::Queue(1, 1),
            Ast::Queue(1, 2),
            Ast::Queue(1, 3),
            Ast::Wait(0.5),
            Ast::Sound(Sound::transform_step(2)),
            Ast::Queue(1, 4),
            Ast::Queue(1, 5),
            Ast::Queue(1, 6),
            Ast::Queue(1, 7),
            Ast::Wait(0.5),
            Ast::Sound(Sound::transform_step(3)),
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
