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
use crate::elements::cutscene::JumpNode;
use crate::gui::{Resources, Sound};

// ========================================================================= //

const ARGONY: i32 = 4;
const BRIDGE: i32 = -1;
const ELEVATOR_LEFT: i32 = 1;
const ELEVATOR_RIGHT: i32 = 2;
const ELINSA: i32 = 5;
const MEZURE: i32 = 3;
const RELYNG: i32 = -2;
const SHIP: i32 = 8;
const SYSTEM: i32 = 0;
const THRUST_TOP: i32 = 9;
const THRUST_BOTTOM: i32 = 10;
const UGRENT: i32 = 7;
const YTTRIS: i32 = 6;

const ELEVATOR_INDICES: &[usize] = &[2, 3];
const RELYNG_INDICES: &[usize] = &[4, 7];
const THRUST_INDICES: &[usize] = &[0, 1, 2, 1];

const BOOM_START: i32 = 1000;
const BOOM_INDICES: &[usize] = &[0, 1, 2, 3, 4];
const BOOM_POSITIONS: &[(i32, i32)] = &[(290, 228), (250, 208), (318, 188)];

const FIRE_START: i32 = 2000;
const FIRE_INDICES: [&[usize]; 4] =
    [&[0, 1, 2, 3], &[1, 2, 3, 0], &[2, 3, 0, 1], &[3, 0, 1, 2]];
const FIRE_POSITIONS: &[(i32, i32)] = &[
    (112, 176),
    (144, 176),
    (440, 176),
    (120, 272),
    (176, 272),
    (208, 272),
    (240, 272),
    (276, 288),
    (348, 288),
    (380, 288),
    (424, 304),
];

