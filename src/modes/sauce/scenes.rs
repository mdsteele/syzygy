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

const ARGONY: i32 = 1;
const UGRENT: i32 = 2;
const YTTRIS: i32 = 3;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("cross_sauce"),
            Ast::Place(UGRENT, "chars/ugrent", 0, (436, 240)),
            Ast::Queue(1, 1),  // Display "RHYME TIME".
            Ast::Wait(1.0),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 240)),
            Ast::Slide(YTTRIS, (188, 240), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Ooh, rhymes!  Are\n\
                       we doing poetry?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 0),  // Clear display.
            Ast::Wait(0.5),
            Ast::Queue(1, 2),  // Display "THYME CLIMB".
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "I'm inspecting this\n\
                       security checkpoint.\n\
                       It's delicate work."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Can I help?\n\
                       I love poetry!"),
            Ast::Loop(0, 2, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (188, 240), 0.5),
            ]))),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "No!  This isn't\n\
                       poetry, Yttris!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Queue(1, 0),  // Clear display.
                Ast::Wait(0.5),
                Ast::Queue(1, 3),  // Display "SUBLIME ENZYME".
            ]),
            Ast::Seq(vec![
                Ast::Place(ARGONY, "chars/argony", 0, (-16, 240)),
                Ast::Slide(ARGONY, (144, 240), false, true, 1.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                          "Oh, let her\n\
                           help, Ugrent."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Slide(YTTRIS, (244, 240), true, true, 0.75),
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Yeah!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "With respect, Lady Argony,\n\
                       I am $itrying$r  to keep us all\n\
                       safe, and this inspection\n\
                       needs to be done $ijust so."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Doing things ``just so'' is not\n\
                       exactly Yttris' strong suit."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "All facts of which I am well\n\
                       aware, dear.  And you don't\n\
                       need to call me ``Lady.''"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "But in my experience, having an\n\
                       extra set of eyes and a fresh\n\
                       perspective on a problem usually\n\
                       leads to a better result."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "$L* Plus, in case you *\n      * haven't heard, *\n\
                       * I'm great at finding *\n      * rhyming words! *"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "...Please don't undercut\n\
                       my argument, Yttris."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 0),  // Clear display.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Fine, if you insist..."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1),  // Show clues.
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
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "If we get stuck on one of these,\n\
                       we can always just try every letter\n\
                       until we get the first one.  That\n\
                       might be enough to get us unstuck."),
        ]),
    ];
    (ARGONY, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(UGRENT, TalkStyle::Thought, TalkPos::NW,
                      "$iGrumble$r...I don't\n\
                       even $ilike$r  rhymes."),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "$L* Although his manner *\n      * tends to chafe, *\n\
                       * Ugrent works to *\n      * keep us safe! *"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Ugh, Yttris, no more\n\
                       rhymes than we have to!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Anybody want a...cashew?"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::W,
                          "Gah!"),
            ]),
        ]),
    ];
    (YTTRIS, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Queue(0, 0),  // Hide clues.
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "That looks to be\n\
                       the lot of them."),
        ]),
        Ast::Par(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Hooray, I got to help!"),
            Ast::Loop(0, 2, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(YTTRIS, (244, 240), 0.5),
            ]))),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Yes.  Well.  Everything\n\
                       seems to be fine here."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (454, 240), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "I must be moving on\n\
                       to the next section.\n\
                       Good day, ladies."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(UGRENT, (592, 240), true, false, 0.75),
            Ast::Remove(UGRENT),
            Ast::Wait(1.0),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 4),  // Display "TOUGH BLUFF".
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Huh?  We did this\n\
                       one already."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::NE,
                      "If that's everything here, I\n\
                       should be moving on as well."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(ARGONY, (-16, 240), true, false, 1.0),
                Ast::Remove(ARGONY),
            ]),
            Ast::Seq(vec![
                Ast::Wait(1.0),
                Ast::Sound(Sound::beep()),
                Ast::Queue(1, 5),  // Display "ENOUGH STUFF".
                Ast::Wait(0.75),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Yeah, I think we've done more\n\
                           than enough stuff here.  I think\n\
                           I might need a break from poetry!"),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Sound(Sound::beep()),
                Ast::Queue(1, 6),  // Display "BOUGH FLUFF?".
            ]),
            Ast::Seq(vec![
                Ast::Slide(YTTRIS, (344, 240), true, true, 1.0),
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NW,
                          "You can't fool me!\n\
                           Those don't even rhyme."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (592, 240), true, false, 1.0),
            Ast::Wait(0.75),
            Ast::Queue(1, 0),  // Clear display.
            Ast::Wait(0.25),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 7),  // Display "F   G U    F  R".
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
