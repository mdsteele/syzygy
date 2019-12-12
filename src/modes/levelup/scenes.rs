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

const BOOM: i32 = 100;
const ELINSA: i32 = 1;
const UGRENT: i32 = 2;
const PLATFORM: i32 = 0;

const BOOM_INDICES: &[usize] = &[0, 1, 2, 3, 4];
const PLATFORM_INDICES: &[usize] = &[2, 3];

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("level_up"),
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(ELINSA, "chars/elinsa", 0, (152, 416)),
                    Ast::Slide(ELINSA, (152, 272), false, false, 1.0),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                              "$iGrumble$r...well,\n\
                               there's nothing for it."),
                ]),
                Ast::Seq(vec![
                    Ast::Sound(Sound::platform_shift(5)),
                    Ast::Place(PLATFORM, "shift/platforms", 2, (152, 448)),
                    Ast::Anim(PLATFORM, "shift/platforms",
                              PLATFORM_INDICES, 2),
                    Ast::Slide(PLATFORM, (152, 304), false, false, 1.0),
                    Ast::SetSprite(PLATFORM, "shift/platforms", 2),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "I $istill$r  can't believe that\n\
                       Mezure started the engines\n\
                       without even telling me."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "That kid's got a lot to\n\
                       learn to administrate\n\
                       this ship and crew!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Ah, crud, and now I\n\
                       really $ido$r  need to\n\
                       check on the upper\n\
                       Cold Storage levels."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Up, elevator!"),
            Ast::Seq(vec![
                Ast::Wait(0.15),
                Ast::Sound(Sound::platform_shift(2)),
                Ast::Loop(3, 3, Box::new(Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Slide(ELINSA, (152, 268), false, false, 0.1),
                        Ast::Slide(ELINSA, (152, 272), false, false, 0.1),
                    ]),
                    Ast::Seq(vec![
                        Ast::Slide(PLATFORM, (152, 300), false, false, 0.1),
                        Ast::Slide(PLATFORM, (152, 304), false, false, 0.1),
                    ]),
                ]))),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "I said $iup$r, you\n\
                       stupid machine!"),
            Ast::Seq(vec![
                Ast::Wait(0.15),
                Ast::Sound(Sound::platform_shift(2)),
                Ast::Loop(3, 3, Box::new(Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Slide(ELINSA, (152, 264), false, false, 0.1),
                        Ast::Slide(ELINSA, (152, 272), false, false, 0.1),
                    ]),
                    Ast::Seq(vec![
                        Ast::Slide(PLATFORM, (152, 296), false, false, 0.1),
                        Ast::Slide(PLATFORM, (152, 304), false, false, 0.1),
                    ]),
                ]))),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Great."),
        ]),
        Ast::Seq(vec![
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 80)),
            Ast::Slide(UGRENT, (110, 80), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SE,
                      "Elinsa!  I could\n\
                       use your help with\n\
                       something."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Not $inow,$r  Ugrent!\n\
                       You've already sent\n\
                       me off on one fool's\n\
                       errand; I don't have\n\
                       time for another."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SE,
                      "I think there's\n\
                       something wrong in\n\
                       Cold Storage."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh$r...well, I\n\
                       guess I was headed\n\
                       that way anyway."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "But this elevator is on\n\
                       the fritz, so you're going\n\
                       to have to wait for me\n\
                       to jury-rig something."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SE,
                      "``Jury-rig?''  That doesn't\n\
                       sound like it's up to code."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Whatever, it's fine, I'll\n\
                       fix it properly later."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (160, 272), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "I just need to\n\
                       open up the..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 1), // Make crossword visible.
            Ast::Wait(0.5),
            Ast::Slide(ELINSA, (152, 272), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "...ah, there we go."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NE,
                      "These words are\n\
                       all too long..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NE,
                      "This is a bit dotty."),
        ]),
    ];
    (ELINSA, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SE,
                      "This is $idefinitely$r\n\
                       not up to code."),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources, visible: Rect) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Queue(0, 0), // Animate crossword center word.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Looks good!"),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1), // Hilight crossword center word.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SE,
                      "Are you sure\n\
                       this is safe?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Eh, it's a one-off.\n\
                       It'll be fine."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Okay, now let's\n\
                       try this again:\n\
                       $iup,$r  elevator!"),
        ]),
        Ast::Par(vec![
            Ast::Slide(ELINSA, (152, -32), false, false, 1.0),
            Ast::Seq(vec![
                Ast::Sound(Sound::platform_shift_double(5)),
                Ast::Anim(PLATFORM, "shift/platforms",
                          PLATFORM_INDICES, 2),
                Ast::Slide(PLATFORM, (152, 0), false, false, 1.0),
                Ast::SetSprite(PLATFORM, "shift/platforms", 2),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.2),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::E,
                          "Waaahh!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Slide(UGRENT, (72, 80), true, true, 0.3),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Slide(ELINSA, (152, 80), false, false, 0.3),
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(ELINSA, (112, 80), 0.5),
                ]),
                Ast::Seq(vec![
                    Ast::Sound(Sound::platform_shift_double(6)),
                    Ast::Anim(PLATFORM, "shift/platforms",
                              PLATFORM_INDICES, 2),
                    Ast::Slide(PLATFORM, (152, 448), false, false, 1.2),
                    Ast::Remove(PLATFORM),
                    Ast::Sound(Sound::explosion_small()),
                    Ast::Shake(4),
                    Ast::Place(BOOM, "chars/boom", 0,
                               (152, visible.bottom() + 16)),
                    Ast::Anim(BOOM, "chars/boom", BOOM_INDICES, 2),
                    Ast::Slide(BOOM, (152, visible.bottom() - 34),
                               false, false, 0.4),
                    Ast::Remove(BOOM),
                ]),
            ]),
            Ast::Wait(2.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SE,
                      "So...you'll still\n\
                       ``fix it properly''\n\
                       later, right?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "...yeah."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(UGRENT, (-16, 80), true, false, 0.75),
                Ast::Remove(UGRENT),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Slide(ELINSA, (-16, 80), true, false, 1.0),
                Ast::Remove(ELINSA),
                Ast::Wait(0.75),
            ]),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
