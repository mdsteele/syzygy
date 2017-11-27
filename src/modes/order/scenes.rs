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

pub const ARGONY: i32 = 1;
pub const MEZURE: i32 = 2;
pub const RELYNG: i32 = -1;
pub const YTTRIS: i32 = 3;

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
                       meet-and-greet or somesuch\n\
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
            Ast::Slide(MEZURE, (592, 304), true, false, 1.0),
            Ast::Remove(MEZURE),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
