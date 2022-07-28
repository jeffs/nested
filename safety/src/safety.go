package safety

type List []int

func Append(items, suffix List) List {
	return append(items, suffix...)
}

func MakeAppender(suffix List) func(List) List {
	return func(items List) List {
		return Append(items, suffix)
	}
}
