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

use crate::save::util::Tomlable;
use super::basic::{BasicTree, TreeOp};

// ========================================================================= //

#[derive(Debug, Eq, PartialEq)]
pub struct RedBlackTree {
    basic: BasicTree,
}

impl RedBlackTree {
    pub fn new() -> RedBlackTree { RedBlackTree { basic: BasicTree::new() } }

    pub fn signature(&self) -> Vec<(i32, i32, bool)> { self.basic.signature() }

    pub fn from_signature(signature: Vec<(i32, i32, bool)>) -> RedBlackTree {
        RedBlackTree::from_basic(BasicTree::from_signature(signature))
    }

    fn red_black_violation(basic: &BasicTree) -> Option<String> {
        let mut black_height: Option<i32> = None;
        if let Some(root_key) = basic.root() {
            if basic.is_red(root_key) {
                return Some(format!("Root ({}) is red.", root_key));
            }
            let mut stack = vec![(root_key, false, 0)];
            while let Some((key, red_parent, mut black_depth)) = stack.pop() {
                let is_red = basic.is_red(key);
                if is_red {
                    if red_parent {
                        return Some(format!("Red node ({}) has red parent.",
                                            key));
                    }
                } else {
                    black_depth += 1;
                }
                if let Some(child_key) = basic.left_child(key) {
                    stack.push((child_key, is_red, black_depth));
                } else if let Some(height) = black_height {
                    if black_depth != height {
                        return Some(format!("Leaf ({}) has black depth {} \
                                             instead of {}.",
                                            key,
                                            black_depth,
                                            height));
                    }
                } else {
                    black_height = Some(black_depth);
                }
                if let Some(child_key) = basic.right_child(key) {
                    stack.push((child_key, is_red, black_depth));
                } else if let Some(height) = black_height {
                    if black_depth != height {
                        return Some(format!("Leaf ({}) has black depth {} \
                                             instead of {}.",
                                            key,
                                            black_depth,
                                            height));
                    }
                } else {
                    black_height = Some(black_depth);
                }
            }
        }
        None
    }

    fn from_basic(basic: BasicTree) -> RedBlackTree {
        if RedBlackTree::red_black_violation(&basic).is_some() {
            RedBlackTree::new()
        } else {
            RedBlackTree { basic: basic }
        }
    }

    pub fn as_basic(&self) -> &BasicTree { &self.basic }

    pub fn len(&self) -> usize { self.basic.len() }

    pub fn contains(&self, key: i32) -> bool { self.basic.contains(key) }

    pub fn keys(&self) -> Vec<i32> { self.basic.keys() }

    pub fn root(&self) -> Option<i32> { self.basic.root() }

    pub fn parent(&self, key: i32) -> Option<i32> { self.basic.parent(key) }

    pub fn sibling(&self, key: i32) -> Option<i32> { self.basic.sibling(key) }

    pub fn left_child(&self, key: i32) -> Option<i32> {
        self.basic.left_child(key)
    }

    pub fn right_child(&self, key: i32) -> Option<i32> {
        self.basic.right_child(key)
    }

    pub fn is_red(&self, key: i32) -> bool { self.basic.is_red(key) }

    fn sibling_opt(&self, child: Option<i32>, parent_key: i32) -> Option<i32> {
        if let Some(child_key) = child {
            self.sibling(child_key)
        } else if let Some(sibling_key) = self.left_child(parent_key) {
            Some(sibling_key)
        } else {
            self.right_child(parent_key)
        }
    }

    fn is_red_opt(&self, node: Option<i32>) -> bool {
        if let Some(key) = node {
            self.is_red(key)
        } else {
            false
        }
    }

    fn op_set_red(&mut self, ops: &mut Vec<TreeOp>,
                  mut keycolors: Vec<(i32, bool)>) {
        keycolors.retain(|&(key, red)| red != self.is_red(key));
        if !keycolors.is_empty() {
            keycolors.sort();
            for &(key, red) in keycolors.iter() {
                self.basic.set_is_red(key, red);
            }
            ops.push(TreeOp::SetRed(keycolors));
        }
    }

