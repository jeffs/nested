package safety

func Append(items, suffix []int) []int {
	return append(items, suffix...)
}

func MakeAppender(suffix []int) func([]int) []int {
	return func(items []int) []int {
		return Append(items, suffix)
	}
}
