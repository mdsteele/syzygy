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

mod action;
mod canvas;
mod element;
mod event;
mod font;
mod sprite;
mod window;

pub use sdl2::rect::{Point, Rect};
pub use self::action::{Action, ActionBuilder};
pub use self::canvas::{Align, Canvas};
pub use self::element::{Element, GroupElement, SubrectElement};
pub use self::event::{Event, EventQueue};
pub use self::font::Font;
pub use self::sprite::Sprite;
pub use self::window::Window;

// ========================================================================= //