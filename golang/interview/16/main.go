package main

import (
	"slices"
	"strings"
	"time"
)

// SlowSort sorts a slice of integers using a very inefficient algorithm (bubble sort)
// TODO: Optimize this function to be more efficient
func SlowSort(data []int) []int {
	// Make a copy to avoid modifying the original
	result := make([]int, len(data))
	copy(result, data)

	slices.Sort(result)

	return result
}

// OptimizedSort is your optimized version of SlowSort
// It should produce identical results but perform better
func OptimizedSort(data []int) []int {
	// TODO: Implement a more efficient sorting algorithm
	// Hint: Consider using sort package or a more efficient algorithm
	return SlowSort(data) // Replace this with your optimized implementation
}

// InefficientStringBuilder builds a string by repeatedly concatenating
// TODO: Optimize this function to be more efficient
func InefficientStringBuilder(parts []string, repeatCount int) string {
	var result strings.Builder

	for _, part := range parts {
		result.WriteString(part)
	}
	result.WriteString(strings.Repeat(result.String(), repeatCount-1))

	return result.String()
}

// OptimizedStringBuilder is your optimized version of InefficientStringBuilder
// It should produce identical results but perform better
func OptimizedStringBuilder(parts []string, repeatCount int) string {
	// TODO: Implement a more efficient string building method
	// Hint: Consider using strings.Builder or bytes.Buffer
	return InefficientStringBuilder(parts, repeatCount) // Replace this with your optimized implementation
}

// ExpensiveCalculation performs a computation with redundant work
// It computes the sum of all fibonacci numbers up to n
// TODO: Optimize this function to be more efficient
func ExpensiveCalculation(n int) int {
	if n <= 0 {
		return 0
	}
	memo := make(map[int]int)

	sum := 0
	for i := 1; i <= n; i++ {
		sum += fibonacci(i, memo)
	}

	return sum
}

// Helper function that computes the fibonacci number at position n
func fibonacci(n int, memo map[int]int) int {
	if n <= 1 {
		return n
	}
	if res, ok := memo[n]; ok {
		return res
	}
	result := fibonacci(n-1, memo) + fibonacci(n-2, memo)
	memo[n] = result
	return result
}

// OptimizedCalculation is your optimized version of ExpensiveCalculation
// It should produce identical results but perform better
func OptimizedCalculation(n int) int {
	// TODO: Implement a more efficient calculation method
	// Hint: Consider memoization or avoiding redundant calculations
	return ExpensiveCalculation(n) // Replace this with your optimized implementation
}

// HighAllocationSearch searches for all occurrences of a substring and creates a map with their positions
// TODO: Optimize this function to reduce allocations
func HighAllocationSearch(text, substr string) map[int]string {
	result := make(map[int]string)
	lowerText := strings.ToLower(text)
	lowerSubstr := strings.ToLower(substr)

	i := 0
	for i < len(text) {
		nextMatchIdx := strings.Index(lowerText[i:], lowerSubstr)
		if nextMatchIdx == -1 {
			break
		}
		i += nextMatchIdx
		result[i] = text[i : i+len(substr)]
		i++
	}

	return result
}

// OptimizedSearch is your optimized version of HighAllocationSearch
// It should produce identical results but perform better with fewer allocations
func OptimizedSearch(text, substr string) map[int]string {
	// TODO: Implement a more efficient search method with fewer allocations
	// Hint: Consider avoiding temporary string allocations and reusing memory
	return HighAllocationSearch(text, substr) // Replace this with your optimized implementation
}

// A function to simulate CPU-intensive work for benchmarking
// You don't need to optimize this; it's just used for testing
func SimulateCPUWork(duration time.Duration) {
	start := time.Now()
	for time.Since(start) < duration {
		// Just waste CPU cycles
		for i := 0; i < 1000000; i++ {
			_ = i
		}
	}
}
