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

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("if_memory_serves"),
            Ast::Place(0, "chars/ugrent", 0, (-16, 128)),
            Ast::Slide(0, (122, 128), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Let's solve a puzzle."),
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
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Much better."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (-16, 128), true, false, 0.5),
            Ast::Remove(0),
            Ast::Wait(1.0),
            Ast::Queue(1, 0),
            Ast::Queue(1, 1),
            Ast::Queue(1, 2),
            Ast::Queue(1, 3),
            Ast::Wait(0.5),
            Ast::Queue(1, 4),
            Ast::Queue(1, 5),
            Ast::Queue(1, 6),
            Ast::Queue(1, 7),
            Ast::Wait(0.5),
            Ast::Queue(1, 8),
            Ast::Queue(1, 9),
            Ast::Queue(1, 10),
            Ast::Queue(1, 11),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
