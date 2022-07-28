package safety

type List []int

func Concatenate(items, suffix List) List {
	return append(items, suffix...)
}
