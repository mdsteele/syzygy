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

use rand;
use std::cmp;
use std::collections::{HashMap, HashSet};
use toml;

use crate::save::util::Tomlable;

// ========================================================================= //

pub const MAX_REMOVALS: i32 = 2;

const STARTING_PIECES: i32 = 18;

const FORMATION_LINE_LENGTH: usize = 4;

const MINIMAX_DEPTH: i32 = 9;

// ========================================================================= //

const YOU_VALUE: i8 = 1;
const SRB_VALUE: i8 = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Team {
    You,
    SRB,
}

impl Team {
    fn from_value(value: i8) -> Option<Team> {
        debug_assert!(Team::is_valid_value(value));
        if value == YOU_VALUE {
            Some(Team::You)
        } else if value == SRB_VALUE {
            Some(Team::SRB)
        } else {
            None
        }
    }

    fn value(self) -> i8 {
        match self {
            Team::You => YOU_VALUE,
            Team::SRB => SRB_VALUE,
        }
    }

    fn is_valid_value(value: i8) -> bool {
        value >= 0 && value <= 2
    }

    pub fn opponent(self) -> Team {
        match self {
            Team::You => Team::SRB,
            Team::SRB => Team::You,
        }
    }
}

// ========================================================================= //

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coords {
    row: i32,
    col: i32,
}

impl Coords {
    pub fn new(row: i32, col: i32) -> Coords {
        debug_assert!(row >= 0 && row < 8);
        debug_assert!(col >= 0 && col < 8 - row);
        Coords { row, col }
    }

    pub fn from_index(mut index: usize) -> Option<Coords> {
        for num_cols in 1..9 {
            if index < num_cols {
                return Some(Coords::new(8 - num_cols as i32, index as i32));
            }
            index -= num_cols;
        }
        None
    }

    /// Returns the row.  Row 0 is the bottom row of the pyramid; row 7 is the
    /// top cell.
    pub fn row(&self) -> i32 {
        self.row
    }

    /// Returns the column.  Column 0 is the left cell of the row.
    pub fn col(&self) -> i32 {
        self.col
    }

    fn index(&self) -> usize {
        let n = 7 - self.row;
        (self.col + n * (n + 1) / 2) as usize
    }

    pub fn all() -> CoordsIter {
        CoordsIter { row: 0, col: 0 }
    }

    fn all_above_row(row: i32) -> CoordsIter {
        CoordsIter { row: row + 1, col: 0 }
    }
}

pub struct CoordsIter {
    row: i32,
    col: i32,
}

impl Iterator for CoordsIter {
    type Item = Coords;

    fn next(&mut self) -> Option<Coords> {
        if self.row >= 8 {
            None
        } else {
            let coords = Coords::new(self.row, self.col);
            self.col += 1;
            if self.col >= 8 - self.row {
                self.col = 0;
                self.row += 1;
            }
            Some(coords)
        }
    }
}

// ========================================================================= //

#[derive(Debug, Eq, PartialEq)]
pub enum Move {
    Place {
        at: Coords,
        formation: Vec<Coords>,
        remove: Vec<Coords>,
    },
    Jump {
        from: Coords,
        to: Coords,
        formation: Vec<Coords>,
        remove: Vec<Coords>,
    },
}

// ========================================================================= //

const NUM_CELLS: usize = 2 * STARTING_PIECES as usize;

