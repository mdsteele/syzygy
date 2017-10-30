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

use std::collections::{HashMap, HashSet};
use toml;

use save::util::Tomlable;

// ========================================================================= //

#[derive(Debug, Eq, PartialEq)]
pub enum TreeOp {
    Insert(i32),
    Remove(i32),
    RotateLeft(i32),
    RotateRight(i32),
    SetRed(Vec<(i32, bool)>),
}

impl TreeOp {
    pub fn is_set_red(&self) -> bool {
        match self {
            &TreeOp::SetRed(_) => true,
            _ => false,
        }
    }
}

// ========================================================================= //

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    parent: Option<i32>,
    left_child: Option<i32>,
    right_child: Option<i32>,
    is_red: bool,
}

// ========================================================================= //

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicTree {
    nodes: HashMap<i32, Node>,
    root: Option<i32>,
}

impl BasicTree {
    pub fn new() -> BasicTree {
        BasicTree {
            nodes: HashMap::new(),
            root: None,
        }
    }

    pub fn from_signature(signature: Vec<(i32, i32, bool)>) -> BasicTree {
        let mut root: Option<i32> = None;
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        let mut parents: HashMap<i32, i32> = HashMap::new();
        for (key, parent_key, is_red) in signature.into_iter() {
            if nodes.contains_key(&key) {
                continue;
            }
            let opt_parent = if parent_key != key {
                parents.insert(key, parent_key);
                Some(parent_key)
            } else if root.is_some() {
                continue;
            } else {
                root = Some(key);
                None
            };
            let node = Node {
                parent: opt_parent,
                left_child: None,
                right_child: None,
                is_red: is_red,
            };
            nodes.insert(key, node);
        }
        if root.is_none() && !nodes.is_empty() {
            return BasicTree::new();
        }
        for (&key, &parent_key) in parents.iter() {
            if let Some(parent_node) = nodes.get_mut(&parent_key) {
                let child = if key < parent_key {
                    &mut parent_node.left_child
                } else {
                    &mut parent_node.right_child
                };
                if child.is_some() {
                    return BasicTree::new();
                }
                *child = Some(key);
            } else {
                return BasicTree::new();
            }
        }
        // Check for loops:
        for &key in nodes.keys() {
            let mut key = key;
            let mut visited = HashSet::new();
            visited.insert(key);
            while let Some(&parent_key) = parents.get(&key) {
                if visited.contains(&parent_key) {
                    return BasicTree::new();
                }
                visited.insert(parent_key);
                key = parent_key;
            }
        }
        BasicTree {
            nodes: nodes,
            root: root,
        }
    }

    pub fn signature(&self) -> Vec<(i32, i32, bool)> {
        let mut signature = Vec::new();
        for (&key, node) in self.nodes.iter() {
            signature.push((key, node.parent.unwrap_or(key), node.is_red));
        }
        signature.sort();
        signature
    }

    pub fn len(&self) -> usize { self.nodes.len() }

    pub fn contains(&self, key: i32) -> bool { self.nodes.contains_key(&key) }

    pub fn keys(&self) -> Vec<i32> { self.nodes.keys().cloned().collect() }

    pub fn root(&self) -> Option<i32> { self.root }

    pub fn parent(&self, key: i32) -> Option<i32> {
        self.nodes.get(&key).and_then(|node| node.parent)
    }

    pub fn sibling(&self, key: i32) -> Option<i32> {
        if let Some(parent_key) = self.parent(key) {
            if key < parent_key {
                self.right_child(parent_key)
            } else {
                self.left_child(parent_key)
            }
        } else {
            None
        }
    }

    pub fn left_child(&self, key: i32) -> Option<i32> {
        self.nodes.get(&key).and_then(|node| node.left_child)
    }

    pub fn right_child(&self, key: i32) -> Option<i32> {
        self.nodes.get(&key).and_then(|node| node.right_child)
    }

    pub fn is_red(&self, key: i32) -> bool {
        if let Some(node) = self.nodes.get(&key) {
            node.is_red
        } else {
            false
        }
    }

