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

const MEZURE: i32 = 1;
const SRB: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("system_failure"),
            Ast::Wait(0.5),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 192)),
            Ast::Slide(MEZURE, (120, 192), false, true, 0.75),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "So this must be the\n\
                       system status console."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "That...is a lot of red."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "...and I'm guessing\n\
                       that red is bad."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "How are we ever going\n\
                       to repair all of this?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(SRB, "chars/srb", 0, (592, 192)),
            Ast::Slide(SRB, (448, 192), false, true, 0.75),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::NW,
                      "Have no fear,\n\
                       for I am here!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Who're you?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(SRB, (448, 155), true, true, 0.5),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::NW,
                      "Well, my full designation is\n\
                       System Repair Bot TX32."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Slide(SRB, (435, 162), true, true, 0.5),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::NW,
                      "But you can call me..."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::SetSprite(SRB, "chars/srb", 3),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Good, TalkPos::NW,
                          "System Repair Bot!"),
            ]),
            Ast::Loop(0, 0, Box::new(Ast::Seq(vec![
                Ast::Slide(SRB, (435, 160), false, false, 0.1),
                Ast::Slide(SRB, (435, 164), false, false, 0.2),
                Ast::Slide(SRB, (435, 162), false, false, 0.1),
            ]))),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "That's...kind of a mouthful."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 2),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Are you sure we couldn't\n\
                       shorten that to, like,\n\
                       ``SysBot'' or something?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 4),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Or maybe ``SysRep?''"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Or ``Syssy?''"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.15),
                Ast::SetSprite(SRB, "chars/srb", 7),
                Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Sound(Sound::talk_lo()),
                        Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                                  "No!  It's ``System\n\
                                   Repair Bot!''"),
                    ]),
                    Ast::Loop(0, 0, Box::new(Ast::Seq(vec![
                        Ast::Slide(SRB, (435, 160), false, false, 0.1),
                        Ast::Slide(SRB, (435, 164), false, false, 0.2),
                        Ast::Slide(SRB, (435, 162), false, false, 0.1),
                    ]))),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 2),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Okay, okay, ``System\n\
                       Repair Bot'' it is."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Slide(SRB, (416, 96), true, true, 0.5),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "And I can help!  See all\n\
                       these red indicators?  These\n\
                       represent ship subsystems\n\
                       that are broken."),
        ]),
        Ast::Seq(vec![
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "Engines, external sensors,\n\
                       navigational control..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Slide(SRB, (416, 144), true, true, 0.5),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "To get the ship moving again,\n\
                       you'll need to travel around\n\
                       the ship, repairing these systems.\n\
                       Focus on the engines first, so\n\
                       we can get to our destination!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "What's our destination?  And\n\
                       what about the external\n\
                       sensors and that other stuff?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "Oh don't worry about that!\n\
                       The most important thing is\n\
                       that we get moving again."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, if you say so.  I guess\n\
                       I'd better get started, then."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "Great!  I might pop up from\n\
                       time to time to help you out."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Sounds great.  See you\n\
                       later, System Repair Bot!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(MEZURE, (-16, 192), true, false, 0.75),
                Ast::Remove(MEZURE),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::SetSprite(SRB, "chars/srb", 1),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                          "Later!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Wait(0.75),
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Wait(0.75),
            Ast::SetSprite(SRB, "chars/srb", 6),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::SW,
                      "Heh heh heh..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Slide(SRB, (592, 144), true, false, 0.75),
            Ast::Remove(SRB),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
