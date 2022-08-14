func TestMakeAppenderMutate(t *testing.T) {
	t.Skip()
	suffix := []int{3, 4}
	append34 := MakeAppender(suffix)
	want := []int{1, 2, 3, 4}
	check := func() {
		if got := append34([]int{1, 2}); !reflect.DeepEqual(got, want) {
			t.Errorf("append34([]int{1, 2}) = %v; want %v", got, want)
		}
	}
	check()       // OK
	suffix[0] = 5 // Backdoor mutation of shared state
	check()       // FAIL
}
