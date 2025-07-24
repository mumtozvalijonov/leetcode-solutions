// 94. Binary Tree Inorder Traversal

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

fn main() {
    let mut root = TreeNode::new(1);
    let mut right = TreeNode::new(2);
    right.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    root.left = None;
    root.right = Some(Rc::new(RefCell::new(right)));

    println!(
        "{:?}",
        Solution::inorder_traversal(Some(Rc::new(RefCell::new(root))))
    );
}

impl Solution {
    fn _traverse(node: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>) {
        match node {
            Some(node) => {
                Self::_traverse(node.borrow().left.clone(), path);
                path.push(node.borrow().val);
                Self::_traverse(node.borrow().right.clone(), path);
            }
            None => {}
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::_traverse(root, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let root = None;
        assert_eq!(Solution::inorder_traversal(root), vec![]);
    }

    #[test]
    fn test_single_node_tree() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::inorder_traversal(root), vec![1]);
    }

    #[test]
    fn test_left_skewed_tree() {
        let mut root = TreeNode::new(3);
        let mut left1 = TreeNode::new(2);
        left1.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.left = Some(Rc::new(RefCell::new(left1)));

        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_right_skewed_tree() {
        let mut root = TreeNode::new(1);
        let mut right1 = TreeNode::new(2);
        right1.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.right = Some(Rc::new(RefCell::new(right1)));

        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_balanced_tree() {
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1, 2, 3]
        );
    }
}