    pub fn set_is_red(&mut self, key: i32, red: bool) {
        if let Some(node) = self.nodes.get_mut(&key) {
            node.is_red = red;
        }
    }

    pub fn perform_op(&mut self, op: &TreeOp) {
        match op {
            &TreeOp::Insert(key) => {
                self.insert(key);
            }
            &TreeOp::Remove(key) => {
                self.remove(key);
            }
            &TreeOp::RotateLeft(key) => {
                self.rotate_left(key);
            }
            &TreeOp::RotateRight(key) => {
                self.rotate_right(key);
            }
            &TreeOp::SetRed(ref keycolors) => {
                for &(key, red) in keycolors.iter() {
                    self.set_is_red(key, red);
                }
            }
        }
    }

    pub fn insert(&mut self, new_key: i32) -> bool {
        if self.nodes.contains_key(&new_key) {
            false
        } else if let Some(root_key) = self.root {
            let mut parent_key = root_key;
            loop {
                let parent_node = self.nodes.get_mut(&parent_key).unwrap();
                if new_key < parent_key {
                    if let Some(left_key) = parent_node.left_child {
                        parent_key = left_key;
                    } else {
                        parent_node.left_child = Some(new_key);
                        break;
                    }
                } else {
                    if let Some(right_key) = parent_node.right_child {
                        parent_key = right_key;
                    } else {
                        parent_node.right_child = Some(new_key);
                        break;
                    }
                }
            }
            let node = Node {
                parent: Some(parent_key),
                left_child: None,
                right_child: None,
                is_red: true,
            };
            self.nodes.insert(new_key, node);
            debug_assert!(self.is_valid());
            true
        } else {
            debug_assert!(self.nodes.is_empty());
            let node = Node {
                parent: None,
                left_child: None,
                right_child: None,
                is_red: true,
            };
            self.nodes.insert(new_key, node);
            self.root = Some(new_key);
            debug_assert!(self.is_valid());
            true
        }
    }

    pub fn remove(&mut self, key: i32) -> bool {
        let (opt_parent, opt_left_child, opt_right_child, was_red) = {
            if let Some(node) = self.nodes.get(&key) {
                (node.parent, node.left_child, node.right_child, node.is_red)
            } else {
                return false;
            }
        };
        match (opt_left_child, opt_right_child) {
            (None, None) => {
                self.set_child(opt_parent, key, None);
            }
            (Some(left_child_key), None) => {
                self.set_child(opt_parent, key, Some(left_child_key));
                self.set_parent(left_child_key, opt_parent);
            }
            (None, Some(right_child_key)) => {
                self.set_child(opt_parent, key, Some(right_child_key));
                self.set_parent(right_child_key, opt_parent);
            }
            (Some(left_child_key), Some(right_child_key)) => {
                let mut predecessor_key = left_child_key;
                while let Some(next_key) = self.right_child(predecessor_key) {
                    predecessor_key = next_key;
                }
                if predecessor_key != left_child_key {
                    let pred_parent = self.parent(predecessor_key);
                    let pred_child = self.left_child(predecessor_key);
                    if let Some(pred_child_key) = pred_child {
                        self.set_parent(pred_child_key, pred_parent);
                    }
                    self.set_child(pred_parent, predecessor_key, pred_child);
                    self.set_parent(left_child_key, Some(predecessor_key));
                }
                self.set_parent(right_child_key, Some(predecessor_key));
                self.set_child(opt_parent, key, Some(predecessor_key));
                let node = self.nodes.get_mut(&predecessor_key).unwrap();
                node.parent = opt_parent;
                node.right_child = Some(right_child_key);
                if predecessor_key != left_child_key {
                    node.left_child = Some(left_child_key)
                }
                node.is_red = was_red;
            }
        }
        self.nodes.remove(&key);
        debug_assert!(self.is_valid());
        true
    }

    pub fn rotate_left(&mut self, key: i32) -> bool { self.rotate(key, true) }

    pub fn rotate_right(&mut self, key: i32) -> bool {
        self.rotate(key, false)
    }