    pub fn insert(&mut self, mut key: i32) -> Vec<TreeOp> {
        let mut ops: Vec<TreeOp> = Vec::new();
        if !self.basic.insert(key) {
            return ops;
        }
        ops.push(TreeOp::Insert(key));
        loop {
            if let Some(parent_key) = self.parent(key) {
                if !self.is_red(parent_key) {
                    debug_assert!(self.is_valid(), "ops: {:?}", ops);
                    return ops;
                }
                if let Some(aunt_key) = self.sibling(parent_key) {
                    if self.is_red(aunt_key) {
                        let grandparent_key = self.parent(parent_key).unwrap();
                        self.op_set_red(
                            &mut ops,
                            vec![
                                (parent_key, false),
                                (aunt_key, false),
                                (grandparent_key, true),
                            ],
                        );
                        key = grandparent_key;
                        continue;
                    }
                }
                break;
            } else {
                debug_assert_eq!(self.root(), Some(key));
                self.op_set_red(&mut ops, vec![(key, false)]);
                debug_assert!(self.is_valid(), "ops: {:?}", ops);
                return ops;
            }
        }
        {
            let parent_key = self.parent(key).unwrap();
            let grandparent_key = self.parent(parent_key).unwrap();
            if key > parent_key && parent_key < grandparent_key {
                self.basic.rotate_left(parent_key);
                ops.push(TreeOp::RotateLeft(parent_key));
                key = parent_key;
            } else if key < parent_key && parent_key > grandparent_key {
                self.basic.rotate_right(parent_key);
                ops.push(TreeOp::RotateRight(parent_key));
                key = parent_key;
            }
        }
        {
            let parent_key = self.parent(key).unwrap();
            let grandparent_key = self.parent(parent_key).unwrap();
            self.op_set_red(
                &mut ops,
                vec![(parent_key, false), (grandparent_key, true)],
            );
            if key < parent_key {
                self.basic.rotate_right(grandparent_key);
                ops.push(TreeOp::RotateRight(grandparent_key));
            } else {
                self.basic.rotate_left(grandparent_key);
                ops.push(TreeOp::RotateLeft(grandparent_key));
            }
        }
        debug_assert!(self.is_valid(), "ops: {:?}", ops);
        ops
    }

    pub fn remove(&mut self, key: i32) -> Vec<TreeOp> {
        let mut ops: Vec<TreeOp> = Vec::new();
        let (mut child, mut parent, was_red) =
            if let Some(left_child_key) = self.left_child(key) {
                if self.right_child(key).is_some() {
                    let mut predecessor_key = left_child_key;
                    while let Some(next_key) =
                        self.basic.right_child(predecessor_key)
                    {
                        predecessor_key = next_key;
                    }
                    let parent = if predecessor_key == left_child_key {
                        Some(predecessor_key)
                    } else {
                        self.parent(predecessor_key)
                    };
                    (self.left_child(predecessor_key),
                     parent,
                     self.is_red(predecessor_key))
                } else {
                    (Some(left_child_key), self.parent(key), self.is_red(key))
                }
            } else {
                (self.right_child(key), self.parent(key), self.is_red(key))
            };
        if !self.basic.remove(key) {
            debug_assert!(self.is_valid(), "ops: {:?}", ops);
            return ops;
        }
        ops.push(TreeOp::Remove(key));
        if was_red {
            debug_assert!(self.is_valid(), "ops: {:?}", ops);
            return ops;
        }
        if let Some(child_key) = child {
            if self.is_red(child_key) {
                self.op_set_red(&mut ops, vec![(child_key, false)]);
                debug_assert!(self.is_valid(), "ops: {:?}", ops);
                return ops;
            }
        }
        loop {
            if self.root() == child {
                debug_assert!(self.is_valid(), "ops: {:?}", ops);
                return ops;
            }
            let parent_key = parent.unwrap();
            let mut sibling = self.sibling_opt(child, parent_key);
            if let Some(sibling_key) = sibling {
                if self.is_red(sibling_key) {
                    self.op_set_red(
                        &mut ops,
                        vec![(parent_key, true), (sibling_key, false)],
                    );
                    if sibling_key > parent_key {
                        self.basic.rotate_left(parent_key);
                        ops.push(TreeOp::RotateLeft(parent_key));
                        sibling = self.right_child(parent_key);
                    } else {
                        self.basic.rotate_right(parent_key);
                        ops.push(TreeOp::RotateRight(parent_key));
                        sibling = self.left_child(parent_key);
                    }
                }
            }
            if let Some(sibling_key) = sibling {
                if !self.is_red(parent_key) && !self.is_red(sibling_key) &&
                    !self.is_red_opt(self.left_child(sibling_key)) &&
                    !self.is_red_opt(self.right_child(sibling_key))
                {
                    self.op_set_red(&mut ops, vec![(sibling_key, true)]);
                    child = Some(parent_key);
                    parent = self.parent(parent_key);
                    continue;
                }
            }
            break;
        }
        let parent_key = parent.unwrap();
        if let Some(mut sibling_key) =
            self.sibling_opt(child, parent.unwrap())
        {
            if !self.is_red(sibling_key) {
                let left_niece = self.left_child(sibling_key);
                let right_niece = self.right_child(sibling_key);
                if self.is_red(parent_key) && !self.is_red_opt(left_niece) &&
                    !self.is_red_opt(right_niece)
                {
                    self.op_set_red(
                        &mut ops,
                        vec![(sibling_key, true), (parent_key, false)],
                    );
                    debug_assert!(self.is_valid(), "ops: {:?}", ops);
                    return ops;
                }
                if sibling_key > parent_key && self.is_red_opt(left_niece) &&
                    !self.is_red_opt(right_niece)
                {
                    let left_niece_key = left_niece.unwrap();
                    self.op_set_red(
                        &mut ops,
                        vec![(sibling_key, true), (left_niece_key, false)],
                    );
                    self.basic.rotate_right(sibling_key);
                    ops.push(TreeOp::RotateRight(sibling_key));
                    sibling_key = left_niece_key;
                } else if sibling_key < parent_key &&
                           !self.is_red_opt(left_niece) &&
                           self.is_red_opt(right_niece)
                {
                    let right_niece_key = right_niece.unwrap();
                    self.op_set_red(
                        &mut ops,
                        vec![(sibling_key, true), (right_niece_key, false)],
                    );
                    self.basic.rotate_left(sibling_key);
                    ops.push(TreeOp::RotateLeft(sibling_key));
                    sibling_key = right_niece_key;
                }
            }
            let parent_is_red = self.is_red(parent_key);
            self.op_set_red(
                &mut ops,
                vec![(sibling_key, parent_is_red), (parent_key, false)],
            );
            if sibling_key > parent_key {
                if let Some(right_niece_key) = self.right_child(sibling_key) {
                    self.op_set_red(&mut ops, vec![(right_niece_key, false)]);
                }
                self.basic.rotate_left(parent_key);
                ops.push(TreeOp::RotateLeft(parent_key));
            } else {
                if let Some(left_niece_key) = self.left_child(sibling_key) {
                    self.op_set_red(&mut ops, vec![(left_niece_key, false)]);
                }
                self.basic.rotate_right(parent_key);
                ops.push(TreeOp::RotateRight(parent_key));
            }
        }
        debug_assert!(self.is_valid(), "ops: {:?}", ops);
        ops
    }

