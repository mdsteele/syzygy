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

mod ast;
mod scene;
mod theater;

pub use self::ast::{Ast, TalkStyle};
pub use self::scene::{DarkNode, JumpNode, LightNode, LoopNode, ParallelNode,
                      PlaceNode, QueueNode, RemoveNode, Scene, SceneNode,
                      SequenceNode, SetBgNode, SlideNode, SoundNode, TalkNode,
                      WaitNode};
pub use self::theater::{TalkPos, Theater};

// ========================================================================= //
