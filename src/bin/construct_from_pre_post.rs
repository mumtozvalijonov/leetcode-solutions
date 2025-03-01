// 889. Construct Binary Tree from Preorder and Postorder Traversal
use std::cell::RefCell;
use std::collections::HashMap;
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
    let _ = Solution::construct_from_pre_post(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![4, 5, 3, 6, 2, 8, 7, 1],
    );
}

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Build a hash map for quick lookup of postorder indices.
        let mut post_index = HashMap::new();
        for (i, &val) in postorder.iter().enumerate() {
            post_index.insert(val, i);
        }
        Self::build(
            &preorder,
            0,
            preorder.len(),
            &postorder,
            0,
            postorder.len(),
            &post_index,
        )
    }

    fn build(
        preorder: &Vec<i32>,
        pre_start: usize,
        pre_end: usize,
        postorder: &Vec<i32>,
        post_start: usize,
        post_end: usize,
        post_index: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start >= pre_end {
            return None;
        }
        let root_val = preorder[pre_start];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        // If there's only one node, it's a leaf.
        if pre_end - pre_start == 1 {
            return Some(root);
        }

        // The left subtree's root is the next element in preorder.
        let left_root_val = preorder[pre_start + 1];
        // Find the index of the left subtree's root in postorder.
        let left_root_index = post_index[&left_root_val];
        // Determine the size of the left subtree.
        let left_size = left_root_index - post_start + 1;

        root.borrow_mut().left = Self::build(
            preorder,
            pre_start + 1,
            pre_start + 1 + left_size,
            postorder,
            post_start,
            post_start + left_size,
            post_index,
        );
        root.borrow_mut().right = Self::build(
            preorder,
            pre_start + 1 + left_size,
            pre_end,
            postorder,
            post_start + left_size,
            post_end - 1, // Exclude the current root in postorder.
            post_index,
        );
        Some(root)
    }
}