    pub fn is_valid(&self) -> bool {
        if !self.basic.is_valid() {
            return false;
        }
        if let Some(error) = RedBlackTree::red_black_violation(&self.basic) {
            println!("Red-black violation: {}", error);
            println!("Tree signature: {:?}", self.signature());
            return false;
        }
        true
    }
}

impl Tomlable for RedBlackTree {
    fn to_toml(&self) -> toml::Value { self.basic.to_toml() }

    fn from_toml(value: toml::Value) -> RedBlackTree {
        RedBlackTree::from_basic(BasicTree::from_toml(value))
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use crate::save::util::Tomlable;
    use super::{RedBlackTree, TreeOp};

    #[test]
    fn insertion() {
        let mut tree = RedBlackTree::new();
        assert_eq!(tree.root(), None);

        //   1
        assert_eq!(tree.insert(1),
                   vec![TreeOp::Insert(1), TreeOp::SetRed(vec![(1, false)])]);
        assert_eq!(tree.root(), Some(1));
        assert!(!tree.is_red(1));

        //     2*
        //    /
        //   1
        assert_eq!(tree.insert(2), vec![TreeOp::Insert(2)]);
        assert_eq!(tree.right_child(1), Some(2));
        assert!(tree.is_red(2));

        // 1*  3*
        //  \ /
        //   2
        assert_eq!(
            tree.insert(3),
            vec![
                TreeOp::Insert(3),
                TreeOp::SetRed(vec![(1, true), (2, false)]),
                TreeOp::RotateLeft(1),
            ]
        );
        assert_eq!(tree.root(), Some(2));
        assert!(!tree.is_red(2));
        assert_eq!(tree.left_child(2), Some(1));
        assert!(tree.is_red(1));
        assert_eq!(tree.right_child(2), Some(3));
        assert!(tree.is_red(3));

        //       5*
        //      /
        // 1   3
        //  \ /
        //   2
        assert_eq!(
            tree.insert(5),
            vec![
                TreeOp::Insert(5),
                TreeOp::SetRed(vec![(1, false), (2, true), (3, false)]),
                TreeOp::SetRed(vec![(2, false)]),
            ]
        );
        assert!(!tree.is_red(1));
        assert!(!tree.is_red(2));
        assert!(!tree.is_red(3));
        assert!(tree.is_red(5));

        //   3*  5*
        //    \ /
        // 1   4
        //  \ /
        //   2
        assert_eq!(
            tree.insert(4),
            vec![
                TreeOp::Insert(4),
                TreeOp::RotateRight(5),
                TreeOp::SetRed(vec![(3, true), (4, false)]),
                TreeOp::RotateLeft(3),
            ]
        );
        assert!(!tree.is_red(1));
        assert!(!tree.is_red(2));
        assert!(tree.is_red(3));
        assert!(!tree.is_red(4));
        assert!(tree.is_red(5));
    }

    #[test]
    fn removal_1() {
        //   8*
        //  /
        // 6
        let mut tree = RedBlackTree::new();
        tree.insert(6);
        tree.insert(8);
        assert_eq!(tree.len(), 2);

        // 6
        assert_eq!(tree.remove(8), vec![TreeOp::Remove(8)]);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.right_child(6), None);

        // <empty>
        assert_eq!(tree.remove(6), vec![TreeOp::Remove(6)]);
        assert_eq!(tree.root(), None);
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn removal_2() {
        //   5*  7*
        //    \ /
        // 3   6
        //  \ /
        //   4
        let mut tree = RedBlackTree::from_signature(vec![
            (3, 4, false),
            (4, 4, false),
            (5, 6, true),
            (6, 4, false),
            (7, 6, true),
        ]);
        assert_eq!(tree.len(), 5);

        //   5*
        //  /
        // 4   7
        //  \ /
        //   6
        assert_eq!(
            tree.remove(3),
            vec![
                TreeOp::Remove(3),
                TreeOp::SetRed(vec![(7, false)]),
                TreeOp::RotateLeft(4),
            ]
        );
        assert_eq!(
            tree.signature(),
            vec![(4, 6, false), (5, 4, true), (6, 6, false), (7, 6, false)]
        );
    }

    #[test]
    fn removal_3() {
        //   5   7
        //    \ /
        // 3   6*
        //  \ /
        //   4
        let mut tree = RedBlackTree::from_signature(vec![
            (3, 4, false),
            (4, 4, false),
            (5, 6, false),
            (6, 4, true),
            (7, 6, false),
        ]);
        assert_eq!(tree.len(), 5);

        //       7*
        //      /
        // 3   5
        //  \ /
        //   4
        assert_eq!(
            tree.remove(6),
            vec![
                TreeOp::Remove(6),
                TreeOp::SetRed(vec![(5, false), (7, true)]),
            ]
        );
        assert_eq!(
            tree.signature(),
            vec![(3, 4, false), (4, 4, false), (5, 4, false), (7, 5, true)]
        );
    }

    #[test]
    fn removal_4() {
        // 1*  4   6
        //  \   \ /
        //   2   5*
        //    \ /
        //     3
        let mut tree = RedBlackTree::from_signature(vec![
            (1, 2, true),
            (2, 3, false),
            (3, 3, false),
            (4, 5, false),
            (5, 3, true),
            (6, 5, false),
        ]);
        assert_eq!(tree.len(), 6);

        //     4   6
        //      \ /
        //   1   5*
        //    \ /
        //     3
        tree.remove(2);
        assert_eq!(
            tree.signature(),
            vec![
                (1, 3, false),
                (3, 3, false),
                (4, 5, false),
                (5, 3, true),
                (6, 5, false),
            ]
        );
    }

    #[test]
    fn toml_round_trip() {
        // <empty>
        let mut tree = RedBlackTree::new();
        assert_eq!(RedBlackTree::from_toml(tree.to_toml()), tree);

        //   1
        tree.insert(1);
        assert_eq!(RedBlackTree::from_toml(tree.to_toml()), tree);

        //     2*
        //    /
        //   1
        tree.insert(2);
        assert_eq!(RedBlackTree::from_toml(tree.to_toml()), tree);

        // 1*  3*
        //  \ /
        //   2
        tree.insert(3);
        assert_eq!(RedBlackTree::from_toml(tree.to_toml()), tree);

        //       5*
        //      /
        // 1   3
        //  \ /
        //   2
        tree.insert(5);
        assert_eq!(RedBlackTree::from_toml(tree.to_toml()), tree);
    }
}

// ========================================================================= //
