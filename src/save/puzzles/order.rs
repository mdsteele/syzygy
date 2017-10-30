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

use std::cmp;
use toml;

use save::{Access, Location};
use save::util::{ACCESS_KEY, Tomlable, pop_array, to_table};
use super::PuzzleState;

// ========================================================================= //

const ORDER_KEY: &str = "order";
const ROW_KEY: &str = "row";

const INITIAL_ORDER: &[usize; 6] = &[0, 1, 2, 3, 4, 5];
const SOLVED_ORDERS: &[[usize; 6]] = &[
    [3, 4, 0, 2, 5, 1], // Letters
    [4, 3, 0, 5, 2, 1], // Numbers
    [4, 2, 1, 0, 3, 5], // Elements
    [1, 4, 2, 5, 3, 0], // Shapes
    [4, 3, 2, 5, 0, 1], // Colors
    [1, 2, 5, 4, 3, 0],
]; // Characters

// ========================================================================= //

pub struct OrderState {
    access: Access,
    row: usize,
    order: [usize; 6],
}

impl OrderState {
    pub fn solve(&mut self) {
        self.access = Access::Solved;
        self.row = SOLVED_ORDERS.len();
    }

    pub fn current_row(&self) -> usize { self.row }

    pub fn row_order(&self, row: usize) -> &[usize] {
        debug_assert!(row < SOLVED_ORDERS.len());
        if row < self.row {
            &SOLVED_ORDERS[row]
        } else if row == self.row {
            &self.order
        } else {
            INITIAL_ORDER
        }
    }

    pub fn move_tile(&mut self, from: usize, to: usize) {
        debug_assert!(self.row < SOLVED_ORDERS.len());
        debug_assert!(from < self.order.len());
        debug_assert!(to < self.order.len());
        if from < to {
            for index in from..to {
                self.order.swap(index, index + 1);
            }
        } else if from > to {
            for index in (to..from).rev() {
                self.order.swap(index, index + 1);
            }
        }
        if self.order == SOLVED_ORDERS[self.row] {
            self.row += 1;
            self.order = *INITIAL_ORDER;
            if self.row == SOLVED_ORDERS.len() {
                self.access = Access::Solved;
            }
        }
    }
}

impl PuzzleState for OrderState {
    fn location() -> Location { Location::PointOfOrder }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { self.order != *INITIAL_ORDER }

    fn reset(&mut self) { self.order = *INITIAL_ORDER; }

    fn replay(&mut self) {
        self.access = Access::BeginReplay;
        self.row = 0;
        self.order = *INITIAL_ORDER;
    }
}

impl Tomlable for OrderState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_solved() {
            table.insert(ROW_KEY.to_string(),
                         toml::Value::Integer(self.row as i64));
            let order = self.order
                .iter()
                .map(|&idx| toml::Value::Integer(idx as i64))
                .collect();
            table.insert(ORDER_KEY.to_string(), toml::Value::Array(order));
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> OrderState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let row = if access.is_solved() {
            SOLVED_ORDERS.len()
        } else {
            let row = i32::pop_from_table(&mut table, ROW_KEY);
            cmp::min(cmp::max(0, row) as usize, SOLVED_ORDERS.len() - 1)
        };
        let mut order = [0; 6];
        for (index, value) in pop_array(&mut table, ORDER_KEY)
            .into_iter()
            .enumerate()
        {
            if index >= order.len() {
                break;
            }
            let value = i32::from_toml(value);
            let value = cmp::min(cmp::max(0, value) as usize, order.len() - 1);
            order[index] = value;
        }
        for (index, value) in order.clone().into_iter().enumerate() {
            if order[..index].contains(value) {
                order = *INITIAL_ORDER;
                break;
            }
        }
        OrderState {
            access: access,
            row: row,
            order: order,
        }
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use toml;

    use save::Access;
    use save::util::{ACCESS_KEY, Tomlable};
    use super::{INITIAL_ORDER, ORDER_KEY, OrderState, ROW_KEY, SOLVED_ORDERS};

    #[test]
    fn toml_round_trip() {
        let mut state = OrderState::from_toml(toml::Value::Boolean(false));
        state.access = Access::Replaying;
        state.row = 4;
        state.order = [3, 1, 4, 2, 5, 0];

        let state = OrderState::from_toml(state.to_toml());
        assert_eq!(state.access, Access::Replaying);
        assert_eq!(state.row, 4);
        assert_eq!(state.order, [3, 1, 4, 2, 5, 0]);
    }

    #[test]
    fn from_empty_toml() {
        let state = OrderState::from_toml(toml::Value::Boolean(false));
        assert_eq!(state.access, Access::Unvisited);
        assert_eq!(state.row, 0);
        assert_eq!(state.order, *INITIAL_ORDER);
    }

    #[test]
    fn from_solved_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Solved.to_toml());

        let state = OrderState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Solved);
        assert_eq!(state.row, SOLVED_ORDERS.len());
    }

    #[test]
    fn from_invalid_row_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ROW_KEY.to_string(), toml::Value::Integer(77));
        let state = OrderState::from_toml(toml::Value::Table(table));
        assert_eq!(state.row, SOLVED_ORDERS.len() - 1);

        let mut table = toml::value::Table::new();
        table.insert(ROW_KEY.to_string(), toml::Value::Integer(-77));
        let state = OrderState::from_toml(toml::Value::Table(table));
        assert_eq!(state.row, 0);
    }

    #[test]
    fn from_invalid_order_toml() {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), Access::Unsolved.to_toml());
        table.insert(ROW_KEY.to_string(), toml::Value::Integer(2));
        let order = [4, 2, 3, 3, 1, 5]
            .iter()
            .cloned()
            .map(toml::Value::Integer)
            .collect();
        table.insert(ORDER_KEY.to_string(), toml::Value::Array(order));

        let state = OrderState::from_toml(toml::Value::Table(table));
        assert_eq!(state.access, Access::Unsolved);
        assert_eq!(state.row, 2);
        assert_eq!(state.order, *INITIAL_ORDER);
    }
}

// ========================================================================= //
