package main

import "fmt"

func main() {
	var headB = ListNode{Val: 3}
	var headA = ListNode{Val: 1, Next: &headB}
	var node = getIntersectionNode(&headA, &headB)
	fmt.Println(node)
}
