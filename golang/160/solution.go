package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func getLinkedListLen(head *ListNode) int {
	var result int

	var cur = head
	for {
		if cur != nil {
			result += 1
		} else {
			break
		}
		cur = cur.Next
	}
	return result
}

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	var aLen = getLinkedListLen(headA)
	var bLen = getLinkedListLen(headB)

	for {
		if aLen > bLen {
			headA = headA.Next
			aLen--
		} else if bLen > aLen {
			headB = headB.Next
			bLen--
		} else {
			break
		}
	}
	
	for {
		if headA == nil {
			break
		}
		if headA == headB {
			return headA
		}
		headA = headA.Next
		headB = headB.Next
	}
	return nil
}
