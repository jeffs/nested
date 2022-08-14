func TestMakeAppenderDangle(t *testing.T) {
	append34 := func() func([]int) []int {
		suffix := []int{3, 4}
		return MakeAppender(suffix)
	}()
	want := []int{1, 2, 3, 4}
	if got := append34([]int{1, 2}); !reflect.DeepEqual(got, want) {
		t.Errorf("append34([]int{1, 2}) = %v; want %v", got, want)
	}
}
