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

const AIRLOCK_START: i32 = 100;
const ARGONY: i32 = 13;
const BOOM_START: i32 = 200;
const CHARGE: i32 = 20;
const ELINSA: i32 = 14;
const FIRE_1: i32 = 301;
const FIRE_2: i32 = 302;
const FIRE_3: i32 = 303;
const MEZURE: i32 = 11;
const RELYNG: i32 = 10;
const RELYNG_BG: i32 = -1;
const SHIP: i32 = 3;
const SHIP2: i32 = 4;
const SHIP3: i32 = 5;
const SHOWER_START: i32 = 400;
const SRB: i32 = 1;
const SYSTEM: i32 = 2;
const THRUST_TOP: i32 = 6;
const THRUST_BOTTOM: i32 = 7;
const UGRENT: i32 = 12;
const XANADU_III: i32 = 8;
const XANADU_IV: i32 = 9;
const XANADU_IV_GLOW: i32 = 16;
const YTTRIS: i32 = 15;
const YTTRIS_BG: i32 = -2;

const BOOM_INDICES: &[usize] = &[0, 1, 2, 3, 4];
const CHARGE_INDICES: &[usize] = &[0, 1, 2];
const FIRE_INDICES: &[usize] = &[0, 1, 2, 3];
const SHOWER_INDICES: &[usize] = &[0, 1, 2];
const SMOKE_INDICES: &[usize] = &[3, 4, 4];
const SRB_TUMBLE_INDICES: &[usize] = &[0, 5, 6, 7];
const TINYBOOM_INDICES: &[usize] = &[0, 1, 2];
const THRUST_INDICES: &[usize] = &[0, 1, 2, 1];
const X4_GLOW_INDICES: &[usize] = &[0, 1, 2];
const X4_INDICES: &[&[usize]] = &[&[0, 1], &[1, 2], &[2, 3], &[3, 4]];

