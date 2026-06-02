package main

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

func strStr(haystack string, needle string) int {
	i, j := 0, 0
	lpsVec := lps(needle)

	for i < len(haystack) {
		if haystack[i] != needle[j] {
			if j != 0 {
				j = lpsVec[j-1]
			} else {
				i++
			}
		} else {
			i++
			j++
			if j == len(needle) {
				return i - len(needle)
			}
		}
	}
	return -1
}
