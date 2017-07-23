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

use toml;

use save::{Access, Location};
use save::tree::{RedBlackTree, TreeOp};
use save::util::{ACCESS_KEY, Tomlable, to_table};
use super::PuzzleState;

// ========================================================================= //

const TREE_KEY: &str = "tree";

const SOLVED_SIGNATURE: [i8; 15] = [2, 4, 2, 0, -6, 7, 12, 9, -7, -11, 9, -4,
                                    14, 12, 14];

const MIN_KEY: i32 = 1;
const MAX_KEY: i32 = 15;
const TOTAL_NUM_KEYS: usize = 15;
const MAX_REMOVED_KEYS: usize = 3;
const MIN_KEYS_ON_TREE: usize = TOTAL_NUM_KEYS - MAX_REMOVED_KEYS;

// ========================================================================= //

pub struct BlackState {
    access: Access,
    tree: RedBlackTree,
    is_initial: bool,
}

impl BlackState {
    fn initial_tree() -> RedBlackTree {
        let mut tree = RedBlackTree::new();
        tree.insert(8);
        tree.insert(4);
        tree.insert(12);
        tree.insert(2);
        tree.insert(6);
        tree.insert(10);
        tree.insert(14);
        tree.insert(1);
        tree.insert(3);
        tree.insert(5);
        tree.insert(7);
        tree.insert(9);
        tree.insert(11);
        tree.insert(13);
        tree.insert(15);
        tree
    }

    pub fn solve(&mut self) {
        self.set_from_signature(&SOLVED_SIGNATURE);
        debug_assert!(self.is_solved());
    }

    pub fn tree(&self) -> &RedBlackTree { &self.tree }

    pub fn signature(&self) -> [i8; 15] {
        let mut signature: [i8; 15] = [-128; 15];
        for (key, parent_key, is_red) in self.tree.signature().into_iter() {
            debug_assert!(key >= MIN_KEY && key <= MAX_KEY);
            debug_assert!(parent_key >= MIN_KEY && parent_key <= MAX_KEY);
            signature[(key - 1) as usize] = if parent_key == key {
                0
            } else if is_red {
                -parent_key as i8
            } else {
                parent_key as i8
            };
        }
        signature
    }

    pub fn set_from_signature(&mut self, signature: &[i8; 15]) {
        let mut rb_signature = Vec::new();
        for (index, &parent) in signature.iter().enumerate() {
            if parent == -128 {
                continue;
            }
            let key = (index as i32) + 1;
            if parent == 0 {
                rb_signature.push((key, key, false));
            } else {
                let parent_key = (parent as i32).abs();
                rb_signature.push((key, parent_key, parent < 0));
            }
        }
        let tree = RedBlackTree::from_signature(rb_signature);
        let num_keys = tree.len();
        debug_assert!(num_keys >= MIN_KEYS_ON_TREE);
        self.tree = tree;
        self.is_initial = self.tree == BlackState::initial_tree();
        self.check_if_solved();
    }

    pub fn insert(&mut self, key: i32) -> Vec<TreeOp> {
        if key < MIN_KEY || key > MAX_KEY {
            return Vec::new();
        }
        let ops = self.tree.insert(key);
        self.is_initial = self.tree == BlackState::initial_tree();
        self.check_if_solved();
        ops
    }

    pub fn remove(&mut self, key: i32) -> Vec<TreeOp> {
        if self.tree.len() <= MIN_KEYS_ON_TREE {
            return Vec::new();
        }
        let ops = self.tree.remove(key);
        self.is_initial = self.tree == BlackState::initial_tree();
        self.check_if_solved();
        ops
    }

    fn check_if_solved(&mut self) {
        let mut height = 0;
        let mut key = 10;
        while let Some(parent_key) = self.tree.parent(key) {
            height += 1;
            key = parent_key;
        }
        if height >= 5 {
            self.access = Access::Solved;
        }
    }
}

impl PuzzleState for BlackState {
    fn location() -> Location { Location::BlackAndBlue }

    fn access(&self) -> Access { self.access }

    fn access_mut(&mut self) -> &mut Access { &mut self.access }

    fn can_reset(&self) -> bool { !self.is_initial }

    fn reset(&mut self) {
        self.tree = BlackState::initial_tree();
        self.is_initial = true;
    }
}

impl Tomlable for BlackState {
    fn to_toml(&self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert(ACCESS_KEY.to_string(), self.access.to_toml());
        if !self.is_initial {
            table.insert(TREE_KEY.to_string(), self.tree.to_toml());
        }
        toml::Value::Table(table)
    }

    fn from_toml(value: toml::Value) -> BlackState {
        let mut table = to_table(value);
        let access = Access::pop_from_table(&mut table, ACCESS_KEY);
        let mut tree = RedBlackTree::pop_from_table(&mut table, TREE_KEY);
        let num_keys = tree.len();
        if num_keys < MIN_KEYS_ON_TREE || num_keys > TOTAL_NUM_KEYS {
            tree = BlackState::initial_tree();
        } else if tree.keys()
                      .into_iter()
                      .any(|key| key < MIN_KEY || key > MAX_KEY) {
            tree = BlackState::initial_tree();
        }
        let is_initial = tree == BlackState::initial_tree();
        let mut state = BlackState {
            access: access,
            tree: tree,
            is_initial: is_initial,
        };
        if !state.is_initial && !state.is_solved() {
            state.check_if_solved();
        }
        state
    }
}

// ========================================================================= //
