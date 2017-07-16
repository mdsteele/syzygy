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

const MEZURE: i32 = 0;
const NEXT: i32 = 1;
const RELYNG: i32 = -1;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("light_syrup"),
            Ast::Dark(true),
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
                      "Okay, calm down Mezure.\n\
                       I should get these lights\n\
                       turned on before I search\n\
                       any farther."),
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
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Queue(-1, 0), // Hide next-color view.
            Ast::Wait(0.25),
            Ast::Dark(false),
            Ast::Remove(NEXT),
            Ast::Slide(MEZURE, (592, 288), false, false, 1.0),
            Ast::Remove(MEZURE),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
