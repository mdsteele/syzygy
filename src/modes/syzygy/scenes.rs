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

const ARGONY: i32 = 1;
const ELINSA: i32 = 2;
const MEZURE: i32 = 5;
const RELYNG: i32 = 4;
const UGRENT: i32 = 3;
const YTTRIS: i32 = 0;

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::SetBg("system_syzygy"),
            Ast::Wait(1.0),
            Ast::Queue(1, -1),  // Display "SYZYGY" on progress bar.
            Ast::Wait(2.0),
            Ast::Queue(1, -2),  // Finish progress bar animation.
            Ast::Place(ELINSA, "chars/elinsa", 0, (-16, 80)),
            Ast::Slide(ELINSA, (250, 80), false, true, 1.0),
            Ast::Place(ARGONY, "chars/argony", 0, (-16, 80)),
            Ast::Slide(ARGONY, (175, 80), false, true, 1.0),
            Ast::Place(YTTRIS, "chars/yttris", 0, (-16, 80)),
            Ast::Slide(YTTRIS, (100, 80), false, true, 1.0),
            Ast::Place(UGRENT, "chars/ugrent", 0, (592, 80)),
            Ast::Slide(UGRENT, (325, 80), false, true, 1.0),
            Ast::Place(RELYNG, "chars/relyng", 0, (592, 80)),
            Ast::Slide(RELYNG, (400, 80), false, true, 1.0),
            Ast::Place(MEZURE, "chars/mezure", 0, (592, 80)),
            Ast::Slide(MEZURE, (475, 80), false, true, 1.0),
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_yttris_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "Woohoo, we're in!")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(YTTRIS, TalkStyle::Normal, TalkPos::SE,
                      "See you all\n\
                       later!")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(YTTRIS, (224, 208), 1.0),
            Ast::Remove(YTTRIS),
            Ast::Queue(3, 1), // Advance stage.
            Ast::Queue(2, 0), // Turn on first indicator light.
            Ast::Wait(1.0),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, so\n\
                       what's next?")
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
        ]),
    ];
    (POST_YTTRIS_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_argony_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(ARGONY, TalkStyle::Normal, TalkPos::SE,
                      "All done.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ARGONY, (262, 208), 1.0),
            Ast::Remove(ARGONY),
            Ast::Queue(3, 1), // Advance stage.
            Ast::Queue(2, 1), // Turn on second indicator light.
            Ast::Wait(1.0),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, so\n\
                       what's next?")
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
        ]),
    ];
    (POST_ARGONY_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_elinsa_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
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
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(ELINSA, (300, 208), 1.0),
            Ast::Remove(ELINSA),
            Ast::Queue(3, 1), // Advance stage.
            Ast::Queue(2, 2), // Turn on third indicator light.
            Ast::Wait(1.0),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, so\n\
                       what's next?")
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
        ]),
    ];
    (POST_ELINSA_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_ugrent_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
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
            Ast::Wait(1.0),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, so\n\
                       what's next?")
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
        ]),
    ];
    (POST_UGRENT_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_post_relyng_scene(resources: &mut Resources) -> (i32, Scene) {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
            Ast::Wait(0.5),
            Ast::Queue(0, 0),  // Hide puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish hide animation.
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(RELYNG, TalkStyle::Normal, TalkPos::SE,
                      "Finished.")
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(RELYNG, (376, 208), 1.0),
            Ast::Remove(RELYNG),
            Ast::Queue(3, 1), // Advance stage.
            Ast::Queue(2, 4), // Turn on fifth indicator light.
            Ast::Wait(1.0),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
            Ast::Sound(Sound::talk_hi()),
            Ast::Talk(MEZURE, TalkStyle::Normal, TalkPos::SW,
                      "Okay, so\n\
                       what's next?")
        ]),
        Ast::Seq(vec![
            Ast::Queue(0, 1),  // Reveal puzzle.
            Ast::Wait(1.5),
            Ast::Queue(0, -1),  // Finish reveal animation.
        ]),
    ];
    (POST_RELYNG_SCENE, Ast::compile_scene(resources, ast))
}

// ========================================================================= //

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Sound(Sound::solve_puzzle_chime()),
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
                      "Let's do this!"),
        ]),
        Ast::Seq(vec![
            Ast::Sound(Sound::small_jump()),
            Ast::Jump(MEZURE, (414, 208), 1.0),
            Ast::Remove(MEZURE),
            Ast::Queue(2, 5), // Turn on sixth indicator light.
            Ast::Wait(1.0),
            Ast::Queue(2, -1), // Finish indicator light animation.
            Ast::Wait(0.5),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
