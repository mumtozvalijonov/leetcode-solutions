use crate::linked_list::ListNode;

use super::solution::Solution;

#[test]
fn test_1() {
    let head = ListNode::build_linked_list(Vec::from([1, 0, 1]));
    assert_eq!(Solution::get_decimal_value(head), 5);
}

#[test]
fn test_2() {
    let head = ListNode::build_linked_list(Vec::from([1, 1, 1]));
    assert_eq!(Solution::get_decimal_value(head), 7);
}
