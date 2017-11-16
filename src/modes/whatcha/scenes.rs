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

const MEZURE: i32 = 1;
const MEZURE_BG: i32 = -1;
const RELYNG: i32 = 2;

const DOOR_UPPER: i32 = -2;
const DOOR_LOWER: i32 = -3;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("whatcha_column_1"),
            Ast::Place(DOOR_UPPER, "tiles/caution_walls", 5, (408, 288)),
            Ast::Place(DOOR_LOWER, "tiles/caution_walls", 4, (408, 304)),
            Ast::Wait(0.5),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 240)),
            Ast::Slide(MEZURE, (176, 240), false, true, 1.0),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Now what's $ithis$r\n\
                       all about?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (208, 256), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (240, 272), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Some kind of sealed\n\
                       chamber, looks like."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (272, 288), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (304, 304), 0.5),
            Ast::Slide(MEZURE, (348, 304), false, true, 0.5),
            Ast::Par(vec![
                Ast::Slide(DOOR_UPPER, (408, 272), true, false, 0.5),
                Ast::Slide(DOOR_LOWER, (408, 320), true, false, 0.5),
            ]),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Is that...a $itree!?$r"),
        ]),
        Ast::Seq(vec![
            Ast::Remove(MEZURE),
            Ast::Place(MEZURE_BG, "chars/mezure", 0, (348, 304)),
            Ast::Slide(MEZURE_BG, (450, 304), true, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE_BG, TalkStyle::Normal, TalkPos::NW,
                      "This must be some kind of bio-dome.\n\
                       Why would we need that on this ship?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE_BG, TalkStyle::Normal, TalkPos::NW,
                      "I'd better take a look inside."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE_BG, (592, 304), true, false, 0.75),
            Ast::Remove(MEZURE_BG),
            Ast::Wait(0.5),
            Ast::Remove(DOOR_UPPER),
            Ast::Remove(DOOR_LOWER),
            Ast::SetBg("whatcha_column_2"),
            Ast::Queue(0, 1), // Show columns.
            Ast::Wait(0.5),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 304)),
            Ast::Slide(MEZURE, (138, 304), false, true, 1.0),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "This thing seems fine, I guess."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (180, 304), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Hmm.  I'm not sure\n\
                       if I should be here."),
        ]),
        Ast::Seq(vec![
            Ast::Place(RELYNG, "chars/relyng", 0, (592, 160)),
            Ast::Slide(RELYNG, (528, 160), false, false, 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (496, 192), 0.5),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                              "Aha!"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                              "Augh!"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(MEZURE, (160, 304), 0.25),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "You again!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Me again."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (464, 208), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "What's this I hear about how\n\
                       you're not supposed to be here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "A stowaway, are you?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "What?  No!  I just meant that if\n\
                       there's nothing broken in this area,\n\
                       then I ought to be coordinating\n\
                       repairs elsewhere."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Why are $iyou$r  always sneaking\n\
                       around, anyway?  Have you\n\
                       got something to hide?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "We've all got $isomething$r\n\
                       to hide, kid.  In my case,\n\
                       it's the only way to find\n\
                       what I'm looking for."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Yeah?  And what's that?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "The truth, of course."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "What's that supposed to mean?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (496, 192), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "By the way, ``this thing'' isn't\n\
                       ``fine.''  It's broken, and it\n\
                       needs to be fixed."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (528, 160), 0.5),
            Ast::Slide(RELYNG, (592, 160), true, false, 0.5),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, how was I supposed\n\
                       to know that?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "And I still don't even\n\
                       know that guy's name!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (176, 304), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh.$r  Guess I'd better\n\
                       try to fix these columns\n\
                       before I move on."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::NE,
                      "What $ido$r  you\n\
                       call `em?"),
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
            Ast::Seq((0..6).map(|index| Ast::Seq(vec![
                Ast::Wait(0.075),
                Ast::Queue(1, index),
            ])).collect()),
            Ast::Wait(0.25),
            Ast::Seq((0..6).map(|index| Ast::Seq(vec![
                Ast::Wait(0.075),
                Ast::Queue(2, index),
            ])).collect()),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, that's that, I guess."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I wish I knew what that\n\
                       sneaky guy's deal was."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (400, 304), true, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Gotta get back to\n\
                       work, though."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (432, 320), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (464, 352), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (496, 400), 0.5),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Place(RELYNG, "chars/relyng", 0, (592, 160)),
            Ast::Slide(RELYNG, (528, 160), false, false, 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (496, 192), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Heh.  Nice work, kid."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (464, 208), 0.5),
            Ast::Slide(RELYNG, (352, 208), true, false, 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (304, 176), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                      "The question is, though:\n\
                       What are you $ireally$r\n\
                       doing on this ship?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(RELYNG, (256, 176), true, false, 0.4),
            Ast::Slide(RELYNG, (224, 208), false, false, 0.2),
            Ast::Slide(RELYNG, (208, 208), false, false, 0.1),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (168, 304), 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NE,
                      "I intend to find out."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(RELYNG, (400, 304), true, false, 1.0),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (496, 400), 0.85),
            Ast::Remove(RELYNG),
            Ast::Wait(0.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
