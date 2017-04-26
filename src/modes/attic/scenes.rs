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
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("a_light_in_the_attic"),
            Ast::Dark(true),
            Ast::Place(0, "chars/mezure", 0, (-16, 320)),
            Ast::Light(0, true),
            Ast::Slide(0, (88, 320), true, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Whoa.  It sure is dark up here.")
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (104, 320), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Hmm.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "STIK, huh?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "What's this place even for?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Maybe I can just pass through...")
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (176, 320), true, false, 0.5),
            Ast::Par(vec![
                Ast::Sound(Sound::character_collision()),
                Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Ow!"),
                Ast::Jump(0, (160, 320), 0.5),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Place(1, "chars/argony", 0, (-16, 112)),
            Ast::Light(1, true),
            Ast::Slide(1, (170, 112), false, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                      "Stumbling about in the dark, child?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "In more ways than one."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "I'm supposed to be helping to\n\
                       repair the system, but I barely\n\
                       even know what I'm doing."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE, "I see."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                      "Perhaps I might offer\n\
                       a suggestion, child?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Oh!  Um, yes please."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                      "Perhaps, instead of blundering\n\
                       around into walls like an idiot, you\n\
                       should turn the lights on first."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Oh."),
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
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                      "You should probably\n\
                       start with the top row.")
        ]),
    ];
    (1, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(0, TalkStyle::Thought, TalkPos::NE,
                      "Why couldn't there just be\n\
                       a light switch or something?")
        ]),
    ];
    (0, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.25),
            Ast::Queue(0, 1),
            Ast::Wait(0.1),
            Ast::Queue(3, 1),
            Ast::Wait(0.1),
            Ast::Queue(4, 1),
            Ast::Wait(0.1),
            Ast::Queue(9, 1),
            Ast::Wait(0.1),
            Ast::Queue(10, 1),
            Ast::Wait(0.1),
            Ast::Queue(13, 1),
            Ast::Wait(0.1),
            Ast::Queue(15, 1),
            Ast::Wait(0.1),
            Ast::Dark(false),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Hey, I can see!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                      "And, once again, age and wisdom and\n\
                       patience triumph over...whatever it\n\
                       is that you kids are into these days."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Thank you, I appreciate\n\
                       the help, Ms-"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "...er, I don't think I\n\
                       caught your name, sorry."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                      "That's because you didn't\n\
                       think to ask before."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                      "The name's Argony, child."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "I'm Mezure.  Pleased to meet you."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SE,
                      "Likewise.  Now get back to work,\n\
                       whippersnapper.  The system is a wreck."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Yes, ma'am!"),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(0, (208, 304), 0.5),
                Ast::Slide(0, (256, 304), false, false, 0.25),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(0, (304, 288), 0.4),
                Ast::Slide(0, (592, 288), false, false, 1.0),
                Ast::Remove(0),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(1, TalkStyle::Thought, TalkPos::SE,
                      "I wonder where Relyng's\n\
                       gone off to?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(1, (-16, 112), true, false, 0.5),
            Ast::Remove(1),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
