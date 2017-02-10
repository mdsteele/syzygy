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

pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("the_y_factor"),
            Ast::Wait(0.25),
            Ast::Place(0, "chars/tezure", 0, (-16, 256)),
            Ast::Slide(0, (64, 256), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Whoa.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(0, (112, 272), 0.5),
            Ast::Slide(0, (150, 272), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "What's this place?")
        ]),
        Ast::Wait(0.75),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Place(1, "chars/yttris", 0, (592, 128)),
                Ast::Slide(1, (480, 128), false, true, 0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                          "Welcome to the factory,\n\
                           brave traveller!!")
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.35),
                Ast::Par(vec![
                    Ast::Sound(Sound::small_jump()),
                    Ast::Jump(0, (130, 272), 0.25),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(0, TalkStyle::Normal, TalkPos::NE, "Augh!")
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                      "Fear not, brave traveller!\n\
                       My name is Yttris, and I\n\
                       mean you no harm.")
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (136, 272), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Sorry, you just startled me.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "I'm not actually a\n\
                       traveller.  I'm Tezure, the\n\
                       new administrator process.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                      "Oh, that's okay!  I'm sure\n\
                       you're a very brave\n\
                       administrator process.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Um, thanks, I guess.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Might I ask what\n\
                       you work on here?")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW, "Of course!")
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.75),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "...What do you\n\
                       work on here, Yttris?")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                      "Oh, I do a bit of this and that.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                      "Right now I'm supposed to be\n\
                       fixing this lexical component.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "It looks fine to me.\n\
                       What's wrong with it?")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                      "It's the wrong word, silly!")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Well, maybe I could help.\n\
                       What's the correct word?")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                      "Haha, no idea!  Let's work\n\
                       on it together.  It'll be fun!")
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::transform_final()),
            Ast::Wait(2.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(1, TalkStyle::Normal, TalkPos::SW,
                      "That was easy!  We just had\n\
                       to use our imaginations."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(1, (592, 128), true, false, 0.5),
            Ast::Remove(1),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Wait, so is it fixed now?")
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "I guess it's fixed now.")
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (163, 272), true, false, 0.4),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(0, (221, 272), 0.5),
            Ast::Slide(0, (372, 272), false, true, 0.8),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NW,
                      "I dunno, I thought ``maximize''\n\
                       was a perfectly good word.")
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (464, 272), true, false, 0.65),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(0, (512, 272), 0.5),
            Ast::Slide(0, (592, 272), true, false, 0.35),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
