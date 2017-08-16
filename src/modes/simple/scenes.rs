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

const UGRENT: i32 = 2;
const YTTRIS: i32 = 1;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("plane_and_simple"),
            Ast::Place(YTTRIS, "chars/yttris", 0, (444, 288)),
            Ast::Wait(0.75),
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 304)),
            Ast::Slide(UGRENT, (220, 304), false, true, 1.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Hey there, Ugrent!\n\
                       How's it going?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Yttris.  You know anything about\n\
                       the character mismatches in the\n\
                       security checkpoint back there?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Huh?  No.  What are\n\
                       you talking about?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Never mind.  What are\n\
                       you working on here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Oh, this thing?  This is the ship's\n\
                       starboard planar switchboard."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "That explosion shook all the\n\
                       connectors loose, so now I have\n\
                       to redo them all.  Booorrring!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (436, 288), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Oooh, maybe I should wire\n\
                       them all up $ibackwards$r  this\n\
                       time! $iThat$r  would be fun!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (250, 304), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "What!?  No!  Yttris, right\n\
                       now we need to get the ship\n\
                       back to normal.  This is no\n\
                       time for improvising!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Aw, you're no fun."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Look, the last thing we need\n\
                       here is another explosion because\n\
                       we didn't do things by the book."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (200, 304), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Let's just take this\n\
                       thing in stages and do\n\
                       them one at a time."),
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
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
