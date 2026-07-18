use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() {
            return q.is_none();
        }

        if q.is_none() {
            return false;
        }

        let p_root = p.unwrap();
        let q_root = q.unwrap();
        let p_root = p_root.borrow();
        let q_root = q_root.borrow();

        if p_root.val != q_root.val {
            return false;
        }

        let is_left_same = Solution::is_same_tree(p_root.left.clone(), q_root.left.clone());
        if !is_left_same {
            return false;
        }

        let p_right = p_root.right.clone();
        let q_right = q_root.right.clone();

        Solution::is_same_tree(p_right, q_right)
    }
}

#[cfg(test)]
mod tests {}
