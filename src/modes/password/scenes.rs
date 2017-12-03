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

pub const PRE_SLIDERS_SCENE: i32 = 1000;

const ARGONY: i32 = 1;
const BRIDGE_START: i32 = -99;
const ELINSA: i32 = 3;
const MEZURE: i32 = 5;
const RELYNG: i32 = -1;
const SYSTEM: i32 = 0;
const UGRENT: i32 = 4;
const YTTRIS: i32 = 2;

// ========================================================================= //

pub fn crossword_index_for_slot(slot: i32) -> Option<usize> {
    if slot == ELINSA {
        Some(0)
    } else if slot == ARGONY {
        Some(1)
    } else if slot == MEZURE {
        Some(2)
    } else if slot == YTTRIS {
        Some(3)
    } else if slot == UGRENT {
        Some(4)
    } else if slot == RELYNG {
        Some(5)
    } else {
        None
    }
}

pub fn slot_for_crossword_index(index: usize) -> i32 {
    match index {
        0 => ELINSA,
        1 => ARGONY,
        2 => MEZURE,
        3 => YTTRIS,
        4 => UGRENT,
        5 => RELYNG,
        _ => panic!("Invalid crossword index: {}", index),
    }
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("password_file"),
            Ast::Place(SYSTEM, "chars/system", 0, (288, 208)),
            Ast::Seq((0..10).map(|index| {
                Ast::Place(BRIDGE_START + index, "tiles/miniblocks", 14,
                           (216 + 16 * index, 344))
            }).collect()),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 160)),
            Ast::Slide(MEZURE, (96, 160), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "So...what's this place that\n\
                       the System Repair Bot didn't\n\
                       want us to get to?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(ARGONY, "chars/argony", 0, (592, 96)),
            Ast::Slide(ARGONY, (454, 96), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "This, dear, is the System\n\
                       Password File."),
        ]),
        Ast::Seq(vec![
            Ast::Place(YTTRIS, "chars/yttris", 0, (592, 160)),
            Ast::Slide(YTTRIS, (480, 160), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::W,
                      "Oooh!  I don't think I've\n\
                       ever been in here before."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "System passwords?  You mean that\n\
                       the bot didn't want us to get back the\n\
                       passwords to regain control of the ship?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 96)),
            Ast::Slide(ELINSA, (122, 96), false, true, 1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Pfft.  There's nothing $ito$r  control right\n\
                       now.  Navigation and helm are still\n\
                       offline.  We're just flying on auto-pilot."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Until we finish fixing those, neither\n\
                       we $inor$r  that robot thing are going\n\
                       to be controlling anything."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "Then why did it care about\n\
                       keeping us out of here?"),
        ]),
        Ast::Seq(vec![
            Ast::Place(UGRENT, "chars/ugrent", 0, (-16, 320)),
            Ast::Slide(UGRENT, (144, 320), false, true, 1.0),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "You misunderstand.  I doubt it's\n\
                       about this room at all, per se.\n\
                       It's about what's right below us."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "This room is just the vault\n\
                       that seals that section off."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "Okay, I'll bite.  What's in\n\
                       the section right below us?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "That's classified."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Oh, knock if off Ugrent.  Look\n\
                       at the big picture here.  We\n\
                       need all hands on deck right\n\
                       now.  Leaving the child in the\n\
                       dark isn't going to help us."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "Hmph.  Rules\n\
                       are rules."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SW,
                      "Oh fine, be that way.  We'll all\n\
                       get to see it soon enough anyway,\n\
                       because we need to get down there\n\
                       and see what that bot's game was."),
        ]),
        Ast::Seq(vec![
            Ast::Place(RELYNG, "chars/relyng", 0, (432, 352)),
            Ast::Slide(RELYNG, (432, 336), false, false, 0.75),
            Ast::Wait(0.5),
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (432, 320), 0.5),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Easier said than done."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "Everything's on lockdown right now.\n\
                       We're each going to have to enter in\n\
                       our passwords to open up the vault."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                      "It won't open unless all six of us\n\
                       do it.  That includes you, new kid."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "Who, me?  I don't\n\
                       know any passwords!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "Maybe someone can\n\
                       do mine for me?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "That's not a good idea."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Queue(0, 1), // Reveal crosswords.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Queue(1, 1), // Display speech.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_pre_sliders_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Queue(0, 0), // Hide crosswords.
            Ast::Wait(1.0),
            Ast::Seq((1..7).map(|col| Ast::Seq(vec![
                Ast::Sound(Sound::beep()),
                Ast::Queue(2, col),
                Ast::Wait(0.5),
            ])).collect()),
        ]),
    ];
    (PRE_SLIDERS_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Queue(0, 0), // Hide crosswords.
            Ast::Queue(2, 6), // Show sliders.
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(1.0),
            Ast::Queue(2, 0), // Hide sliders.
            Ast::Wait(0.5),
            Ast::Sound(Sound::beep()),
            Ast::Talk(SYSTEM, TalkStyle::System, TalkPos::NE,
                      "Access granted."),
        ]),
        Ast::Seq(vec![
            Ast::Seq((0..5).map(|index| {
                Ast::Seq(vec![
                    Ast::Wait(0.1),
                    Ast::Sound(Sound::platform_shift(1)),
                    Ast::Remove(BRIDGE_START + 4 - index),
                    Ast::Remove(BRIDGE_START + index + 5),
                ])
            }).collect()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Oh.  Oh no."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "I think I just realized\n\
                       what the System Repair\n\
                       Bot's plan was."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::NE,
                      "I think we all\n\
                       just realized it."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                      "Um, I didn't.  What are\n\
                       you two talking about?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (98, 96), true, true, 0.5),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Well, if we're right, we need\n\
                       to get down there, $inow."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(ELINSA, (-16, 96), true, false, 0.5),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::NW,
                          "Agreed."),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(ARGONY, (592, 96), true, false, 0.75),
                Ast::SetPos(ARGONY, (592, 320)),
                Ast::Slide(ARGONY, (384, 320), false, false, 1.0),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(ARGONY, (344, 400), 0.8),
                Ast::Remove(ARGONY),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.2),
                Ast::Slide(UGRENT, (192, 320), true, false, 0.5),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(UGRENT, (232, 400), 0.8),
                Ast::Remove(UGRENT),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.3),
                Ast::SetPos(ELINSA, (-16, 320)),
                Ast::Slide(ELINSA, (192, 320), false, false, 1.0),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(ELINSA, (232, 400), 0.8),
                Ast::Remove(ELINSA),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.4),
                Ast::Slide(RELYNG, (384, 320), true, false, 0.5),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(RELYNG, (344, 400), 0.8),
                Ast::Remove(RELYNG),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Slide(YTTRIS, (592, 160), true, false, 0.75),
                Ast::Wait(0.5),
                Ast::SetPos(YTTRIS, (592, 320)),
                Ast::Slide(YTTRIS, (384, 320), false, false, 1.0),
                Ast::Par(vec![
                    Ast::Seq(vec![
                        Ast::Sound(Sound::talk_hi()),
                        Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::NE,
                                  "Wheee!"),
                    ]),
                    Ast::Seq(vec![
                        Ast::Sound(Sound::small_jump()),
                        Ast::Jump(YTTRIS, (272, 400), 1.1),
                        Ast::Remove(YTTRIS),
                    ]),
                ]),
            ]),
            Ast::Seq(vec![
                Ast::Wait(1.0),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E, "Huh?"),
            ]),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::E,
                          "Wait-  Wait for me!"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Slide(MEZURE, (-16, 160), true, false, 0.5),
                Ast::SetPos(MEZURE, (-16, 320)),
                Ast::Slide(MEZURE, (192, 320), false, false, 0.75),
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(MEZURE, (260, 400), 0.8),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Remove(MEZURE),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::{crossword_index_for_slot, slot_for_crossword_index};

    #[test]
    fn slot_index_round_trip() {
        for index in 0..6 {
            let slot = slot_for_crossword_index(index);
            assert_eq!(crossword_index_for_slot(slot), Some(index));
        }
    }
}

// ========================================================================= //
