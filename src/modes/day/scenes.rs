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

const ELINSA: i32 = 2;
const MEZURE: i32 = 1;
const PLATFORM: i32 = 0;
const SHIP: i32 = 3;
const THRUST_TOP: i32 = 4;
const THRUST_BOTTOM: i32 = 5;

const PLATFORM_INDICES: &[usize] = &[2, 3];
const THRUST_INDICES: &[usize] = &[0, 1, 2, 1];

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("plane_as_day"),
            Ast::Wait(1.0),
            Ast::Place(MEZURE, "chars/mezure", 0, (-16, 288)),
            Ast::Slide(MEZURE, (115, 288), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Well, here it is.\n\
                       The engine room."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Let's see if we can\n\
                       get this thing fixed."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Once the ship's moving\n\
                       again, we can continue\n\
                       with our mission."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Which is...wait, what\n\
                       $iis$r  our mission?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE, TalkStyle::Thought, TalkPos::NE,
                      "Hopefully someone will\n\
                       tell me eventually."),
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
            Ast::Shake(2),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Whoa.  I think we're\n\
                       starting to move."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Place(ELINSA, "chars/elinsa", 0, (120, -48)),
                Ast::Slide(ELINSA, (120, 112), false, false, 1.0),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                          "What the heck is\n\
                           going on down here?"),
            ]),
            Ast::Seq(vec![
                Ast::Sound(Sound::platform_shift(5)),
                Ast::Place(PLATFORM, "shift/platforms", 2, (120, -16)),
                Ast::Anim(PLATFORM, "shift/platforms", PLATFORM_INDICES, 2),
                Ast::Slide(PLATFORM, (120, 144), false, false, 1.0),
                Ast::SetSprite(PLATFORM, "shift/platforms", 2),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I, uh, fixed the engines?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "You WHAT!?"),
        ]),
        Ast::Seq(vec![
            Ast::Shake(2),
            Ast::Wait(0.5),
            Ast::Remove(MEZURE),
            Ast::Remove(ELINSA),
            Ast::Remove(PLATFORM),
            Ast::Queue(-2, 0), // Hide pipe grid
            Ast::SetBg("space"),
            Ast::Place(SHIP, "prolog/ship", 0, (288, 216)),
            Ast::Wait(1.0),
            Ast::Sound(Sound::explosion_small()),
            Ast::SetBg("white"),
            Ast::Wait(0.05),
            Ast::SetBg("space"),
            Ast::Queue(-1, 1), // Show moving stars
            Ast::Place(THRUST_TOP, "prolog/thrust", 0, (334, 198)),
            Ast::Anim(THRUST_TOP, "prolog/thrust", THRUST_INDICES, 3),
            Ast::Place(THRUST_BOTTOM, "prolog/thrust", 0, (334, 208)),
            Ast::Anim(THRUST_BOTTOM, "prolog/thrust", THRUST_INDICES, 3),
            Ast::Wait(1.5),
            Ast::Remove(SHIP),
            Ast::Remove(THRUST_TOP),
            Ast::Remove(THRUST_BOTTOM),
            Ast::Queue(-1, 0), // Hide moving stars
            Ast::SetBg("plane_as_day"),
            Ast::Queue(-2, 1), // Show pipe grid
            Ast::Place(MEZURE, "chars/mezure", 0, (115, 288)),
            Ast::Place(ELINSA, "chars/elinsa", 0, (120, 112)),
            Ast::Place(PLATFORM, "shift/platforms", 2, (120, 144)),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "You...you...do you\n\
                       have any idea what\n\
                       you've done!?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I...got us moving again?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Yes!  Exactly!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "We haven't fixed the\n\
                       navigational controls yet,\n\
                       Mezure.  We're moving, but\n\
                       we can't steer the ship!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Oh...dear.  Uh, I guess\n\
                       we'd better pull the plug\n\
                       on the engines, then?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "We can't!  Not safely, anyway, not\n\
                       without getting the shocks working\n\
                       again first, $iwhich they aren't$r."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "It's a wonder that you didn't\n\
                       $ialready$r  blow us up by turning\n\
                       the engines on like that, let\n\
                       alone shutting them back $ioff$r."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "I'm sorry!\n\
                       I didn't know!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (128, 112), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Okay, $ithink,$r Elinsa!\n\
                       Maybe this isn't so bad."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "I happened to already check\n\
                       the autopilot, so we know we're\n\
                       still on our previous course...\n\
                       I already fixed the gyro and the\n\
                       attitude thrusters, so our path\n\
                       should be stable enough..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "So I guess we're going to end up\n\
                       where we were already heading\n\
                       before this disaster.  We'll just\n\
                       have to try to finish fixing the\n\
                       ship before we get there."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Where's ``there?''  Where\n\
                       is the ship headed?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(ELINSA, (120, 112), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "I've got to get going.\n\
                       There's a lot to do, and\n\
                       now we have a deadline!"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Slide(ELINSA, (120, -48), false, false, 1.0),
                Ast::Remove(ELINSA),
            ]),
            Ast::Seq(vec![
                Ast::Sound(Sound::platform_shift(5)),
                Ast::Anim(PLATFORM, "shift/platforms", PLATFORM_INDICES, 2),
                Ast::Slide(PLATFORM, (120, -16), false, false, 1.0),
                Ast::Remove(PLATFORM),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                          "W- Wait!"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::NE,
                      "Why does no one ever\n\
                       $itell$r  me anything?"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (-16, 288), true, false, 0.75),
            Ast::Remove(MEZURE),
            Ast::Wait(1.0),
            Ast::Queue(0, 1),
            Ast::Wait(0.1),
            Ast::Queue(1, 2),
            Ast::Wait(0.1),
            Ast::Queue(0, 0),
            Ast::Wait(0.1),
            Ast::Queue(1, 0),
            Ast::Wait(0.1),
            Ast::Queue(0, 2),
            Ast::Wait(0.1),
            Ast::Queue(1, 1),
            Ast::Wait(0.1),
            Ast::Wait(1.0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
