package main

import (
	"container/heap"
	"fmt"
)

type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h IntHeap) Peek() int          { return h[0] }

func (h *IntHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

type KthLargest struct {
	capacity int
	h        IntHeap
}

func Constructor(k int, nums []int) KthLargest {
	h := make(IntHeap, 0, k)

	for _, v := range nums {
		if len(h) < k {
			heap.Push(&h, v)
		} else if v > h.Peek() {
			h[0] = v
			heap.Fix(&h, 0)
		}
	}

	return KthLargest{
		capacity: k,
		h:        h,
	}
}

func (this *KthLargest) Add(val int) int {
	if len(this.h) < this.capacity {
		heap.Push(&this.h, val)
		return this.h.Peek()
	}

	if val > this.h.Peek() {
		this.h[0] = val
		heap.Fix(&this.h, 0)
	}

	return this.h.Peek()
}

func main() {
	kl := Constructor(1, []int{})
	fmt.Println(kl.Add(2))
	fmt.Println(kl.Add(10))
	fmt.Println(kl.Add(9))
	fmt.Println(kl.Add(9))
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * obj := Constructor(k, nums);
 * param_1 := obj.Add(val);
 */
