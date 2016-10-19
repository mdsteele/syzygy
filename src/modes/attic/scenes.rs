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

use elements::{Ast, Scene};
use gui::Resources;

// ========================================================================= //

pub fn intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Par(vec![
            Ast::Dark(true),
            Ast::Seq(vec![
                Ast::Wait(0.5),
                Ast::Place(1, "Argony", (-16, 112)),
                Ast::Light(1, true),
                Ast::Slide(1, (130, 112), false, false, 1.0),
                Ast::Slide(1, (170, 112), false, true, 1.0),
            ]),
            Ast::Seq(vec![
                Ast::Place(0, "Tezure", (-16, 320)),
                Ast::Light(0, true),
                Ast::Slide(0, (176, 320), true, false, 1.0),
                Ast::Wait(1.0),
                Ast::Loop(3, 3, Box::new(Ast::Seq(vec![
                    Ast::Slide(0, (90, 320), true, true, 1.0),
                    Ast::Slide(0, (130, 320), true, true, 1.0),
                ]))),
                Ast::Dark(false),
            ]),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
