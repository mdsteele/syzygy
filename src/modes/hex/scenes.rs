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

use std::f64::consts::FRAC_PI_3;

use elements::{Ast, Scene, TalkPos, TalkStyle};
use gui::{Resources, Sound};

// ========================================================================= //

const MEZURE: i32 = 1;
const SYSTEM: i32 = 0;
const YTTRIS: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("hex_spangled"),
            Ast::Place(SYSTEM, "chars/system", 0, (112, 240)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NE,
                      "Error: Factory FAB unit\n\
                       corruption detected."),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 80)),
                    Ast::Slide(YTTRIS, (68, 80), false, false, 0.5),
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(YTTRIS, (128, 112), 0.5),
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(YTTRIS, (192, 96), 0.5),
                    Ast::Slide(YTTRIS, (370, 96), false, false, 0.75),
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(YTTRIS, (432, 112), 0.5),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(1.5),
                    Ast::Place(MEZURE, "chars/mezure", 0, (-16, 80)),
                    Ast::Slide(MEZURE, (68, 80), false, true, 0.75),
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(MEZURE, (120, 112), 0.5),
                ]),
            ]),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Aw, drat.  The\n\
                       fabricator\n\
                       is busted."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "The what?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "You know, the fabricator.\n\
                       This thing.  We use it for\n\
                       making spare parts."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                          "Ah well, Elinsa will\n\
                           fix it eventually."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Slide(YTTRIS, (480, 112), true, false, 0.5),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (508, 80), 0.5),
            ]),
            Ast::Seq(vec![
                Ast::Wait(1.0),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E, "Wait!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW, "Huh?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Well, making new spare\n\
                       sounds pretty important\n\
                       for repairs!  Shouldn't\n\
                       we fix this right now?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Oh.  I mean, I guess so.\n\
                       I don't really know how."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Me neither.  But maybe we\n\
                       can work on it together?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "I mean, I'm supposed to be\n\
                       helping to coordinate repairs,\n\
                       and...well, it seems like we\n\
                       should get this fabricator thing\n\
                       fixed right away.  You know, to\n\
                       make the rest easier."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (456, 112), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Oh!  Okay, sure then, brave\n\
                       administrator!  Lead the way."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Er, right.  Um, any idea\n\
                       what this thing looks like\n\
                       when it's $inot$r  broken?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (456, 112), 0.25),
            Ast::Queue(0, 1), // Show solution.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Oh sure, like this!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Great!  Let's see\n\
                       what we can do."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, -2), // Finish solution animation.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "I think this will be\n\
                       easiest if we work row by\n\
                       row, from top to bottom."),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_system_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NE,
                      "Warning: Factory FAB\n\
                       unit is still offline."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Yes, $ithank you$r  for that\n\
                       update.  We're working on it."),
        ]),
    ];
    (SYSTEM, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                          "Here I go!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E, "Huh?"),
            ]),
            Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (356, 182), 0.75),
                Ast::Seq((0..18).map(|index| {
                    let theta = 0.25 * FRAC_PI_3 * index as f64;
                    let (sin, cos) = theta.sin_cos();
                    let x = 320 + (32.0 * cos).round() as i32;
                    let y = 176 + (32.0 * sin).round() as i32 + 8;
                    Ast::Seq(vec![
                        Ast::Wait(0.05),
                        Ast::SetPos(YTTRIS, (x, y)),
                        Ast::Queue(2, index - 23),
                    ])
                }).collect()),
                Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Sound(Sound::small_jump()),
                        Ast::Jump(YTTRIS, (456, 112), 0.75),
                    ]),
                    Ast::Seq(vec![
                        Ast::Wait(0.25),
                        Ast::Sound(Sound::talk_hi()),
                        Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::W,
                                  "Wheee!"),
                    ]),
                    Ast::Seq((18..24).map(|index| {
                        Ast::Seq(vec![
                            Ast::Wait(0.041 * (index / 2 - 8) as f64),
                            Ast::Queue(2, index - 23),
                        ])
                    }).collect()),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::W,
                      "That was fun!"),
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
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NE,
                      "FAB unit online\n\
                       and operational."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Hooray, we did it!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "There, see?  Just look at\n\
                       what we can accomplish\n\
                       when we work together!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (430, 112), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "That's very inspiring!\n\
                       You should put that\n\
                       on a poster!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (472, 112), true, false, 0.4),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (512, 80), 0.5),
            Ast::Slide(YTTRIS, (592, 80), false, false, 0.5),
            Ast::Remove(YTTRIS),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "Is she...mocking me?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 2),
            Ast::Wait(0.1),
            Ast::Queue(1, 3),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "No, I said ``ME.''\n\
                       ``Mocking ME.''"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "I think you spelled it\n\
                       backwards down in there."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 0),
            Ast::Wait(0.1),
            Ast::Queue(1, 1),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "No, ``IN.''  ``Down IN there.''  I think\n\
                       you have the wrong vowel?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 4),
            Ast::Wait(0.1),
            Ast::Queue(1, 5),
            Ast::Wait(0.1),
            Ast::Queue(1, 6),
            Ast::Wait(0.1),
            Ast::Queue(1, 7),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "I have no idea what\n\
                       that even means."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (144, 112), true, false, 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (192, 96), 0.5),
            Ast::Slide(MEZURE, (320, 96), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Anyway, uh, hopefully this\n\
                       thing is working now?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (368, 96), true, false, 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (440, 112), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (512, 80), 0.5),
            Ast::Slide(MEZURE, (592, 80), false, false, 0.5),
            Ast::Remove(MEZURE),
            Ast::Wait(0.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
