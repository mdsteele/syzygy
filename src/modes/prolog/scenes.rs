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

pub fn compile_scene(resources: &mut Resources) -> Scene {
    let ast = vec![
        Ast::Seq(vec![
            // TODO: create background(s) for prolog
            Ast::SetBg("a_light_in_the_attic"),
            Ast::Place(0, "chars/tezure", 0, (-16, 320)),
            Ast::Slide(0, (160, 320), true, false, 1.0),
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
