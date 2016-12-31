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

pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("tread_lightly"),
            Ast::Place(0, "chars/tezure", 0, (-16, 288)),
            Ast::Slide(0, (144, 288), false, false, 1.0),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(0, (186, 304), 0.5),
            Ast::Slide(0, (215, 304), false, true, 0.35),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Hmm."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
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
            Ast::Light(0, true),
            Ast::Dark(true),
            Ast::Wait(0.5),
            Ast::Slide(0, (410, 304), true, false, 1.0),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(0, (448, 288), 0.5),
            Ast::Slide(0, (592, 288), false, false, 0.5),
            Ast::Remove(0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
