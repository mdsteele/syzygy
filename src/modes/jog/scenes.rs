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
const YTTRIS: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("jog_your_memory"),
            Ast::Place(ARGONY, "chars/argony", 0, (285, 144)),
            Ast::Wait(1.0),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 144)),
            Ast::Slide(YTTRIS, (100, 144), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Ah, Yttris, you're just in\n\
                       time to help me with this."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (132, 144), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Lad- Lady Argony!?  How did you\n\
                       get all the way down here already?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "I know my way around this\n\
                       ship pretty well, you know."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (110, 144), true, true, 0.35),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "But, but the bridge...if you had\n\
                       already crossed it, shouldn't it\n\
                       have been gone when I got there?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (122, 144), true, true, 0.35),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Oh no!  We're $iboth$r  going\n\
                       to be trapped down here!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "...You know you can just\n\
                       press the reset button to\n\
                       bring it back, right?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (170, 144), true, true, 0.2),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                              " $iTHERE'S A RESET \n\
                               BUTTON!?"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                              "$iOw my ears$r  yes\n\
                               Yttris, there is."),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Now, dear, I could really\n\
                       use your help fixing\n\
                       these memory banks."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (132, 144), true, true, 0.4),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Oh, right!  I came down here\n\
                       because I think there might be a\n\
                       really important clue stored here!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Well then, let's see\n\
                       if we can find it."),
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
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Pencil and paper might\n\
                       not save us this time."),
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::SE,
                      "Now if only I could\n\
                       remember which clue\n\
                       I was looking for..."),
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
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "That's the last memory bank fixed.\n\
                       Now then, what was this clue you\n\
                       said you thought was stored here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Um, I sorta forget.  I think\n\
                       maybe it was a cake recipe?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "...You came all the way down here,\n\
                       despite thinking you'd be trapped\n\
                       beyond the bridge, because you were\n\
                       looking for a cake recipe?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "I'm not actually sure, but\n\
                       it does seem plausible!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_lo()),
                Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                          "Sadly, it does."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Par(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                              "Sadly, it does-\n\
                               Hold on now,\n\
                               what's this?"),
                    Ast::Slide(ARGONY, (310, 144), true, true, 0.5),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "According to this memory bank,\n\
                       it seems that just after the initial\n\
                       disaster, primary engine repairs were\n\
                       delegated to the ``system repair bot.''"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "I didn't know we had\n\
                       a system repair bot!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "$iWe don't.$r  Something\n\
                       fishy is going on here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "All the other context seems to have\n\
                       been erased, though, so I'm not\n\
                       exactly sure what to make of this."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (164, 144), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Hmm, let me see-\n\
                       $iOh no!!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      " $iThe cake recipe got erased too!$r "),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (-16, 144), true, false, 0.5),
            Ast::Remove(YTTRIS),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Well, I'm glad we all have\n\
                       our priorities straight here."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (-16, 144), true, false, 1.0),
            Ast::Remove(ARGONY),
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
            Ast::Wait(1.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
