// 1028. Recover a Tree From Preorder Traversal

use std::cell::RefCell;
use std::collections::VecDeque;
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

    fn print_bf(root: Rc<RefCell<TreeNode>>) {
        let mut queue = VecDeque::from([root]);

        while let Some(node) = queue.pop_front() {
            print!("{} ", node.borrow().val);

            if let Some(left) = node.borrow().left.clone() {
                queue.push_back(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                queue.push_back(right);
            }
        }
    }
}

struct Solution;

fn main() {
    // let root = Rc::new(RefCell::new(TreeNode::new(1)));
    // root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    // root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // if let Some(left) = &root.borrow().left {
    //     let mut left = left.borrow_mut();
    //     left.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    //     left.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // }
    // if let Some(right) = &root.borrow().right {
    //     let mut right = right.borrow_mut();
    //     right.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    //     right.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    // }

    let root = Solution::recover_from_preorder("1-401--349---90--88".to_string());
    TreeNode::print_bf(root.unwrap());
}

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let bytes = traversal.as_bytes();
        let n = bytes.len();
        let mut pos = 0;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        while pos < n {
            // Count '-' characters to determine the node's depth.
            let mut depth = 0;
            while pos < n && bytes[pos] == b'-' {
                depth += 1;
                pos += 1;
            }

            // Manually construct the integer by processing each digit.
            let mut num = 0;
            while pos < n && bytes[pos].is_ascii_digit() {
                num = num * 10 + (bytes[pos] - b'0') as i32;
                pos += 1;
            }

            let node = Rc::new(RefCell::new(TreeNode {
                val: num,
                left: None,
                right: None,
            }));

            // Ensure the stack's size matches the current depth.
            // Pop nodes until we find the correct parent.
            while stack.len() > depth {
                stack.pop();
            }

            // If there's a parent node, attach the current node as its left or right child.
            if let Some(parent) = stack.last() {
                let mut parent_ref = parent.borrow_mut();
                if parent_ref.left.is_none() {
                    parent_ref.left = Some(node.clone());
                } else {
                    parent_ref.right = Some(node.clone());
                }
            }

            // Push the current node onto the stack.
            stack.push(node);
        }

        // The first element in the stack is the root of the tree.
        stack.first().cloned()
    }
}
