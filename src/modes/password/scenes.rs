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
            Ast::SetBg("password_file"),
            Ast::Place(2, "chars/mezure", 0, (-16, 160)),
            Ast::Slide(2, (96, 160), false, true, 1.0),
            Ast::Place(3, "chars/yttris", 0, (592, 160)),
            Ast::Slide(3, (480, 160), false, true, 1.0),
            Ast::Place(4, "chars/ugrent", 0, (-16, 320)),
            Ast::Slide(4, (144, 320), false, true, 1.0),
            Ast::Place(0, "chars/elinsa", 0, (-16, 96)),
            Ast::Slide(0, (122, 96), false, true, 1.0),
            Ast::Place(1, "chars/argony", 0, (592, 96)),
            Ast::Slide(1, (454, 96), false, true, 1.0),
            Ast::Place(5, "chars/relyng", 0, (592, 320)),
            Ast::Slide(5, (432, 320), false, true, 1.0),
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
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SE, "Looks good!"),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
