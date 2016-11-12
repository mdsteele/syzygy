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

use gui::Element;
use save::Game;

// ========================================================================= //

pub enum PuzzleCmd {
    Back,
    Info,
    Undo,
    Redo,
    Reset,
    Replay,
    Solve,
}

// ========================================================================= //

pub trait PuzzleView: Element<Game, PuzzleCmd> {
    fn info_text(&self, game: &Game) -> &'static str;

    fn undo(&mut self, game: &mut Game);

    fn redo(&mut self, game: &mut Game);

    fn reset(&mut self, game: &mut Game);

    fn replay(&mut self, game: &mut Game);

    fn solve(&mut self, game: &mut Game);
}

// ========================================================================= //
