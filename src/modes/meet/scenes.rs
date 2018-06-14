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

const SYSTEM: i32 = 0;
const ELINSA: i32 = 1;
const ELINSA_BG: i32 = -1;
const MEZURE: i32 = 2;

const DOOR_UPPER_L: i32 = -2;
const DOOR_UPPER_R: i32 = -3;
const DOOR_LOWER_L: i32 = -4;
const DOOR_LOWER_R: i32 = -5;

const ICE_BLOCK_L: i32 = -6;
const ICE_BLOCK_R: i32 = -7;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources, visible: Rect) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("ice_to_meet_you_1"),
            Ast::Queue(0, 0), // Hide ice grid
            Ast::Place(SYSTEM, "chars/system", 0, (112, 144)),
            Ast::Place(DOOR_UPPER_L, "tiles/caution_walls", 12, (472, 304)),
            Ast::Place(DOOR_UPPER_R, "tiles/caution_walls", 13, (488, 304)),
            Ast::Place(DOOR_LOWER_L, "tiles/caution_walls", 10, (472, 320)),
            Ast::Place(DOOR_LOWER_R, "tiles/caution_walls", 11, (488, 320)),
            Ast::Wait(0.5),
            Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 320)),
            Ast::Slide(ELINSA, (170, 320), false, true, 1.0),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_annoyed_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Well, $ithat$r  was\n\
                       a waste of time."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "I should be out fixing the ship's\n\
                       critical systems, not getting\n\
                       dragged off on stupid errands."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Well, whatever.  That's done.\n\
                       Maybe now I can finally\n\
                       focus on my actual job."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Place(MEZURE, "chars/mezure", 0,
                           (visible.right() + 16, 320)),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::W,
                          "Heeellp!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Sound(Sound::talk_lo()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                          "Oh, good grief."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(MEZURE),
            Ast::Slide(ELINSA, (448, 320), true, true, 1.0),
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NW,
                      "System, get this\n\
                       stupid door open!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::SE,
                      "Affirmative."),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Slide(DOOR_UPPER_L, (472, 288), true, false, 0.5),
                Ast::Slide(DOOR_UPPER_R, (488, 288), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_L, (472, 336), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_R, (488, 336), true, false, 0.5),
            ]),
            Ast::Swap(ELINSA, ELINSA_BG),
            Ast::Slide(ELINSA_BG, (592, 320), true, false, 0.5),
            Ast::Remove(ELINSA_BG),
            Ast::Wait(0.5),
            Ast::Par(vec![
                Ast::Slide(DOOR_UPPER_L, (472, 304), true, false, 0.5),
                Ast::Slide(DOOR_UPPER_R, (488, 304), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_L, (472, 320), true, false, 0.5),
                Ast::Slide(DOOR_LOWER_R, (488, 320), true, false, 0.5),
            ]),
            Ast::Wait(1.5),
            Ast::Remove(SYSTEM),
            Ast::Remove(DOOR_UPPER_L),
            Ast::Remove(DOOR_UPPER_R),
            Ast::Remove(DOOR_LOWER_L),
            Ast::Remove(DOOR_LOWER_R),
            Ast::SetBg("ice_to_meet_you_2"),
            Ast::Queue(0, 1), // Show ice grid
            Ast::Place(MEZURE, "chars/mezure", 0, (336, 320)),
            Ast::Place(ICE_BLOCK_L, "tiles/ice", 0, (360, 320)),
            Ast::Place(ICE_BLOCK_R, "tiles/ice", 1, (376, 320)),
            Ast::Wait(0.5),
            Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 320)),
            Ast::Slide(ELINSA, (165, 320), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "It's $ic-c-c-cold$r  in here!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "No kidding.  This is where we\n\
                       keep historical records data in cold\n\
                       storage.  What are you doing in here?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "I came in to see if anything\n\
                       needed fixing, but this passage\n\
                       is too narrow to get through, and\n\
                       I couldn't get the door back open."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "$iSigh$r.  Didn't anyone ever\n\
                       teach you not to close yourself\n\
                       inside a refrigerator?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                      "No.  Who would have taught me that?\n\
                       Didn't we go over the $iliterally born\n\
                       today$r  thing when we first met?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Fine, whatever.  Let's\n\
                       get you out of here."),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Slide(ELINSA, (-48, 320), true, false, 1.0),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Slide(MEZURE, (-16, 320), true, false, 1.0),
                ]),
            ]),
            Ast::Wait(1.25),
            Ast::Par(vec![
                Ast::Slide(MEZURE, (250, 320), false, true, 1.0),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Slide(ELINSA, (170, 320), false, true, 1.0),
                ]),
            ]),
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "...Oooorr, maybe the system\n\
                       closed the door behind me\n\
                       when I came in here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "What do we do now!?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Relax, we'll figure\n\
                       a way out of this."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Let's see if we can push that\n\
                       block of ice out of the way."),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Slide(ELINSA, (325, 320), true, true, 0.75),
                Ast::Slide(MEZURE, (345, 320), true, true, 0.75),
            ]),
            Ast::Wait(0.5),
            Ast::Loop(2, 2, Box::new(Ast::Seq(vec![
                Ast::Sound(Sound::device_slide()),
                Ast::SetPos(ELINSA, (326, 320)),
                Ast::SetPos(MEZURE, (346, 320)),
                Ast::Wait(0.25),
                Ast::SetPos(ELINSA, (325, 320)),
                Ast::SetPos(MEZURE, (345, 320)),
                Ast::Wait(0.75),
            ]))),
            Ast::Par(vec![
                Ast::Slide(ELINSA, (180, 320), true, true, 0.75),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Slide(MEZURE, (250, 320), true, true, 0.75),
                ]),
            ]),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Won't budge.  Hmm.\n\
                       Must be frozen solid."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "You know, I think it's colder in here\n\
                       than it's supposed to be.  Which\n\
                       will also complicate data retrieval\n\
                       whenever we need to do that."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Let's get that fixed\n\
                       and maybe it'll help."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Okay, yeah!  We\n\
                       can do this!"),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "We'll need to take care of\n\
                       the green square first."),
        ]),
    ];
    (ELINSA, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Why can't we open\n\
                       the door from the\n\
                       inside, anyway?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Don't look at me, I didn't\n\
                       design this thing!"),
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
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "There we go.  The temperature\n\
                       should get back to normal now."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "It does feel a bit\n\
                       warmer in here.  Well,\n\
                       less cold, anyway."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Okay.  Let's try moving\n\
                       that block again."),
        ]),
        Ast::Seq(vec![
            Ast::Par(vec![
                Ast::Slide(ELINSA, (325, 320), true, true, 0.75),
                Ast::Slide(MEZURE, (345, 320), true, true, 0.75),
            ]),
            Ast::Wait(0.5),
            Ast::Sound(Sound::device_slide()),
            Ast::Par(vec![
                Ast::Slide(ELINSA, (357, 320), false, false, 0.4),
                Ast::Slide(MEZURE, (377, 320), false, false, 0.4),
                Ast::Slide(ICE_BLOCK_L, (392, 320), false, false, 0.4),
                Ast::Slide(ICE_BLOCK_R, (408, 320), false, false, 0.4),
            ]),
            Ast::Sound(Sound::character_collision()),
            Ast::Wait(0.25),
            Ast::Par(vec![
                Ast::Slide(ELINSA, (220, 320), true, true, 0.75),
                Ast::Seq(vec![
                    Ast::Wait(0.25),
                    Ast::Slide(MEZURE, (290, 320), true, true, 0.75),
                ]),
            ]),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "We did it!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Now that these controls aren't\n\
                       stuck, I think we should be able\n\
                       to get that door back open, too."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Excellent!  Thanks for the\n\
                       save, Elinsa.  I'm really glad\n\
                       we have you on this crew."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Um...thanks, I guess."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "I should really be getting\n\
                       back to work, though."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(ELINSA, (120, 320), true, true, 1.0),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "Um, Elinsa?  I...I hate to ask you\n\
                           this, but do think you could check\n\
                           on the rest of cold storage first?"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "It's just...well, it seems like a\n\
                       safety hazard right now, and\n\
                       you seem like the person\n\
                       most qualified to fix it."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Look, Mezure, I\n\
                       really need to..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "$i...Sigh.$r  You're right.  If you\n\
                       got trapped in here, someone\n\
                       else could too, so we should\n\
                       probably fix it right away.\n\
                       And you'd be more effective\n\
                       organizing repairs elsewhere."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "Yeah, okay.  I'll\n\
                       take care of it."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Thanks so much, Elinsa!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (-16, 320), true, false, 0.75),
            Ast::Remove(MEZURE),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::NE,
                      "How do I always get\n\
                       stuck with these jobs?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (368, 320), true, true, 1.25),
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA, TalkStyle::Thought, TalkPos::NW,
                      "At least maybe I can\n\
                       get some peace and\n\
                       quiet in here."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ELINSA, (400, 304), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ELINSA, (448, 288), 0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ELINSA, (496, 272), 0.5),
            Ast::Slide(ELINSA, (592, 272), false, false, 0.5),
            Ast::Remove(ELINSA),
            Ast::Wait(1.0),
            Ast::Queue(1, 0),
            Ast::Queue(1, 1),
            Ast::Wait(0.1),
            Ast::Queue(1, 2),
            Ast::Queue(1, 3),
            Ast::Wait(0.1),
            Ast::Queue(1, 4),
            Ast::Wait(0.1),
            Ast::Queue(1, 5),
            Ast::Queue(1, 6),
            Ast::Wait(0.1),
            Ast::Queue(1, 7),
            Ast::Queue(1, 8),
            Ast::Wait(0.1),
            Ast::Queue(1, 9),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
