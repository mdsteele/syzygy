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

const MEZURE: i32 = 0;
const YTTRIS: i32 = 1;

const WEST_DOOR_UPPER: i32 = -4;
const WEST_DOOR_LOWER: i32 = -3;
const EAST_DOOR_UPPER: i32 = -2;
const EAST_DOOR_LOWER: i32 = -1;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("connect_the_dots"),
            Ast::Place(WEST_DOOR_UPPER, "tiles/caution_walls", 5, (64, 80)),
            Ast::Place(WEST_DOOR_LOWER, "tiles/caution_walls", 4, (64, 96)),
            Ast::Place(EAST_DOOR_UPPER, "tiles/caution_walls", 5, (512, 80)),
            Ast::Place(EAST_DOOR_LOWER, "tiles/caution_walls", 4, (512, 96)),
            Ast::Place(YTTRIS, "chars/yttris", 0, (310, 96)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Hmm, I wonder what color decor\n\
                       would go best with this bridge..."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (344, 96), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Maybe some magenta curtains?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_thought()),
                Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::SE,
                          "It's so tricky when\n\
                           there's only sixteen\n\
                           colors to choose from."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Place(MEZURE, "chars/mezure", 0, (-16, 96)),
                Ast::Slide(MEZURE, (244, 96), true, true, 1.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                          "Oh, hello there."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Par(vec![
                    Ast::Slide(WEST_DOOR_UPPER, (64, 64), false, false, 0.25),
                    Ast::Slide(WEST_DOOR_LOWER, (64, 112), false, false, 0.25),
                ]),
                Ast::Wait(0.5),
                Ast::Par(vec![
                    Ast::Slide(WEST_DOOR_UPPER, (64, 80), false, false, 0.25),
                    Ast::Slide(WEST_DOOR_LOWER, (64, 96), false, false, 0.25),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Pardon me, but do you happen to know\n\
                       the way to the engine room? I'm\n\
                       supposed to be coordinating repairs."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Hi!  I'm Yttris!"),
            Ast::Loop(0, 2, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (344, 96), 0.5),
            ]))),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Yes, of course!  The engine\n\
                       room is right this way."),
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Slide(YTTRIS, (492, 96), true, false, 0.5),
                Ast::Sound(Sound::character_collision()),
                Ast::Slide(YTTRIS, (474, 96), false, true, 0.25),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW, "Huh?"),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "This door is supposed to\n\
                       open automatically..."),
            Ast::Seq(vec![
                Ast::Slide(YTTRIS, (492, 96), true, false, 0.25),
                Ast::Sound(Sound::character_collision()),
                Ast::Slide(YTTRIS, (474, 96), false, true, 0.25),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Why isn't this stupid\n\
                       door working?"),
            Ast::Seq(vec![
                Ast::Slide(YTTRIS, (492, 96), true, false, 0.25),
                Ast::Sound(Sound::character_collision()),
                Ast::Slide(YTTRIS, (474, 96), false, true, 0.25),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "I, uh, think I've seen\n\
                       this problem before."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (444, 96), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "Wait a minute, do I know you?"),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Well-"),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                          "Are you..."),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                      "$i...THE DOOR-FIXER?"),
            Ast::Loop(0, 3, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (444, 96), 0.5),
            ]))),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Well, not exactly-"),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                          "That's great!  I'll just leave\n\
                           this in your capable hands, then."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Slide(YTTRIS, (-16, 96), true, false, 1.0),
                    Ast::Remove(YTTRIS),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Par(vec![
                        Ast::Slide(WEST_DOOR_UPPER, (64, 64),
                                   false, false, 0.25),
                        Ast::Slide(WEST_DOOR_LOWER, (64, 112),
                                   false, false, 0.25),
                    ]),
                    Ast::Wait(0.5),
                    Ast::Par(vec![
                        Ast::Slide(WEST_DOOR_UPPER, (64, 80),
                                   false, false, 0.25),
                        Ast::Slide(WEST_DOOR_LOWER, (64, 96),
                                   false, false, 0.25),
                    ]),
                ]),
            ]),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "$iSigh.$r  Let's take a\n\
                       look at this thing..."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1), // Make laser field visible.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Well, at least this one is\n\
                       already two thirds done."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "If I could just figure out\n\
                       where the splitter goes..."),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Slide(EAST_DOOR_UPPER, (512, 64), true, false, 0.5),
                Ast::Slide(EAST_DOOR_LOWER, (512, 112), true, false, 0.5),
            ]),
            Ast::Remove(EAST_DOOR_UPPER),
            Ast::Remove(EAST_DOOR_LOWER),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "All righty, then."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (210, 96), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Hey, Yttris!  I\n\
                       fixed the door."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW, "Yttris?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Slide(MEZURE, (244, 96), true, true, 0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Well, maybe I can find\n\
                       someone else to help me\n\
                       find the engine room."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 96), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Seq((0..7).map(|index| {
                Ast::Seq(vec![
                    Ast::Queue(1, index),
                    Ast::Wait(0.1),
                ])
            }).collect()),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
