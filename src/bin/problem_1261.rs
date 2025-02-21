// 1261. Find Elements in a Contaminated Binary Tree

use std::{cell::RefCell, rc::Rc};

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
struct FindElements {
    memo: [bool; 1_000_001],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn _recover(root: Rc<RefCell<TreeNode>>, memo: &mut [bool]) {
        let root = root.borrow();
        if let Some(left) = root.left.clone() {
            let val = root.val * 2 + 1;
            left.borrow_mut().val = val;
            let idx = val as usize;
            if idx < memo.len() {
                memo[idx] = true;
            } else {
                return;
            }
            FindElements::_recover(left, memo);
        }

        if let Some(right) = root.right.clone() {
            let val = root.val * 2 + 2;
            right.borrow_mut().val = root.val * 2 + 2;
            let idx = val as usize;
            if idx < memo.len() {
                memo[idx] = true;
            } else {
                return;
            }
            FindElements::_recover(right, memo);
        }
    }

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let root = root.unwrap();
        root.borrow_mut().val = 0;
        let mut memo = [false; 1_000_001];
        memo[0] = true;
        FindElements::_recover(root, &mut memo);

        FindElements { memo }
    }

    fn find(&self, target: i32) -> bool {
        self.memo[target as usize]
    }
}

fn main() {
    // Construct the binary tree: [-1, null, -1, -1, null, -1]
    let root = Rc::new(RefCell::new(TreeNode::new(-1)));
    let right1 = Rc::new(RefCell::new(TreeNode::new(-1)));
    let right2 = Rc::new(RefCell::new(TreeNode::new(-1)));
    let right3 = Rc::new(RefCell::new(TreeNode::new(-1)));

    root.borrow_mut().right = Some(right1.clone());
    right1.borrow_mut().left = Some(right2.clone());
    right2.borrow_mut().right = Some(right3.clone());

    // Initialize FindElements with the contaminated tree
    let obj = FindElements::new(Some(root));

    // Test the find method with the given targets
    let targets = [2, 3, 4, 5];
    for &target in &targets {
        let result = obj.find(target);
        println!("find({}): {}", target, result);
    }
}
