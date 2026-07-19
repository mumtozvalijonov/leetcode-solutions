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

struct Solution {}

impl Solution {
    fn explore_max_depth(n: Option<Rc<RefCell<TreeNode>>>, cur_depth: i32) -> i32 {
        if n.is_none() {
            return cur_depth - 1;
        }

        let n = n.unwrap();
        let n = n.borrow();
        let left = n.left.clone();
        let right = n.right.clone();

        Solution::explore_max_depth(left, cur_depth + 1)
            .max(Solution::explore_max_depth(right, cur_depth + 1))
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::explore_max_depth(root, 1)
    }
}
