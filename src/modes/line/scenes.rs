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

const UGRENT: i32 = 0;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("cross_the_line"),
            Ast::Queue(0, 1), // Display "SAFE FACE"
            Ast::Wait(0.5),
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 272)),
            Ast::Slide(UGRENT, (122, 272), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "No security breaches so\n\
                       far that I can see."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "This area appears to\n\
                       be in order as well."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (160, 256), 0.5),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Hmm...or is it?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (210, 256), true, true, 0.5),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Aha!  This $ihas$r  been\n\
                       tampered with."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 0), // Select "S" in grid 1.
            Ast::Queue(2, 2), // Select "C" in grid 2.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "These two character sets\n\
                       should match.  But they don't.\n\
                       Something's wrong here..."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, -1), // Unselect grid 1.
            Ast::Queue(2, -1), // Unselect grid 2.
            Ast::Queue(0, -1), // Display blank grids.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "I suppose it could just be a\n\
                       malfunction.  Or, it could be something\n\
                       more.  Sabotage, perhaps?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (288, 256), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "I'd better inspect each\n\
                       set, just to be safe."),
        ]),
        Ast::Queue(0, 0), // Display puzzle grids.
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "All it takes is patience\n\
                       and a keen eye.")
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "This is no mere malfunction.\n\
                       Someone has deliberately\n\
                       altered these databanks."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (320, 256), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Is this connected somehow to the\n\
                       disaster that damaged the ship?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (416, 256), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "We need to figure out who did this."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (454, 272), 0.5),
            Ast::Slide(UGRENT, (592, 272), false, false, 0.5),
            Ast::Remove(UGRENT),
            Ast::Wait(1.0),
            Ast::Sound(Sound::beep()),
            Ast::Queue(3, 1), // Hide extra solution characters.
            Ast::Wait(0.15),
            Ast::Queue(3, 0), // Show extra solution characters.
            Ast::Wait(0.15),
            Ast::Queue(3, 1), // Hide extra solution characters.
            Ast::Wait(2.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
