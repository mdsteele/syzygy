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
use save::pyramid::Coords;
use super::coords::{PYRAMID_TILE_SIZE, coords_to_pt};

// ========================================================================= //

pub const MIDDLE_SCENE: i32 = 1000;
pub const LOSE_GAME_SCENE: i32 = 1001;

const HINTS_START: i32 = 2000;
const HILIGHTS_START: i32 = -1000;

const ARGONY: i32 = 5;
const BRIDGE_START: i32 = -99;
const BOOM_START: i32 = 100;
const ELINSA: i32 = 4;
const MEZURE: i32 = 1;
const RELYNG: i32 = -100;
const RELYNG_FG: i32 = 7;
const SRB: i32 = 6;
const UGRENT: i32 = 2;
const YTTRIS: i32 = 3;

const BOOM_INDICES: &[usize] = &[0, 1, 2, 3, 4];

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("system_failure"),
            Ast::Seq((0..36).filter_map(|index| {
                if index == 19 || index == 32 {
                    None
                } else {
                    Some(Ast::Queue(6, index)) // Force chip to be red.
                }
            }).collect()),
            Ast::Wait(0.5),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 192)),
            Ast::Slide(MEZURE, (120, 192), false, true, 0.75),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "So this must be the\n\
                       system status console."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "That...is a lot of red."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "...and I'm guessing\n\
                       that red is bad."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "How are we ever going\n\
                       to repair all of this?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(SRB, "chars/srb", 0, (592, 192)),
            Ast::Slide(SRB, (448, 192), false, true, 0.75),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::NW,
                      "Have no fear,\n\
                       for I am here!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Who're you?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(SRB, (448, 155), true, true, 0.5),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::NW,
                      "Well, my full designation is\n\
                       System Repair Bot TX32."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Slide(SRB, (435, 162), true, true, 0.5),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::NW,
                      "But you can call me..."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::SetSprite(SRB, "chars/srb", 3),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Good, TalkPos::NW,
                          "System Repair Bot!"),
            ]),
            Ast::Loop(0, 0, Box::new(Ast::Seq(vec![
                Ast::Slide(SRB, (435, 160), false, false, 0.1),
                Ast::Slide(SRB, (435, 164), false, false, 0.2),
                Ast::Slide(SRB, (435, 162), false, false, 0.1),
            ]))),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "That's...kind of a mouthful."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 2),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Are you sure we couldn't\n\
                       shorten that to, like,\n\
                       ``SysBot'' or something?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 4),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Or maybe ``SysRep?''"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Or ``Syssy?''"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.15),
                Ast::SetSprite(SRB, "chars/srb", 7),
                Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Sound(Sound::talk_lo()),
                        Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                                  "No!  It's ``System\n\
                                   Repair Bot!''"),
                    ]),
                    Ast::Loop(0, 0, Box::new(Ast::Seq(vec![
                        Ast::Slide(SRB, (435, 160), false, false, 0.1),
                        Ast::Slide(SRB, (435, 164), false, false, 0.2),
                        Ast::Slide(SRB, (435, 162), false, false, 0.1),
                    ]))),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 4),
            Ast::Wait(0.5),
            Ast::SetSprite(SRB, "chars/srb", 2),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Okay, okay, ``System\n\
                       Repair Bot'' it is."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Slide(SRB, (426, 96), true, true, 0.5),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "And I can help!  See all\n\
                       these red indicators?  These\n\
                       represent ship subsystems\n\
                       that are broken."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "Engines, external sensors,\n\
                       navigational control..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Slide(SRB, (426, 144), true, true, 0.5),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "To get the ship moving again,\n\
                       you'll need to travel around\n\
                       the ship, repairing these systems.\n\
                       Focus on the engines first, so\n\
                       we can get to our destination!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "What's our destination?  And\n\
                       what about the external\n\
                       sensors and that other stuff?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "Oh don't worry about that!\n\
                       The most important thing is\n\
                       that we get moving again."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, if you say so.  I guess\n\
                       I'd better get started, then."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "Great!  I might pop up from\n\
                       time to time to help you out."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Sounds great.  See you\n\
                       later, System Repair Bot!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(MEZURE, (-16, 192), true, false, 0.75),
                Ast::Remove(MEZURE),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::SetSprite(SRB, "chars/srb", 1),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                          "Later!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Wait(0.75),
            // TODO: play sound here
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Wait(0.75),
            Ast::SetSprite(SRB, "chars/srb", 6),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::SW,
                      "Heh heh heh..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Slide(SRB, (592, 144), true, false, 0.75),
            Ast::Remove(SRB),
            Ast::Wait(1.0),
            Ast::Queue(6, -1), // Stop forcing chips to be red.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_middle_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 192)),
            Ast::Slide(MEZURE, (128, 192), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, I think we've\n\
                       finally done it."),
        ]),
        Ast::Seq(vec![
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 192)),
            Ast::Slide(UGRENT, (96, 192), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "``Bridge shall extend\n\
                       after finish repair.''"),
        ]),
        Ast::Seq(vec![
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 192)),
            Ast::Slide(YTTRIS, (64, 192), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "And it looks like\n\
                       repairs are done?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 128)),
            Ast::Slide(ELINSA, (96, 128), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Just about.  I mean, manual nav\n\
                       control is still shot, but autopilot\n\
                       is working and still locked to our\n\
                       destination, so we should be good."),
        ]),
        Ast::Seq(vec![
            Ast::Place(ARGONY, "chars/argony", 0, (-16, 128)),
            Ast::Slide(ARGONY, (64, 128), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::E,
                      "Well, then, let's get\n\
                       that bridge extended."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Seq((0..15).map(|index| {
                    Ast::Seq(vec![
                        Ast::Sound(Sound::platform_shift(1)),
                        Ast::Place(BRIDGE_START + index,
                                   "tiles/miniblocks", 14,
                                   (144 + 16 * index, 208)),
                        Ast::Wait(0.1),
                    ])
                }).collect()),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.9),
                Ast::Place(SRB, "chars/srb", 5, (592, 192)),
                Ast::Slide(SRB, (448, 192), false, false, 0.25),
                Ast::Par(vec![
                    Ast::Sound(Sound::character_collision()),
                    Ast::Queue(7, 1), // Change "SHALL" to "SHANT"
                    Ast::Seq((0..16).map(|index| {
                        Ast::Seq(vec![
                            Ast::Remove(BRIDGE_START + 15 - index),
                            Ast::Wait(0.05),
                        ])
                    }).collect()),
                    Ast::SetSprite(SRB, "chars/srb", 6),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW, "Nope!"),
                    Ast::Seq(vec![
                        Ast::Wait(0.25),
                        Ast::Sound(Sound::talk_hi()),
                        Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                                  "Huh!?"),
                    ]),
                ]),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::beep()),
                Ast::Queue(6, 9),
                Ast::Queue(6, 10),
                Ast::Wait(0.1),
                Ast::Queue(6, 3),
                Ast::Queue(6, 4),
                Ast::Queue(6, 8),
                Ast::Queue(6, 11),
                Ast::Queue(6, 15),
                Ast::Queue(6, 16),
                Ast::Wait(0.1),
                Ast::Queue(6, 2),
                Ast::Queue(6, 5),
                Ast::Queue(6, 7),
                Ast::Queue(6, 14),
                Ast::Queue(6, 17),
                Ast::Queue(6, 21),
                Ast::Queue(6, 22),
                Ast::Wait(0.1),
                Ast::Queue(6, 1),
                Ast::Queue(6, 6),
                Ast::Queue(6, 13),
                Ast::Queue(6, 20),
                Ast::Queue(6, 23),
                Ast::Queue(6, 27),
                Ast::Queue(6, 28),
                Ast::Wait(0.1),
                Ast::Queue(6, 0),
                Ast::Queue(6, 12),
                Ast::Queue(6, 19),
                Ast::Queue(6, 26),
                Ast::Queue(6, 29),
                Ast::Queue(6, 33),
                Ast::Queue(6, 34),
                Ast::Wait(0.1),
                Ast::Queue(6, 18),
                Ast::Queue(6, 25),
                Ast::Queue(6, 32),
                Ast::Queue(6, 35),
                Ast::Wait(0.1),
                Ast::Queue(6, 24),
                Ast::Queue(6, 31),
                Ast::Wait(0.1),
                Ast::Queue(6, 30),
                Ast::Wait(1.0),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                          "While you fools have been\n\
                           scurrying around fixing the\n\
                           irrelevant systems I sabotaged,\n\
                           I've been enacting my REAL plan."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::E,
                      "Oh yeah?  And what's that?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 6),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                      "Ha!  Wouldn't you\n\
                       like to know!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                          "You've all been playing\n\
                           right into my hands.  Little\n\
                           Mezure here even did me the\n\
                           favor of restarting the engines!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                          "...gulp..."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "You...you haven't won yet!\n\
                       We'll find a way to beat you!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 6),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                      "Oh?  You think so?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                      "Well, if you want to cross this\n\
                       bridge so badly...how about\n\
                       we play a little game for it?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Queue(8, 1), // Animate moving chips to supply points.
            Ast::Wait(0.5),
            Ast::SetSprite(SRB, "chars/srb", 6),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                      "First to the top wins.\n\
                       I'll even let you go first!\n\
                       Heh heh heh..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Queue(0, 1), // Hide dashboard.
            Ast::Queue(4, 1), // Mark middle scene as done.
        ]),
    ];
    (MIDDLE_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_argony_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::E,
                      "Forming lines is the\n\
                       key to victory here."),
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::E,
                      "Eh, when in doubt I say just\n\
                       try the greedy strategy."),
        ]),
    ];
    (ELINSA, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "If we work together,\n\
                       I think we can win!"),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_srb_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 6),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                      "You fools will\n\
                       never beat me!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "We'll just see\n\
                       about that!"),
        ]),
    ];
    (SRB, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Try to think about how to\n\
                       block the robot's best moves."),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Par(vec![
            Ast::Loop(0, 2, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (64, 192), 0.5),
            ]))),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Don't forget to jump\n\
                       pieces when you can!"),
        ]),
    ];
    (YTTRIS, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_lose_game_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_lo()),
            Ast::Seq((1..9).map(|n| {
                let start = n * (n - 1) / 2;
                let end = n * (n + 1) / 2;
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Seq((start..end).map(|index| {
                        Ast::Queue(2, index) // Hilight piece red.
                    }).collect()),
                ])
            }).collect()),
            Ast::Wait(0.5),
            Ast::SetSprite(SRB, "chars/srb", 6),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                      "Ha!  You lose!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                      "You know, I think I'll\n\
                       let you fools try again.\n\
                       You'll never beat me, and\n\
                       soon it will be too late!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 5),
            Ast::Queue(5, 1), // Reset board.
        ]),
    ];
    (LOSE_GAME_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Queue(0, 1), // Hide dashboard.
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Seq((1..9).map(|n| {
                let start = n * (n - 1) / 2;
                let end = n * (n + 1) / 2;
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Seq((start..end).map(|index| {
                        Ast::Queue(1, index) // Hilight piece green.
                    }).collect()),
                ])
            }).collect()),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::SetSprite(SRB, "chars/srb", 7),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW, "What!?"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE, "Yes!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 8),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Place(RELYNG, "chars/relyng", 3, (218, 208)),
                    Ast::Slide(RELYNG, (282, 216), false, false, 0.35),
                    Ast::Slide(RELYNG, (370, 216), false, false, 0.5),
                    Ast::Slide(RELYNG, (434, 208), false, false, 0.35),
                    Ast::Remove(RELYNG),
                ]),
                Ast::Seq((0..19).map(|index| {
                    Ast::Seq(vec![
                        Ast::Sound(Sound::platform_shift(1)),
                        Ast::Place(BRIDGE_START + index,
                                   "tiles/miniblocks", 14,
                                   (144 + 16 * index, 208)),
                        Ast::Wait(0.1),
                    ])
                }).collect()),
            ]),
            Ast::Wait(0.75),
            Ast::Queue(3, 2), // Turn whole board red.
            Ast::Queue(1, -1), // Clear hilights.
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::SetSprite(SRB, "chars/srb", 7),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Shake(4),
                    Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW, "NO!!"),
                ]),
                Ast::Seq((0..19).map(|index| {
                    Ast::Seq(vec![
                        Ast::Remove(BRIDGE_START + 18 - index),
                        Ast::Wait(0.05),
                    ])
                }).collect()),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::talk_annoyed_hi()),
                    Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::E, "Hey!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                      "I shall have no more of this\n\
                       nonsense!  I will not allow\n\
                       you...you...PESTS to\n\
                       interfere with my plans!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Place(RELYNG, "chars/relyng", 2, (490, 224)),
                Ast::Slide(RELYNG, (490, 208), false, true, 0.75),
                Ast::Wait(0.75),
                Ast::Slide(RELYNG, (490, 224), true, false, 0.75),
                Ast::Remove(RELYNG),
            ]),
            Ast::Seq(vec![
                Ast::SetSprite(SRB, "chars/srb", 6),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                          "This ship is under my\n\
                           complete control now!\n\
                           It's too late to stop me!"),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                          "In fact, I-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Place(RELYNG, "chars/relyng", 0, (490, 224)),
                Ast::Slide(RELYNG, (456, 202), false, false, 0.2),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.35),
                Ast::Sound(Sound::character_collision()),
                Ast::SetSprite(SRB, "chars/srb", 7),
                Ast::Par(vec![
                    Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NW,
                              "In fact, I-\n\
                               -Aauugh!"),
                    Ast::Seq(vec![
                        Ast::Place(BOOM_START, "chars/boom", 0, (448, 184)),
                        Ast::Anim(BOOM_START, "chars/boom", BOOM_INDICES, 2),
                        Ast::Wait(0.4),
                        Ast::Remove(BOOM_START),
                    ]),
                    Ast::Seq(vec![
                        Ast::Slide(SRB, (390, -32), false, true, 0.75),
                        Ast::Remove(SRB),
                        Ast::Wait(0.25),
                        Ast::Par(vec![
                            Ast::Seq(vec![
                                Ast::Sound(Sound::talk_hi()),
                                Ast::Talk(UGRENT, TalkStyle::Normal,
                                          TalkPos::NE,
                                          "Ha!  Excellent\n\
                                           work, Relyng!"),
                            ]),
                            Ast::Seq(vec![
                                Ast::Wait(0.5),
                                Ast::Sound(Sound::talk_hi()),
                                Ast::Talk(MEZURE, TalkStyle::Normal,
                                          TalkPos::E, "Whoa!"),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Par((1..8).map(|index| {
                    let slot = BOOM_START + index;
                    Ast::Seq(vec![
                        Ast::Wait(0.15 + 0.1 * index as f64),
                        Ast::Place(slot, "chars/boom", 0,
                                   (192 + 32 * index, 32 * index)),
                        Ast::Anim(slot, "chars/boom", BOOM_INDICES, 1),
                        Ast::Wait(0.2),
                        Ast::Remove(slot),
                    ])
                }).collect()),
                Ast::Par((1..6).map(|index| {
                    let slot = BOOM_START + index;
                    Ast::Seq(vec![
                        Ast::Wait(0.9 + 0.1 * index as f64),
                        Ast::Place(slot, "chars/boom", 0,
                                   (416 - 32 * index, 224 + 32 * index)),
                        Ast::Anim(slot, "chars/boom", BOOM_INDICES, 1),
                        Ast::Wait(0.2),
                        Ast::Remove(slot),
                    ])
                }).collect()),
                Ast::Seq(vec![
                    Ast::Place(SRB, "chars/srb", 9, (192, 0)),
                    Ast::Slide(SRB, (416, 224), false, false, 0.8),
                    Ast::Slide(SRB, (192, 448), false, false, 0.8),
                    Ast::Sound(Sound::explosion_small()),
                    Ast::Shake(6),
                ]),
            ]),
            Ast::Wait(1.0),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (456, 192), 0.5),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::W,
                      "Good riddance."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Madam, I am pleased to\n\
                       report that I have found and\n\
                       disposed of our saboteur."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::E,
                      "Thank you, Relyng.  Now be a\n\
                       dear and get us that bridge back."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                          "But of course."),
            ]),
            Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(RELYNG, (456, 192), 0.35),
                Ast::Sound(Sound::character_collision()),
                Ast::Queue(3, 1), // Turn whole board green.
                Ast::Seq((0..19).map(|index| {
                    Ast::Seq(vec![
                        Ast::Place(BRIDGE_START + index,
                                   "tiles/miniblocks", 14,
                                   (144 + 16 * index, 208)),
                        Ast::Wait(0.05),
                    ])
                }).collect()),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Well, let's get a move on!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Wait(0.2),
                Ast::Slide(ARGONY, (-16, 128), true, false, 0.75),
                Ast::Wait(0.5),
                Ast::SetPos(ARGONY, (-16, 192)),
                Ast::Slide(ARGONY, (592, 192), false, false, 2.0),
                Ast::Remove(ARGONY),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.4),
                Ast::Slide(ELINSA, (-16, 128), true, false, 0.75),
                Ast::Wait(0.75),
                Ast::SetPos(ELINSA, (-16, 192)),
                Ast::Slide(ELINSA, (592, 192), false, false, 1.75),
                Ast::Remove(ELINSA),
            ]),
            Ast::Seq(vec![
                Ast::Slide(UGRENT, (592, 192), true, false, 1.75),
                Ast::Remove(UGRENT),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Slide(YTTRIS, (592, 192), true, false, 1.5),
                Ast::Remove(YTTRIS),
            ]),
            Ast::Seq(vec![
                Ast::Wait(2.0),
                Ast::Slide(MEZURE, (358, 192), true, true, 2.0),
            ]),
            Ast::Seq(vec![
                Ast::Wait(5.0),
                Ast::Sound(Sound::talk_annoyed_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                          "You cooperating with\n\
                           that thing $iprobably$r\n\
                           wasn't a good idea."),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "But I didn't kn-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_annoyed_lo()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                          "We'll have\n\
                           words later."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Swap(RELYNG, RELYNG_FG),
            Ast::Slide(RELYNG_FG, (592, 192), true, false, 0.75),
            Ast::Remove(RELYNG_FG),
            Ast::Wait(1.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "$iSigh$r...maybe not the ideal\n\
                       first day on the job."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 192), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(0.75),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
const PASSWORD_HINTS: &[(&[usize], &str)] = &[
    (&[4, 0, 1], "Three are divided\ninto unequal angles."),
    (&[9, 2, 5], "Three are split\ninto equal memories."),
    (&[7, 8], "Two are broken\nup in plane sight."),
    (&[6, 3, 10], "Three have\nmixed the point."),
    (&[11, 12, 13, 14],
     "Four cross forwards and back,\nscrambled and jumbled."),
    (&[16, 15, 17], "Three are\non the level."),
    (&[19, 18, 20], "Three are\nsimple factors."),
    (&[22, 21, 23, 24], "Four are skating\non thin ice."),
    (&[26, 25, 27], "Three are rows\nwithin columns."),
    (&[29, 28, 30], "Three have basic\nconnections."),
    (&[32, 31, 33], "Three are full of\nlight and magic."),
    (&[34, 35], "Two have\nthe blues."),
];

pub fn num_hints() -> usize { PASSWORD_HINTS.len() }

fn coords_to_actor_pos(coords: Coords) -> (i32, i32) {
    let pt = coords_to_pt(coords);
    (pt.x() + PYRAMID_TILE_SIZE / 2, pt.y() + PYRAMID_TILE_SIZE)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_hint_scene(resources: &mut Resources, index: usize)
                          -> (i32, Scene) {
    let hint = PASSWORD_HINTS[index].1;
    let ast = vec![
        Ast::Seq(PASSWORD_HINTS[index].0.iter().enumerate().map(|(i, &j)| {
            let slot = HILIGHTS_START + i as i32;
            let coords = Coords::from_index(j).unwrap();
            Ast::Place(slot, "failure/chips", 3, coords_to_actor_pos(coords))
        }).collect()),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(HILIGHTS_START, TalkStyle::Good, TalkPos::Auto, hint),
        ]),
        Ast::Seq((0..PASSWORD_HINTS[index].0.len()).map(|i| {
            Ast::Remove(HILIGHTS_START + i as i32)
        }).collect()),
    ];
    (HINTS_START + index as i32, Ast::compile_scene(resources, ast))
}

pub fn hint_scene_for_coords(coords: Coords) -> i32 {
    for (number, &(indices, _)) in PASSWORD_HINTS.iter().enumerate() {
        for &index in indices.iter() {
            if Coords::from_index(index) == Some(coords) {
                return HINTS_START + number as i32;
            }
        }
    }
    return HINTS_START;
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::PASSWORD_HINTS;

    #[test]
    fn password_hint_indices() {
        let mut all_indices: HashSet<usize> = HashSet::new();
        for &(indices, _) in PASSWORD_HINTS.iter() {
            for &index in indices.iter() {
                assert!(!all_indices.contains(&index),
                        "Repeated index: {}",
                        index);
                all_indices.insert(index);
            }
        }
        for index in 0..36 {
            assert!(all_indices.contains(&index), "Missing index: {}", index);
        }
    }
}

// ========================================================================= //
