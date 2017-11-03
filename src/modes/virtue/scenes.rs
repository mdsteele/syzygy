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
            Ast::SetBg("virtue_or_ice"),
            Ast::Place(0, "chars/mezure", 0, (-16, 304)),
            Ast::Slide(0, (404, 304), true, true, 1.0),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Let's have a look."),
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
            Ast::Slide(0, (592, 304), true, false, 1.0),
            Ast::Remove(0),
            Ast::Wait(1.0),
            Ast::Queue(1, 0),
            Ast::Wait(0.1),
            Ast::Queue(1, 1),
            Ast::Queue(1, 2),
            Ast::Wait(0.1),
            Ast::Queue(1, 3),
            Ast::Queue(1, 4),
            Ast::Wait(0.1),
            Ast::Queue(1, 5),
            Ast::Queue(1, 6),
            Ast::Wait(0.1),
            Ast::Wait(0.1),
            Ast::Queue(1, 7),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
