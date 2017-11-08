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

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("plane_as_day"),
            Ast::Wait(1.0),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 288)),
            Ast::Slide(MEZURE, (115, 288), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, here it is.\n\
                       The engine room."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Let's see if we can\n\
                       get this thing fixed."),
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
                      "Once the ship's moving\n\
                       again, we can continue\n\
                       with our mission."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Which is...wait, what\n\
                       $iis$r  our mission?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::NE,
                      "Hopefully someone will\n\
                       tell me eventually."),
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
            Ast::Wait(1.0),
            Ast::Slide(MEZURE, (-16, 288), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Queue(0, 1),
            Ast::Wait(0.1),
            Ast::Queue(1, 2),
            Ast::Wait(0.1),
            Ast::Queue(0, 0),
            Ast::Wait(0.1),
            Ast::Queue(1, 0),
            Ast::Wait(0.1),
            Ast::Queue(0, 2),
            Ast::Wait(0.1),
            Ast::Queue(1, 1),
            Ast::Wait(0.1),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
