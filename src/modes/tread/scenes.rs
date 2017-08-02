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
const RELYNG: i32 = -1;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("tread_lightly"),
            Ast::Wait(0.25),
            Ast::Place(RELYNG, "chars/relyng", 1, (400, 336)),
            Ast::Slide(RELYNG, (400, 320), false, true, 0.5),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::SetSprite(RELYNG, "chars/relyng", 0),
                    Ast::Wait(0.1),
                    Ast::Slide(RELYNG, (400, 336), true, false, 0.5),
                ]),
                Ast::Seq(vec![
                    Ast::Place(MEZURE, "chars/mezure", 0, (-16, 288)),
                    Ast::Slide(MEZURE, (144, 288), false, false, 1.0),
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(MEZURE, (186, 304), 0.5),
                    Ast::Slide(MEZURE, (215, 304), false, true, 0.35),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                              "Hmm, at least the\n\
                               lights are on in $ihere."),
                ]),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "I guess\n\
                           that means--"),
            ]),
            Ast::Seq(vec![
                Ast::Slide(MEZURE, (368, 304), true, false, 0.75),
                Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Sound(Sound::small_jump()),
                        Ast::Jump(MEZURE, (354, 304), 0.5),
                    ]),
                    Ast::Seq(vec![
                        Ast::Sound(Sound::talk_hi()),
                        Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                                  "Augh!"),
                    ]),
                ]),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.4),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(RELYNG, (400, 304), 0.75),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                          "And we can't have\n\
                           that, now, can we?"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Who are you!?  And what\n\
                       are you doing sneaking\n\
                       around here like that?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Ha!  Wouldn't you\n\
                       like to know!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Um, yes.  $iI would.$r\n\
                       That's why I asked."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(RELYNG, (386, 304), false, true, 0.15),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Well, kid, I should be asking\n\
                       you the same question.  Just\n\
                       who are $iyou$r?  I've never\n\
                       seen you around here before."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "My name's Mezure.  I'm, uh, the\n\
                       new administrator process."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Ha!  A likely story.  Tell\n\
                       me, Mezure, how do I know\n\
                       you're not a saboteur?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Oh, I dunno, because I'm actually\n\
                       $ifixing things$r instead of sneaking\n\
                       around and ambushing passersby\n\
                       with annoying questions?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Leaving all the lights on\n\
                       everywhere you go doesn't\n\
                       count as ``fixing things,'' kid.\n\
                       This node here is draining\n\
                       unnecessary power."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "If you know what's good for\n\
                       you, you'll shut it down."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(-1, 1), // Show next-letter view.
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
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::NW,
                      "A little personal\n\
                       space, please?")
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "I think you'll find that\n\
                       $isneakiness$r is exactly\n\
                       what you need here.")
        ]),
    ];
    (RELYNG, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Queue(-1, 0), // Hide next-letter view.
            Ast::Wait(0.25),
            Ast::Queue(0, 1),
            Ast::Wait(0.1),
            Ast::Queue(2, 1),
            Ast::Wait(0.1),
            Ast::Queue(4, 1),
            Ast::Wait(0.1),
            Ast::Queue(7, 1),
            Ast::Wait(0.1),
            Ast::Queue(8, 1),
            Ast::Wait(0.1),
            Ast::Queue(9, 1),
            Ast::Wait(0.1),
            Ast::Queue(10, 1),
            Ast::Wait(0.1),
            Ast::Light(MEZURE, true),
            Ast::Dark(true),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Heh heh heh...\n\
                       Excellent work!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(RELYNG, (386, 336), 0.75),
                Ast::Remove(RELYNG),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "Wait!  Just where\n\
                           are you going?"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Darn.  I never even\n\
                       got that guy's name."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (410, 304), true, false, 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (448, 288), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Maybe I shouldn't\n\
                       have trusted him..."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 288), true, false, 0.75),
            Ast::Remove(MEZURE),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
