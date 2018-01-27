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

pub const YTTRIS: i32 = 1;
pub const DOOR_UPPER: i32 = -3;
pub const DOOR_LOWER: i32 = -2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("point_of_no_return"),
            Ast::Place(DOOR_UPPER, "tiles/caution_walls", 5, (505, 160)),
            Ast::Place(DOOR_LOWER, "tiles/caution_walls", 4, (505, 176)),
            Ast::Wait(0.5),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 176)),
            Ast::Slide(YTTRIS, (100, 176), true, true, 1.0),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Let's have a look."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Seq((0..16).map(|col| {
                Ast::Queue(0, col) // Hide bridge
            }).collect()),
            Ast::Remove(DOOR_UPPER),
            Ast::Remove(DOOR_LOWER),
            Ast::SetPos(YTTRIS, (508, 176)),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Slide(YTTRIS, (592, 176), true, false, 1.0),
            Ast::Remove(YTTRIS),
            Ast::Wait(1.0),
            Ast::Queue(1, 1),
            Ast::Wait(0.05),
            Ast::Queue(1, 9),
            Ast::Wait(0.05),
            Ast::Queue(1, 12),
            Ast::Wait(0.05),
            Ast::Queue(1, 2),
            Ast::Wait(0.05),
            Ast::Queue(1, 0),
            Ast::Wait(0.05),
            Ast::Queue(1, 5),
            Ast::Wait(0.05),
            Ast::Queue(1, 8),
            Ast::Wait(0.05),
            Ast::Queue(1, 3),
            Ast::Wait(0.05),
            Ast::Queue(1, 4),
            Ast::Wait(0.05),
            Ast::Queue(1, 14),
            Ast::Wait(0.05),
            Ast::Queue(1, 13),
            Ast::Wait(0.05),
            Ast::Queue(1, 15),
            Ast::Wait(0.05),
            Ast::Queue(1, 11),
            Ast::Wait(0.05),
            Ast::Queue(1, 7),
            Ast::Wait(0.05),
            Ast::Queue(1, 6),
            Ast::Wait(0.05),
            Ast::Queue(1, 10),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
