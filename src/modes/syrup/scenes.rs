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
use gui::{Rect, Resources, Sound};

// ========================================================================= //

const BRIDGE_START: i32 = -99;
const MEZURE: i32 = 2;
const NEXT: i32 = 1;
const RELYNG: i32 = -1;
const SPIKES: i32 = -100;
const SYSTEM: i32 = 0;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("light_syrup"),
            Ast::Dark(true),
            Ast::Place(SPIKES, "chars/spikes", 0, (264, 334)),
            Ast::Place(SYSTEM, "chars/system", 0, (128, 80)),
            Ast::Wait(0.5),
            Ast::Place(RELYNG, "chars/relyng", 0, (160, 304)),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 288)),
            Ast::Light(MEZURE, true),
            Ast::Slide(MEZURE, (100, 288), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Lights are out up\n\
                       here, too, eh?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I guess I should probably\n\
                       get them turned back on."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Let's see..."),
            ]),
            Ast::Seq(vec![
                Ast::Slide(MEZURE, (132, 288), true, false, 0.5),
                Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Sound(Sound::talk_lo()),
                        Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                                  "Augh!"),
                    ]),
                    Ast::Seq(vec![
                        Ast::Sound(Sound::small_jump()),
                        Ast::Jump(MEZURE, (116, 288), 0.25),
                    ]),
                ]),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Who...who's there?"),
            ]),
            Ast::Seq(vec![
                Ast::Slide(RELYNG, (160, 320), true, false, 0.25),
                Ast::Remove(RELYNG),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (140, 288), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Hello?  Anybody?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (199, 288), true, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Where'd they go?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "...Okay, calm down, Mezure."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                       "I should probably get these\n\
                       lights turned on before I\n\
                       search any farther."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Don't want to stub\n\
                       my toe again."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(-1, 1), // Show next-color view.
            Ast::Place(NEXT, "chars/invis", 0, (472, 104)),
            Ast::Light(NEXT, true),
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
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "This is kind of like solving\n\
                       three separate puzzles at once.")
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources, visible: Rect) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Remove(NEXT),
            Ast::Queue(-1, 0), // Hide next-color view.
            Ast::Wait(0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Excellent!  Now, let's see what I\n\
                       was about to run into in the dark...")
        ]),
        Ast::Seq(vec![
            Ast::Dark(false),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Oh.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "...You know, someone really needs to\n\
                       get OSHA to take a look at this place.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "System, could you, uh, get me a\n\
                       bridge over this spike pit?")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Affirmative.  Extending bridge.")
        ]),
        Ast::Seq(vec![
            Ast::Seq((0..8).map(|index| {
                Ast::Seq(vec![
                    Ast::Sound(Sound::platform_shift(1)),
                    Ast::Place(BRIDGE_START + index, "tiles/miniblocks", 14,
                               (220 + 16 * index, 304)),
                    Ast::Wait(0.1),
                ])
            }).collect()),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Ah, thanks muchly.")
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (170, 288), true, true, 0.5),
            Ast::Slide(MEZURE, (592, 288), true, false, 1.2),
            Ast::SetPos(MEZURE, (visible.right() + 16, 288)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::W,
                      "...Wait, why do we\n\
                       even $ihave$r a spike pit?")
        ]),
        Ast::Seq(vec![
            // Hilight L in red:
            Ast::Queue(3, 1),
            Ast::Wait(0.1),
            Ast::Queue(8, 1),
            Ast::Wait(0.1),
            Ast::Queue(13, 1),
            Ast::Wait(0.1),
            Ast::Queue(14, 1),
            Ast::Wait(0.5),
            // Hilight L in green:
            Ast::Queue(0, 2),
            Ast::Wait(0.1),
            Ast::Queue(4, 2),
            Ast::Wait(0.1),
            Ast::Queue(9, 2),
            Ast::Wait(0.1),
            Ast::Queue(10, 2),
            Ast::Wait(0.5),
            // Hilight Y in blue:
            Ast::Queue(5, 3),
            Ast::Wait(0.1),
            Ast::Queue(7, 3),
            Ast::Wait(0.1),
            Ast::Queue(11, 3),
            Ast::Wait(0.1),
            Ast::Queue(16, 3),
            Ast::Wait(0.5),
            // Show EDER letters:
            Ast::Queue(-2, 0),
            Ast::Wait(1.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
