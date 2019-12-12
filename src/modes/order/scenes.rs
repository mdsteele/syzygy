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

pub const ARGONY: i32 = 3;
pub const MEZURE: i32 = 1;
pub const RELYNG: i32 = -1;
pub const YTTRIS: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("point_of_order"),
            Ast::Place(ARGONY, "chars/argony", 0, (242, 320)),
            Ast::Wait(1.0),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 336)),
            Ast::Slide(MEZURE, (64, 336), false, false, 0.4),
            Ast::Slide(MEZURE, (80, 320), false, false, 0.15),
            Ast::Slide(MEZURE, (185, 320), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Ah, there you are,\n\
                       child.  Did you get\n\
                       lost on the way here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I think Yttris took me\n\
                       on the scenic route."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Yes, that sounds\n\
                       like something\n\
                       she would do."),
        ]),
        Ast::Seq(vec![
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 336)),
            Ast::Slide(YTTRIS, (64, 336), false, false, 0.4),
            Ast::Slide(YTTRIS, (80, 320), false, false, 0.15),
            Ast::Slide(YTTRIS, (108, 320), false, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Hi everyone!\n\
                       Sorry I'm late."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "So, um, what\n\
                       are we all doing\n\
                       up here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "We're figuring out\n\
                       what's going on.  Just\n\
                       as soon as he gets here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "As soon as $iwho$r\n\
                       gets here?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Place(RELYNG, "chars/relyng", 5, (320, 304)),
                Ast::Slide(RELYNG, (306, 304), true, true, 0.35),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                          "That would be me."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.45),
                Ast::Par(vec![
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(MEZURE, (185, 320), 0.25),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E, "Augh!"),
                ]),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::SetSprite(RELYNG, "chars/relyng", 0),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(RELYNG, (285, 309), 0.3),
            ]),
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                          "Mezure, meet Relyng.\n\
                           Relyng, Mezure."),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "So you're\n\
                       the one!"),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                      "So you're\n\
                       the one..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Wait, Mezure,\n\
                       you haven't met\n\
                       Relyng yet?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "This is that guy who\n\
                       keeps sneaking around\n\
                       and acting suspicious!\n\
                       Who is he, anyway?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Ha!  I could ask the\n\
                       same about you, kid."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Relyng is our...detective, you\n\
                       might say.  He keeps an eye\n\
                       on things and finds things out."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "Relyng, Mezure is our\n\
                       new administrator, here\n\
                       to help with the disaster."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "You'll pardon me\n\
                       if I have my doubts."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "What!?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh$r...yes, that\n\
                       is your nature."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Why don't we all\n\
                       have a look at the\n\
                       personnel file here so\n\
                       we can sort this out?"),
        ]),
        Ast::Queue(0, 1), // Show tiles.
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
                      "Ship's personnel are\n\
                       listed in the last row\n\
                       here.  We'll have to fix\n\
                       the other rows first."),
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::NE,
                      "Is this guy really\n\
                       part of the crew?"),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(RELYNG, TalkStyle::Thought, TalkPos::NW,
                      "Is this kid really\n\
                       part of the crew?"),
        ]),
    ];
    (RELYNG, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "How did these two\n\
                       not meet yet?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Under normal circumstances,\n\
                       we probably would've had a\n\
                       meet-and-greet or some such\n\
                       nonsense for Mezure here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "With cake!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Yes, well, I'm afraid\n\
                       our little disaster\n\
                       precluded cake."),
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
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Huh.  There he is,\n\
                       listed right between\n\
                       me and Ugrent."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "And there $iyou$r  are,\n\
                       listed right between\n\
                       Elinsa and me."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "Well then, now that that's\n\
                       settled, we can get down to\n\
                       business.  What have you\n\
                       found out so far, Relyng?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "I'd prefer not to share\n\
                       confidential information\n\
                       in such an open forum."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "Oh, the blazes with that,\n\
                       Relyng.  We've $ijust verified$r\n\
                       that everyone here is a member\n\
                       of the crew.  Whatever you can\n\
                       tell me, you can tell them."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "Now, spit it out."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "$iSigh$r...very\n\
                       well, madam."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "As you may have already\n\
                       suspected, there is clear\n\
                       evidence of sabotage."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Someone's trying to\n\
                           stop our mission?"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Oh no!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "It doesn't appear\n\
                       to be that simple."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Lots of systems were\n\
                       damaged, but most of it\n\
                       is easily repairable."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "In fact, a lot of the repairs\n\
                       are already done, and the\n\
                       rest won't take long.  We\n\
                       should be shipshape soon."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "So it seems that either\n\
                       our saboteur is lousy at\n\
                       their job, or...something\n\
                       deeper is going on."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "I see."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "You'll keep\n\
                       investigating,\n\
                       won't you?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Oh, you'd better\n\
                       believe I will."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "Good."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(ARGONY, (80, 320), true, false, 1.0),
                Ast::Slide(ARGONY, (64, 336), false, false, 0.15),
                Ast::Slide(ARGONY, (-16, 336), false, false, 0.5),
                Ast::Remove(ARGONY),
            ]),
            Ast::Seq(vec![
                Ast::Wait(1.3),
                Ast::Slide(YTTRIS, (80, 320), true, false, 0.5),
                Ast::Slide(YTTRIS, (64, 336), false, false, 0.15),
                Ast::Slide(YTTRIS, (-16, 336), false, false, 0.4),
                Ast::Remove(YTTRIS),
                Ast::Wait(0.75),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "So why did you doubt I\n\
                           was on the crew?  I've\n\
                           been working to fix the\n\
                           ship this whole time!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Don't take it\n\
                       personally, kid."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (306, 304), 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "It's my job to\n\
                       doubt $ieverything."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(RELYNG, "chars/relyng", 5),
            Ast::Slide(RELYNG, (320, 304), true, true, 0.35),
            Ast::Remove(RELYNG),
            Ast::Wait(1.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Why is everyone on\n\
                       this ship so $iweird?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (80, 320), true, false, 0.75),
            Ast::Slide(MEZURE, (64, 336), false, false, 0.15),
            Ast::Slide(MEZURE, (-16, 336), false, false, 0.4),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 15),
            Ast::Queue(1, 4),
            Ast::Queue(1, 5),
            Ast::Queue(1, 17),
            Ast::Queue(1, 3),
            Ast::Queue(1, 1),
            Ast::Queue(1, 0),
            Ast::Queue(1, 13),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
