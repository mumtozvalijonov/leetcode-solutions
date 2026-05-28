package main

import (
	"container/list"
	"fmt"
)

type Node struct {
	key, value, count int
}

type LFUCache struct {
	capacity int
	nodeMap  map[int]*list.Element
	listMap  map[int]*list.List
}

func Constructor(capacity int) LFUCache {
	nodeMap := make(map[int]*list.Element, capacity)
	listMap := make(map[int]*list.List)
	return LFUCache{capacity: capacity, nodeMap: nodeMap, listMap: listMap}
}

func (this *LFUCache) Evict() {
	for i := 1; i <= 200_000; i++ {
		if l, ok := this.listMap[i]; ok {
			e := l.Front()
			l.Remove(e)
			if l.Len() == 0 {
				delete(this.listMap, i)
			}
			n := e.Value.(*Node)
			// fmt.Println("Evict")
			delete(this.nodeMap, n.key)
			break
		}
	}
}

func (this *LFUCache) Size() int {
	size := 0
	for _, v := range this.listMap {
		size += v.Len()
	}
	return size
}

func (this *LFUCache) Get(key int) int {
	// fmt.Printf("Get %d: ", key)
	e, ok := this.nodeMap[key]
	if !ok {
		// fmt.Println(-1)
		return -1
	}
	n := e.Value.(*Node)

	// Remove from current frequency bucket
	currentBucket := this.listMap[n.count]
	currentBucket.Remove(e)
	if currentBucket.Len() == 0 {
		// fmt.Printf("Here %d\n", len(this.listMap))
		delete(this.listMap, n.count)
		// fmt.Printf("Here %d\n", len(this.listMap))
	}

	// Move to the next frequence bucket
	n.count++
	if nextBucket, ok := this.listMap[n.count]; ok {
		e = nextBucket.PushBack(n)
	} else {
		newList := list.New()
		e = newList.PushBack(n)
		this.listMap[n.count] = newList
	}
	this.nodeMap[key] = e

	// fmt.Println(n.value)
	return n.value

}

func (this *LFUCache) Put(key int, value int) {
	// fmt.Println("Put", key, value)
	e, ok := this.nodeMap[key]
	if !ok {
		if this.Size() == this.capacity {
			this.Evict()
		}
		e = &list.Element{Value: &Node{key: key}}
	}

	n := e.Value.(*Node)
	n.value = value

	// Remove from current frequency bucket
	if ok {
		currentBucket := this.listMap[n.count]
		currentBucket.Remove(e)
		if currentBucket.Len() == 0 {
			delete(this.listMap, n.count)
		}
	}

	n.count++
	if nextBucket, ok := this.listMap[n.count]; ok {
		e = nextBucket.PushBack(n)
	} else {
		nextBucket = list.New()
		e = nextBucket.PushBack(n)
		this.listMap[n.count] = nextBucket

	}

	this.nodeMap[key] = e
}

func main() {
	lfu := Constructor(3)
	lfu.Put(2, 2)
	lfu.Put(1, 1)
	fmt.Println("Size:", lfu.Size())
	lfu.Get(2)
	fmt.Println("Size:", lfu.Size())
	lfu.Get(1)
	fmt.Println("Size:", lfu.Size())
	lfu.Get(2)
	fmt.Println("Size:", lfu.Size())
	lfu.Put(3, 3)
	fmt.Println("Size:", lfu.Size())
	lfu.Put(4, 4)
	fmt.Println("Size:", lfu.Size())
	lfu.Get(3)
	fmt.Println("Size:", lfu.Size())
	lfu.Get(2)
	fmt.Println("Size:", lfu.Size())
	lfu.Get(1)
	fmt.Println("Size:", lfu.Size())
	lfu.Get(4)

}
