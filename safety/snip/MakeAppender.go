func MakeAppender(suffix []int) func([]int) []int {
	return func(items []int) []int {
		return Append(items, suffix)
	}
}

func TestMakeAppender(t *testing.T) {
	append34 := MakeAppender([]int{3, 4})
	want := []int{1, 2, 3, 4}
	if got := append34([]int{1, 2}); !reflect.DeepEqual(got, want) {
		t.Errorf("append34([]int{1, 2}) = %v; want %v", got, want)
	}
}