    fn rotate(&mut self, key: i32, left: bool) -> bool {
        let opt_child = if left {
            self.right_child(key)
        } else {
            self.left_child(key)
        };
        if let Some(child_key) = opt_child {
            let opt_parent = self.parent(key);
            let opt_grandchild = if left {
                self.left_child(child_key)
            } else {
                self.right_child(child_key)
            };
            {
                let node = self.nodes.get_mut(&key).unwrap();
                node.parent = Some(child_key);
                if left {
                    node.right_child = opt_grandchild;
                } else {
                    node.left_child = opt_grandchild;
                }
            }
            {
                let child_node = self.nodes.get_mut(&child_key).unwrap();
                child_node.parent = opt_parent;
                if left {
                    child_node.left_child = Some(key);
                } else {
                    child_node.right_child = Some(key);
                }
            }
            if let Some(parent_key) = opt_parent {
                let parent_node = self.nodes.get_mut(&parent_key).unwrap();
                if child_key < parent_key {
                    debug_assert_eq!(parent_node.left_child, Some(key));
                    parent_node.left_child = Some(child_key);
                } else {
                    debug_assert_eq!(parent_node.right_child, Some(key));
                    parent_node.right_child = Some(child_key);
                }
            } else {
                debug_assert_eq!(self.root, Some(key));
                self.root = Some(child_key);
            }
            if let Some(grandchild_key) = opt_grandchild {
                self.nodes.get_mut(&grandchild_key).unwrap().parent =
                    Some(key);
            }
            debug_assert!(self.is_valid());
            true
        } else {
            false
        }
    }

    fn set_parent(&mut self, child_key: i32, new_parent: Option<i32>) {
        debug_assert!(self.nodes.contains_key(&child_key));
        self.nodes.get_mut(&child_key).unwrap().parent = new_parent;
    }

    fn set_child(&mut self, opt_parent: Option<i32>, old_child_key: i32,
                 new_child: Option<i32>) {
        if let Some(parent_key) = opt_parent {
            debug_assert!(self.nodes.contains_key(&parent_key));
            let parent_node = self.nodes.get_mut(&parent_key).unwrap();
            if old_child_key < parent_key {
                debug_assert_eq!(parent_node.left_child, Some(old_child_key));
                parent_node.left_child = new_child;
            } else {
                debug_assert_eq!(parent_node.right_child, Some(old_child_key));
                parent_node.right_child = new_child;
            }
        } else {
            debug_assert_eq!(self.root, Some(old_child_key));
            self.root = new_child;
        }
    }

    pub fn is_valid(&self) -> bool {
        if let Some(root_key) = self.root {
            if let Some(root_node) = self.nodes.get(&root_key) {
                if let Some(parent_key) = root_node.parent {
                    println!("Root node has parent ({}).", parent_key);
                    return false;
                }
            } else {
                println!("Root key ({}) not in tree.", root_key);
                return false;
            }
        } else if !self.nodes.is_empty() {
            println!("No root, but tree has {} nodes.", self.nodes.len());
            return false;
        }
        for (&key, node) in self.nodes.iter() {
            if let Some(parent_key) = node.parent {
                if let Some(parent_node) = self.nodes.get(&parent_key) {
                    if key < parent_key {
                        if parent_node.left_child != Some(key) {
                            println!("Node {} is not left child ({:?}) of \
                                      its parent ({}).",
                                     key,
                                     parent_node.left_child,
                                     parent_key);
                            return false;
                        }
                    } else {
                        if parent_node.right_child != Some(key) {
                            println!("Node {} is not right child ({:?}) of \
                                      its parent ({}).",
                                     key,
                                     parent_node.right_child,
                                     parent_key);
                            return false;
                        }
                    }
                } else {
                    println!("Parent ({}) of {} not in tree.",
                             parent_key,
                             key);
                    return false;
                }
            } else if self.root != Some(key) {
                println!("Node {} has no parent, but isn't root ({:?}).",
                         key,
                         self.root);
                return false;
            }
            if let Some(left_key) = node.left_child {
                if left_key >= key {
                    println!("Left child ({}) of {} is out of order.",
                             left_key,
                             key);
                    return false;
                }
                if let Some(left_node) = self.nodes.get(&left_key) {
                    if left_node.parent != Some(key) {
                        println!("Left child ({}) of {} has wrong parent \
                                  ({:?}).",
                                 left_key,
                                 key,
                                 left_node.parent);
                    }
                } else {
                    println!("Left child ({}) of {} not in tree.",
                             left_key,
                             key);
                    return false;
                }
            }
            if let Some(right_key) = node.right_child {
                if right_key <= key {
                    println!("Right child ({}) of {} is out of order.",
                             right_key,
                             key);
                    return false;
                }
                if let Some(right_node) = self.nodes.get(&right_key) {
                    if right_node.parent != Some(key) {
                        println!("Right child ({}) of {} has wrong parent \
                                  ({:?}).",
                                 right_key,
                                 key,
                                 right_node.parent);
                    }
                } else {
                    println!("Right child ({}) of {} not in tree.",
                             right_key,
                             key);
                    return false;
                }
            }
        }
        true
    }
}

