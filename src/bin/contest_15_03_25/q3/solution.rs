use std::{collections::BTreeSet, rc::Rc};

use super::tree::*;

pub struct Solution;

impl Solution {
    /// For each removal of one word from `words`, returns the length of the longest common prefix among any k strings
    /// from the remaining array. If the remaining array has fewer than k strings, returns 0.
    pub fn longest_common_prefix(words: Vec<String>, k: i32) -> Vec<i32> {
        let n = words.len();
        // If removing one word leaves fewer than k words, answer is 0 for every removal.
        if n - 1 < k as usize {
            return vec![0; n];
        }

        // Compute the maximum word length (this is the maximum possible depth in the trie).
        let max_depth = words.iter().map(|w| w.len()).max().unwrap_or(0);

        // We'll maintain an array for each depth (0..=max_depth) counting how many nodes at that depth have count >= k.
        // (Depth 0 corresponds to the root; we won’t consider it in the final answer.)
        let mut depth_counter = vec![0; max_depth + 1];
        // And a BTreeSet that contains all depths for which the count is nonzero.
        let mut flagged: BTreeSet<usize> = BTreeSet::new();

        // Create the trie root. (Depth 0.)
        let root = Rc::new(Node::new(0));

        // Insert every word into the trie.
        // (Each inserted character updates the node’s count and—if it crosses the threshold—updates our global counter.)
        for word in &words {
            Self::traverse_and_update(&root, word, 1, k, &mut depth_counter, &mut flagged);
        }

        let mut result = vec![0; n];

        // For each word, temporarily remove it, query the longest common prefix, then reinsert it.
        for (i, word) in words.iter().enumerate() {
            // Remove the word (delta = -1) along its path.
            Self::traverse_and_update(&root, word, -1, k, &mut depth_counter, &mut flagged);

            // After removal, if the total number of words remaining is less than k, answer is 0.
            // (This case is handled by the problem statement.)
            if n - 1 < k as usize {
                result[i] = 0;
            } else {
                // Query: the answer is the maximum depth among nodes with count >= k.
                // We ignore depth 0 since it represents the empty prefix.
                let ans = flagged.iter().next_back().cloned().unwrap_or(0);
                result[i] = ans as i32;
            }

            // Reinsert the word (delta = +1).
            Self::traverse_and_update(&root, word, 1, k, &mut depth_counter, &mut flagged);
        }

        result
    }

    /// Traverses the trie along `word`, updating each node's count by `delta` (which can be +1 or -1).
    /// As each node's count is updated, we check if it crosses the threshold k and update the global
    /// `depth_counter` and `flagged` set accordingly.
    fn traverse_and_update(
        root: &Rc<Node>,
        word: &str,
        delta: i32,
        k: i32,
        depth_counter: &mut Vec<i32>,
        flagged: &mut BTreeSet<usize>,
    ) {
        // Start from the root.
        let mut node = root.clone();
        // For each character in the word:
        for c in word.chars() {
            // Get (or create) the child node for character `c`.
            let child = {
                let mut children = node.children.borrow_mut();
                // If the child does not exist, create a new node with depth = parent's depth + 1.
                children
                    .entry(c)
                    .or_insert_with(|| Rc::new(Node::new(node.depth + 1)))
                    .clone()
            };

            // Update the child's count.
            let old_count = *child.count.borrow();
            let new_count = old_count + delta;
            *child.count.borrow_mut() = new_count;

            // If the node's count crosses the threshold k, update the depth_counter.
            if old_count < k && new_count >= k {
                depth_counter[child.depth] += 1;
                // If this is the first node at this depth meeting the threshold, record the depth.
                if depth_counter[child.depth] == 1 {
                    flagged.insert(child.depth);
                }
            } else if old_count >= k && new_count < k {
                depth_counter[child.depth] -= 1;
                // If no node at this depth meets the threshold anymore, remove the depth.
                if depth_counter[child.depth] == 0 {
                    flagged.remove(&child.depth);
                }
            }

            // Continue down the trie.
            node = child;
        }
    }
}
