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

const ARGONY: i32 = 0;
const YTTRIS: i32 = 1;

#[cfg_attr(rustfmt, rustfmt_skip)]
const PLATFORMS: &[(i32, usize, i32, i32, i32)] = &[
    (-1, 0, 384, 416, 128),
    (-2, 1, 496, 464, 160),
    (-3, 0, 384, 416, 192),
    (-4, 1, 496, 464, 224),
    (-5, 0, 384, 416, 256),
    (-6, 1, 496, 464, 288),
];

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("point_of_view"),
            Ast::Wait(0.5),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 304)),
            Ast::Slide(YTTRIS, (216, 304), false, true, 1.0),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Huh?  Where'd Elinsa go?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (420, 304), true, true, 1.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Did I take a wrong\n\
                       turn somewhere?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (456, 304), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "This shaft looks,\n\
                       uh...kind of tall."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (456, 304), 0.55),
            Ast::Wait(1.0),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (456, 304), 0.7),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW, "Hmm."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Slide(YTTRIS, (456, 230), false, true, 0.5),
            Ast::Wait(0.5),
            Ast::Slide(YTTRIS, (456, 304), true, false, 0.5),
            Ast::Sound(Sound::character_collision()),
            Ast::Shake(4),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Darn.  I don't think I\n\
                       can jump that high."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (380, 304), true, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "I guess this is\n\
                       just a dead end!"),
        ]),
        Ast::Seq(vec![
            Ast::Place(ARGONY, "chars/argony", 0, (-16, 304)),
            Ast::Slide(ARGONY, (136, 304), false, true, 1.0),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                              "Well, I wouldn't jump\n\
                               to conclusions, Yttris."),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                              "Oh!  Lady Argony!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "Now you cut that out.  Just\n\
                       because Ugrent insists\n\
                       on calling me that doesn't\n\
                       mean you should too."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "As I was about to say,\n\
                       I've been on this ship a\n\
                       long time, and places like\n\
                       this are never a dead end."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "As I recall, that shaft has\n\
                       some retractable platforms.\n\
                       We just need to activate them."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                          "But...but what if they're\n\
                           broken because of the disaster?\n\
                           We'll be trapped!!"),
            ]),
            Ast::Loop(0, 0, Box::new(Ast::Seq(vec![
                Ast::Slide(YTTRIS, (375, 304), false, false, 0.1),
                Ast::Wait(0.2),
                Ast::Slide(YTTRIS, (385, 304), false, false, 0.2),
                Ast::Wait(0.2),
                Ast::Slide(YTTRIS, (380, 304), false, false, 0.1),
            ]))),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                          "Well, Yttris.  If this disaster of ours\n\
                           broke the platforms, then - here's\n\
                           a novel idea - we should $ifix them."),
            ]),
            Ast::Loop(0, 0, Box::new(Ast::Seq(vec![
                Ast::Slide(YTTRIS, (375, 304), false, false, 0.1),
                Ast::Wait(0.2),
                Ast::Slide(YTTRIS, (385, 304), false, false, 0.2),
                Ast::Wait(0.2),
                Ast::Slide(YTTRIS, (380, 304), false, false, 0.1),
            ]))),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.2),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW, "Oh!  Right!"),
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
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "You should concentrate\n\
                       on the red pieces first.")
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::NW,
                      "I never really looked\n\
                       at it that way before!")
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
            Ast::Queue(0, 1), // Show first letter.
            Ast::Wait(1.0),
            Ast::Par(PLATFORMS.iter().enumerate().map(
                |(index, &(slot, sprite, x0, x1, y))| {
                    Ast::Seq(vec![
                        Ast::Wait(0.1 * index as f64),
                        Ast::Place(slot, "point/platforms", sprite, (x0, y)),
                        Ast::Sound(Sound::platform_shift(1)),
                        Ast::Slide(slot, (x1, y), false, true, 0.5),
                    ])
                }).collect()),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "There you are.  Together\n\
                       we are better than the sum\n\
                       of our parts, or whatever.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Gee, thanks Lady Argony!")
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_lo()),
                Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                          "I thought\n\
                           I told you-")
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                          "Plus, the colors look so much\n\
                           prettier arranged this way!")
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (416, 304), true, false, 0.25),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (464, 272), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (416, 240), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (464, 208), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (416, 176), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (464, 144), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (416, 112), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (496, 112), 0.6),
            Ast::Slide(YTTRIS, (592, 112), false, false, 0.4),
            Ast::Remove(YTTRIS),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ARGONY, TalkStyle::Thought, TalkPos::NE,
                      "Sigh.  Why are these young\n\
                       ones always so stubborn?")
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (-16, 304), true, false, 1.0),
            Ast::Wait(1.0),
            // Show remaining letters:
            Ast::Seq((2..11).map(|index| {
                Ast::Seq(vec![
                    Ast::Queue(0, index),
                    Ast::Wait(0.05),
                ])
            }).collect()),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