impl Tomlable for BasicTree {
    fn to_toml(&self) -> toml::Value {
        let mut array = toml::value::Array::new();
        for (key, parent, is_red) in self.signature().into_iter() {
            let mut item = toml::value::Array::new();
            item.push(toml::Value::Integer(key as i64));
            item.push(toml::Value::Integer(parent as i64));
            item.push(toml::Value::Integer(if is_red { 1 } else { 0 }));
            array.push(toml::Value::Array(item));
        }
        toml::Value::Array(array)
    }

    fn from_toml(value: toml::Value) -> BasicTree {
        let mut signature: Vec<(i32, i32, bool)> = Vec::new();
        for item in Vec::<Vec<i32>>::from_toml(value).into_iter() {
            if item.len() != 3 {
                continue;
            }
            signature.push((item[0], item[1], item[2] != 0));
        }
        BasicTree::from_signature(signature)
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use save::util::Tomlable;
    use super::BasicTree;

    #[test]
    fn tree_structure() {
        // 1   3   6
        //  \ /   /
        //   2   5
        //    \ /
        //     4
        let mut tree = BasicTree::new();
        assert!(tree.insert(4));
        assert!(tree.insert(2));
        assert!(tree.insert(5));
        assert!(tree.insert(1));
        assert!(tree.insert(3));
        assert!(tree.insert(6));

        assert_eq!(tree.root(), Some(4));
        assert_eq!(tree.parent(4), None);
        assert_eq!(tree.sibling(4), None);

        assert_eq!(tree.parent(2), Some(4));
        assert_eq!(tree.sibling(2), Some(5));
        assert_eq!(tree.left_child(2), Some(1));
        assert_eq!(tree.right_child(2), Some(3));

        assert_eq!(tree.parent(3), Some(2));
        assert_eq!(tree.sibling(3), Some(1));
        assert_eq!(tree.left_child(3), None);
        assert_eq!(tree.right_child(3), None);

        assert_eq!(tree.parent(5), Some(4));
        assert_eq!(tree.sibling(5), Some(2));
        assert_eq!(tree.left_child(5), None);
        assert_eq!(tree.right_child(5), Some(6));

        assert_eq!(tree.parent(6), Some(5));
        assert_eq!(tree.sibling(6), None);
        assert_eq!(tree.left_child(6), None);
        assert_eq!(tree.right_child(6), None);
    }

    #[test]
    fn insertion() {
        let mut tree = BasicTree::new();
        assert_eq!(tree.root(), None);
        assert_eq!(tree.len(), 0);

        //   5
        assert!(tree.insert(5));
        assert!(tree.contains(5));
        assert_eq!(tree.root(), Some(5));
        assert_eq!(tree.len(), 1);
        assert!(!tree.insert(5));
        assert_eq!(tree.len(), 1);

        // 3
        //  \
        //   5
        assert!(tree.insert(3));
        assert!(tree.contains(3));
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.parent(3), Some(5));
        assert_eq!(tree.left_child(5), Some(3));

        //   4
        //  /
        // 3
        //  \
        //   5
        assert!(tree.insert(4));
        assert!(tree.contains(4));
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.parent(4), Some(3));
        assert_eq!(tree.right_child(3), Some(4));

