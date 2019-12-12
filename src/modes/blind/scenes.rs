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

const ARGONY: i32 = 4;
const ELINSA: i32 = 1;
const MEZURE: i32 = 3;
const UGRENT: i32 = 2;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("three_blind_ice"),
            Ast::Wait(0.5),
            Ast::Place(UGRENT, "chars/ugrent", 0, (592, 288)),
            Ast::Slide(UGRENT, (448, 288), false, true, 1.0),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (400, 304), 0.5),
            Ast::Place(ELINSA, "chars/elinsa", 0, (592, 288)),
            Ast::Slide(ELINSA, (448, 288), false, true, 1.0),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "So what was it you\n\
                       wanted my help with?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "This thing."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Okay, I mean, sure,\n\
                       it does look broken."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "But I was coming up\n\
                       here to fix it anyway."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Yes, but look at $iwhat's$r  broken.\n\
                       Don't you see which storage\n\
                       segments have been deleted?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Not really.\n\
                       Which ones?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "...I don't know either.\n\
                       I was hoping you would."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_annoyed_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                          "You've $igot$r  to be kidding-"),
            ]),
            Ast::Seq(vec![
                Ast::Place(MEZURE, "chars/mezure", 0, (592, 160)),
                Ast::Slide(MEZURE, (496, 160), false, true, 0.75),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "Hello, you two!"),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_annoyed_lo()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                          "Ugh, not you again."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "How's progress in here?"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Ugrent's being paranoid again.\n\
                       He thinks there's something\n\
                       suspicious about the damage\n\
                       to this storage section."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Oh.  What seems\n\
                       suspicious about it?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Don't you see which storage\n\
                       segments have been deleted?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Pfft, no.  $iI$r  don't know\n\
                       how this thing works!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "$iSigh.$r  Where's the Lady\n\
                       Argony when we need her?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Look, Ugrent, if that'll\n\
                       be all, then why don't you\n\
                       and Mezure give me some\n\
                       room so I can fix this thing?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (364, 320), 0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "I'll continue to inspect\n\
                       this section while you work.\n\
                       Maybe I can turn something up."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (460, 192), 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "I guess I can help be a\n\
                       second pair of eyes!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NW,
                      "Somehow, this always\n\
                       seems to happen to me."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NW,
                      "I think we want to put\n\
                       the red triangle in place\n\
                       $isecond$r-to-last."),
        ]),
    ];
    (ELINSA, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "Maybe if you slide that\n\
                       one left?  No, right!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Ugh, please Mezure,\n\
                       just let me work."),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(UGRENT, TalkStyle::Thought, TalkPos::NW,
                      "You'd think we'd have a\n\
                       more understandable\n\
                       storage system than this."),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "There, it's fixed."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Except for whatever it\n\
                       was that got deleted."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Whatever's still missing, it's not\n\
                       one of the things that keeps this ship\n\
                       from flying into a star, which is $ikinda$r\n\
                       what I'm focused on right now, thanks."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "Meanwhile, I'm got other\n\
                       ship-explodey-prevention\n\
                       things to fix, which I should\n\
                       probably get to ASAP."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (592, 288), true, false, 0.75),
            Ast::Remove(ELINSA),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "And I guess I should\n\
                       go check on how the\n\
                       others are doing."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (496, 160), 0.5),
            Ast::Slide(MEZURE, (592, 160), false, false, 0.5),
            Ast::Remove(MEZURE),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NW,
                      "Hmph.  I still say\n\
                       something's fishy here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (400, 304), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (448, 288), 0.5),
            Ast::Slide(UGRENT, (592, 288), false, false, 0.75),
            Ast::Remove(UGRENT),
            Ast::Wait(1.0),
            Ast::Place(ARGONY, "chars/argony", 0, (592, 160)),
            Ast::Slide(ARGONY, (496, 160), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::W,
                      "Ah, good, it looks like\n\
                       Elinsa has already been\n\
                       by to fix this section."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::W,
                      "Hmm, but it looks like the\n\
                       ATLATL programming reference\n\
                       manual got deleted somehow?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::W,
                      "Shame to lose that.\n\
                       Kids these days just\n\
                       don't appreciate those\n\
                       old-timey systems."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::W,
                      "Ah, well, I suppose we're\n\
                       not likely to need to reprogram\n\
                       the ATLATL.  We can re-upload\n\
                       the manual when we get home."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ARGONY, (592, 160), true, false, 1.0),
            Ast::Remove(ARGONY),
            Ast::Wait(1.0),
            Ast::Queue(1, 0),
            Ast::Wait(0.1),
            Ast::Queue(1, 1),
            Ast::Queue(1, 2),
            Ast::Wait(0.1),
            Ast::Queue(1, 3),
            Ast::Queue(1, 4),
            Ast::Wait(0.1),
            Ast::Queue(1, 5),
            Ast::Queue(1, 6),
            Ast::Wait(0.1),
            Ast::Queue(1, 7),
            Ast::Wait(1.0),
            Ast::Queue(1, 8),
            Ast::Wait(0.1),
            Ast::Queue(1, 9),
            Ast::Queue(1, 10),
            Ast::Wait(0.1),
            Ast::Queue(1, 11),
            Ast::Queue(1, 12),
            Ast::Queue(1, 13),
            Ast::Wait(0.1),
            Ast::Queue(1, 14),
            Ast::Queue(1, 15),
            Ast::Queue(1, 16),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
