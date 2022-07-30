// Go Append

func Append(items, suffix []int) []int {
	return append(items, suffix...)
}

func TestAppend(t *testing.T) {
	want := []int{1, 2, 3, 4}
	if got := Append([]int{1, 2}, []int{3, 4}); !reflect.DeepEqual(got, want) {
		t.Errorf("Append([]int{1, 2}, []int{3, 4}) = %v; want %v", got, want)
	}
}
