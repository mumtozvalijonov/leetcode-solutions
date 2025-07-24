package main

import (
	"testing"
)

func TestIntersectionNode(t *testing.T) {
	var tests = []struct {
		name         string
		intersectVal int
		listA        []int
		listB        []int
		skipA        int
		skipB        int
	}{
		{"Intersected at '8'", 8, []int{4, 1, 8, 4, 5}, []int{5, 6, 1, 8, 4, 5}, 2, 3},
		{"Intersected at '2'", 2, []int{1, 9, 1, 2, 4}, []int{3, 2, 4}, 3, 1},
		{"No intersection", 0, []int{2, 6, 4}, []int{1, 5}, 3, 2},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var intersectionNode, headA, tailA, headB, tailB *ListNode

			for _, val := range tt.listA {
				if headA == nil {
					headA = &ListNode{Val: val}
					tailA = headA
				} else {
					tailA.Next = &ListNode{Val: val}
					tailA = tailA.Next
				}
				if tt.skipA == 0 {
					intersectionNode = tailA
				}
				tt.skipA--
			}

			for _, val := range tt.listB {
				if tt.skipB == 0 {
					if headB == nil {
						headB = intersectionNode
						tailB = intersectionNode
					} else {
						tailB.Next = intersectionNode
						tailB = tailB.Next
					}
					break
				}
				if headB == nil {
					headB = &ListNode{Val: val}
					tailB = headB
				} else {
					tailB.Next = &ListNode{Val: val}
					tailB = tailB.Next
				}
				tt.skipB--
			}

			ans := getIntersectionNode(headA, headB)
			if ans != intersectionNode || (tt.intersectVal != 0 && (ans == nil || ans.Val != tt.intersectVal)) {
				t.Errorf("got %+v, want %+v", ans, intersectionNode)
			}
		})
	}
}
