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

const MEZURE: i32 = 1;
const SRB: i32 = -1;
const UGRENT: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("star_crossed"),
            Ast::Queue(1, 0), // Hide word list.
            Ast::Wait(1.0),
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 272)),
            Ast::Slide(UGRENT, (200, 272), false, true, 1.0),
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Hmph.  Thought for sure\n\
                       I'd seen someone sneaking\n\
                       around in Main Engineering\n\
                       earlier, but Elinsa and I\n\
                       couldn't find anything."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "At least this security\n\
                       barrier seems to still\n\
                       be in place."),
        ]),
        Ast::Seq(vec![
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 272)),
            Ast::Slide(MEZURE, (90, 272), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Hey Ugrent!  How is the\n\
                       security sweep going?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "It's fine.  Shouldn't you\n\
                       be working on repairs?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, I'm, uh..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Yes?  Spit it out."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I...we're $iall$r  working on\n\
                       repairs, Ugrent.  And my job is\n\
                       to coordinate our efforts, right?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "I suppose\n\
                       that's true."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "So, I'm, um, checking on your\n\
                       progress.  And if you're all done\n\
                       with the sweep, then I-  I'd like to,\n\
                       you know, assign you a new task."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Hmph.  Guess you're\n\
                       growing up fast."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Well, as I said, everything\n\
                       looks fine so far.  But I'll\n\
                       want to make another round to\n\
                       double-check everything again."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Oh.  Okay.  Uh, that\n\
                       sounds good then."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, before you go, could you help\n\
                       me get through this checkpoint?\n\
                       I wanted to check the engine room."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Bad idea.  That's\n\
                       a hazardous area\n\
                       of the ship."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "$iThat's$r  a hazardous\n\
                       area of the ship?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "There are no guard rails $ianywhere$r.\n\
                       I almost fell into a pit of $ispikes$r  up\n\
                       in the power station.  People are\n\
                       routinely getting stuck behind\n\
                       doors that won't open, and the\n\
                       bridge was $ion fire$r  earlier."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "I've...been meaning to\n\
                       have some of those\n\
                       things addressed."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Look, I know a lot of that stuff\n\
                       is broken from the disaster.\n\
                       All I'm saying is, I'm willing to\n\
                       face a few hazards to help\n\
                       get the ship fixed."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "And all $iI'm$r  saying is that\n\
                       I think you should let Elinsa\n\
                       take care of the engine, and go\n\
                       work on something else.  My job\n\
                       is to keep everyone on this crew\n\
                       safe, Mezure, you included."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I appreciate your concern,\n\
                       Ugrent, but Elinsa has a lot on\n\
                       her plate, and I can't prioritize\n\
                       her work until I can assess the\n\
                       damage to all parts of the ship,\n\
                       $iengine room$r  included."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Ugh, fine.\n\
                       I suppose\n\
                       you're right."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Well, gimme a hand with\n\
                       this thing, then.  It's a\n\
                       real pain to open."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Thanks, Ugrent!\n\
                       Where do we start?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.35),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 1), // Show word list.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::NE,
                      "Wow, I stood up to Ugrent!\n\
                       I guess Argony was right."),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "These words can only be\n\
                       found in a certain order."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "What's the order?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "No idea.  I probably\n\
                       should have written\n\
                       it down somewhere."),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Queue(0, 1), // Animate final word.
            Ast::Wait(1.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "There you go.  Just\n\
                       be careful in there."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Thanks, will do!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (-16, 272), true, false, 1.0),
            Ast::Remove(UGRENT),
            Ast::Slide(MEZURE, (300, 272), true, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Whew!  Let's go see how\n\
                       bad the damage is."),
        ]),
        Ast::Seq(vec![
            Ast::Place(SRB, "chars/srb", 0, (440, 56)),
            Ast::Slide(SRB, (440, 140), false, true, 0.75),
            Ast::Par(vec![
                Ast::SetSprite(SRB, "chars/srb", 3),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Good, TalkPos::W,
                          "Iiiiiitt's everyone's favorite\n\
                           System Repair Bot!"),
                Ast::Loop(0, 0, Box::new(Ast::Seq(vec![
                    Ast::Slide(SRB, (440, 138), false, false, 0.1),
                    Ast::Slide(SRB, (440, 142), false, false, 0.2),
                    Ast::Slide(SRB, (440, 140), false, false, 0.1),
                ]))),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Hello again!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::W,
                      "The engine room is just\n\
                       ahead!  Get that fixed,\n\
                       and we're in business!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 2),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Actually, I was only going\n\
                       to inspect it for now, and\n\
                       ask Elinsa to fix it later."),
        ]),
        Ast::Par(vec![
            Ast::SetSprite(SRB, "chars/srb", 7),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::W,
                      "NO NO NO YOU NEED TO\n\
                       FIX IT RIGHT AWAY!"),
            Ast::Loop(0, 0, Box::new(Ast::Seq(vec![
                Ast::Slide(SRB, (440, 138), false, false, 0.1),
                Ast::Slide(SRB, (440, 142), false, false, 0.2),
                Ast::Slide(SRB, (440, 140), false, false, 0.1),
            ]))),
       ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 4),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Okay, okay, chill.\n\
                       I'll see what I can do."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::W,
                      "Great!  Then we'll\n\
                       be all set soon."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::SetSprite(SRB, "chars/srb", 2),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "You know, I've been\n\
                           meaning to ask you-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Par(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(SRB, TalkStyle::Good, TalkPos::W,
                              "OOPS SORRY I MUST\n\
                               BE GOING NOW!"),
                    Ast::Slide(SRB, (440, 56), true, false, 0.75),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "What a strange robot."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 272), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Queue(0, 0), // Cancel animating final word.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