#[derive(Clone)]
pub struct Board {
    cells: Vec<i8>,
    you: i32,
    srb: i32,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: vec![0; NUM_CELLS],
            you: STARTING_PIECES,
            srb: STARTING_PIECES,
        }
    }

    fn from_cells(cells: Vec<i8>) -> Board {
        debug_assert_eq!(cells.len(), NUM_CELLS);
        let you = cells.iter().filter(|&&v| v == YOU_VALUE).count() as i32;
        let srb = cells.iter().filter(|&&v| v == SRB_VALUE).count() as i32;
        if you > STARTING_PIECES || srb > STARTING_PIECES {
            return Board::new();
        }
        Board { cells, you: STARTING_PIECES - you, srb: STARTING_PIECES - srb }
    }

    pub fn is_empty(&self) -> bool {
        self.you == STARTING_PIECES && self.srb == STARTING_PIECES
    }

    pub fn you_supply(&self) -> i32 {
        self.you
    }

    pub fn srb_supply(&self) -> i32 {
        self.srb
    }

    fn get(&self, coords: Coords) -> i8 {
        self.cells[coords.index()]
    }

    pub fn piece_at(&self, coords: Coords) -> Option<Team> {
        Team::from_value(self.get(coords))
    }

    pub fn set_piece_at(&mut self, coords: Coords, team: Team) {
        let index = coords.index();
        debug_assert_eq!(self.cells[index], 0);
        self.cells[index] = team.value();
        match team {
            Team::You => {
                self.you -= 1;
            }
            Team::SRB => {
                self.srb -= 1;
            }
        }
    }

    pub fn with_piece_at(&self, coords: Coords, team: Team) -> Board {
        let mut board = self.clone();
        board.set_piece_at(coords, team);
        board
    }

    pub fn remove_piece(&mut self, coords: Coords) {
        let index = coords.index();
        let value = self.cells[index];
        if value == YOU_VALUE {
            self.you += 1;
        } else if value == SRB_VALUE {
            self.srb += 1;
        }
        self.cells[index] = 0;
    }

    pub fn with_removed(&self, coords: Coords) -> Board {
        let mut board = self.clone();
        board.remove_piece(coords);
        board
    }

    pub fn can_place_at(&self, coords: Coords) -> bool {
        let row = coords.row;
        let col = coords.col;
        self.get(Coords::new(row, col)) == 0
            && (row == 0
                || self.get(Coords::new(row - 1, col)) != 0
                    && self.get(Coords::new(row - 1, col + 1)) != 0)
    }

    pub fn can_remove_from(&self, coords: Coords) -> bool {
        let row = coords.row;
        let col = coords.col;
        row == 7
            || (col == 0 || self.get(Coords::new(row + 1, col - 1)) == 0)
                && (col == 7 - row || self.get(Coords::new(row + 1, col)) == 0)
    }

    pub fn possible_move_starts(&self, team: Team) -> HashSet<Coords> {
        let mut results = HashSet::new();
        let value = team.value();
        for coords in Coords::all() {
            if self.can_place_at(coords) {
                results.insert(coords);
            } else if self.get(coords) == value && self.can_remove_from(coords)
            {
                let board = self.with_removed(coords);
                for dest in Coords::all_above_row(coords.row()) {
                    if board.can_place_at(dest) {
                        results.insert(coords);
                        break;
                    }
                }
            }
        }
        results
    }

    pub fn possible_jump_dests(&self, from: Coords) -> HashSet<Coords> {
        let mut results = HashSet::new();
        let board = self.with_removed(from);
        for coords in Coords::all_above_row(from.row()) {
            if board.can_place_at(coords) {
                results.insert(coords);
            }
        }
        results
    }

    pub fn possible_removals(&self, team: Team) -> HashSet<Coords> {
        self.possible_removals_vec(team).into_iter().collect()
    }

    // This is a relatively hot function when calling best_srb_move, so it's
    // been optimized a bit.
    fn possible_removals_vec(&self, team: Team) -> Vec<Coords> {
        // There can never be more than 8 possible removals (one for each
        // column), so we can pre-allocate the vector.  (There's a debug_assert
        // below to check that this limit is correct.)
        let results_capacity = 8;
        let mut results = Vec::with_capacity(results_capacity);
        let value = team.value();
        // The most straightforward implementation would be this:
        //     for coords in Coords::all() {
        //         if self.get(coords) == value &&
        //            self.can_remove_from(coords)
        //         {
        //             results.push(coords);
        //         }
        //     }
        // However, the below version makes finding the best SRB move about
        // twice as fast in practice (a potentially multi-second savings).
        for col in 0..7 {
            let mut row = 7 - col;
            while row >= 0 {
                let coords = Coords::new(row, col);
                let cell = self.get(coords);
                if cell == 0 {
                    row -= 1;
                    continue;
                } else if cell == value && self.can_remove_from(coords) {
                    results.push(coords);
                }
                break;
            }
        }
        debug_assert!(results.len() <= results_capacity);
        results
    }

    pub fn formation_at(&self, coords: Coords) -> Option<Vec<Coords>> {
        let value = self.get(coords);
        if value == 0 {
            return None;
        }
        // Horizontal:
        {
            let row = coords.row();
            let lower_col =
                cmp::max(0, coords.col() + 1 - FORMATION_LINE_LENGTH as i32);
            let upper_col =
                cmp::min(8 - row, coords.col() + FORMATION_LINE_LENGTH as i32);
            if upper_col - lower_col >= FORMATION_LINE_LENGTH as i32 {
                let mut formation = Vec::with_capacity(FORMATION_LINE_LENGTH);
                for col in lower_col..upper_col {
                    let coords = Coords::new(row, col);
                    if self.get(coords) == value {
                        formation.push(coords);
                        if formation.len() >= FORMATION_LINE_LENGTH {
                            return Some(formation);
                        }
                    } else {
                        formation.clear();
                    }
                }
            }
        }
        // Upwards diagonal:
        {
            let col = coords.col();
            let lower_row =
                cmp::max(0, coords.row() + 1 - FORMATION_LINE_LENGTH as i32);
            let upper_row =
                cmp::min(8 - col, coords.row() + FORMATION_LINE_LENGTH as i32);
            if upper_row - lower_row >= FORMATION_LINE_LENGTH as i32 {
                let mut formation = Vec::with_capacity(FORMATION_LINE_LENGTH);
                for row in lower_row..upper_row {
                    let coords = Coords::new(row, col);
                    if self.get(coords) == value {
                        formation.push(coords);
                        if formation.len() >= FORMATION_LINE_LENGTH {
                            return Some(formation);
                        }
                    } else {
                        formation.clear();
                    }
                }
            }
        }
        // Downwards diagonal:
        {
            let lower_row =
                cmp::max(0, coords.row() + 1 - FORMATION_LINE_LENGTH as i32);
            let upper_row = cmp::min(
                coords.row() + coords.col() + 1,
                coords.row() + FORMATION_LINE_LENGTH as i32,
            );
            if upper_row - lower_row >= FORMATION_LINE_LENGTH as i32 {
                let mut formation = Vec::with_capacity(FORMATION_LINE_LENGTH);
                for row in lower_row..upper_row {
                    let coords =
                        Coords::new(row, coords.col() + coords.row() - row);
                    if self.get(coords) == value {
                        formation.push(coords);
                        if formation.len() >= FORMATION_LINE_LENGTH {
                            return Some(formation);
                        }
                    } else {
                        formation.clear();
                    }
                }
            }
        }
        None
    }

    fn favoribility(&self, team: Team) -> f64 {
        match team {
            Team::You => self.you as f64 / self.srb as f64,
            Team::SRB => self.srb as f64 / self.you as f64,
        }
    }

    pub fn best_srb_move(&self) -> Move {
        let depth = MINIMAX_DEPTH;
        let mut best_score = 0.0;
        let mut best_moves = Vec::new();
        for (mov, board) in self.all_moves(Team::SRB) {
            let score =
                1.0 / board.minimax(depth, 0.0, 1.0 / best_score, Team::You);
            if score > best_score {
                best_score = score;
                best_moves = vec![mov];
            } else if score == best_score {
                best_moves.push(mov);
            }
        }
        if cfg!(debug_assertions) {
            if best_moves.len() > 1 {
                println!(
                    "Choosing randomly between {} equally-good moves \
                     (score = {})",
                    best_moves.len(),
                    best_score
                );
            } else {
                println!("Found single best move (score = {})", best_score);
            }
        }
        rand::seq::sample_iter(&mut rand::thread_rng(), best_moves, 1)
            .expect("no moves were possible")
            .pop()
            .unwrap()
    }

    /// Returns the best board score that the given team can guarantee getting
    /// if it gets to make the next move.
    fn minimax(
        &self,
        depth: i32,
        mut alpha: f64,
        beta: f64,
        team: Team,
    ) -> f64 {
        debug_assert!(depth >= 0);
        if depth == 0 || self.you == 0 || self.srb == 0 {
            return self.favoribility(team);
        }
        let mut best = 0.0;
        for (_, board) in self.all_moves(team) {
            let score = 1.0
                / board.minimax(
                    depth - 1,
                    1.0 / beta,
                    1.0 / alpha,
                    team.opponent(),
                );
            if score > best {
                best = score;
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }
        best
    }

    fn all_moves(&self, team: Team) -> Vec<(Move, Board)> {
        let mut moves = Vec::new();
        for coords in Coords::all() {
            if self.can_place_at(coords) {
                let board2 = self.with_piece_at(coords, team);
                if let Some(formation) = board2.formation_at(coords) {
                    for (remove, board3) in board2.all_removals(team) {
                        moves.push((
                            Move::Place {
                                at: coords,
                                formation: formation.clone(),
                                remove,
                            },
                            board3,
                        ));
                    }
                } else {
                    moves.push((
                        Move::Place {
                            at: coords,
                            formation: Vec::new(),
                            remove: Vec::new(),
                        },
                        board2,
                    ));
                }
            } else if self.get(coords) == team.value()
                && self.can_remove_from(coords)
            {
                let board2 = self.with_removed(coords);
                for coords2 in Coords::all_above_row(coords.row) {
                    if board2.can_place_at(coords2) {
                        let board3 = board2.with_piece_at(coords2, team);
                        if let Some(formation) = board3.formation_at(coords2) {
                            for (remove, board4) in board3.all_removals(team) {
                                moves.push((
                                    Move::Jump {
                                        from: coords,
                                        to: coords2,
                                        formation: formation.clone(),
                                        remove,
                                    },
                                    board4,
                                ));
                            }
                        } else {
                            moves.push((
                                Move::Jump {
                                    from: coords,
                                    to: coords2,
                                    formation: Vec::new(),
                                    remove: Vec::new(),
                                },
                                board3,
                            ));
                        }
                    }
                }
            }
        }
        moves
    }

    fn all_removals(&self, team: Team) -> Vec<(Vec<Coords>, Board)> {
        let mut results = HashMap::new();
        for coords1 in self.possible_removals_vec(team) {
            let board2 = self.with_removed(coords1);
            let removals2 = board2.possible_removals_vec(team);
            if removals2.is_empty() {
                let key = board2.cells.clone();
                results.insert(key, (vec![coords1], board2));
            } else {
                for coords2 in removals2 {
                    let board3 = board2.with_removed(coords2);
                    let key = board3.cells.clone();
                    results.insert(key, (vec![coords2, coords1], board3));
                }
            }
        }
        results.into_iter().map(|(_, value)| value).collect()
    }
}

