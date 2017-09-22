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

pub const UGRENT: i32 = 2;
pub const YTTRIS: i32 = 1;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("shift_gears_1"),
            Ast::Queue(-1, 0), // Hide platforms/arrows
            Ast::Wait(0.75),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 240)),
            Ast::Slide(YTTRIS, (144, 240), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Okay, main engineering!\n\
                       Now I just need to-"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "...Wait."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "This isn't main engineering!\n\
                       ...Where am I???"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Aah!  I'm lost!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (176, 256), 0.4),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (208, 272), 0.4),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (248, 288), 0.4),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (296, 304), 0.4),
                Ast::Slide(YTTRIS, (592, 304), false, false, 1.0),
                Ast::Remove(YTTRIS),
                Ast::SetBg("shift_gears_2"),
                Ast::Queue(-1, 1), // Show platforms/arrows
                Ast::Place(UGRENT, "chars/ugrent", 0, (112, 304)),
                Ast::Wait(0.5),
                Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 304)),
                Ast::Slide(YTTRIS, (80, 304), false, false, 0.25),
                Ast::Par(vec![
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(UGRENT, (112, 304), 0.25),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::E,
                              "Augh!"),
                    Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                              "Oh, hi Ugrent!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Yttris!  Don't startle\n\
                       me like that!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Sorry!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Ugrent, I think I'm lost!\n\
                       Where's main engineering?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "You're on the wrong deck,\n\
                       Yttris.  It's just above us."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "I $iwas$r  headed in that\n\
                       direction, but now these\n\
                       carriages are all out of\n\
                       whack, and these controls\n\
                       for repositioning them\n\
                       are $iabsurd."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Ooh, I've done these\n\
                       before!  Let me try!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "...Sorry, but I think you'd\n\
                       better let me handle this,\n\
                       Yttris.  This is another\n\
                       ``just so'' kind of situation."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "But I think I can-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::E,
                          "Yttris, please!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "These carriages can be very\n\
                       finicky and dangerous, and I\n\
                       really don't want you to get\n\
                       knocked off and get hurt trying\n\
                       to ride them back and forth."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Just wait here while I work\n\
                       my way up first.  I think there\n\
                       might be some better controls\n\
                       up there, so I can arrange\n\
                       safer passage for you."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Oh, okay, if you insist..."),
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
            Ast::Talk(UGRENT, TalkStyle::Thought, TalkPos::Auto,
                      "I need to make sure that I'm on\n\
                       the $iright-hand$r  side of that\n\
                       barrier up there when I jump\n\
                       onto the uppermost carriage."),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Wow, Ugrent, you're almost as\n\
                       good at this as Elinsa was!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::Auto,
                      "That compliment wouldn't\n\
                       be necessary if these\n\
                       carriages actually had\n\
                       $ireasonable controls."),
        ]),
    ];
    (YTTRIS, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                      "There!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                      "Just hang tight down there,\n\
                       Yttris!  I'm going to check up\n\
                       ahead to see if there's a control\n\
                       panel for these carriages, so I\n\
                       can get you up here too."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (592, 64), true, false, 0.75),
            Ast::Remove(UGRENT),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Huh.  I was skeptical at first,\n\
                       but I guess that $iwas$r  a pretty\n\
                       good way to get up there!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Queue(0, 9),
                Ast::Queue(1, 8),
                Ast::Queue(2, 7),
                Ast::Queue(3, 4),
                Ast::Queue(4, 6),
                Ast::Queue(5, 3),
                Ast::Queue(6, 2),
                Ast::Queue(7, 1),
                Ast::Wait(1.5),
            ]),
            Ast::Seq(vec![
                Ast::Slide(YTTRIS, (112, 304), true, true, 0.3),
                Ast::Wait(0.7),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "I'm a little suprised\n\
                           he didn't just do it\n\
                           this way, though."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (176, 280), 0.65),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (208, 248), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (240, 216), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (272, 184), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (336, 184), 0.65),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (368, 152), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (400, 120), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (432, 88), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (496, 64), 0.65),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW, "Voila!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "See?  It didn't need\n\
                       to be so hard."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                          "Hey Ugrent,\n\
                           wait up!"),
            ]),
            Ast::Seq(vec![
                Ast::Slide(YTTRIS, (592, 64), true, false, 0.5),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(YTTRIS),
            Ast::Wait(0.5),
            Ast::Queue(-2, 0), // Move all platforms to final position.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
