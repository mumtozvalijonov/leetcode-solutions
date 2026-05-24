package main

type node struct {
	key, value int
	prev, next *node
}

type list struct {
	head, tail *node
	len        int
}

func (l *list) PushBack(n *node) {
	n.prev = l.tail
	n.next = nil

	if l.tail != nil {
		l.tail.next = n
	}

	if l.head == nil {
		l.head = n
	}

	l.tail = n
	l.len++
}

func (l *list) MoveToBack(n *node) {
	if l.tail == n {
		return
	}

	l.Remove(n)
	l.PushBack(n)
}

func (l *list) Remove(n *node) {
	if n == nil {
		return
	}

	if n.prev != nil {
		n.prev.next = n.next
	}
	if n.next != nil {
		n.next.prev = n.prev
	}

	if n == l.head {
		l.head = n.next
	}
	if n == l.tail {
		l.tail = n.prev
	}

	n.prev = nil
	n.next = nil
	l.len--
}

type LRUCache struct {
	capacity int
	l        *list
	m        map[int]*node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		capacity: capacity,
		l:        &list{},
		m:        make(map[int]*node, capacity),
	}
}

func (this *LRUCache) Get(key int) int {
	n, ok := this.m[key]
	if !ok {
		return -1
	}

	this.l.MoveToBack(n)
	return n.value
}

func (this *LRUCache) Put(key int, value int) {
	if this.capacity == 0 {
		return
	}

	if n, ok := this.m[key]; ok {
		n.value = value
		this.l.MoveToBack(n)
		return
	}

	if this.l.len == this.capacity {
		toDelete := this.l.head
		this.l.Remove(toDelete)
		delete(this.m, toDelete.key)
	}

	newNode := &node{
		key:   key,
		value: value,
	}

	this.l.PushBack(newNode)
	this.m[key] = newNode
}

func main() {}
