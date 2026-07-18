package main

func main() {

}

type Iterator struct{}

func (this *Iterator) hasNext() bool {
	// Returns true if the iteration has more elements.
	return false
}

func (this *Iterator) next() int {
	return 0 // Returns the next element in the iteration.
}

type PeekingIterator struct {
	peeked *int
	inner  *Iterator
}

func Constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{
		peeked: nil,
		inner:  iter,
	}
}

func (this *PeekingIterator) hasNext() bool {
	return this.peeked != nil || this.inner.hasNext()
}

func (this *PeekingIterator) next() int {
	if this.peeked != nil {
		value := *this.peeked
		this.peeked = nil
		return value
	}

	return this.inner.next()
}

func (this *PeekingIterator) peek() int {
	if this.peeked != nil {
		return *this.peeked
	}

	value := this.inner.next()
	this.peeked = &value
	return *this.peeked
}
