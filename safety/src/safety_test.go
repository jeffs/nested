package safety

import (
	"reflect"
	"testing"
)

func TestAppend(t *testing.T) {
	want := []int{1, 2, 3, 4}
	if got := Append([]int{1, 2}, []int{3, 4}); !reflect.DeepEqual(got, want) {
		t.Errorf("Append([]int{1, 2}, []int{3, 4}) = %v; want %v", got, want)
	}
}

func TestAppendRepeatedly(t *testing.T) {
	var items []int
	for _, suffix := range [][]int{[]int{8, 6}, []int{7, 5}, []int{3, 0, 9}} {
		items = Append(items, suffix)
	}
	want := []int{8, 6, 7, 5, 3, 0, 9}
	if !reflect.DeepEqual(items, want) {
		t.Errorf("items = %v; want %v", items, want)
	}
}

func TestAppendReuse(t *testing.T) {
	items := []int{1}
	want := []int{1, 2}
	for i := 0; i < 2; i++ {
		if got := Append(items, []int{2}); !reflect.DeepEqual(got, want) {
			t.Errorf("Append(items, []int{2}) = %v; want %v", got, want) // OK
		}
	}
}

func TestMakeAppender(t *testing.T) {
	append34 := MakeAppender([]int{3, 4})
	want := []int{1, 2, 3, 4}
	if got := append34([]int{1, 2}); !reflect.DeepEqual(got, want) {
		t.Errorf("append34([]int{1, 2}) = %v; want %v", got, want)
	}
}

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
