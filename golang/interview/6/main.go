// Package challenge6 contains the solution for Challenge 6.
package challenge6

import (
	"regexp"
	"strings"
)

var apostropheRegex = regexp.MustCompile(`['’]`)
var nonAlphanumericRegex = regexp.MustCompile(`[^a-zA-Z0-9]+`)

func CountWordFrequency(s string) map[string]int {
	s = strings.ToLower(s)

	// let's -> lets
	s = apostropheRegex.ReplaceAllString(s, "")

	// spaces, tabs, newlines, punctuation -> word separators
	s = nonAlphanumericRegex.ReplaceAllString(s, " ")

	counts := make(map[string]int)

	for word := range strings.FieldsSeq(s) {
		counts[word]++
	}

	return counts
}
