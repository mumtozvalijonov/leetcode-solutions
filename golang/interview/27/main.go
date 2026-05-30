package generics

import (
	"errors"
	"slices"
)

// ErrEmptyCollection is returned when an operation cannot be performed on an empty collection
var ErrEmptyCollection = errors.New("collection is empty")

//
// 1. Generic Pair
//

// Pair represents a generic pair of values of potentially different types
type Pair[T, U any] struct {
	First  T
	Second U
}

// NewPair creates a new pair with the given values
func NewPair[T, U any](first T, second U) Pair[T, U] {
	return Pair[T, U]{First: first, Second: second}
}

// Swap returns a new pair with the elements swapped
func (p Pair[T, U]) Swap() Pair[U, T] {
	return Pair[U, T]{First: p.Second, Second: p.First}
}

//
// 2. Generic Stack
//

// Stack is a generic Last-In-First-Out (LIFO) data structure
type Stack[T any] struct {
	internal []*T
}

// NewStack creates a new empty stack
func NewStack[T any]() *Stack[T] {
	return &Stack[T]{internal: make([]*T, 0)}
}

// Push adds an element to the top of the stack
func (s *Stack[T]) Push(value T) {
	s.internal = append(s.internal, &value)
}

// Pop removes and returns the top element from the stack
// Returns an error if the stack is empty
func (s *Stack[T]) Pop() (T, error) {
	var zero T
	var err error
	if !s.IsEmpty() {
		idx := s.Size() - 1
		zero = *s.internal[idx]
		s.internal = s.internal[:idx]
	} else {
		err = ErrEmptyCollection
	}
	return zero, err
}

// Peek returns the top element without removing it
// Returns an error if the stack is empty
func (s *Stack[T]) Peek() (T, error) {
	var zero T
	var err error
	if !s.IsEmpty() {
		zero = *s.internal[s.Size()-1]
	} else {
		err = ErrEmptyCollection
	}
	return zero, err
}

// Size returns the number of elements in the stack
func (s *Stack[T]) Size() int {
	return len(s.internal)
}

// IsEmpty returns true if the stack contains no elements
func (s *Stack[T]) IsEmpty() bool {
	return s.Size() == 0
}

//
// 3. Generic Queue
//

// Queue is a generic First-In-First-Out (FIFO) data structure
type Queue[T any] struct {
	internal []*T
}

// NewQueue creates a new empty queue
func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{internal: make([]*T, 0)}
}

// Enqueue adds an element to the end of the queue
func (q *Queue[T]) Enqueue(value T) {
	q.internal = append(q.internal, &value)
}

// Dequeue removes and returns the front element from the queue
// Returns an error if the queue is empty
func (q *Queue[T]) Dequeue() (T, error) {
	var zero T
	var err error
	if !q.IsEmpty() {
		zero = *q.internal[0]
		q.internal = q.internal[1:]
	} else {
		err = ErrEmptyCollection
	}
	return zero, err
}

// Front returns the front element without removing it
// Returns an error if the queue is empty
func (q *Queue[T]) Front() (T, error) {
	var zero T
	var err error
	if !q.IsEmpty() {
		zero = *q.internal[0]
	} else {
		err = ErrEmptyCollection
	}
	return zero, err
}

// Size returns the number of elements in the queue
func (q *Queue[T]) Size() int {
	// TODO: Implement this method
	return len(q.internal)
}

// IsEmpty returns true if the queue contains no elements
func (q *Queue[T]) IsEmpty() bool {
	return q.Size() == 0
}

//
// 4. Generic Set
//

// Set is a generic collection of unique elements
type Set[T comparable] struct {
	internal map[T]bool
}

// NewSet creates a new empty set
func NewSet[T comparable]() *Set[T] {
	return &Set[T]{internal: make(map[T]bool)}
}

// Add adds an element to the set if it's not already present
func (s *Set[T]) Add(value T) {
	s.internal[value] = true
}

// Remove removes an element from the set if it exists
func (s *Set[T]) Remove(value T) {
	delete(s.internal, value)
}

// Contains returns true if the set contains the given element
func (s *Set[T]) Contains(value T) bool {
	_, ok := s.internal[value]
	return ok
}

// Size returns the number of elements in the set
func (s *Set[T]) Size() int {
	return len(s.internal)
}

// Elements returns a slice containing all elements in the set
func (s *Set[T]) Elements() []T {
	keys := make([]T, 0, len(s.internal))
	for k, _ := range s.internal {
		keys = append(keys, k)
	}
	return keys
}

// Union returns a new set containing all elements from both sets
func Union[T comparable](s1, s2 *Set[T]) *Set[T] {
	s3 := NewSet[T]()
	for _, k := range slices.Concat(s1.Elements(), s2.Elements()) {
		s3.Add(k)
	}
	return s3
}

// Intersection returns a new set containing only elements that exist in both sets
func Intersection[T comparable](s1, s2 *Set[T]) *Set[T] {
	s3 := NewSet[T]()
	for _, k := range s1.Elements() {
		if s2.Contains(k) {
			s3.Add(k)
		}
	}

	for _, k := range s2.Elements() {
		if s1.Contains(k) {
			s3.Add(k)
		}
	}
	return s3
}

// Difference returns a new set with elements in s1 that are not in s2
func Difference[T comparable](s1, s2 *Set[T]) *Set[T] {
	s3 := NewSet[T]()

	for _, k := range s1.Elements() {
		if !s2.Contains(k) {
			s3.Add(k)
		}
	}

	return s3
}

//
// 5. Generic Utility Functions
//

// Filter returns a new slice containing only the elements for which the predicate returns true
func Filter[T any](slice []T, predicate func(T) bool) []T {
	result := make([]T, 0)

	for _, v := range slice {
		if predicate(v) {
			result = append(result, v)
		}
	}

	return result
}

// Map applies a function to each element in a slice and returns a new slice with the results
func Map[T, U any](slice []T, mapper func(T) U) []U {
	result := make([]U, 0)

	for _, v := range slice {
		result = append(result, mapper(v))
	}

	return result
}

// Reduce reduces a slice to a single value by applying a function to each element
func Reduce[T, U any](slice []T, initial U, reducer func(U, T) U) U {
	for _, v := range slice {
		initial = reducer(initial, v)
	}
	return initial
}

// Contains returns true if the slice contains the given element
func Contains[T comparable](slice []T, element T) bool {
	return slices.Contains(slice, element)
}

// FindIndex returns the index of the first occurrence of the given element or -1 if not found
func FindIndex[T comparable](slice []T, element T) int {
	return slices.Index(slice, element)
}

// RemoveDuplicates returns a new slice with duplicate elements removed, preserving order
func RemoveDuplicates[T comparable](slice []T) []T {
	s := NewSet[T]()
	result := make([]T, 0)
	for _, v := range slice {
		if !s.Contains(v) {
			result = append(result, v)
			s.Add(v)
		}
	}
	return result
}
