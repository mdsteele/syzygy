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

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("prolog_space"),
            Ast::Queue(2, 1), // Show large moving starfield
            Ast::Place(0, "chars/invis", 0, (224, 240)),
            Ast::Wait(1.0), // TODO: Show the ship
            Ast::Talk(0, TalkStyle::System, TalkPos::NE,
                      "Somewhere in deep space..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::SetBg("prolog_security"),
            Ast::Queue(2, 2), // Show moving stars through windows
            Ast::Queue(1, 1), // Show status indicator
            Ast::Place(0, "chars/system", 0, (464, 208)),
            Ast::Place(1, "chars/ugrent", 0, (224, 240)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(1, TalkStyle::Thought, TalkPos::NE, "Zzzz..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Remove(0),
            Ast::Remove(1),
            Ast::Queue(1, 0), // Hide status indicator
            Ast::SetBg("prolog_space"),
            Ast::Queue(2, 1), // Show large moving starfield
            Ast::Wait(0.5),
            Ast::Sound(Sound::explosion_small()), // TODO: Show ship 'splosion
            Ast::Queue(2, 0), // Hide moving stars
            Ast::Shake(16),
            Ast::Wait(0.5),
            Ast::SetBg("prolog_security"),
            Ast::Queue(1, 2), // Show status indicator
            Ast::Place(0, "chars/system", 0, (464, 208)),
            Ast::Place(1, "chars/ugrent", 0, (224, 240)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "What in blazes was that!?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(0, TalkStyle::System, TalkPos::NW, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(1, (272, 240), true, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "Ugrent to Bridge, what's\n\
                       going on up there?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::explosion_small()),
            Ast::Shake(10),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "Bridge, this is security!\n\
                       Do you read?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.6),
            Ast::Slide(1, (208, 240), true, true, 0.4),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "Comm must be out up there.\n\
                       I'd better go check..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (160, 304), 0.75),
            Ast::Slide(1, (-16, 304), false, false, 0.4),
            Ast::Queue(1, 0), // Hide status indicator
            Ast::SetBg("prolog_bridge"),
            Ast::Place(0, "chars/system", 0, (432, 112)),
            Ast::Seq(FIRE_POSITIONS.iter().enumerate().map(|(index, &pos)| {
                let slot = FIRE_SLOTS_START + index as i32;
                Ast::Seq(vec![
                    Ast::Place(slot, "chars/fire", 0, pos),
                    Ast::Anim(slot, "chars/fire", &FIRE_INDICES[index % 4], 2),
                ])
            }).collect()),
            Ast::Place(1, "chars/ugrent", 0, (592, 304)),
            Ast::Slide(1, (490, 304), false, true, 0.3),
            Ast::Wait(0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NW, "Gah!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(1, (592, 304), true, false, 0.3),
            Ast::Seq((0..FIRE_POSITIONS.len()).map(|index| {
                Ast::Remove(FIRE_SLOTS_START + index as i32)
            }).collect()),
            Ast::SetBg("prolog_security"),
            Ast::Queue(1, 3), // Show status indicator
            Ast::Place(0, "chars/system", 0, (464, 208)),
            Ast::Place(1, "chars/ugrent", 0, (-16, 304)),
            Ast::Slide(1, (144, 304), false, true, 0.3),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "System, get me ship-wide broadcast!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(0, TalkStyle::System, TalkPos::NW, "Broadcast ready."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "All hands, this is security!"),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 0), // Hide status indicator
            Ast::SetBg("wrecked_angle"),
            Ast::Place(-1, "wrecked/bridge", 0, (432, 320)),
            Ast::Place(0, "chars/system", 0, (480, 96)),
            Ast::Place(1, "chars/elinsa", 0, (348, 304)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(0, TalkStyle::Comm, TalkPos::SW,
                              "We are experiencing a\n\
                               ship-wide emergency."),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::explosion_small()),
                    Ast::Shake(10),
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(1, TalkStyle::Normal, TalkPos::NW,
                              "Gee, ya think?"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(-1),
            Ast::SetBg("the_y_factor"),
            Ast::Place(0, "chars/system", 0, (80, 80)),
            Ast::Place(1, "chars/yttris", 0, (488, 128)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(0, TalkStyle::Comm, TalkPos::SE,
                              "Bridge is down.  Our mission\n\
                               may be in jeopardy."),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.75),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(1, TalkStyle::Normal, TalkPos::SW, "Oh no!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::SetBg("a_light_in_the_attic"), // TODO
            Ast::Place(0, "chars/system", 0, (496, 272)),
            Ast::Place(1, "chars/argony", 0, (168, 112)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(0, TalkStyle::Comm, TalkPos::NW,
                              "We need all hands on deck\n\
                               working on repairs."),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.75),
                    Ast::Sound(Sound::talk_lo()),
                    Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                              "Sigh...so much for\n\
                               retirement."),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(1),
            Ast::SetBg("tread_lightly"), // TODO
            Ast::Place(0, "chars/system", 0, (488, 164)),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(0, TalkStyle::Comm, TalkPos::NW,
                              "And we need to figure out\n\
                               what the heck just happened!"),
                ]),
                Ast::Seq(vec![
                    Ast::Place(-1, "chars/relyng", 0, (200, 336)),
                    Ast::Slide(-1, (200, 316), false, false, 0.75),
                    Ast::Wait(0.5),
                    Ast::Anim(-1, "chars/relyng", RELYNG_INDICES_1, 15),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Anim(-1, "chars/relyng", RELYNG_INDICES_2, 0),
            Ast::Slide(-1, (200, 336), true, false, 0.5),
            Ast::Remove(-1),
            Ast::Wait(0.25),
            Ast::SetBg("prolog_security"),
            Ast::Queue(1, 4), // Show status indicator
            Ast::Place(0, "chars/system", 0, (464, 208)),
            Ast::Place(1, "chars/ugrent", 0, (144, 304)),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE, "End broadcast."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "System, get a new administrator process\n\
                       spawned.  We're going to need one."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                              "And get that fire put out!"),
                ]),
                Ast::Slide(1, (592, 304), true, false, 1.0),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(1),
            // TODO: Write the rest of this cutscene.
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

const FIRE_SLOTS_START: i32 = 10;
const FIRE_POSITIONS: &[(i32, i32)] =
    &[(112, 176), (144, 176), (440, 176), (120, 272), (176, 272), (208, 272),
      (240, 272), (276, 288), (348, 288), (380, 288), (424, 304)];
const FIRE_INDICES: [&[usize]; 4] =
    [&[0, 1, 2, 3], &[1, 2, 3, 0], &[2, 3, 0, 1], &[3, 0, 1, 2]];

const RELYNG_INDICES_1: &[usize] = &[1, 2];
const RELYNG_INDICES_2: &[usize] = &[0];

// ========================================================================= //
