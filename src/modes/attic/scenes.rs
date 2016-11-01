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
use gui::Resources;

// ========================================================================= //

pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Par(vec![
            Ast::Dark(true),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Place(1, "Argony", (-16, 112)),
                Ast::Light(1, true),
                Ast::Slide(1, (130, 112), false, false, 1.0),
                Ast::Slide(1, (170, 112), false, true, 1.0),
                Ast::Loop(0, -1, Box::new(Ast::Seq(vec![
                    Ast::Jump(1, (170, 112), 0.5),
                ]))),
            ]),
            Ast::Seq(vec![
                Ast::Place(0, "Tezure", (-16, 320)),
                Ast::Light(0, true),
                Ast::Slide(0, (176, 320), true, false, 1.0),
                Ast::Jump(0, (160, 320), 0.5),
                Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                          "Ouch!  I ran into\nthat wall.")
            ]),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Talk(1, TalkStyle::Thought, TalkPos::SE,
                      "Yes.  Yes you did."),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Dark(false),
            Ast::Queue(0, 1),
            Ast::Wait(0.1),
            Ast::Queue(3, 1),
            Ast::Wait(0.1),
            Ast::Queue(4, 1),
            Ast::Wait(0.1),
            Ast::Queue(9, 1),
            Ast::Wait(0.1),
            Ast::Queue(10, 1),
            Ast::Wait(0.1),
            Ast::Queue(13, 1),
            Ast::Wait(0.1),
            Ast::Queue(15, 1),
            Ast::Wait(0.1),
            Ast::Talk(0, TalkStyle::Normal, TalkPos::NE,
                      "Oh, it says CAUTION."),
        ]),
        Ast::Seq(vec![
            Ast::Wait(0.5),
            Ast::Jump(0, (208, 304), 0.5),
            Ast::Slide(0, (256, 304), false, false, 0.25),
            Ast::Jump(0, (304, 288), 0.4),
            Ast::Slide(0, (592, 288), false, false, 1.0),
            Ast::Remove(0),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
