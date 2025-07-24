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

    pub fn build_linked_list(values: Vec<i32>) -> Option<Box<Self>> {
        let mut values = values;
        values.reverse();
        let mut head = None;
        for v in values {
            if head.is_none() {
                head = Some(Box::new(ListNode::new(v)));
            } else {
                let mut tmp = Box::new(ListNode::new(v));
                tmp.next = head;
                head = Some(tmp);
            }
        }
        head
    }
}
