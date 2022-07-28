package safety

type List []int

func Append(items, suffix List) List {
	return append(items, suffix...)
}
