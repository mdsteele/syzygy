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

const ELINSA: i32 = 1;
const MEZURE: i32 = 3;
const SRB: i32 = 4;
const UGRENT: i32 = 2;

const WEST_DOOR_UPPER: i32 = -1;
const WEST_DOOR_LOWER: i32 = -2;
const EAST_DOOR_UPPER: i32 = -3;
const EAST_DOOR_LOWER: i32 = -4;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources, visible: Rect) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("missed_connections"),
            Ast::Place(WEST_DOOR_UPPER, "tiles/caution_walls", 5, (64, 288)),
            Ast::Place(WEST_DOOR_LOWER, "tiles/caution_walls", 4, (64, 304)),
            Ast::Place(EAST_DOOR_UPPER, "tiles/caution_walls", 5, (480, 288)),
            Ast::Place(EAST_DOOR_LOWER, "tiles/caution_walls", 4, (480, 304)),
            Ast::Wait(1.0),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(MEZURE, "chars/mezure", 0, (-16, 304)),
                    Ast::Slide(MEZURE, (262, 304), true, true, 1.25),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                              "Hmm, more doors."),
                ]),
                Ast::Seq(vec![
                    Ast::Par(vec![
                        Ast::Slide(WEST_DOOR_UPPER, (64, 272),
                                   false, false, 0.25),
                        Ast::Slide(WEST_DOOR_LOWER, (64, 320),
                                   false, false, 0.25),
                    ]),
                    Ast::Wait(0.5),
                    Ast::Par(vec![
                        Ast::Slide(WEST_DOOR_UPPER, (64, 288),
                                   false, false, 0.25),
                        Ast::Slide(WEST_DOOR_LOWER, (64, 304),
                                   false, false, 0.25),
                    ]),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Place(SRB, "chars/srb", 2, (592, 128)),
            Ast::Slide(SRB, (484, 128), false, true, 1.0),
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "Why, it's the plucky new\n\
                       administrator process!\n\
                       How are repairs going?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Oh, hi there, System\n\
                       Repair Bot!  Repairs are\n\
                       going well so far, I think."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 1),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "That's great!  Just keep working\n\
                       your way back towards the engine\n\
                       room so we can get the ship moving,\n\
                       and we'll be in good shape."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Gotcha.  We should probably\n\
                       get the external sensors\n\
                       fixed too, right?"),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                      "Oh, uh, naaaah, don't\n\
                       worry about those!\n\
                       The ship's already on\n\
                       the right course."),
        ]),
        Ast::Seq(vec![
            Ast::SetSprite(SRB, "chars/srb", 0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Uh, are you sure?  I mean,\n\
                       without the sensors, how\n\
                       can we be sure which way\n\
                       we're even pointed?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::SetSprite(SRB, "chars/srb", 3),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(SRB, TalkStyle::Good, TalkPos::SW,
                          "Oops, sorry,\n\
                           gotta go!"),
            ]),
            Ast::Slide(SRB, (592, 128), true, false, 0.75),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                          "Hey, where are you-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 304)),
                        Ast::Slide(UGRENT, (180, 304), true, true, 1.0),
                        Ast::Remove(SRB),
                        Ast::Sound(Sound::talk_lo()),
                        Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                                  "Hey, you!  Have you\n\
                                   seen Elinsa recently?\n\
                                   We need to talk to her."),
                    ]),
                    Ast::Seq(vec![
                        Ast::Par(vec![
                            Ast::Slide(WEST_DOOR_UPPER, (64, 272),
                                       false, false, 0.25),
                            Ast::Slide(WEST_DOOR_LOWER, (64, 320),
                                       false, false, 0.25),
                        ]),
                        Ast::Wait(0.5),
                        Ast::Par(vec![
                            Ast::Slide(WEST_DOOR_UPPER, (64, 288),
                                       false, false, 0.25),
                            Ast::Slide(WEST_DOOR_LOWER, (64, 304),
                                       false, false, 0.25),
                        ]),
                    ]),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Oh, hi Ugrent.  Elinsa's that\n\
                       engineer, right?  I think I\n\
                       met her a little while back."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Let's see, last time I saw her, she\n\
                       had just...um, well, she'd just fallen\n\
                       through a broken bridge.  Into...a\n\
                       deep pit with no apparent way out?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "What!?"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.35),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "She insisted\n\
                           she'd be fine!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(1.0),
                Ast::Place(ELINSA, "chars/elinsa", 0,
                           (visible.right() + 16, 304)),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                          "Oy!"),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "Is that you, Elinsa?"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.75),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                          "Coming!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (500, 304), true, false, 0.35),
            Ast::Sound(Sound::character_collision()),
            Ast::Slide(ELINSA, (518, 304), false, true, 0.25),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                              "Ow!"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                              "Oh boy, not this again..."),
                ]),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (500, 304), true, false, 0.25),
            Ast::Sound(Sound::character_collision()),
            Ast::Slide(ELINSA, (518, 304), false, true, 0.25),
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                              "Is this door\n\
                               not working?"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                              "Do $iany$r  of the doors\n\
                               on this ship work?"),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(1.0),
                    Ast::Sound(Sound::talk_hi()),
                    Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                              "...A few do."),
                ]),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                          "Well, someone over\n\
                           there needs to get this\n\
                           stupid door open!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NW,
                          "All right, let's get\n\
                           this panel open..."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1), // Make laser field visible.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Good grief, this\n\
                       one's a mess."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                      "What's going on\n\
                       over there?  I can't\n\
                       see anything."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Don't worry Elinsa!\n\
                       Everything's under\n\
                       control here."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                          "Do you even\n\
                           know how to\n\
                           fix this thing?"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                          "...Sort of?"),
            ]),
        ]),
    ];
    (ELINSA, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::NE,
                      "Hmm...which green\n\
                       laser do we use?"),
        ]),
    ];
    (MEZURE, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Elinsa, these door\n\
                       controls aren't\n\
                       designed very well."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::W,
                      "Hey, don't blame me,\n\
                       they were already like\n\
                       this before my time!"),
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
            Ast::Par(vec![
                Ast::Slide(EAST_DOOR_UPPER, (480, 272), true, false, 0.5),
                Ast::Slide(EAST_DOOR_LOWER, (480, 320), true, false, 0.5),
            ]),
            Ast::Remove(EAST_DOOR_UPPER),
            Ast::Remove(EAST_DOOR_LOWER),
            Ast::Wait(1.0),
            Ast::Slide(MEZURE, (592, 304), true, false, 1.0),
            Ast::Remove(MEZURE),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
