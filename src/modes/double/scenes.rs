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
use crate::gui::{Resources, Sound};

// ========================================================================= //

const MEZURE: i32 = 2;
const RELYNG_BG: i32 = -1;
const RELYNG_FG: i32 = 0;
const UGRENT: i32 = 1;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("double_cross"),
            Ast::Queue(1, 1), // Show first word pair.
            Ast::Place(UGRENT, "chars/ugrent", 0, (216, 256)),
            Ast::Wait(1.0),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 272)),
            Ast::Slide(MEZURE, (122, 272), false, true, 1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "You there!  How is\n\
                       progress coming?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, I think I've got\n\
                       main power back online."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Good.  Move along and\n\
                       check on what the rest of\n\
                       the crew is working on, and\n\
                       see if they need any help."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Oh.  Uh, okay."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Wait(0.3),
                Ast::Queue(1, 0), // Hide word pair.
                Ast::Wait(0.5),
                Ast::Queue(1, 2), // Show second word pair.
            ]),
            Ast::Seq(vec![
                Ast::Slide(MEZURE, (156, 272), true, false, 0.5),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(MEZURE, (192, 256), 0.5),
                Ast::Slide(MEZURE, (332, 256), true, true, 1.0),
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "Um, what are\n\
                           $iyou$r  working on?"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Inspecting the character\n\
                       sets at this checkpoint.\n\
                       I need to re-double-check\n\
                       each one a second time."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Do you...need help?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW, "Oh."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(MEZURE, (368, 240), 0.5),
                Ast::Slide(MEZURE, (467, 240), true, true, 0.75),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.3),
                Ast::Queue(1, 0), // Hide word pair.
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "By the way..."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "There is reason to suspect\n\
                       that the damage to the ship\n\
                       may be at least partly the\n\
                       result of sabotage."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Have you seen anyone around\n\
                       here acting suspiciously?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "Well-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.1),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "Or run into anyone that isn't\n\
                           supposed to be on board?"),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "Actually-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.1),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "Because if you see anyone\n\
                           like that, you should\n\
                           definitely let me know."),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "If you'll-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.1),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "After a disaster like this,\n\
                           we can't be too careful."),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "Yes, but-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.1),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "Now get back to organizing\n\
                           repairs.  I'll finish inspecting\n\
                           this security checkpoint."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "...Okey-dokey."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 240), true, false, 0.75),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Slide(UGRENT, (226, 256), false, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Now then, where were we?"),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1), // Show clues.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(UGRENT, TalkStyle::Thought, TalkPos::NE,
                      "These pairs are all opposites,\n\
                       so figuring out one is usually\n\
                       enough to figure out the other.")
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Queue(0, 0), // Hide clues.
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "These all seem\n\
                       to be in order."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (332, 256), true, false, 0.75),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (368, 240), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Time to go sweep\n\
                       the next area."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (592, 240), true, false, 1.0),
            Ast::Remove(UGRENT),
            Ast::Wait(1.0),
            Ast::Place(RELYNG_BG, "chars/relyng", 3, (400, 160)),
            Ast::Slide(RELYNG_BG, (400, 176), false, false, 1.0),
            Ast::Wait(1.0),
            Ast::Remove(RELYNG_BG),
            Ast::Place(RELYNG_FG, "chars/relyng", 0, (400, 176)),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG_FG, (400, 240), 0.9),
            Ast::Wait(0.75),
            Ast::SetSprite(RELYNG_FG, "chars/relyng", 1),
            Ast::Wait(0.75),
            Ast::SetSprite(RELYNG_FG, "chars/relyng", 2),
            Ast::Wait(0.75),
            Ast::SetSprite(RELYNG_FG, "chars/relyng", 0),
            Ast::Slide(RELYNG_FG, (592, 240), true, false, 1.0),
            Ast::Remove(RELYNG_FG),
            Ast::Wait(1.0),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 3), // Show metapuzzle clue.
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
