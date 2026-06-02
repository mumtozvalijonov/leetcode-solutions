package main

import "container/list"

type StreamChecker struct {
	words        []string
	lpsVecs      [][]int
	matchIndices []int
	buffer       *list.List
	bufSize      int
	matchFrom    []*list.Element // from which letter per word should matching start
}

func lps(word string) []int {
	result := make([]int, len(word))
	i, j := 1, 0

	for i < len(word) {
		if word[i] == word[j] {
			j++
			result[i] = j
			i++
		} else if j != 0 {
			j = result[j-1]
		} else {
			result[i] = 0
			i++
		}
	}

	return result
}

func Constructor(words []string) StreamChecker {
	lpsVecs := make([][]int, len(words))
	bufSize := 0

	for i, word := range words {
		if len(word) > bufSize {
			bufSize = len(word)
		}
		lpsVecs[i] = lps(word)
	}

	return StreamChecker{
		words:        words,
		lpsVecs:      lpsVecs,
		matchIndices: make([]int, len(words)),
		buffer:       list.New(),
		bufSize:      bufSize,
		matchFrom:    make([]*list.Element, len(words)),
	}
}

func (this *StreamChecker) Query(letter byte) bool {
	this.buffer.PushBack(letter)
	if this.buffer.Len() > this.bufSize {
		this.buffer.Remove(this.buffer.Front())
	}
	found := false

	for i, word := range this.words {
		j := &this.matchIndices[i]
		if this.matchFrom[i] == nil {
			this.matchFrom[i] = this.buffer.Back()
		}
		for this.matchFrom[i] != nil {
			if word[*j] == this.matchFrom[i].Value.(byte) {
				*j++
				if *j == len(word) {
					found = true
					*j = this.lpsVecs[i][*j-1]
				}
				this.matchFrom[i] = this.matchFrom[i].Next()
			} else if *j != 0 {
				*j = this.lpsVecs[i][*j-1]
			} else {
				this.matchFrom[i] = this.matchFrom[i].Next()
			}
		}
	}

	return found
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * obj := Constructor(words);
 * param_1 := obj.Query(letter);
 */
