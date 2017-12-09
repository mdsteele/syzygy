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

pub const POST_YTTRIS_SCENE: i32 = 1000;
pub const POST_ARGONY_SCENE: i32 = 1001;
pub const POST_ELINSA_SCENE: i32 = 1002;
pub const POST_UGRENT_SCENE: i32 = 1003;
pub const POST_RELYNG_SCENE: i32 = 1004;

const YTTRIS: i32 = 5;
const ARGONY: i32 = 4;
const ELINSA: i32 = 3;
const UGRENT: i32 = 1;
const RELYNG: i32 = 0;
const MEZURE: i32 = 2;

const YTTRIS_1: i32 = 10 + YTTRIS;
const ARGONY_1: i32 = 10 + ARGONY;
const ELINSA_1: i32 = 10 + ELINSA;
const UGRENT_1: i32 = 10 + UGRENT;
const RELYNG_1: i32 = 10 + RELYNG;
const MEZURE_1: i32 = 10 + MEZURE;

const ARGONY_2: i32 = 20 + ARGONY;
const ELINSA_2: i32 = 20 + ELINSA;
const UGRENT_2: i32 = 20 + UGRENT;
const RELYNG_2: i32 = 20 + RELYNG;
const MEZURE_2: i32 = 20 + MEZURE;

const ELINSA_3: i32 = 30 + ELINSA;
const UGRENT_3: i32 = 30 + UGRENT;
const RELYNG_3: i32 = 30 + RELYNG;
const MEZURE_3: i32 = 30 + MEZURE;

const UGRENT_4: i32 = 40 + UGRENT;
const RELYNG_4: i32 = 40 + RELYNG;
const MEZURE_4: i32 = 40 + MEZURE;

const RELYNG_5: i32 = 50 + RELYNG;
const MEZURE_5: i32 = 50 + MEZURE;

