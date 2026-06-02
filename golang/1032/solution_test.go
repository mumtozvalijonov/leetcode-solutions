package main

import (
	"testing"
)

func TestSolution(t *testing.T) {
	var tests = []struct {
		name     string
		words    []string
		stream   []byte
		expected []bool
	}{
		{
			"Case 1",
			[]string{"cd", "f", "kl"},
			[]byte{'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'},
			[]bool{false, false, false, true, false, true, false, false, false, false, false, true},
		},
		{
			"Case 2",
			[]string{"ab", "ba", "aaab", "abab", "baa"},
			[]byte{'a', 'a', 'a', 'a', 'a', 'b', 'a', 'b', 'a', 'b', 'b', 'b', 'a', 'b', 'a', 'b', 'b', 'b', 'b', 'a', 'b', 'a', 'b', 'a', 'a', 'a', 'b', 'a', 'a', 'a'},
			[]bool{false, false, false, false, false, true, true, true, true, true, false, false, true, true, true, true, false, false, false, true, true, true, true, true, true, false, true, true, true, false},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if len(tt.expected) != len(tt.stream) {
				t.Fatalf("len mismatch")
			}
			sc := Constructor(tt.words)
			for i, letter := range tt.stream {
				res := sc.Query(letter)
				exp := tt.expected[i]
				if res != exp {
					t.Errorf("got %+v, want %+v at index %d", res, exp, i)
				}
			}
		})
	}
}
