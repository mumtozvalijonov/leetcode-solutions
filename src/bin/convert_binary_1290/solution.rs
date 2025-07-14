use crate::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut cur = head.clone();
        let mut result = 0;
        while let Some(node) = cur {
            result <<= 1;
            result += node.val;
            cur = node.next;
        }
        result
    }
}
