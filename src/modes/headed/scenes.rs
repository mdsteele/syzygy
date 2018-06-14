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

const ELINSA: i32 = 1;
const MEZURE: i32 = 4;
const UGRENT: i32 = 2;
const YTTRIS: i32 = 3;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("level_headed"),
            Ast::Wait(1.0),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 192)),
            Ast::Slide(YTTRIS, (80, 192), false, false, 0.75),
            Ast::Slide(YTTRIS, (96, 208), false, false, 0.15),
            Ast::Slide(YTTRIS, (112, 208), false, false, 0.1),
            Ast::Slide(YTTRIS, (144, 240), false, false, 0.3),
            Ast::Slide(YTTRIS, (162, 240), false, true, 0.3),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Hmm..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (200, 272), 0.65),
            Ast::Slide(YTTRIS, (208, 272), true, false, 0.15),
            Ast::Slide(YTTRIS, (240, 304), false, false, 0.3),
            Ast::Slide(YTTRIS, (266, 304), false, true, 0.3),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "This isn't the\n\
                       right way, is it?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 192)),
            Ast::Slide(ELINSA, (72, 192), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Considering this is a\n\
                       dead end, probably not."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Unless you're here\n\
                       to repair the thermal\n\
                       regulator, too."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (248, 304), true, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Oh, Elinsa!  What\n\
                       are you doing here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "...I'm here to repair\n\
                       the thermal regulator,\n\
                       Yttris.  Our $ilittle disaster$r\n\
                       knocked it out."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(ELINSA, (80, 192), true, false, 0.15),
                Ast::Slide(ELINSA, (96, 208), false, false, 0.15),
                Ast::Slide(ELINSA, (112, 208), false, false, 0.1),
                Ast::Slide(ELINSA, (144, 240), false, false, 0.3),
                Ast::Slide(ELINSA, (160, 240), false, true, 0.3),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "I'm all worried, Elinsa.\n\
                           About this disaster.\n\
                           Aren't you?  We don't even\n\
                           know exactly what it was!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "I'm too busy to be\n\
                       worried.  We'll fix it now,\n\
                       and figure it out later."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "But...what if that's\n\
                       not the right way to go?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 192)),
            Ast::Slide(UGRENT, (72, 192), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Considering this is a\n\
                       dead end, it probably isn't."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Unless you're here\n\
                       to inspect the thermal\n\
                       regulator, too."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "What?  No, we were talking\n\
                       about our worries."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "What are $iyou$r  doing\n\
                       here, Ugrent?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "...I'm here to inspect\n\
                       the thermal regulator,\n\
                       Elinsa.  To see if it\n\
                       was sabotaged."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Whatever, it's $ibroken$r.\n\
                       I'm fixing it.  You don't\n\
                       need to ``inspect'' it."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "We should make sure.\n\
                       It pays to be careful.\n\
                       A lesson you could take\n\
                       to heart, you know."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "I'm too busy to be careful!\n\
                       I've got deadlines!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Haven't you ever heard\n\
                       the expression, ``Move\n\
                       fast and break things?''"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Huh?  I thought you were\n\
                           trying to repair things?"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_annoyed_lo()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "We can't afford to take\n\
                           risks right now, Elinsa!\n\
                           We're already in a bind!"),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          "Wait-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.15),
                Ast::Sound(Sound::talk_annoyed_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                          "When you're in a bind is\n\
                           exactly when you $ihave$r\n\
                           to take risks, Ugrent!"),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_annoyed_lo()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "Maybe if you weren't so-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.15),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                          " $iSTOP!!$r "),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Both of you, calm down!\n\
                       This is no time for us\n\
                       to be getting worked up!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "...Is $iYttris$r  telling us\n\
                       not to get worked up?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "...I guess that means we\n\
                       went a little too far."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "If you two are just going\n\
                       to argue, then $iI'll$r  fix the\n\
                       thermal whatchamacallit."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.25),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, 1), // Make crossword visible.
            Ast::Wait(0.25),
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
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NE,
                      "Well, at least now I'm\n\
                       not the one that has to\n\
                       fix this with Ugrent\n\
                       breathing down my neck."),
        ]),
    ];
    (ELINSA, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Are we sure letting Yttris\n\
                       do this is a good idea?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "It's not, but at least\n\
                       we're here to supervise."),
        ]),
    ];
    (UGRENT, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::NE,
                      "How come none of\n\
                       these words fit?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(YTTRIS, TalkStyle::Thought, TalkPos::NE,
                      "This is pretty arduous!"),
        ]),
    ];
    (YTTRIS, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Queue(0, 0), // animate crossword center word
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "There!  Now I'd call that a\n\
                       $ithoroughly-repaired$r\n\
                       thermal thingamawhatsit."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(YTTRIS, (240, 304), true, false, 0.3),
            Ast::Slide(YTTRIS, (208, 272), false, false, 0.3),
            Ast::Slide(YTTRIS, (200, 272), false, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                      "Toodles!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (162, 240), 0.65),
            Ast::Slide(YTTRIS, (144, 240), false, false, 0.15),
            Ast::Slide(YTTRIS, (112, 208), false, false, 0.3),
            Ast::Slide(YTTRIS, (96, 208), false, false, 0.1),
            Ast::Slide(YTTRIS, (80, 192), false, false, 0.1),
            Ast::Slide(YTTRIS, (-16, 192), false, false, 0.5),
            Ast::Remove(YTTRIS),
            Ast::Wait(1.0),
            Ast::Slide(UGRENT, (80, 192), true, false, 0.3),
            Ast::Slide(UGRENT, (96, 208), false, false, 0.15),
            Ast::Slide(UGRENT, (106, 208), false, true, 0.3),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh$r...I suppose\n\
                       Yttris was right."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "I offer my apologies\n\
                       for losing my temper."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Aw, don't worry about it.\n\
                       And me too, I guess."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "I guess getting into a\n\
                       fight wasn't the right\n\
                       way to go, was it?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 192)),
            Ast::Slide(MEZURE, (72, 192), false, true, 0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, this looks like\n\
                       a dead end, so it\n\
                       probably wasn't."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Unless you're here\n\
                       to check out...whatever\n\
                       that thing is?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Thermal regulator.\n\
                       And yes, we were."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Oh.  I guess you\n\
                       fixed it together?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Sorta?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "That's great!  Thanks\n\
                       for being a team.  Keep\n\
                       up the great work!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (-16, 192), true, false, 0.75),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "That kid sure gets on\n\
                       my nerves sometimes."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Another thing we have\n\
                       in common, I suppose."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(UGRENT, (96, 208), true, false, 0.3),
                Ast::Slide(UGRENT, (80, 192), false, false, 0.15),
                Ast::Slide(UGRENT, (-16, 192), false, false, 0.5),
                Ast::Remove(UGRENT),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Slide(ELINSA, (144, 240), true, false, 0.3),
                Ast::Slide(ELINSA, (112, 208), false, false, 0.3),
                Ast::Slide(ELINSA, (96, 208), false, false, 0.1),
                Ast::Slide(ELINSA, (80, 192), false, false, 0.15),
                Ast::Slide(ELINSA, (-16, 192), false, false, 0.5),
                Ast::Remove(ELINSA),
                Ast::Wait(1.0),
                Ast::Queue(0, 1), // hilight crossword center word
            ]),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
