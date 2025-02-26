from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        dummy_node = ListNode(42)

        while head:
            if head.next == dummy_node:
                return True
            prev = head
            head = head.next
            prev.next = dummy_node

        return False