impl Tomlable for Board {
    fn to_toml(&self) -> toml::Value {
        toml::Value::Array(
            self.cells
                .iter()
                .map(|&value| toml::Value::Integer(value as i64))
                .collect(),
        )
    }

    fn from_toml(value: toml::Value) -> Board {
        let mut cells = Vec::<i8>::from_toml(value);
        cells.retain(|&v| Team::is_valid_value(v));
        cells.resize(NUM_CELLS, 0);
        Board::from_cells(cells)
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::f64;

    use super::{Board, Coords, Move, Team, NUM_CELLS};
    use crate::save::util::Tomlable;

    #[test]
    fn team_values() {
        assert!(Team::is_valid_value(Team::You.value()));
        assert_eq!(Team::from_value(Team::You.value()), Some(Team::You));
        assert!(Team::is_valid_value(Team::SRB.value()));
        assert_eq!(Team::from_value(Team::SRB.value()), Some(Team::SRB));
        assert!(Team::is_valid_value(0));
        assert_eq!(Team::from_value(0), None);
    }

    #[test]
    fn all_coords() {
        assert_eq!(Coords::all().count(), NUM_CELLS);
    }

    #[test]
    fn coords_index() {
        let mut indices = HashSet::new();
        for coords in Coords::all() {
            let index = coords.index();
            assert!(index < NUM_CELLS);
            assert!(!indices.contains(&index));
            indices.insert(index);
        }
    }

    #[test]
    fn coords_from_index() {
        assert_eq!(Coords::from_index(0), Some(Coords::new(7, 0)));
        assert_eq!(Coords::from_index(1), Some(Coords::new(6, 0)));
        assert_eq!(Coords::from_index(2), Some(Coords::new(6, 1)));
        assert_eq!(Coords::from_index(3), Some(Coords::new(5, 0)));
        assert_eq!(Coords::from_index(35), Some(Coords::new(0, 7)));
        assert_eq!(Coords::from_index(36), None);
    }

    #[test]
    fn coords_index_round_trip() {
        for coords in Coords::all() {
            let index = coords.index();
            assert_eq!(Coords::from_index(index), Some(coords));
        }
    }

    #[test]
    fn board_is_empty() {
        let mut board = Board::new();
        assert!(board.is_empty());
        board.set_piece_at(Coords::new(0, 1), Team::You);
        assert!(!board.is_empty());
        board.remove_piece(Coords::new(0, 1));
        assert!(board.is_empty());
        board.set_piece_at(Coords::new(0, 6), Team::SRB);
        assert!(!board.is_empty());
        board.remove_piece(Coords::new(0, 6));
        assert!(board.is_empty());
    }

    #[test]
    fn board_toml_round_trip() {
        let mut board = Board::new();
        board.set_piece_at(Coords::new(0, 1), Team::You);
        board.set_piece_at(Coords::new(0, 2), Team::SRB);
        board.set_piece_at(Coords::new(0, 3), Team::You);
        board.set_piece_at(Coords::new(0, 5), Team::SRB);
        board.set_piece_at(Coords::new(1, 2), Team::You);
        let toml = board.to_toml();
        let board2 = Board::from_toml(toml);
        assert_eq!(board2.you, board.you);
        assert_eq!(board2.srb, board.srb);
        assert_eq!(board2.piece_at(Coords::new(1, 2)), Some(Team::You));
        assert_eq!(board2.piece_at(Coords::new(0, 5)), Some(Team::SRB));
    }

    #[test]
    fn board_all_removals() {
        let mut board = Board::new();
        board.set_piece_at(Coords::new(0, 1), Team::SRB);
        board.set_piece_at(Coords::new(0, 2), Team::SRB);
        board.set_piece_at(Coords::new(1, 1), Team::SRB);
        board.set_piece_at(Coords::new(0, 4), Team::SRB);
        let mut actual_removals: Vec<Vec<Coords>> = board
            .all_removals(Team::SRB)
            .into_iter()
            .map(|(remove, _)| remove)
            .collect();
        actual_removals.sort();
        for removal in actual_removals.iter_mut() {
            removal.sort();
        }
        let expected_removals = vec![
            vec![Coords::new(0, 1), Coords::new(1, 1)],
            vec![Coords::new(0, 2), Coords::new(1, 1)],
            vec![Coords::new(0, 4), Coords::new(1, 1)],
        ];
        assert_eq!(actual_removals, expected_removals);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn board_favoribility() {
        let board = Board::new();
        assert_eq!(board.you_supply(), 18);
        assert_eq!(board.srb_supply(), 18);
        assert_eq!(board.favoribility(Team::You), 1.0);
        assert_eq!(board.favoribility(Team::SRB), 1.0);

        let board = Board::from_cells(vec![0,
                                         0, 0,
                                       0, 0, 0,
                                     0, 0, 0, 0,
                                   0, 0, 0, 0, 0,
                                 2, 1, 2, 1, 2, 1,
                               2, 2, 1, 2, 2, 1, 2,
                             1, 2, 1, 2, 2, 2, 1, 2]);
        assert_eq!(board.you_supply(), 10);
        assert_eq!(board.srb_supply(), 5);
        assert_eq!(board.favoribility(Team::You), 2.0);
        assert_eq!(board.favoribility(Team::SRB), 0.5);

        let board = Board::from_cells(vec![0,
                                         0, 0,
                                       0, 0, 0,
                                     1, 2, 1, 2,
                                   2, 1, 2, 2, 1,
                                 2, 1, 2, 1, 2, 1,
                               2, 2, 1, 2, 2, 1, 2,
                             1, 2, 1, 2, 2, 2, 1, 2]);
        assert_eq!(board.you_supply(), 6);
        assert_eq!(board.srb_supply(), 0);
        assert_eq!(board.favoribility(Team::You), f64::INFINITY);
        assert_eq!(board.favoribility(Team::SRB), 0.0);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn board_minimax() {
        // Create a board where, if you go first, you will always lose, but the
        // SRB goes first, it can win by doing a jump.
        let board = Board::from_cells(vec![0,
                                         0, 0,
                                       1, 2, 0,
                                     2, 1, 1, 2,
                                   1, 1, 2, 2, 1,
                                 1, 2, 2, 1, 1, 2,
                               2, 2, 1, 1, 2, 2, 1,
                             1, 1, 2, 2, 1, 1, 2, 2]);
        assert_eq!(board.you_supply(), 2);
        assert_eq!(board.srb_supply(), 2);
        // Test that best_srb_move() finds the winning move.
        assert_eq!(board.best_srb_move(), Move::Jump {
            from: Coords::new(4, 3),
            to: Coords::new(6, 0),
            formation: vec![],
            remove: vec![],
        });
        // With high enough minimax depth, we can see that you definitely lose
        // if you go first (score 0), and the SRB wins if it goes first (score
        // infinity).
        assert_eq!(board.minimax(9, 0.0, f64::INFINITY, Team::You), 0.0);
        assert_eq!(board.minimax(9, 0.0, f64::INFINITY, Team::SRB),
                   f64::INFINITY);
        // With a minimax depth of one, we can only see the results of the
        // first move: if you make a move, you end up with a supply of 1
        // vs. the SRB's 2 (score 1/2), but if the SRB makes a move, it can
        // maintain the 2/2 supply ratio by making a jump (score 1).
        assert_eq!(board.minimax(1, 0.0, f64::INFINITY, Team::You), 0.5);
        assert_eq!(board.minimax(1, 0.0, f64::INFINITY, Team::SRB), 1.0);
    }
}

// ========================================================================= //