        //   4
        //  /
        // 3   6
        //  \ /
        //   5
        assert!(tree.insert(6));
        assert!(tree.contains(6));
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.parent(6), Some(5));
        assert_eq!(tree.right_child(5), Some(6));
    }

    #[test]
    fn removal_1() {
        // 1   4   7
        //  \ /   /
        //   3   6
        //    \ /
        //     5
        let mut tree = BasicTree::new();
        assert!(tree.insert(5));
        assert!(tree.insert(3));
        assert!(tree.insert(4));
        assert!(tree.insert(6));
        assert!(tree.insert(1));
        assert!(tree.insert(7));

        // 1   4
        //  \ /
        //   3   7
        //    \ /
        //     5
        assert!(tree.remove(6));
        assert!(!tree.contains(6));
        assert_eq!(tree.right_child(5), Some(7));
        assert_eq!(tree.parent(7), Some(5));

        //     4
        //    /
        //   1   7
        //    \ /
        //     5
        assert!(tree.remove(3));
        assert!(!tree.contains(3));
        assert_eq!(tree.left_child(5), Some(1));
        assert_eq!(tree.parent(1), Some(5));
        assert_eq!(tree.left_child(1), None);
        assert_eq!(tree.right_child(1), Some(4));
        assert_eq!(tree.parent(4), Some(1));

        //   1   7
        //    \ /
        //     4
        assert!(tree.remove(5));
        assert!(!tree.contains(5));
        assert_eq!(tree.root(), Some(4));
        assert_eq!(tree.parent(4), None);
        assert_eq!(tree.left_child(4), Some(1));
        assert_eq!(tree.parent(1), Some(4));
        assert_eq!(tree.right_child(4), Some(7));
        assert_eq!(tree.parent(7), Some(4));

        //   1
        //    \
        //     4
        assert!(tree.remove(7));
        assert!(!tree.contains(7));
        assert_eq!(tree.left_child(4), Some(1));
        assert_eq!(tree.right_child(4), None);

        //     4
        assert!(tree.remove(1));
        assert!(!tree.contains(1));
        assert_eq!(tree.left_child(4), None);
        assert_eq!(tree.right_child(4), None);

        // <empty>
        assert!(tree.remove(4));
        assert!(!tree.contains(4));
        assert_eq!(tree.root(), None);
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn removal_2() {
        // 1
        //  \
        //   2   4
        //    \ /
        //     3
        let mut tree = BasicTree::new();
        assert!(tree.insert(3));
        assert!(tree.insert(2));
        assert!(tree.insert(1));
        assert!(tree.insert(4));

        //   1   4
        //    \ /
        //     2
        assert!(tree.remove(3));
        assert!(!tree.contains(3));
        assert_eq!(tree.root(), Some(2));
        assert_eq!(tree.parent(2), None);
        assert_eq!(tree.left_child(2), Some(1));
        assert_eq!(tree.parent(1), Some(2));
        assert_eq!(tree.right_child(2), Some(4));
        assert_eq!(tree.parent(4), Some(2));
    }

    #[test]
    fn removal_3() {
        // 2
        //  \
        //   3
        //  /
        // 1   5
        //  \ /
        //   4
        let mut tree = BasicTree::new();
        assert!(tree.insert(4));
        assert!(tree.insert(5));
        assert!(tree.insert(1));
        assert!(tree.insert(3));
        assert!(tree.insert(2));

        //   2
        //  /
        // 1   5
        //  \ /
        //   3
        assert!(tree.remove(4));
        assert!(!tree.contains(4));
        assert_eq!(tree.root(), Some(3));
        assert_eq!(tree.parent(3), None);
        assert_eq!(tree.left_child(3), Some(1));
        assert_eq!(tree.parent(1), Some(3));
        assert_eq!(tree.right_child(3), Some(5));
        assert_eq!(tree.parent(5), Some(3));
        assert_eq!(tree.left_child(1), None);
        assert_eq!(tree.right_child(1), Some(2));
        assert_eq!(tree.parent(2), Some(1));
    }

    #[test]
    fn rotation() {
        //     4   6
        //      \ /
        //   2   5
        //    \ /
        //     3
        //    /
        //   1
        let mut tree = BasicTree::new();
        assert!(tree.insert(1));
        assert!(tree.insert(3));
        assert!(tree.insert(2));
        assert!(tree.insert(5));
        assert!(tree.insert(4));
        assert!(tree.insert(6));
        assert_eq!(tree.root(), Some(1));
        assert_eq!(tree.right_child(1), Some(3));
        assert_eq!(tree.parent(3), Some(1));
        assert_eq!(tree.left_child(3), Some(2));
        assert_eq!(tree.right_child(3), Some(5));
        assert_eq!(tree.parent(4), Some(5));
        assert_eq!(tree.parent(5), Some(3));
        assert_eq!(tree.left_child(5), Some(4));
        assert_eq!(tree.right_child(5), Some(6));

        // 2   4
        //  \ /
        //   3   6
        //    \ /
        //     5
        //    /
        //   1
        assert!(tree.rotate_left(3));
        assert_eq!(tree.root(), Some(1));
        assert_eq!(tree.right_child(1), Some(5));
        assert_eq!(tree.parent(3), Some(5));
        assert_eq!(tree.left_child(3), Some(2));
        assert_eq!(tree.right_child(3), Some(4));
        assert_eq!(tree.parent(4), Some(3));
        assert_eq!(tree.parent(5), Some(1));
        assert_eq!(tree.left_child(5), Some(3));
        assert_eq!(tree.right_child(5), Some(6));

        //     4   6
        //      \ /
        //   2   5
        //    \ /
        //     3
        //    /
        //   1
        assert!(tree.rotate_right(5));
        assert_eq!(tree.root(), Some(1));
        assert_eq!(tree.right_child(1), Some(3));
        assert_eq!(tree.parent(3), Some(1));
        assert_eq!(tree.left_child(3), Some(2));
        assert_eq!(tree.right_child(3), Some(5));
        assert_eq!(tree.parent(4), Some(5));
        assert_eq!(tree.parent(5), Some(3));
        assert_eq!(tree.left_child(5), Some(4));
        assert_eq!(tree.right_child(5), Some(6));

        //   2 4   6
        //  /   \ /
        // 1     5
        //  \   /
        //    3
        assert!(tree.rotate_left(1));
        assert_eq!(tree.root(), Some(3));
        assert_eq!(tree.right_child(1), Some(2));
        assert_eq!(tree.parent(3), None);
        assert_eq!(tree.left_child(3), Some(1));
        assert_eq!(tree.right_child(3), Some(5));
        assert_eq!(tree.parent(2), Some(1));
        assert_eq!(tree.parent(5), Some(3));
    }

    #[test]
    fn toml_round_trip() {
        // <empty>
        let mut tree = BasicTree::new();
        assert_eq!(BasicTree::from_toml(tree.to_toml()), tree);

        //     4
        assert!(tree.insert(4));
        assert_eq!(BasicTree::from_toml(tree.to_toml()), tree);

        //   2   5
        //    \ /
        //     4
        assert!(tree.insert(2));
        assert!(tree.insert(5));
        assert_eq!(BasicTree::from_toml(tree.to_toml()), tree);

        // 1   3   6
        //  \ /   /
        //   2   5
        //    \ /
        //     4
        assert!(tree.insert(1));
        assert!(tree.insert(3));
        assert!(tree.insert(6));
        assert_eq!(BasicTree::from_toml(tree.to_toml()), tree);
    }

    #[test]
    fn from_invalid_signature_with_loop() {
        let tree = BasicTree::from_signature(
            vec![(1, 1, false), (2, 3, true), (3, 2, true)],
        );
        assert_eq!(tree.len(), 0);
    }
}

// ========================================================================= //
