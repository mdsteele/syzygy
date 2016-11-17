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

pub const ELINSA_SLOT: i32 = 0;

// ========================================================================= //

pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("shifting_ground_1"),
            Ast::Queue(-1, 0), // Hide platforms/arrows
            Ast::Place(0, "chars/elinsa", 0, (432, 320)),
            Ast::Wait(1.0),
            Ast::Slide(0, (592, 320), true, false, 1.0),
            Ast::Place(0, "chars/elinsa", 0, (-16, 320)),
            Ast::SetBg("shifting_ground_2"),
            Ast::Queue(-1, 1), // Show platforms/arrows
            Ast::Place(1, "chars/yttris", 0, (112, 320)),
            Ast::Slide(0, (80, 320), false, true, 0.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SW, "Hooray"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (592, 64), true, false, 0.5),
            Ast::Remove(0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
