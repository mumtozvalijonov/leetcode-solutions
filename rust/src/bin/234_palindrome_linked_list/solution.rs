pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn split_reverse_first_half(
        head: Option<Box<ListNode>>,
        mid_point: usize,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;

        for _ in 0..mid_point {
            // steal out the next node if it exists
            if let Some(mut node) = curr {
                // detach the rest:
                curr = node.next.take();
                // reverse one link:
                node.next = prev;
                // push onto `prev`
                prev = Some(node);
            } else {
                break;
            }
        }

        // now `prev` is the head of the reversed prefix,
        // and `curr` is the untouched remainder.
        (prev, curr)
    }

    fn get_size(head: &Option<Box<ListNode>>) -> usize {
        let mut size = 0;
        let mut cur_node = head;
        while cur_node.is_some() {
            size += 1;
            cur_node = &cur_node.as_ref().unwrap().next;
        }
        size
    }

    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        // Go until end of LinkedList to determine its size
        let size = Self::get_size(&head);

        // Determine mid-point and if the node in the middle should be considered
        // If the number of nodes is odd, the node in the middle can be skipped.
        let (mid_point, skip_mid) = (size / 2, size % 2 == 1);

        // Revert the linked list from its mid point towards head, i.e. head becomes a tail, a middle node becomes head
        let (mut first, mut second) = Self::split_reverse_first_half(head, mid_point);
        if skip_mid {
            second = second.unwrap().next;
        }
        
        while let Some((f, s)) = first.zip(second) {
            if f.val != s.val {
                return false;
            }
            first = f.next;
            second = s.next;
        }
        
        true
    }
}
