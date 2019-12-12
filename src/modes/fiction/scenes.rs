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
use crate::gui::{Resources, Sound};

// ========================================================================= //

const ARGONY: i32 = 1;
const MEZURE: i32 = 2;
const SYSTEM: i32 = 0;
const YTTRIS: i32 = 3;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("fact_or_fiction"),
            Ast::Place(SYSTEM, "chars/system", 0, (288, 80)),
            Ast::Wait(2.0),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SW,
                      "ERROR: Internal sensor log data\n\
                       is inconsistent/corrupted."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SW,
                      "Possible malicious\n\
                       data fabrication\n\
                       has been detected."),
        ]),
        Ast::Seq(vec![
            Ast::Place(ARGONY, "chars/argony", 0, (592, 64)),
            Ast::Slide(ARGONY, (445, 64), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Hoo boy.  We can't\n\
                       have that, now.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SW,
                      "Begin repair process."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Let's get cracking.")
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_argony_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "We need to get the\n\
                       $ireal$r  story here."),
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::transform_final()),
            Ast::Wait(2.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Well now, I'd say\n\
                       that's looking better."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 128)),
                Ast::Slide(YTTRIS, (218, 128), false, true, 1.0),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                          "...so then the Alliance broke\n\
                           off from the rest of our\n\
                           Joint Federation, and now\n\
                           we're at war with them.")
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Place(MEZURE, "chars/mezure", 0, (-16, 128)),
                Ast::Slide(MEZURE, (160, 128), false, true, 1.0),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Uh, okay...")
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(YTTRIS, (290, 128), true, true, 0.75),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SW,
                          "The war's been dragging on for,\n\
                           like, forever, but don't worry!\n\
                           We're nowhere near the front.\n\
                           This isn't even a military ship.")
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.35),
                Ast::Slide(MEZURE, (210, 128), true, true, 0.75),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "So wait, how did this\n\
                       war get started again?")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "For that we can thank\n\
                       the Zenith Incident of\n\
                       four years ago.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Oh, hi Argony!  I didn't\n\
                       see you up there.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "And this...galactic\n\
                       federation we're in has\n\
                       been around since...when?")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "About two centuries.  The key\n\
                       technology that really made it\n\
                       possible was the development\n\
                       of the ATLATL in 2235.")
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                          "The what?")
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.2),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                          "Oh!  I almost forgot!\n\
                           I was supposed to show you\n\
                           those broken sensors!")
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                          "Huh?")
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.35),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                          "You know, the broken\n\
                           life-support sensors.")
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                          "$iWhat!?")
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.35),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                          "C'mon, let's go!")
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Slide(YTTRIS, (592, 128), true, false, 0.75),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(YTTRIS),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Uh, please excuse\n\
                       me, ma'am.")
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 128), true, false, 0.75),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Well, seems like\n\
                       everything's under\n\
                       control here.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "I'm sure those\n\
                       kids will be fine.")
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (592, 64), true, false, 1.0),
            Ast::Remove(ARGONY),
            Ast::Wait(1.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
