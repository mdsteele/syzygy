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

pub const ELINSA_SLOT: i32 = 0;

// ========================================================================= //

pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("shifting_ground_1"),
            Ast::Queue(-1, 0), // Hide platforms/arrows
            Ast::Place(0, "chars/elinsa", 0, (432, 320)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NW, "Ow, my head..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NW,
                      "Okay, so, productivity checklist\n\
                       for today so far:"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NW,
                      "On the upside, that alignment\n\
                       grid got fixed.  Sort of."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NW,
                      "On the downside, now I'm\n\
                       $istuck down here$r."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NW,
                      "What's even down here,\n\
                       anyway?  The sewers?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NW,
                      "I guess there should be some\n\
                       maintenance carriages somewhere\n\
                       nearby...maybe I can rig them to\n\
                       help me climb back up."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (592, 320), true, false, 0.75),
            Ast::Place(0, "chars/elinsa", 0, (-16, 320)),
            Ast::SetBg("shifting_ground_2"),
            Ast::Queue(-1, 1), // Show platforms/arrows
            Ast::Place(1, "chars/yttris", 0, (112, 320)),
            Ast::Slide(0, (80, 320), false, false, 0.25),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Augh!"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(1, TalkStyle::Normal, TalkPos::E,
                              "Oh, hi Elinsa!"),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "...hi Yttris.  What are\n\
                       you doing down here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "Oh, I just thought I'd\n\
                       go out for a walk."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "In...the sewers?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "Yeah!  Aren't the tunnels lovely\n\
                       this time of year?  The color of\n\
                       the mold contrasts beautifully\n\
                       with the brick walls, and-"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Yttris, we're in the middle of\n\
                       a ship-wide emergency.  There are\n\
                       system breakages everywhere.  This\n\
                       is no time to be going for a walk!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                          "Oh no, that's terrible!"),
            ]),
            Ast::Loop(0, 2, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(1, (112, 320), 0.5),
            ]))),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Slide(1, (80, 320), false, false, 0.15),
                Ast::Slide(0, (112, 320), false, false, 0.15),
            ]),
            Ast::Wait(0.1),
            Ast::Par(vec![
                Ast::Slide(1, (112, 320), false, false, 0.15),
                Ast::Jump(0, (144, 304), 0.25),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "We've got to get you back upstairs\n\
                       and on the job right away!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "And I should probably be replacing\n\
                       those smashed life-support sensors!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Wait, what?  Did you just\n\
                       say the life support-"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "No time for that now, Elinsa!\n\
                       You just sit tight.  I'll help you get\n\
                       these platforms arranged so you can-"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Um, thanks Yttris, but I think\n\
                       I've had enough ``help'' for one\n\
                       day already.  I can handle these\n\
                       carriages on my own."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "Oh!  You must have met that\n\
                       new admin process, Tezure!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Yeah."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "Okay, I totally understand.\n\
                       You go ahead and take care\n\
                       of these platforms.  I'll be\n\
                       right behind you!"),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SW, "Done and done."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::SW,
                      "Time to get back to work."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (592, 64), true, false, 0.5),
            Ast::Remove(0),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "Wow, that was a really\n\
                       neat solution, Elinsa!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                      "That was a lot more elegant\n\
                       and clever than how I was\n\
                       going to do it.  I like it!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Queue(0, 9),
                Ast::Queue(1, 8),
                Ast::Queue(2, 7),
                Ast::Queue(3, 6),
                Ast::Queue(4, 5),
                Ast::Queue(5, 4),
                Ast::Queue(6, 3),
                Ast::Wait(1.5),
            ]),
            Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(1, (144, 304), 0.3),
                Ast::Wait(0.7),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(1, TalkStyle::Normal, TalkPos::NE,
                          "I was just going to do it this way."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (224, 320), 0.75),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (272, 280), 0.75),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (304, 248), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (336, 216), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (368, 184), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (400, 152), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (432, 120), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (464, 88), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(1, (496, 64), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW, "Woohoo!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                      "Oh wait, the system is still broken!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                          "Ahh!  Everybody panic!"),
            ]),
            Ast::Seq(vec![
                Ast::Slide(1, (592, 64), false, false, 0.5),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(1),
            Ast::Queue(-2, 0), // Move all platforms to final position.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
