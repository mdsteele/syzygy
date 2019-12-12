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

use crate::gui::Point;
use crate::save::pyramid::Coords;

// ========================================================================= //

pub const PYRAMID_TILE_SIZE: i32 = 32;

const PYRAMID_BOTTOM_ROW_LEFT: i32 = 160;
const PYRAMID_BOTTOM_ROW_TOP: i32 = 288;
const PYRAMID_BOTTOM: i32 = PYRAMID_BOTTOM_ROW_TOP + PYRAMID_TILE_SIZE;

pub fn coords_to_pt(coords: Coords) -> Point {
    let left = PYRAMID_BOTTOM_ROW_LEFT + PYRAMID_TILE_SIZE * coords.col() +
        (PYRAMID_TILE_SIZE / 2) * coords.row();
    let top = PYRAMID_BOTTOM_ROW_TOP - PYRAMID_TILE_SIZE * coords.row();
    Point::new(left, top)
}

pub fn pt_to_coords(pt: Point) -> Option<Coords> {
    if pt.y() > PYRAMID_BOTTOM {
        return None;
    }
    let row = (PYRAMID_BOTTOM - pt.y()) / PYRAMID_TILE_SIZE;
    if row >= 8 {
        return None;
    }
    let left = PYRAMID_BOTTOM_ROW_LEFT + (PYRAMID_TILE_SIZE / 2) * row;
    if pt.x() < left {
        return None;
    }
    let col = (pt.x() - left) / PYRAMID_TILE_SIZE;
    if col >= 8 - row {
        return None;
    }
    Some(Coords::new(row, col))
}

// ========================================================================= //
