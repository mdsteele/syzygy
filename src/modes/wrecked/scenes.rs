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

pub fn compile_intro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Place(1, "Elinsa", (348, 304)),
            Ast::Place(0, "Tezure", (-16, 80)),
            Ast::Slide(0, (140, 80), true, true, 1.0),
            Ast::Wait(1.0),
            Ast::Queue(0, 1),
            Ast::Wait(1.0),
            Ast::Queue(0, 2),
            Ast::Wait(1.0),
            Ast::Queue(0, -3),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //

pub fn compile_outro_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            Ast::Queue(0, 0),
            Ast::Wait(1.0),
            Ast::Slide(0, (-16, 80), true, false, 1.0),
            Ast::Queue(0, -1),
        ]),
    ];
    Ast::compile_scene(resources, ast)
}

// ========================================================================= //
