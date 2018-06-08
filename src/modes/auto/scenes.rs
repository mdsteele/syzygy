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

const ARGONY: i32 = 2;
const MEZURE: i32 = 3;
const RELYNG_BG: i32 = -1;
const RELYNG_FG: i32 = 1;
const SYSTEM: i32 = 0;
const YTTRIS: i32 = 4;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("autofac_tour"),
            Ast::Place(SYSTEM, "chars/system", 0, (288, 80)),
            Ast::Wait(0.5),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 256)),
            Ast::Slide(MEZURE, (260, 256), false, true, 1.25),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Is this still\n\
                       the factory?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 256)),
            Ast::Slide(YTTRIS, (158, 256), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Sure is!  This is\n\
                       the aft end of it.\n\
                       Last section."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Is it...working?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Nope!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "That word up there\n\
                       is totally wrong."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Figures."),
        ]),
        Ast::Par(vec![
            Ast::Loop(0, 1, Box::new(Ast::Seq(vec![
                Ast::Slide(MEZURE, (330, 256), true, true, 1.0),
            ]))),
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "Hmm..."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "What's wrong with it, anyway?\n\
                       An education is a good thing!"),
        ]),
        Ast::Seq(vec![
            Ast::Place(ARGONY, "chars/argony", 0, (592, 256)),
            Ast::Slide(ARGONY, (442, 256), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Galactic peace would be a good\n\
                       thing too, child, but that doesn't\n\
                       mean it's the correct word here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      " Okay, well, what $iis$r \n\
                       the correct word?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Beats me!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Much as it pains me to admit\n\
                       it, I can't recall either."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Well, between the three\n\
                       of us, I guess we'll just\n\
                       have to figure it out."),
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
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "There are over a hundred possible\n\
                       combinations to try, but I believe\n\
                       that only one of them will work."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Ooh!  We should try\n\
                       $ithat$r  one first!"),
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::NW,
                      "You'd think that all these\n\
                       ``correct words'' would be\n\
                       $idocumented$r  somewhere..."),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Maybe we could mash the buttons\n\
                       randomly?  It might give us an\n\
                       idea of what the word could be."),
        ]),
    ];
    (YTTRIS, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::transform_final()),
            Ast::Wait(2.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Booyah!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Factory automation\n\
                       systems reengaged."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Excellent."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Wait, this was supposed\n\
                       to say ``security'' this\n\
                       whole time?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "As in, the security\n\
                       here was disabled\n\
                       until just now?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Apparently.  Why?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "`Scuse me!"),
            ]),
            Ast::Loop(0, 1, Box::new(Ast::Seq(vec![
                Ast::Slide(YTTRIS, (274, 256), true, false, 0.75),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (386, 256), 0.75),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (498, 256), 0.75),
                Ast::Slide(YTTRIS, (592, 256), false, false, 0.5),
                Ast::Remove(YTTRIS),
            ]))),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Earlier, Ugrent mentioned\n\
                           that he thought some of this\n\
                           damage might be sabotage."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Could someone have\n\
                       disabled the security\n\
                       here on purpose?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Hmm.  Perhaps."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NW,
                      "Fortunately, I know\n\
                       $ijust$r  who to ask\n\
                       about that."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (592, 256), true, false, 1.0),
            Ast::Remove(ARGONY),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Wait, who are you\n\
                       talking about?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (592, 256), true, false, 1.0),
            Ast::Remove(MEZURE),
            Ast::Wait(1.5),
            Ast::Place(RELYNG_BG, "chars/relyng", 4, (344, 288)),
            Ast::Slide(RELYNG_BG, (344, 272), false, false, 1.0),
            Ast::Wait(1.0),
            Ast::Sound(Sound::small_jump()),
            Ast::SetSprite(RELYNG_BG, "chars/relyng", 0),
            Ast::Jump(RELYNG_BG, (344, 256), 0.5),
            Ast::Swap(RELYNG_BG, RELYNG_FG),
            Ast::Wait(0.5),
            Ast::Slide(RELYNG_FG, (592, 256), true, false, 1.0),
            Ast::Remove(RELYNG_FG),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