const SHOWER_POSITIONS: &[(i32, i32)] =
    &[(112, 240), (144, 240), (416, 240), (448, 240), (480, 240)];

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("space"),
            Ast::Queue(1, 1), // Show moving starfield
            Ast::Place(SHIP, "prolog/ship", 0, (288, 216)),
            Ast::Place(THRUST_TOP, "prolog/thrust", 0, (334, 198)),
            Ast::Anim(THRUST_TOP, "prolog/thrust", THRUST_INDICES, 3),
            Ast::Place(THRUST_BOTTOM, "prolog/thrust", 0, (334, 208)),
            Ast::Anim(THRUST_BOTTOM, "prolog/thrust", THRUST_INDICES, 3),
            Ast::Wait(2.0),
            Ast::Queue(1, 0), // Hide moving starfield
            Ast::Remove(THRUST_TOP),
            Ast::Remove(THRUST_BOTTOM),
            Ast::SetBg("white"),
            Ast::Wait(0.05),
            Ast::SetBg("space"),
            Ast::Wait(1.0),
            Ast::Remove(SHIP),
            Ast::SetBg("prolog_bridge"),
            Ast::Place(SYSTEM, "chars/system", 0, (432, 112)),
            Ast::Place(FIRE_1, "chars/fire", 0, (120, 272)),
            Ast::Anim(FIRE_1, "chars/fire", FIRE_INDICES, 2),
            Ast::Place(FIRE_2, "chars/fire", 0, (176, 272)),
            Ast::Anim(FIRE_2, "chars/fire", FIRE_INDICES, 2),
            Ast::Place(FIRE_3, "chars/fire", 0, (424, 304)),
            Ast::Anim(FIRE_3, "chars/fire", FIRE_INDICES, 2),
            Ast::Seq(SHOWER_POSITIONS.iter().enumerate().map(|(index, &pos)| {
                let slot = SHOWER_START + index as i32;
                Ast::Seq(vec![
                    Ast::Place(slot, "chars/shower", 0, pos),
                    Ast::Anim(slot, "chars/shower", SHOWER_INDICES, 2),
                ])
            }).collect()),
            Ast::Wait(0.5),
            Ast::Anim(FIRE_2, "chars/boom", SMOKE_INDICES, 5),
            Ast::Wait(0.4),
            Ast::Remove(FIRE_2),
            Ast::Wait(0.1),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SW,
                      "Now arriving in\n\
                       the Xanadu system."),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Anim(FIRE_3, "chars/boom", SMOKE_INDICES, 5),
                    Ast::Wait(0.4),
                    Ast::Remove(FIRE_3),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Anim(FIRE_1, "chars/boom", SMOKE_INDICES, 5),
                    Ast::Wait(0.4),
                    Ast::Remove(FIRE_1),
                ]),
            ]),
            Ast::Seq((0..SHOWER_POSITIONS.len()).map(|index| {
                Ast::Remove(SHOWER_START + index as i32)
            }).collect()),
            Ast::Remove(SYSTEM),
            Ast::SetBg("finale_pit"),
            Ast::Place(SYSTEM, "chars/system", 0, (96, 128)),
            Ast::Place(SRB, "chars/srbdmg", 0, (144, 272)),
            Ast::Seq((0..6).map(|idx| {
                Ast::Place(AIRLOCK_START + idx, "tiles/miniblocks", 14,
                           (248 + 16 * idx, 304))
            }).collect()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                      "Ow, my head..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 1),
            Ast::Wait(0.75),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Executing program\n\
                       ``SYZYGY''..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE, "Oh?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Remove(SYSTEM),
            Ast::Remove(SRB),
            Ast::Seq((0..6).map(|idx| {
                Ast::Remove(AIRLOCK_START + idx)
            }).collect()),
            Ast::SetBg("space"),
            Ast::Queue(2, 1), // Show sun and planets
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(SHIP, "title/ship", 0, (-22, 480)),
                    Ast::Slide(SHIP, (26, 352), false, true, 6.0),
                ]),
                Ast::Seq(vec![
                    Ast::Place(SHIP2, "title/ship", 1, (31, 480)),
                    Ast::Slide(SHIP2, (79, 352), false, true, 6.0),
                ]),
                Ast::Seq(vec![
                    Ast::Place(SHIP3, "title/ship", 2, (84, 480)),
                    Ast::Slide(SHIP3, (132, 352), false, true, 6.0),
                ]),
            ]),
            Ast::Wait(1.0),
            Ast::Remove(SHIP),
            Ast::Remove(SHIP2),
            Ast::Remove(SHIP3),
            Ast::Queue(2, 0), // Hide sun and planets
            Ast::SetBg("finale_pit"),
            Ast::Place(SYSTEM, "chars/system", 0, (96, 128)),
            Ast::Place(SRB, "chars/srbdmg", 3, (144, 272)),
            Ast::Seq((0..6).map(|idx| {
                Ast::Place(AIRLOCK_START + idx, "tiles/miniblocks", 14,
                           (248 + 16 * idx, 304))
            }).collect()),
            Ast::Wait(0.5),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Ship now aligned into\n\
                       planetary syzygy."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Preparing to\n\
                       fire ATLATL..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 2),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                      "Hahaha, you fools\n\
                       are too late!!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                      "Your planet is doomed!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 3),
            Ast::Wait(0.5),
            Ast::Remove(SYSTEM),
            Ast::Remove(SRB),
            Ast::Seq((0..6).map(|idx| {
                Ast::Remove(AIRLOCK_START + idx)
            }).collect()),
            Ast::SetBg("space"),
            Ast::Queue(2, 1), // Show sun and planets
            Ast::Place(SHIP, "title/ship", 0, (26, 352)),
            Ast::Place(SHIP2, "title/ship", 1, (79, 352)),
            Ast::Place(SHIP3, "title/ship", 2, (132, 352)),
            Ast::Wait(1.0),
            Ast::Place(CHARGE, "finale/charge_med", 0, (115, 330)),
            Ast::Anim(CHARGE, "finale/charge_med", CHARGE_INDICES, 2),
            Ast::Wait(2.0),
            Ast::Remove(CHARGE),
            Ast::Remove(SHIP),
            Ast::Remove(SHIP2),
            Ast::Remove(SHIP3),
            Ast::Queue(2, 0), // Hide sun and planets
            Ast::SetBg("system_syzygy"),
            Ast::Queue(3, 1), // Show ATLATL
            Ast::Queue(4, 1), // Turn on ATLATL indicators
            Ast::Place(CHARGE, "finale/charge_big", 0, (120, 216)),
            Ast::Anim(CHARGE, "finale/charge_big", CHARGE_INDICES, 2),
            Ast::Wait(2.0),
            Ast::Remove(CHARGE),
            Ast::Queue(3, 0), // Hide ATLATL
            Ast::SetBg("space"),
            Ast::Place(SHIP, "prolog/ship", 0, (288, 216)),
            Ast::Place(CHARGE, "finale/charge_tiny", 0, (256, 211)),
            Ast::Anim(CHARGE, "finale/charge_tiny", CHARGE_INDICES, 3),
            Ast::Wait(2.0),
            Ast::Remove(SHIP),
            Ast::Remove(CHARGE),
            Ast::SetBg("finale_pit"),
            Ast::Place(SYSTEM, "chars/system", 0, (96, 128)),
            Ast::Place(SRB, "chars/srbdmg", 3, (144, 272)),
            Ast::Seq((0..6).map(|idx| {
                Ast::Place(AIRLOCK_START + idx, "tiles/miniblocks", 14,
                           (248 + 16 * idx, 304))
            }).collect()),
            Ast::Wait(0.5),
            Ast::SetSprite(SRB, "chars/srbdmg", 2),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                      "The humans will\n\
                       soon learn to fear\n\
                       the Alliance!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                          "My victory here will-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::beep()),
                Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                          "Alert: Loading\n\
                           new program..."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 4),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                      "Say what now?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 1),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Executing program\n\
                       ``SYSTEM SYZYGY''..."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 4),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                      "Huh?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 1),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Aligning ship system\n\
                       components into syzygy..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Remove(SYSTEM),
            Ast::Remove(SRB),
            Ast::Seq((0..6).map(|idx| {
                Ast::Remove(AIRLOCK_START + idx)
            }).collect()),
            Ast::SetBg("space"),
            Ast::Place(SHIP, "prolog/ship", 0, (288, 216)),
            Ast::Place(CHARGE, "finale/charge_tiny", 0, (256, 211)),
            Ast::Anim(CHARGE, "finale/charge_tiny", CHARGE_INDICES, 3),
            Ast::Wait(1.0),
            // TODO: show ship aligning
            Ast::Remove(SHIP),
            Ast::Remove(CHARGE),
            Ast::SetBg("finale_pit"),
            Ast::Place(SYSTEM, "chars/system", 0, (96, 128)),
            Ast::Place(SRB, "chars/srbdmg", 1, (144, 272)),
            Ast::Seq((0..6).map(|idx| {
                Ast::Place(AIRLOCK_START + idx, "tiles/miniblocks", 14,
                           (248 + 16 * idx, 304))
            }).collect()),
            Ast::Wait(0.5),
            Ast::Anim(SRB, "chars/srbdmg", SRB_TUMBLE_INDICES, 3),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::platform_shift(1)),
                    Ast::Wait(0.1),
                    Ast::SetBg("finale_pit_2"),
                    Ast::Wait(0.1),
                    Ast::Sound(Sound::platform_shift(1)),
                    Ast::Wait(0.1),
                    Ast::SetBg("finale_pit_3"),
                ]),
                Ast::Slide(SRB, (288, 288), false, false, 0.5),
            ]),
            Ast::SetSprite(SRB, "chars/srbdmg", 8),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                      "Whoa!  Watch what\n\
                       you're doing, there!"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srbdmg", 9),
            Ast::Sound(Sound::platform_shift(1)),
            Ast::Wait(0.1),
            Ast::SetBg("finale_pit_4"),
            Ast::Wait(0.1),
            Ast::Sound(Sound::platform_shift(1)),
            Ast::Wait(0.1),
            Ast::SetBg("finale_pit_5"),
            Ast::Wait(0.75),
            Ast::Sound(Sound::platform_shift(1)),
            Ast::Wait(0.1),
            Ast::SetBg("finale_pit_6"),
            Ast::Wait(0.1),
            Ast::Sound(Sound::platform_shift(1)),
            Ast::Wait(0.1),
            Ast::SetBg("finale_pit_7"),
            Ast::Wait(0.1),
            Ast::Sound(Sound::platform_shift(1)),
            Ast::Wait(0.1),
            Ast::SetBg("finale_pit_8"),
            Ast::Sound(Sound::character_collision()),
            Ast::SetSprite(SRB, "chars/srbdmg", 0),
            Ast::Par(vec![
                Ast::Par((0..3).map(|index| {
                    let slot = BOOM_START + index;
                    Ast::Seq(vec![
                        Ast::Place(slot, "chars/boom", 0,
                                   (256 + 32 * index, 312)),
                        Ast::Anim(slot, "chars/boom", BOOM_INDICES, 1),
                        Ast::Wait(0.2),
                        Ast::Remove(slot),
                    ])
                }).collect()),
                Ast::Seq((0..6).map(|idx| {
                    Ast::Remove(AIRLOCK_START + idx)
                }).collect()),
                Ast::Sound(Sound::explosion_small()),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Evil, TalkPos::NE,
                          "Waaah!!"),
                Ast::Slide(SRB, (288, 416), false, false, 0.5),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(SRB),
            Ast::Wait(0.5),
            Ast::Remove(SYSTEM),
            Ast::SetBg("space"),
            Ast::Place(SHIP, "prolog/ship", 0, (288, 216)),
            Ast::Place(CHARGE, "finale/charge_tiny", 0, (256, 211)),
            Ast::Anim(CHARGE, "finale/charge_tiny", CHARGE_INDICES, 3),
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Par((0..2).map(|index| {
                    let slot = BOOM_START + index;
                    Ast::Seq(vec![
                        Ast::Place(slot, "finale/tinyboom", 0,
                                   (285 + 5 * index, 218)),
                        Ast::Anim(slot, "finale/tinyboom",
                                  TINYBOOM_INDICES, 2),
                        Ast::Wait(0.25),
                        Ast::Remove(slot),
                    ])
                }).collect()),
                Ast::Seq(vec![
                    Ast::Place(SRB, "finale/tinysrb", 0, (288, 216)),
                    Ast::Slide(SRB, (288, 400), false, false, 2.0),
                    Ast::Remove(SRB),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Talk(SRB, TalkStyle::Evil, TalkPos::E, "Aaaah!"),
                ]),
                Ast::Slide(SHIP, (288, 200), false, false, 4.0),
                Ast::Seq(vec![
                    Ast::Slide(CHARGE, (256, 199), false, false, 3.0),
                    Ast::Remove(CHARGE),
                    Ast::Queue(5, 1), // Animate ATLATL beam from ship
                ]),
            ]),
            Ast::Queue(5, 0), // Hide ATLATL beam
            Ast::Remove(SHIP),
            Ast::SetBg("space2"),
            Ast::Place(XANADU_III, "title/xanadu3", 0, (288, 258)),
            Ast::Wait(0.5),
            Ast::Queue(5, 2), // Animate ATLATL beam across screen
            Ast::Wait(2.5),
            Ast::Queue(5, 0), // Hide ATLATL beam
            Ast::Remove(XANADU_III),
            Ast::SetBg("space"),
            Ast::Place(XANADU_IV, "finale/xanadu4big", 0, (288, 242)),
            Ast::Wait(0.5),
            Ast::Queue(5, 3), // Animate ATLATL beam hitting planet
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::spawn_zap()),
                    Ast::Wait(0.8),
                    Ast::Sound(Sound::transform_final()),
                ]),
                Ast::Seq(vec![
                    Ast::Place(XANADU_IV_GLOW, "finale/xanadu4glow", 0,
                               (288, 242)),
                    Ast::Anim(XANADU_IV_GLOW, "finale/xanadu4glow",
                              X4_GLOW_INDICES, 1),
                    Ast::Wait(0.75),
                    Ast::Anim(XANADU_IV, "finale/xanadu4big",
                              X4_INDICES[0], 1),
                    Ast::Wait(0.75),
                    Ast::Anim(XANADU_IV, "finale/xanadu4big",
                              X4_INDICES[1], 1),
                    Ast::Wait(0.75),
                    Ast::Anim(XANADU_IV, "finale/xanadu4big",
                              X4_INDICES[2], 1),
                    Ast::Wait(0.75),
                    Ast::Anim(XANADU_IV, "finale/xanadu4big",
                              X4_INDICES[3], 1),
                    Ast::Wait(0.75),
                    Ast::SetSprite(XANADU_IV, "finale/xanadu4big", 5),
                    Ast::Queue(5, 0), // Hide ATLATL beam
                    Ast::Remove(XANADU_IV_GLOW),
                    Ast::Wait(3.0)
                ]),
            ]),
            Ast::Remove(XANADU_IV),
            Ast::SetBg("system_syzygy"),
            Ast::Queue(3, 1), // Show ATLATL
            Ast::Wait(0.75),
            Ast::Queue(4, 0), // Turn off ATLATL indicators
            Ast::Sound(Sound::small_jump()),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Place(YTTRIS, "chars/yttris", 0, (224, 208)),
                    Ast::Jump(YTTRIS, (100, 80), 1.0),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.05),
                    Ast::Place(ARGONY, "chars/argony", 0, (262, 208)),
                    Ast::Jump(ARGONY, (175, 80), 1.0),
                ]),
                Ast::Seq(vec![
                    Ast::Place(ELINSA, "chars/elinsa", 0, (300, 208)),
                    Ast::Jump(ELINSA, (250, 80), 1.0),
                ]),
                Ast::Seq(vec![
                    Ast::Place(UGRENT, "chars/ugrent", 0, (338, 208)),
                    Ast::Jump(UGRENT, (325, 80), 1.0),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.05),
                    Ast::Place(RELYNG, "chars/relyng", 0, (376, 208)),
                    Ast::Jump(RELYNG, (400, 80), 1.0),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Place(MEZURE, "chars/mezure", 0, (414, 208)),
                    Ast::Jump(MEZURE, (475, 80), 1.0),
                ]),
            ]),
            Ast::Wait(0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "We did it!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Yippee!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                      "That was a rather...unorthodox\n\
                       solution, Mezure."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Hey, I thought it was\n\
                       pretty clever.  And just\n\
                       in the nick of time, too."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "I'd say the\n\
                       child deserves\n\
                       all our thanks."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Aw, shucks.  It was a\n\
                       team effort, after all."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "So, um, what happens now?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "Now our work $ireally$r  begins.\n\
                       We need to introduce the flora in\n\
                       the bio-dome onto the surface so\n\
                       the incoming colonists will have\n\
                       an ecosystem to work with."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Ah, I was wondering what\n\
                       that thing was for."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (115, 80), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Oh no, I forgot!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "I never fixed those\n\
                       life-support sensors!"),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Aaaaaaaaaa!"),
            Ast::Seq(vec![
                Ast::Slide(YTTRIS, (592, 80), true, false, 1.0),
                Ast::Wait(0.5),
                Ast::Remove(YTTRIS),
                Ast::Wait(1.0),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                          "I should go contact HQ and\n\
                           inform them of our success."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (592, 80), true, false, 1.0),
            Ast::Remove(UGRENT),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "And I'd better get the\n\
                       nav system fixed."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (-16, 80), true, false, 1.0),
            Ast::Remove(ELINSA),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "...I have my own\n\
                       affairs to look into."),
        ]),
        Ast::Seq(vec![
            Ast::Swap(RELYNG, RELYNG_BG),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG_BG, (400, 96), 0.5),
            Ast::SetSprite(RELYNG_BG, "chars/relyng", 6),
            Ast::Slide(RELYNG_BG, (400, 112), false, false, 0.2),
            Ast::Remove(RELYNG_BG),
            Ast::Wait(1.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "Yes, child, now the real work begins.\n\
                       And these vagabonds are going to need\n\
                       your organizational oversight more than\n\
                       ever if we're going to get it all done."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "No more puzzles, though?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "No, I think we're all\n\
                       done with puzzles, now."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (100, 80), true, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "Unless, of course, they ever\n\
                       write a sequel to this game."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (-16, 80), true, false, 0.75),
            Ast::Remove(ARGONY),
            Ast::Wait(1.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Wait, what?"),
        ]),
        Ast::Seq(vec![
            Ast::Remove(MEZURE),
            Ast::Queue(3, 0), // Hide ATLATL
            Ast::SetBg("point_of_order"),
            Ast::Wait(0.5),
            Ast::Sound(Sound::bridge_crack()),
            Ast::Queue(11, 1), // Show "SYSTEM"
            Ast::Wait(0.5),
            Ast::Sound(Sound::bridge_crack()),
            Ast::Queue(11, 2), // Show "SYZYGY"
            Ast::Wait(0.5),
            Ast::Sound(Sound::bridge_crack()),
            Ast::Queue(11, 3), // Show "a game by mdsteele"
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(YTTRIS_BG, "chars/invis", 0, (-16, 304)),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS_BG, TalkStyle::Normal, TalkPos::NE,
                              "Aaaaaaaaaaaa!"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::SetPos(YTTRIS_BG, (-15, 304)),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS_BG, TalkStyle::Normal, TalkPos::NE,
                              "Aaaaaaaaaaaa!  Aaaaaa!"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 336)),
                    Ast::Slide(YTTRIS, (64, 336), false, false, 0.3),
                    Ast::Slide(YTTRIS, (80, 320), false, false, 0.15),
                    Ast::Slide(YTTRIS, (262, 320), false, true, 1.0),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                              "Aaa...wait, never mind,\n\
                               the life-support is fine."),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(YTTRIS_BG),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                              "Sorry, false alarm!"),
                ]),
                Ast::Seq(vec![
                    Ast::Slide(YTTRIS, (80, 320), true, false, 0.6),
                    Ast::Slide(YTTRIS, (64, 336), false, false, 0.1),
                    Ast::Slide(YTTRIS, (-16, 336), false, false, 0.3),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(YTTRIS),
            Ast::Queue(11, 0), // Hide title credit
            Ast::SetBg("log_level"),
            Ast::Queue(6, 1), // Show crossword
            Ast::Wait(0.5),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Queue(7, 0), // Animate crossword
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(MEZURE, "chars/mezure", 0, (-16, 160)),
                    Ast::Slide(MEZURE, (145, 160), false, true, 0.75),
                    Ast::Wait(0.75),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                              "See, don't we\n\
                               need guard\n\
                               rails here?"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 160)),
                    Ast::Slide(ELINSA, (84, 160), false, true, 0.75),
                    Ast::Wait(0.75),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                              "Eh, seems\n\
                               unnecessary."),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(ELINSA),
            Ast::Remove(MEZURE),
            Ast::Queue(6, 0), // Hide crossword
            Ast::SetBg("star_crossed"),
            Ast::Queue(8, 1), // Show letter columns
            Ast::Queue(9, 0), // Animate letter columns
            Ast::Place(ARGONY, "chars/argony", 0, (240, 272)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                              "Now then-"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.75),
                    Ast::Slide(ARGONY, (224, 272), false, false, 0.25),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                              "-oh my!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 272)),
            Ast::Slide(UGRENT, (112, 272), false, true, 0.75),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                              "Security measures\n\
                               are back in place,\n\
                               Lady Argony."),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::talk_annoyed_hi()),
                    Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                              "Yes, I can see that."),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(UGRENT),
            Ast::Remove(ARGONY),
            Ast::Queue(10, 0), // Stop animating letter columns
            Ast::Queue(8, 0), // Hide letter columns
            Ast::SetBg("cross_sauce"),
            Ast::Wait(0.5),
            Ast::Sound(Sound::transform_step(1)),
            Ast::Queue(12, 1), // Show special thanks
            Ast::Wait(0.2),
            Ast::Sound(Sound::transform_step(2)),
            Ast::Queue(12, 2),
            Ast::Wait(0.2),
            Ast::Sound(Sound::transform_step(3)),
            Ast::Queue(12, 3),
            Ast::Wait(0.2),
            Ast::Sound(Sound::transform_step(4)),
            Ast::Queue(12, 4),
            Ast::Wait(0.75),
            Ast::Place(RELYNG, "chars/relyng", 0, (592, 240)),
            Ast::Slide(RELYNG, (465, 240), false, true, 1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::W,
                      "Ah, good, I was just coming\n\
                       down here to make sure you\n\
                       didn't forget this part."),
        ]),
        Ast::Seq(vec![
            Ast::Remove(RELYNG),
            Ast::Queue(12, 0), // Hide special thanks
            Ast::SetBg("space"),
            Ast::Place(XANADU_IV, "finale/xanadu4big", 6, (288, 242)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_thought()),
            Ast::Queue(13, 1), // Show "THE END"
            Ast::Wait(0.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