const SHOWER_START: i32 = 3000;
const SHOWER_INDICES: &[usize] = &[0, 1, 2];
const SMOKE_INDICES: &[usize] = &[3, 4, 4];

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("space"),
            Ast::Queue(2, 1), // Show large moving starfield
            Ast::Place(SYSTEM, "chars/invis", 0, (224, 320)),
            Ast::Place(SHIP, "prolog/ship", 0, (288, 216)),
            Ast::Place(THRUST_TOP, "prolog/thrust", 0, (334, 198)),
            Ast::Anim(THRUST_TOP, "prolog/thrust", THRUST_INDICES, 3),
            Ast::Place(THRUST_BOTTOM, "prolog/thrust", 0, (334, 208)),
            Ast::Anim(THRUST_BOTTOM, "prolog/thrust", THRUST_INDICES, 3),
            Ast::Wait(1.0),
            Ast::Queue(6, 1), // Show "Somewhere in deep space..." text
            Ast::Wait(3.5),
            Ast::Queue(6, 0), // Hide "Somewhere in deep space..." text
            Ast::Remove(SHIP),
            Ast::Remove(THRUST_TOP),
            Ast::Remove(THRUST_BOTTOM),
            Ast::SetBg("prolog_security"),
            Ast::Queue(2, 2), // Show moving stars through windows
            Ast::Queue(1, 1), // Show status indicator
            Ast::Place(SYSTEM, "chars/system", 0, (464, 208)),
            Ast::Place(UGRENT, "chars/ugrent", 0, (224, 240)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Security to Bridge,\n\
                       any sign of nearby\n\
                       enemy vessels?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SYSTEM, TalkStyle::Comm, TalkPos::W,
                      " For the last time Ugrent, \n \
                       no, of course there isn't! "),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SYSTEM, TalkStyle::Comm, TalkPos::W,
                      "We're noncombatants in the\n\
                       middle of friendly territory,\n\
                       not a warship on the front!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Hmph.  Can't be\n\
                       too careful."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (252, 240), true, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "System, any sign\n\
                       of enemy vessels?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NW, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Are there any enemy ships\n\
                       within 50 light years?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NW, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (224, 240), true, true, 0.4),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Well, where $iis$r  the\n\
                       nearest enemy ship?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NW, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(UGRENT, TalkStyle::Thought, TalkPos::NE,
                      "Whose idea was it to\n\
                       program this thing\n\
                       in Prolog, anyway?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Remove(SYSTEM),
            Ast::Remove(UGRENT),
            Ast::Queue(1, 0), // Hide status indicator
            Ast::SetBg("space"),
            Ast::Queue(2, 1), // Show large moving starfield
            Ast::Place(SHIP, "prolog/ship", 0, (288, 216)),
            Ast::Place(THRUST_TOP, "prolog/thrust", 0, (334, 198)),
            Ast::Anim(THRUST_TOP, "prolog/thrust", THRUST_INDICES, 3),
            Ast::Place(THRUST_BOTTOM, "prolog/thrust", 0, (334, 208)),
            Ast::Anim(THRUST_BOTTOM, "prolog/thrust", THRUST_INDICES, 3),
            Ast::Wait(0.5),
            Ast::Sound(Sound::explosion_small()),
            Ast::Queue(2, 0), // Hide moving stars
            Ast::Shake(16),
            Ast::Remove(THRUST_TOP),
            Ast::Remove(THRUST_BOTTOM),
            Ast::SetBg("white"),
            Ast::Wait(0.05),
            Ast::SetBg("space"),
            Ast::Par(BOOM_POSITIONS.iter().enumerate().map(|(index, &pos)| {
                let slot = BOOM_START + index as i32;
                Ast::Seq(vec![
                    Ast::Wait(0.1 * index as f64),
                    Ast::Place(slot, "chars/boom", 0, pos),
                    Ast::Anim(slot, "chars/boom", BOOM_INDICES, 1),
                    Ast::Wait(0.2),
                    Ast::Remove(slot),
                ])
            }).collect()),
            Ast::Wait(1.0),
            Ast::Remove(SHIP),
            Ast::SetBg("prolog_security"),
            Ast::Queue(1, 2), // Show status indicator
            Ast::Place(SYSTEM, "chars/system", 0, (464, 208)),
            Ast::Place(UGRENT, "chars/ugrent", 0, (224, 240)),
            Ast::Wait(0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "What in blazes was that!?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NW, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (272, 240), true, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Ugrent to Bridge, what's\n\
                       going on up there?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::explosion_small()),
            Ast::Shake(10),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Bridge, this is security!\n\
                       Do you read?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.6),
            Ast::Slide(UGRENT, (208, 240), true, true, 0.4),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Comm must be out up there.\n\
                       I'd better go check..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (160, 304), 0.75),
            Ast::Slide(UGRENT, (-16, 304), false, false, 0.4),
            Ast::Queue(1, 0), // Hide status indicator
            Ast::SetBg("prolog_bridge"),
            Ast::Place(SYSTEM, "chars/system", 0, (432, 112)),
            Ast::Seq(FIRE_POSITIONS.iter().enumerate().map(|(index, &pos)| {
                let slot = FIRE_START + index as i32;
                Ast::Seq(vec![
                    Ast::Place(slot, "chars/fire", 0, pos),
                    Ast::Anim(slot, "chars/fire", &FIRE_INDICES[index % 4], 2),
                ])
            }).collect()),
            Ast::Place(UGRENT, "chars/ugrent", 0, (592, 304)),
            Ast::Slide(UGRENT, (490, 304), false, true, 0.3),
            Ast::Wait(0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW, "Gah!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (592, 304), true, false, 0.3),
            Ast::Seq((0..FIRE_POSITIONS.len()).map(|index| {
                Ast::Remove(FIRE_START + index as i32)
            }).collect()),
            Ast::SetBg("prolog_security"),
            Ast::Queue(1, 3), // Show status indicator
            Ast::Place(SYSTEM, "chars/system", 0, (464, 208)),
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 304)),
            Ast::Slide(UGRENT, (144, 304), false, true, 0.3),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "System, get me\n\
                       ship-wide broadcast!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NW,
                      "Broadcast ready."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "All hands, this is security!"),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 0), // Hide status indicator
            Ast::Remove(UGRENT),
            Ast::SetBg("wrecked_angle"),
            Ast::Queue(3, 1), // Show wrecked grid
            Ast::Place(BRIDGE, "wrecked/bridge", 0, (432, 320)),
            Ast::Place(SYSTEM, "chars/system", 0, (480, 96)),
            Ast::Place(ELINSA, "chars/elinsa", 0, (348, 304)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(SYSTEM, TalkStyle::Comm, TalkPos::SW,
                              "We are experiencing a\n\
                               ship-wide emergency."),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::explosion_small()),
                    Ast::Shake(10),
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                              "Gee, ya think?"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(BRIDGE),
            Ast::Remove(ELINSA),
            Ast::Queue(3, 0), // Hide wrecked grid
            Ast::SetBg("the_y_factor"),
            Ast::Place(SYSTEM, "chars/system", 0, (80, 80)),
            Ast::Place(YTTRIS, "chars/yttris", 0, (488, 128)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(SYSTEM, TalkStyle::Comm, TalkPos::SE,
                              " Bridge is down.  Our mission \n\
                               may be in jeopardy."),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.75),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                              "Oh no!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(YTTRIS),
            Ast::SetBg("a_light_in_the_attic"),
            Ast::Queue(4, 1), // Show attic grid
            Ast::Place(SYSTEM, "chars/system", 0, (496, 80)),
            Ast::Place(ARGONY, "chars/argony", 0, (168, 112)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(SYSTEM, TalkStyle::Comm, TalkPos::SW,
                              "We need all hands on\n \
                               deck working on repairs. "),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::explosion_small()),
                    Ast::Shake(10),
                    Ast::Dark(true),
                    Ast::Light(ARGONY, true),
                    Ast::Queue(4, -1), // Unsolve attic grid
                    Ast::Wait(0.75),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                              "Sigh...so much for\n\
                               retirement."),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(ARGONY),
            Ast::Dark(false),
            Ast::Queue(4, 0), // Hide attic grid
            Ast::SetBg("cross_the_line"),
            Ast::Place(SYSTEM, "chars/system", 0, (288, 128)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(SYSTEM, TalkStyle::Comm, TalkPos::SW,
                              "And we need to figure out\n \
                               what the heck just happened! "),
                ]),
                Ast::Seq(vec![
                    Ast::Place(RELYNG, "chars/relyng", 6, (333, 288)),
                    Ast::Slide(RELYNG, (333, 272), false, false, 0.75),
                    Ast::Wait(0.5),
                    Ast::Anim(RELYNG, "chars/relyng", RELYNG_INDICES, 15),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(RELYNG, "chars/relyng", 6),
            Ast::Slide(RELYNG, (333, 288), true, false, 0.5),
            Ast::Remove(RELYNG),
            Ast::Wait(0.25),
            Ast::SetBg("prolog_security"),
            Ast::Queue(1, 4), // Show status indicator
            Ast::Place(SYSTEM, "chars/system", 0, (464, 208)),
            Ast::Place(UGRENT, "chars/ugrent", 0, (144, 304)),
            Ast::Place(FIRE_START + 0, "chars/fire", 0, (-6, 304)),
            Ast::Anim(FIRE_START + 0, "chars/fire", &FIRE_INDICES[0], 2),
            Ast::Wait(0.25),
            Ast::Place(FIRE_START + 1, "chars/fire", 0, (26, 304)),
            Ast::Anim(FIRE_START + 1, "chars/fire", &FIRE_INDICES[1], 2),
            Ast::Wait(0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "End broadcast."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.25),
            Ast::Place(FIRE_START + 2, "chars/fire", 0, (58, 304)),
            Ast::Anim(FIRE_START + 2, "chars/fire", &FIRE_INDICES[2], 2),
            Ast::Wait(0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "System, get a new administrator process\n\
                       spawned.  We're going to need one."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.25),
            Ast::Place(FIRE_START + 3, "chars/fire", 0, (90, 304)),
            Ast::Anim(FIRE_START + 3, "chars/fire", &FIRE_INDICES[3], 2),
            Ast::Wait(0.75),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                              "And get that fire put out!"),
                ]),
                Ast::Slide(UGRENT, (592, 304), true, false, 1.0),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(UGRENT),
            Ast::Seq((0..3).map(|index| {
                let slot = SHOWER_START + index;
                let pos = (32 + 32 * index, 304);
                Ast::Seq(vec![
                    Ast::Place(slot, "chars/shower", 0, pos),
                    Ast::Anim(slot, "chars/shower", SHOWER_INDICES, 2),
                ])
            }).collect()),
            Ast::Wait(1.0),
            Ast::Anim(FIRE_START + 3, "chars/boom", SMOKE_INDICES, 5),
            Ast::Wait(0.4),
            Ast::Remove(FIRE_START + 3),
            Ast::Wait(0.6),
            Ast::Queue(1, 0), // Hide status indicator
            Ast::Seq((0..3).map(|index| {
                Ast::Seq(vec![
                    Ast::Remove(SHOWER_START + index),
                    Ast::Remove(FIRE_START + index),
                ])
            }).collect()),
            Ast::SetBg("prolog_spawn"),
            Ast::Place(SYSTEM, "chars/system", 0, (240, 96)),
            Ast::Place(ELEVATOR_LEFT, "shift/platforms", 2, (456, 336)),
            Ast::Place(ELEVATOR_RIGHT, "shift/platforms", 2, (488, 336)),
            Ast::Wait(1.0),
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 304)),
            Ast::Slide(UGRENT, (120, 304), false, true, 1.0),
            Ast::Wait(0.25),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Spawning new administrator\n\
                       process.  Please stand back."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (72, 304), true, true, 0.5),
            Ast::Queue(5, 1), // Show spawning process
            Ast::Sound(Sound::spawn_zap()),
            Ast::Wait(0.8),
            Ast::Sound(Sound::transform_final()),
            Ast::Wait(2.0),
            Ast::Queue(5, 0), // Hide spawning process
            Ast::Place(MEZURE, "chars/mezure", 0, (240, 264)),
            Ast::Wait(0.25),
            Ast::Jump(MEZURE, (240, 272), JumpNode::time_to_fall(8)),
            Ast::Wait(0.5),
            Ast::Slide(UGRENT, (134, 304), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Where...am I?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "You're aboard the $iH.L.S. System$r,\n\
                       which is currently adrift somewhere\n\
                       just south of the middle of nowhere."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (200, 288), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "And...who are you?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Ugrent.  Chief of security.\n\
                       Any other questions?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I think my name's...Mezure?\n\
                       Do I have a job here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "You do now.  You're our\n\
                       new administrator."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (192, 288), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (240, 272), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (288, 288), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (340, 304), 0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Congratulations and\n\
                       welcome to the crew.\n\
                       Now get back to work."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                          "Follow me."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Slide(UGRENT, (488, 304), true, true, 1.0),
                Ast::Sound(Sound::platform_shift(4)),
                Ast::Anim(ELEVATOR_RIGHT, "shift/platforms",
                          ELEVATOR_INDICES, 2),
                Ast::Par(vec![
                    Ast::Slide(UGRENT, (488, 416), false, false, 0.8),
                    Ast::Slide(ELEVATOR_RIGHT, (488, 448), false, false, 0.8),
                ]),
                Ast::Remove(ELEVATOR_RIGHT),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(UGRENT),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Gee, this place sure\n\
                       is friendly, isn't it?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (240, 272), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (288, 288), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (340, 304), 0.5),
            Ast::Slide(MEZURE, (456, 304), false, true, 0.75),
            Ast::Wait(0.25),
            Ast::Sound(Sound::platform_shift(4)),
            Ast::Anim(ELEVATOR_LEFT, "shift/platforms",
                      ELEVATOR_INDICES, 2),
            Ast::Par(vec![
                Ast::Slide(MEZURE, (456, 416), false, false, 0.8),
                Ast::Slide(ELEVATOR_LEFT, (456, 448), false, false, 0.8),
            ]),
            Ast::Remove(ELEVATOR_LEFT),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
