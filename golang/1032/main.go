package main

func main() {
	obj := Constructor([]string{"ab", "ba", "aaab", "abab", "baa"})
	println(obj.Query('a'))
	println(obj.Query('b'))
	println(obj.Query('c'))
	println(obj.Query('d'))
	println(obj.Query('e'))
	println(obj.Query('f'))
	println(obj.Query('g'))
	println(obj.Query('h'))
	println(obj.Query('i'))
	println(obj.Query('j'))
	println(obj.Query('k'))
	println(obj.Query('l'))
}