const MEZURE_6: i32 = 60 + MEZURE;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("system_syzygy"),
            Ast::Wait(1.0),
            Ast::Sound(Sound::beep()),
            Ast::Queue(1, -1),  // Display "SYZYGY" on progress bar.
            Ast::Wait(2.0),
            Ast::Queue(1, -2),  // Finish progress bar animation.
            Ast::Par(vec![
                Ast::Seq(vec![
                    Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 80)),
                    Ast::Slide(ELINSA, (250, 80), false, true, 1.0),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(0.5),
                    Ast::Place(UGRENT, "chars/ugrent", 0, (592, 80)),
                    Ast::Slide(UGRENT, (325, 80), false, true, 1.0),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(1.0),
                    Ast::Place(ARGONY, "chars/argony", 0, (-16, 80)),
                    Ast::Slide(ARGONY, (175, 80), false, true, 1.0),
                ]),
                Ast::Seq(vec![
                    Ast::Wait(1.5),
                    Ast::Place(RELYNG, "chars/relyng", 0, (592, 80)),
                    Ast::Slide(RELYNG, (400, 80), false, true, 1.0),
                ]),
            ]),
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Well...here it is."),
        ]),
        Ast::Seq(vec![
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 80)),
            Ast::Slide(YTTRIS, (100, 80), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Whoa, I always wondered\n\
                       what it looked like!"),
        ]),
        Ast::Seq(vec![
            Ast::Place(MEZURE, "chars/mezure", 0, (592, 80)),
            Ast::Slide(MEZURE, (475, 80), false, true, 1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "What is $ithat$r  thing?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "That, child, is an Automatic Targeting\n\
                       Linear-Amplified Terraforming Laser."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Or, ATLATL for short!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "What's it do?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Assuming the ship is near a star to\n\
                       collect enough solar energy, it can\n\
                       fire a beam that'll warm up an entire\n\
                       planet from across the solar system.\n\
                       It can make an icy planet habitable."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "Our mission was to travel to the\n\
                       Xanadu system and terraform\n\
                       its uninhabited fourth planet,\n\
                       Xanadu-IV, in preparation for\n\
                       the arrival of a human colony.\n\
                       We're almost there now."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                      "Lady Argony!  As I said, our\n\
                       mission is highly classified!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "Only for the colony's protection,\n\
                       Ugrent.  Considering that somebody\n\
                       slipped a rogue robot onto our ship to\n\
                       commit a carefully-planned sabotage,\n\
                       it seems the secret is already out."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "Anyway, as you can imagine, kid, a device\n\
                       that can turn an uninhabitable planet\n\
                       warm would have...unfortunate effects\n\
                       when used on an already-inhabited world."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "And...we've had one of these planet\n\
                       death rays just sitting on board\n\
                       the ship this entire time?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "It $idoes$r  have a safety mechanism.\n\
                       It can only fire when the ship is\n\
                       pointed directly at a preselected\n\
                       target, with the expected planetary\n\
                       conditions.  In this case, Xanadu-IV."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Oh.  So we're fine!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "But..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "$iSigh.$r  Of course\n\
                       there's a but."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "It could still fire even if there\n\
                       happened to be another planet $iin\n\
                       the way$r.  And from of the looks\n\
                       of things, it's been reprogrammed\n\
                       to fire when the ship is $iin syzygy$r\n\
                       with Xanadu-III and Xanadu-IV."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "In syzygy?  What's that?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "It's an astronomical term, dear.\n\
                       It basically means ``lined up with.''\n\
                       Xanadu-III would be directly between\n\
                       the ship and the intended target\n\
                       when the terraforming laser fires."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Oh.  Is Xanadu-III inhabited?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "Yes.  By approximately seven\n\
                       hundred million humans."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Eeek!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "And we're locked out of the controls.\n\
                       Very soon, the ship will drop out of warp\n\
                       in the Xanadu system, and the ATLATL's\n\
                       program will automatically take over."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "We've got to do something!\n\
                       Can't we shut it down?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "The E-stops have all been ripped out.\n\
                       Our saboteur did a thorough job."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Then-  Then it's hopeless!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Well...could we reprogram it?\n\
                       Make us fire from a different\n\
                       position in the star system, so\n\
                       the planets won't be lined up?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "It's...possible.\n\
                       But it's not going\n\
                       to be easy."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "We're going to need to disengage the\n\
                       security barrier, and access the cold\n\
                       firmware.  Then we need to reconnect\n\
                       the control circuits $iand$r  the targeting\n\
                       interface.  That lets us shut down the\n\
                       software locks, and finally reprogram\n\
                       it.  There's going to be a lot of steps."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "Oh, is that all?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "Actually, it's worse than that.\n\
                       This thing is designed to be very\n\
                       tamper-resistant.  After each one\n\
                       of those steps, one of us will have\n\
                       to get into the machine and hold\n\
                       open that bypass to keep everything\n\
                       from resetting.  Whoever's left out\n\
                       here will have to do the remaining\n\
                       steps without their help."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                      "Wonderful."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "C'mon everyone,\n\
                       we can do this!\n\
                       We've got to!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Yeah!  I'll start us off!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Step 1: Disengage security!"),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 1),  // Set progress bar to 1/6.
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
            Ast::Swap(YTTRIS, YTTRIS_1),
            Ast::Swap(ARGONY, ARGONY_1),
            Ast::Swap(ELINSA, ELINSA_1),
            Ast::Swap(UGRENT, UGRENT_1),
            Ast::Swap(RELYNG, RELYNG_1),
            Ast::Swap(MEZURE, MEZURE_1),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_yttris_midscene_1(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS_1, TalkStyle::Normal, TalkPos::SE,
                      "Is it ``decrypt?''\n\
                       Hmm, no, that's too\n\
                       many letters..."),
        ]),
    ];
    (YTTRIS_1, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_argony_midscene_1(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY_1, TalkStyle::Normal, TalkPos::SE,
                      "TODO a1"),
        ]),
    ];
    (ARGONY_1, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene_1(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA_1, TalkStyle::Normal, TalkPos::SE,
                      "Could we hurry this\n\
                       up a bit?  We're kind\n\
                       of on a deadline."),
        ]),
    ];
    (ELINSA_1, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene_1(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT_1, TalkStyle::Normal, TalkPos::SW,
                      "TODO u1"),
        ]),
    ];
    (UGRENT_1, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene_1(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG_1, TalkStyle::Normal, TalkPos::SW,
                      "TODO r1"),
        ]),
    ];
    (RELYNG_1, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene_1(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE_1, TalkStyle::Thought, TalkPos::SW,
                      "And I thought that\n\
                       those $iother$r  mixed-up\n\
                       columns were tricky!"),
        ]),
    ];
    (MEZURE_1, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_yttris_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Swap(YTTRIS_1, YTTRIS),
            Ast::Swap(ARGONY_1, ARGONY),
            Ast::Swap(ELINSA_1, ELINSA),
            Ast::Swap(UGRENT_1, UGRENT),
            Ast::Swap(RELYNG_1, RELYNG),
            Ast::Swap(MEZURE_1, MEZURE),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Seq((0..6).map(|index| Ast::Seq(vec![
                Ast::Wait(0.075),
                Ast::Queue(6, index),
            ])).collect()),
            Ast::Wait(0.25),
            Ast::Seq((0..6).map(|index| Ast::Seq(vec![
                Ast::Wait(0.075),
                Ast::Queue(7, index),
            ])).collect()),
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Woohoo, we're in!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "See you all later!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (224, 208), 1.0),
            Ast::Remove(YTTRIS),
            Ast::Queue(3, 1), // Advance stage.
            Ast::Queue(2, 0), // Turn on first indicator light.
            Ast::Wait(0.5),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.75),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, so\n\
                       what's next?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Uh, let's see.  Step 2\n\
                       is firmware access.\n\
                       The cold storage on\n\
                       this thing is pretty\n\
                       archaic, though."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "``Archaic'' is my\n\
                       middle name, dear.\n\
                       I'll handle this one."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Wait, ``middle name?''\n\
                       Does that mean you have\n\
                       a first and last name?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "Sure she does.  ``Lady\n\
                       Archaic Argony.''"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "Hush, you."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 2),  // Set progress bar to 2/6.
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
            Ast::Swap(ARGONY, ARGONY_2),
            Ast::Swap(ELINSA, ELINSA_2),
            Ast::Swap(UGRENT, UGRENT_2),
            Ast::Swap(RELYNG, RELYNG_2),
            Ast::Swap(MEZURE, MEZURE_2),
        ]),
    ];
    (POST_YTTRIS_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_argony_midscene_2(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY_2, TalkStyle::Normal, TalkPos::SE,
                      "We'll need to make\n\
                       careful use of\n\
                       that reflector."),
        ]),
    ];
    (ARGONY_2, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene_2(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(ELINSA_2, TalkStyle::Thought, TalkPos::SE,
                      "Only $iquad$r-core?\n\
                       Wow, this thing\n\
                       really $iis$r  archaic."),
        ]),
    ];
    (ELINSA_2, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene_2(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT_2, TalkStyle::Normal, TalkPos::SW,
                      "TODO u2"),
        ]),
    ];
    (UGRENT_2, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene_2(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG_2, TalkStyle::Normal, TalkPos::SW,
                      "TODO r2"),
        ]),
    ];
    (RELYNG_2, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene_2(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE_2, TalkStyle::Thought, TalkPos::SW,
                      "I'm sure glad that\n\
                       I don't have to do\n\
                       these hard ones!"),
        ]),
    ];
    (MEZURE_2, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_argony_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Swap(ARGONY_2, ARGONY),
            Ast::Swap(ELINSA_2, ELINSA),
            Ast::Swap(UGRENT_2, UGRENT),
            Ast::Swap(RELYNG_2, RELYNG),
            Ast::Swap(MEZURE_2, MEZURE),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "That's that taken care\n\
                       of.  I'll see you all on\n\
                       the other side."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ARGONY, (262, 208), 1.0),
            Ast::Remove(ARGONY),
            Ast::Queue(3, 1), // Advance stage.
            Ast::Queue(2, 1), // Turn on second indicator light.
            Ast::Wait(0.5),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Okay, looking good so far.\n\
                       Step 3: Wire up the controls."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "I'm pretty sure you're\n\
                       the only one left here\n\
                       who knows how to\n\
                       do that, Elinsa."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Um, probably.  Will you\n\
                       lot be able to handle the\n\
                       rest once I'm in there?"),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                          "Oh, sure."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                          "Definitely."),
            ]),
            Ast::Seq(vec![
                Ast::Wait(1.0),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                          "...Maybe?"),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Oh $iyeah$r.  This is\n\
                       going to go $igreat$r."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "Whatever, it's not like\n\
                       we have a choice."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Fine.  Let's get\n\
                       this thing wired up."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 3),  // Set progress bar to 3/6.
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
            Ast::Swap(ELINSA, ELINSA_3),
            Ast::Swap(UGRENT, UGRENT_3),
            Ast::Swap(RELYNG, RELYNG_3),
            Ast::Swap(MEZURE, MEZURE_3),
        ]),
    ];
    (POST_ARGONY_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_elinsa_midscene_3(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA_3, TalkStyle::Normal, TalkPos::SE,
                      "There'll be four connections\n\
                       to the purple node, and three\n\
                       to each of the other nodes."),
        ]),
    ];
    (ELINSA_3, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene_3(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT_3, TalkStyle::Normal, TalkPos::SW,
                      "TODO u3"),
        ]),
    ];
    (UGRENT_3, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene_3(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG_3, TalkStyle::Normal, TalkPos::SW,
                      "TODO r3"),
        ]),
    ];
    (RELYNG_3, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene_3(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE_3, TalkStyle::Normal, TalkPos::SW,
                      "This is all very\n\
                       confusing."),
        ]),
    ];
    (MEZURE_3, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_elinsa_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Swap(ELINSA_3, ELINSA),
            Ast::Swap(UGRENT_3, UGRENT),
            Ast::Swap(RELYNG_3, RELYNG),
            Ast::Swap(MEZURE_3, MEZURE),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "That's that!")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ELINSA, TalkStyle::Normal, TalkPos::SE,
                      "Okay, I'm going in.\n\
                       Good luck, you three!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ELINSA, (300, 208), 1.0),
            Ast::Remove(ELINSA),
            Ast::Queue(3, 1), // Advance stage.
            Ast::Queue(2, 2), // Turn on third indicator light.
            Ast::Wait(0.5),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "Well, step 4 is the\n\
                       targeting interface.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Wait, don't we need\n\
                       Elinsa for that, too?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                      "I think I can handle it.  I'm no\n\
                       engineer, but I'm familiar with\n\
                       tactical defensive targeting\n\
                       systems.  This should be similar."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "You mean like ``big\n\
                       lasers go pew, pew!''"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_lo()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SW,
                      "$iSlightly$r  more\n\
                       complicated\n\
                       than that."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 4),  // Set progress bar to 4/6.
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
        ]),
        Ast::Seq(vec![
            Ast::Swap(UGRENT, UGRENT_4),
            Ast::Swap(RELYNG, RELYNG_4),
            Ast::Swap(MEZURE, MEZURE_4),
        ]),
    ];
    (POST_ELINSA_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_ugrent_midscene_4(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT_4, TalkStyle::Normal, TalkPos::SW,
                      "There's only room for two beams\n\
                       beams through the center, so we'll\n\
                       need to reconstitute the third\n\
                       color on the right-hand side."),
        ]),
    ];
    (UGRENT_4, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene_4(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG_4, TalkStyle::Normal, TalkPos::SW,
                      "TODO r4"),
        ]),
    ];
    (RELYNG_4, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene_4(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE_4, TalkStyle::Normal, TalkPos::SW,
                      "At least this one doesn't\n\
                       involve any stuck doors."),
        ]),
    ];
    (MEZURE_4, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_ugrent_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Swap(UGRENT_4, UGRENT),
            Ast::Swap(RELYNG_4, RELYNG),
            Ast::Swap(MEZURE_4, MEZURE),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(UGRENT, TalkStyle::Normal, TalkPos::SE,
                      "Finished.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(UGRENT, (338, 208), 1.0),
            Ast::Remove(UGRENT),
            Ast::Queue(3, 1), // Advance stage.
            Ast::Queue(2, 3), // Turn on fourth indicator light.
            Ast::Wait(0.5),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "Well, well, well.  Looks like\n\
                       it's just you and me now, kid.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Uh..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "I've been watching you ever since\n\
                       you joined the crew, you know.\n\
                       Seen you talking with that ``System\n\
                       Repair Bot'' a bunch of times.  You\n\
                       know, that bloke what got us into\n\
                       this whole mess."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                          "I didn't-"),
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.25),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                          "You could have gotten us\n\
                           all into a lot of trouble.\n\
                           Heck, we $iare$r  in a lot\n\
                           of trouble right now."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "I had my doubts about you.\n\
                       But it's okay.  I know you\n\
                       were doing your best to help."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "And I know you\n\
                       won't let us down."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "Now, how's about you and me get\n\
                       those software locks shut down\n\
                       so we can finish this thing, eh?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Uh, yeah."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "...Thanks, Relyng."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 5),  // Set progress bar to 5/6.
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
            Ast::Swap(RELYNG, RELYNG_5),
            Ast::Swap(MEZURE, MEZURE_5),
        ]),
    ];
    (POST_UGRENT_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_relyng_midscene_5(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG_5, TalkStyle::Normal, TalkPos::SW,
                      "The trick here is to\n\
                       take advantage of the\n\
                       edges and corners."),
        ]),
    ];
    (RELYNG_5, Ast::compile_scene(resources, ast))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene_5(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE_5, TalkStyle::Normal, TalkPos::SW,
                      "Just please tell me we're\n\
                       not going to have to do\n\
                       the rest of this in the\n\
                       dark after this step."),
        ]),
    ];
    (MEZURE_5, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_relyng_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Swap(RELYNG_5, RELYNG),
            Ast::Swap(MEZURE_5, MEZURE),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "And...lights out.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SW,
                      "I guess it's all up\n\
                       to you now, kid."),
        ]),
        Ast::Par(vec![
            Ast::Seq(vec![
                Ast::Sound(Sound::small_jump()),
                Ast::Jump(RELYNG, (376, 208), 1.0),
                Ast::Remove(RELYNG),
                Ast::Queue(3, 1), // Advance stage.
                Ast::Queue(2, 4), // Turn on fifth indicator light.
                Ast::Wait(0.5),
                Ast::Queue(2, -1), // Finish indicator light animation.
            ]),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::talk_hi()),
                Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                          "Uh, right."),
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, last step.  What\n\
                       was the last step?"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Security and locks disabled...\n\
                       Uh, controls are hooked up..."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "That's right.  Last step is\n\
                       to reprogram the system."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_annoyed_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Wait, what!?  I'm supposed to\n\
                       reprogram it by myself?  I don't\n\
                       know how to do that!"),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (440, 80), true, true, 0.25),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Relyng, come back!\n\
                       Anyone?  I need help!"),
        ]),
        Ast::Seq(vec![
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "They can't hear me."),
        ]),
        Ast::Seq(vec![
            Ast::Slide(MEZURE, (475, 80), true, true, 0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, stay calm.  The first\n\
                       five steps weren't $iso$r  bad.\n\
                       This one will probably be\n\
                       straightforward, too."),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay.  Yeah.  Let's get\n\
                       this done, and then\n\
                       we're all home free."),
        ]),
        Ast::Seq(vec![
            Ast::Queue(1, 6),  // Set progress bar to 6/6.
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "This, uh...this does\n\
                       not look good."),
        ]),
        Ast::Seq(vec![
            Ast::Swap(MEZURE, MEZURE_6),
        ]),
    ];
    (POST_RELYNG_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_mezure_midscene_6(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_thought()),
            Ast::Talk(MEZURE_6, TalkStyle::Thought, TalkPos::SW,
                      "But...what word goes\n\
                       with ``Syzygy''?"),
        ]),
    ];
    (MEZURE_6, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Swap(MEZURE_6, MEZURE),
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Seq((0..6).map(|index| Ast::Seq(vec![
                Ast::Wait(0.075),
                Ast::Queue(4, index),
            ])).collect()),
            Ast::Seq((0..2).map(|_| Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Sound(Sound::beep()),
                Ast::Seq((0..6).map(|index| Ast::Queue(5, index)).collect()),
                Ast::Queue(1, 0),
                Ast::Queue(1, -2),
                Ast::Wait(0.5),
                Ast::Sound(Sound::beep()),
                Ast::Seq((0..6).map(|index| Ast::Queue(4, index)).collect()),
                Ast::Queue(1, 6),
                Ast::Queue(1, -2),
            ])).collect()),
            Ast::Wait(1.0),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "I...I did it!\n\
                       I really did it!"),
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, let's do this!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (414, 208), 1.0),
            Ast::Remove(MEZURE),
            Ast::Queue(2, 5), // Turn on sixth indicator light.
            Ast::Wait(0.5),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
            // Clean up:
            Ast::Remove(YTTRIS_1),
            Ast::Remove(ARGONY_1),
            Ast::Remove(ELINSA_1),
            Ast::Remove(UGRENT_1),
            Ast::Remove(RELYNG_1),
            Ast::Remove(MEZURE_1),
            Ast::Queue(1, 6),  // Set progress bar to 6/6.
            Ast::Queue(2, 0), // Turn on indicator lights.
            Ast::Queue(2, 1),
            Ast::Queue(2, 2),
            Ast::Queue(2, 3),
            Ast::Queue(2, 4),
            Ast::Queue(2, -1),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
