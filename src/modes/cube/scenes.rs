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
const BRIDGE: i32 = -1;
const ELINSA: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("cube_tangle"),
            Ast::Place(BRIDGE, "wrecked/bridge", 0, (288, 320)),
            Ast::Wait(1.0),
            Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 320)),
            Ast::Slide(ELINSA, (80, 320), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Oof.  Finally made\n\
                       it up out of there."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ELINSA, (144, 304), 0.5),
            Ast::Slide(ELINSA, (216, 304), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Now then, let's see what's..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "...oh, for crying out loud,\n\
                       don't tell me this #@*$$&\n\
                       thing is broken too!"),
        ]),
        Ast::Seq(vec![
            Ast::Place(ARGONY, "chars/argony", 0, (592, 226)),
            Ast::Slide(ARGONY, (462, 224), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Sorry to be the bearer of bad\n\
                       news, Elinsa, but yes, this\n\
                       $irotten$r  thing is broken too."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (202, 304), false, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Oh!  I-  I'm sorry Argony, I\n\
                       didn't know you were there."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Sorry for what?\n\
                       What difference\n\
                       does that make?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Well, I just, uh, I\n\
                       wouldn't have said..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Tut-tut.  It doesn't offend me\n\
                       when you use cuss words in\n\
                       front of me, Elinsa."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "It doesn't?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "No.  It offends me that you use\n\
                       them, regardless of where I am.\n\
                       Watch your language, miss."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh.$r  Yes, ma'am."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Aw, buck up.  When\n\
                       you're as old as I am,\n\
                       you can be cranky too."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (440, 224), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "In the meantime, I know you\n\
                       prefer to work on your own,\n\
                       so why don't I get out of your\n\
                       hair while you apply your\n\
                       considerable talents to\n\
                       getting this gyro fixed."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (592, 226), true, false, 1.0),
            Ast::Remove(ARGONY),
            Ast::Wait(0.5),
            Ast::Slide(ELINSA, (288, 306), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Uh, right.  Let's\n\
                       get this thing..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Um."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Slide(ELINSA, (320, 305), true, true, 1.0),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Right.  Let's get\n\
                       this thing fixed."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1), // Animate solution display.
            Ast::Wait(1.0),
            Ast::Queue(0, -2), // Finish solution animation.
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
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NW,
                      "Let's see...all the\n\
                       cubes are the same...")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NW,
                      "I think this is actually\n\
                       easier than it looks.")
        ]),
    ];
    (ELINSA, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Queue(0, 2), // Animate solution display.
            Ast::Wait(0.5),
            Ast::Queue(1, 0), // Hide sides of cubes.
            Ast::Wait(0.5),
            Ast::Queue(0, -3), // Finish solution animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Ha!  Ain't no problem\n\
                       stands a chance against\n\
                       an engineer!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.25),
            Ast::Sound(Sound::beep()),
            Ast::Queue(2, 0), // Show letter Y.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW, "Huh?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW, "Why, you ask?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Because I'm just that\n\
                       good, aren't I?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.25),
            Ast::Sound(Sound::beep()),
            Ast::Queue(2, 5), // Show letter R.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "No, not ``are.''\n\
                       I said ``aren't.''"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (380, 304), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Wait, why am I arguing\n\
                       with a machine?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (592, 304), true, false, 1.0),
            Ast::Wait(1.0),
            Ast::Sound(Sound::beep()),
            Ast::Queue(2, 1), // Show remaining letters.
            Ast::Wait(0.1),
            Ast::Queue(2, 2),
            Ast::Wait(0.1),
            Ast::Queue(2, 3),
            Ast::Wait(0.1),
            Ast::Queue(2, 4),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
