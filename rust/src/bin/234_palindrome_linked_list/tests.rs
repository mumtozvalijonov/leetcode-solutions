use rstest::rstest;

use crate::solution::{ListNode, Solution};

#[rstest]
#[case(vec![1,2,2,1], true)]
#[case(vec![1,2,5,2,1], true)]
#[case(vec![1,2], false)]
#[case(vec![1,2,1], true)]
fn test(#[case] input: Vec<i32>, #[case] expected: bool) {
    let mut head = Some(Box::new(ListNode::new(input[0])));
    let mut tail = &mut head;
    for &val in input.iter().skip(1) {
        (*tail).as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
        tail = &mut (*tail).as_mut().unwrap().next;
    }

    let actual = Solution::is_palindrome(head);
    assert_eq!(actual, expected);
}
