package safety

func Concatenate(items, suffix []int) []int {
	return append(items, suffix...)
}
