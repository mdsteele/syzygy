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

const RELYNG: i32 = -1;
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
            Ast::Wait(1.0),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 176)),
            Ast::Slide(YTTRIS, (66, 176), false, true, 0.5),
            Ast::Wait(0.25),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                              "Okay, here we-"),
                ]),
                Ast::Seq(vec![
                    Ast::Slide(YTTRIS, (124, 176), true, false, 0.75),
                    Ast::Par(vec![
                        Ast::Seq(vec![
                            Ast::Queue(0, 0), // Hide first bridge tile
                            Ast::Sound(Sound::small_jump()),
                            Ast::Jump(YTTRIS, (100, 176), 0.5),
                            Ast::Queue(-1, 0), // Unhide bridge
                        ]),
                        Ast::Seq(vec![
                            Ast::Wait(0.15),
                            Ast::Sound(Sound::talk_hi()),
                            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                                      "Whoops!"),
                        ]),
                    ]),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE, "Huh?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (124, 176), true, false, 0.5),
            Ast::Queue(0, 0), // Hide first bridge tile
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (100, 176), 0.5),
            Ast::Queue(-1, 0), // Unhide bridge
            Ast::Wait(0.5),
            Ast::Slide(YTTRIS, (124, 176), true, false, 0.4),
            Ast::Queue(0, 0), // Hide first bridge tile
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (100, 176), 0.5),
            Ast::Queue(-1, 0), // Unhide bridge
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "I don't think this is how\n\
                       bridges are supposed to work."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "How am I supposed\n\
                       to get across?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(RELYNG, "chars/relyng", 0, (-16, 176)),
            Ast::Slide(RELYNG, (72, 176), false, false, 0.3),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                              "Sometimes you can only\n\
                               move forward when\n\
                               there's no way back."),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Par(vec![
                        Ast::Sound(Sound::small_jump()),
                        Ast::Jump(YTTRIS, (100, 176), 0.25),
                        Ast::Sound(Sound::talk_hi()),
                        Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                                  "Eeep!"),
                    ]),
                ]),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "How did-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.15),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                          "Sometimes, new doors will\n\
                           open for you only when you've\n\
                           burned your bridges."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Wow...that's...that's a pretty\n\
                       deep metaphor for life, isn't it!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "No, I mean that door over there\n\
                       will literally only open once\n\
                       this whole bridge is gone."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "You're going to have to use up\n\
                       every single tile as you cross it."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "But if the bridge is gone,\n\
                       then...how will I get back?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "What part of ``no way back''\n\
                       was confusing to you?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Um, the ``no way'' part didn't\n\
                       really make a lot of sense."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Wait(0.1),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "...the ``back'' part\n\
                           was fine I guess."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.3),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                          "If you really want to pass\n\
                           beyond that door, you'll have\n\
                           to make your choice."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(RELYNG, (65, 176), true, true, 0.3),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "Choose wisely."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "Your last jump has to land you on\n\
                       the END tile exactly, so think about\n\
                       which tile will get you there."),
        ]),
    ];
    (RELYNG, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::NE,
                      "$iShudder$r...This bridge\n\
                       is kind of creepy!"),
        ]),
    ];
    (YTTRIS, Ast::compile_scene(resources, ast))
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
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                          "I made it!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(RELYNG, (79, 176), true, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "Hmmm."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "...Interesting choice."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(RELYNG, (-16, 176), true, false, 1.0),
            Ast::Remove(RELYNG),
            Ast::Wait(1.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "Um, I guess I'm trapped\n\
                       on this side forever now?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                      "I hope this was worth it!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (592, 176), true, false, 1.0),
            Ast::Remove(YTTRIS),
            Ast::Wait(1.5),
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
