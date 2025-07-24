use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Trie node structure that holds:
/// - children: mapping from character to child node.
/// - count: how many words pass through this node.
/// - depth: the depth (i.e. prefix length) for this node.
pub struct Node {
    pub children: RefCell<HashMap<char, Rc<Node>>>,
    pub count: RefCell<i32>,
    pub depth: usize,
}

impl Node {
    pub fn new(depth: usize) -> Self {
        Node {
            children: RefCell::new(HashMap::new()),
            count: RefCell::new(0),
            depth,
        }
    }
}
