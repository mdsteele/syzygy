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
use gui::{Rect, Resources, Sound};

// ========================================================================= //

const BRIDGE: i32 = -1;
const ELINSA: i32 = 1;
const MEZURE: i32 = 0;
const SYSTEM: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("wrecked_angle"),
            Ast::Place(BRIDGE, "wrecked/bridge", 0, (432, 320)),
            Ast::Place(SYSTEM, "chars/system", 0, (480, 96)),
            Ast::Place(ELINSA, "chars/elinsa", 0, (348, 304)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NW,
                      "Ugh.  Stupid piece of junk.  Why\n\
                       am I even bothering fixing this?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Place(MEZURE, "chars/mezure", 0, (-16, 80)),
                Ast::Slide(MEZURE, (140, 80), true, true, 1.0),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                          "Oh, hi down there!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_thought()),
                Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NW,
                          "Oh, great."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "I'm Mezure, the new\n\
                       administrator process.\n\
                       Who are you?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Elinsa, chief engineer.\n\
                       And I'm extremely busy."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Oh.  What's that thing you're\n\
                       working on?  It looks...broken."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Yes $ithank you$r, how\n\
                       observant of you."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "It's a tri-state quantum ion\n\
                       transmission power alignment grid,\n\
                       for calibrating our attitude thrusters.\n\
                       At the moment, it happens to also be\n\
                       1) shot to heck, and therefore\n\
                       2) a complete waste of my time.\n\
                       Any other questions?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Um, any way I can help?\n\
                       I'm supposed to be helping."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Oh, $isure$r.  Why don't you run\n\
                       down to the supply depot and\n\
                       fetch me a jar of elbow grease?\n\
                       That would be a $ihuge$r  help."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Come on, don't give me that.\n\
                       I wasn't born yesterday."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "...actually, technically\n\
                       I was born today."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Anyway, if you don't\n\
                       want my help, just say so.\n\
                       There's no need to be rude."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "You want to help?  Fine.  I'll\n\
                       tell you how you can help."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "You can fix this for me.  I've got\n\
                       more important things to be doing."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (592, 304), true, false, 0.75),
            Ast::Wait(0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Okay.  Yeah!  I can\n\
                       totally figure this out."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Er, how will I know\n\
                       when it's fixed?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (480, 304), false, true, 0.5),
            Ast::Par(vec![
                Ast::Sound(Sound::talk_lo()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                          "$iSigh.$r  See that big thing?  You\n\
                           need to make it look like this."),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(ELINSA, (480, 304), 0.5),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Queue(0, 1),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE, "Um, okay."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "But...this one has a bunch\n\
                       of giant holes in it."),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Sound(Sound::talk_lo()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                          "Ugh, fine, make it\n\
                           look like this, then."),
                Ast::Seq(vec![
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(ELINSA, (480, 304), 0.5),
                    Ast::Slide(ELINSA, (592, 304), true, false, 0.5),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Queue(0, 2),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(ELINSA),
            Ast::Queue(0, -3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Sure, how hard\n\
                       could this be?"),
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
                      "If I just ignore one color, and\n\
                       focus on the other two, I think\n\
                       everything should fall into place.")
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources, visible: Rect) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Queue(0, 0),
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Hey Elinsa, I did it!"),
        ]),
        Ast::Seq(vec![
            Ast::Place(ELINSA, "chars/elinsa", 0, (592, 304)),
            Ast::Slide(ELINSA, (432, 306), false, true, 0.5),
            Ast::Queue(0, -1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Huh?  You fixed it??"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "I'm...impressed, actually.\n\
                       This really is a big help.  Uh,\n\
                       sorry for being a jerk earlier."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "It's okay.  I know you're\n\
                       probably under a lot of stress."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "Well, is there anything\n\
                       else I can try to help fix?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::bridge_crack()),
            Ast::SetSprite(BRIDGE, "wrecked/bridge", 1),
            Ast::SetPos(ELINSA, (432, 307)),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW, "!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE,
                      "...uh, like maybe\n\
                       that support beam?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::bridge_break()),
                Ast::SetSprite(BRIDGE, "wrecked/bridge", 2),
                Ast::Jump(ELINSA, (432, 416), 0.75),
                Ast::Wait(0.5),
                Ast::Sound(Sound::character_collision()),
                Ast::Shake(4),
                Ast::Wait(0.5),
                Ast::SetPos(ELINSA, (432, visible.bottom() + 32)),
                Ast::Sound(Sound::talk_lo()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW, "%#$$@&!!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SE, "Elinsa!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(MEZURE, (224, 80), 0.5),
                Ast::Slide(MEZURE, (368, 80), false, false, 0.5),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(MEZURE, (416, 144), 0.75),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(MEZURE, (384, 224), 0.75),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Are you okay!?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "I'll be fine.  Just...go find\n\
                       something else to fix."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Are you sure?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Do you actually have any way\n\
                       to pull me back up from there?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Well...not as such, no."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Well then, thank you very much\n\
                       for your kind offer of $ibeing no\n\
                       help whatsover$r, but I'm sure\n\
                       I'll be just fine on my own."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "O...okay.  Good\n\
                       luck down there."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (368, 304), 0.75),
            Ast::Slide(MEZURE, (400, 304), false, false, 0.125),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (464, 304), 0.5),
            Ast::Slide(MEZURE, (592, 304), false, false, 0.5),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      " I can get $imyself$r \n\
                       out of this."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "...I think."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::transform_step(3)),
            Ast::Queue(1, 0),
            Ast::Queue(1, 1),
            Ast::Queue(1, 2),
            Ast::Wait(0.5),
            Ast::Sound(Sound::transform_step(2)),
            Ast::Queue(1, 3),
            Ast::Wait(0.5),
            Ast::Sound(Sound::transform_step(1)),
            Ast::Queue(1, 4),
            Ast::Queue(1, 5),
            Ast::Wait(1.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
