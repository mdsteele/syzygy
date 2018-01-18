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
const ARGONY: i32 = 14;
const BOOM_START: i32 = 200;
const CHARGE: i32 = 20;
const ELINSA: i32 = 13;
const MEZURE: i32 = 12;
const RELYNG: i32 = 10;
const SHIP: i32 = 3;
const SHIP2: i32 = 4;
const SHIP3: i32 = 5;
const SRB: i32 = 1;
const SYSTEM: i32 = 2;
const THRUST_TOP: i32 = 6;
const THRUST_BOTTOM: i32 = 7;
const UGRENT: i32 = 11;
const YTTRIS: i32 = 15;

const BOOM_INDICES: &[usize] = &[0, 1, 2, 3, 4];
const CHARGE_INDICES: &[usize] = &[0, 1, 2];
const SRB_TUMBLE_INDICES: &[usize] = &[0, 5, 6, 7];
const THRUST_INDICES: &[usize] = &[0, 1, 2, 1];

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
            Ast::Wait(0.5),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SW,
                      "Now arriving in\n\
                       the Xanadu system."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
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
            Ast::SetSprite(SRB, "chars/srbdmg", 3),
            Ast::Wait(0.5),
            Ast::Remove(SYSTEM),
            Ast::Remove(SRB),
            Ast::Seq((0..6).map(|idx| {
                Ast::Remove(AIRLOCK_START + idx)
            }).collect()),
            Ast::SetBg("system_syzygy"),
            Ast::Queue(3, 1), // Show ATLATL
            Ast::Queue(4, 1), // Turn on ATLATL indicators
            Ast::Wait(1.0),
            Ast::Place(CHARGE, "finale/charge_big", 0, (120, 216)),
            Ast::Anim(CHARGE, "finale/charge_big", CHARGE_INDICES, 2),
            Ast::Wait(2.0),
            Ast::Remove(CHARGE),
            Ast::Queue(3, 0), // Hide ATLATL
            Ast::SetBg("space"),
            Ast::Place(SHIP, "prolog/ship", 0, (288, 216)),
            // TODO: show emitter charging up (outside view)
            Ast::Wait(2.0),
            Ast::Remove(SHIP),
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
            Ast::Wait(1.0),
            // TODO: show ship aligning
            Ast::Remove(SHIP),
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
                    // TODO: lower-left side of pit rises
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
            // TODO: upper and lower right side of pit move together
            Ast::Wait(0.5),
            // TODO: upper middle section falls, exploding SRB thru airlock
            Ast::SetSprite(SRB, "chars/srbdmg", 6),
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
            Ast::Wait(1.0),
            // TODO: show tiny explosion and point of light as SRB is ejected
            // TODO: ejection makes ship drift up slightly as ATLATL charges
            // TODO: ATLATL fires
            Ast::Remove(SHIP),
            // TODO: cut to Xanadu III, show beam just barely missing
            // TODO: cut to Xanadu IV, show beam just barely hitting
            // TODO: Xanadu IV warms up under beam, then beam turns off
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
            // TODO: Relyng: <something witty>
            // TODO: Mezure: "Wait, what?"
            Ast::Remove(YTTRIS),
            Ast::Remove(ARGONY),
            Ast::Remove(ELINSA),
            Ast::Remove(UGRENT),
            Ast::Remove(RELYNG),
            Ast::Remove(MEZURE),
            Ast::Queue(3, 0), // Hide ATLATL
            Ast::SetBg("space"),
            // TODO: credits
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
