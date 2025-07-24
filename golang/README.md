# LeetCode Solutions - Go

This directory contains LeetCode problem solutions implemented in Go.

## Structure

Each solution should be organized in its own directory with the following naming convention:
- `{problem_number}_{problem_name}/`
  - `solution.go` - Main solution implementation
  - `solution_test.go` - Unit tests
  - `README.md` - Problem description and approach explanation

## Running Solutions

To run a specific solution:
```bash
cd {problem_number}_{problem_name}
go run solution.go
```

To run tests:
```bash
cd {problem_number}_{problem_name}
go test
```

To run all tests:
```bash
go test ./...
```

## Development

Make sure you have Go installed (version 1.21 or later).

Each solution should include:
1. Clear function implementation
2. Comprehensive test cases
3. Time and space complexity analysis
4. Alternative approaches when applicable