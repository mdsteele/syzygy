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

use crate::elements::{Ast, Scene, TalkPos, TalkStyle};
use crate::gui::{Rect, Resources, Sound};

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
                       forward planar switchboard."),
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
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Each node has to connect\n\
                       to each other node, so\n\
                       there can only be so\n\
                       many stages, right?"),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "C'mon, this is an undirected\n\
                       graph we're talking about.\n\
                       Wiring it backwards doesn't\n\
                       even make a difference!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Look, let's just put\n\
                       it back the same\n\
                       way it was before."),
        ]),
    ];
    (YTTRIS, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources, visible: Rect) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "There!  All fixed."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (388, 304), true, true, 1.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Good.  Keep working on\n\
                       repairs.  I need to continue\n\
                       my security sweep."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(UGRENT, (436, 288), 0.5),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(UGRENT, (484, 272), 0.5),
                Ast::Slide(UGRENT, (592, 272), false, false, 0.5),
                Ast::SetPos(UGRENT, (visible.right() + 16, 272)),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                          "Will do!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (388, 304), 0.5),
            Ast::Slide(YTTRIS, (284, 304), true, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "He's gone!  Now's\n\
                       my chance to flip\n\
                       it all backwards!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::W,
                          "I heard that!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Haha!  I'm just\n\
                           $ikidding$r, Ugrent!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::NE,
                      "No sense of\n\
                       humor at all..."),
        ]),
        Ast::Seq(vec![
            Ast::Remove(UGRENT),
            Ast::Slide(YTTRIS, (-16, 304), true, false, 1.0),
            Ast::Remove(YTTRIS),
            Ast::Wait(1.0),
            Ast::Queue(1, 7),
            Ast::Wait(0.1),
            Ast::Queue(0, 0),
            Ast::Queue(1, 6),
            Ast::Wait(0.1),
            Ast::Queue(0, 1),
            Ast::Queue(1, 5),
            Ast::Wait(0.1),
            Ast::Queue(0, 2),
            Ast::Queue(1, 4),
            Ast::Wait(0.1),
            Ast::Queue(0, 3),
            Ast::Queue(1, 3),
            Ast::Wait(0.1),
            Ast::Queue(0, 4),
            Ast::Queue(1, 2),
            Ast::Wait(0.1),
            Ast::Queue(0, 5),
            Ast::Queue(1, 1),
            Ast::Wait(0.1),
            Ast::Queue(1, 0),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
