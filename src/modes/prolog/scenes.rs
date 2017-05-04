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
pub fn compile_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("prolog_space"),
            Ast::Place(0, "chars/invis", 0, (224, 240)),
            Ast::Wait(1.0), // TODO: Show ship flying through space
            Ast::Talk(0, TalkStyle::System, TalkPos::NE,
                      "Somewhere in deep space..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::SetBg("prolog_security"),
            Ast::Place(0, "chars/ugrent", 0, (224, 240)),
            Ast::Place(1, "chars/system", 0, (464, 208)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(0, TalkStyle::Thought, TalkPos::NE, "Zzzz..."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Remove(0),
            Ast::Remove(1),
            Ast::SetBg("prolog_space"),
            Ast::Wait(1.0), // TODO: Show ship 'splosion
            Ast::SetBg("prolog_security"),
            Ast::Place(0, "chars/ugrent", 0, (224, 240)),
            Ast::Place(1, "chars/system", 0, (464, 208)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "What in blazes was that!?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(1, TalkStyle::System, TalkPos::NW, "No."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (272, 240), true, true, 0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Ugrent to Bridge, what's\n\
                       going on up there?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0), // TODO: Secondary explosion; screen shake
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Bridge, this is security!\n\
                       Do you read?"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.6),
            Ast::Slide(0, (208, 240), true, true, 0.4),
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Comm must be out up there.\n\
                       I'd better go check..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(0, (160, 304), 0.75),
            Ast::Slide(0, (-16, 304), false, false, 0.4),
            Ast::SetBg("prolog_bridge"),
            Ast::Place(1, "chars/system", 0, (432, 112)),
            Ast::Seq(FIRE_POSITIONS.iter().enumerate().map(|(index, &pos)| {
                let slot = FIRE_SLOTS_START + index as i32;
                Ast::Seq(vec![
                    Ast::Place(slot, "chars/fire", 0, pos),
                    Ast::Anim(slot, "chars/fire", &FIRE_INDICES[index % 4], 2),
                ])
            }).collect()),
            Ast::Place(0, "chars/ugrent", 0, (592, 304)),
            Ast::Slide(0, (490, 304), false, true, 0.3),
            Ast::Wait(0.3),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NW, "Gah!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(0, (592, 304), true, false, 0.3),
            Ast::Seq((0..FIRE_POSITIONS.len()).map(|index| {
                Ast::Remove(FIRE_SLOTS_START + index as i32)
            }).collect()),
            Ast::SetBg("prolog_security"),
            Ast::Place(1, "chars/system", 0, (464, 208)),
            Ast::Place(0, "chars/ugrent", 0, (-16, 304)),
            Ast::Slide(0, (144, 304), false, true, 0.3),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "System, get me ship-wide broadcast!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::beep()),
            Ast::Talk(1, TalkStyle::System, TalkPos::NW, "Broadcast ready."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "All hands, this is security!"),
        ]),
        Ast::Seq(vec![
            // TODO: Write the rest of this cutscene.
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

const FIRE_SLOTS_START: i32 = 10;
const FIRE_POSITIONS: &[(i32, i32)] =
    &[(112, 176), (144, 176), (440, 176), (120, 272), (176, 272), (208, 272),
      (240, 272), (276, 288), (348, 288), (380, 288), (424, 304)];
const FIRE_INDICES: [&[usize]; 4] =
    [&[0, 1, 2, 3], &[1, 2, 3, 0], &[2, 3, 0, 1], &[3, 0, 1, 2]];

// ========================================================================= //
