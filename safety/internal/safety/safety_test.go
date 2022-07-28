package safety

import (
	"reflect"
	"testing"
)

func TestConcatenate(t *testing.T) {
	got := Concatenate([]int{1, 2}, []int{3, 4})
	want := []int{1, 2, 3, 4}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("Concatenate([]int{1, 2}, []int{3, 4}) = %v; want %v", got, want)
	}
}

func TestConcatenateRepeatedly(t *testing.T) {
	var items []int
	for _, suffix := range [][]int{[]int{8, 6}, []int{7, 5}, []int{3, 0, 9}} {
		items = Concatenate(items, suffix)
	}
	want := []int{8, 6, 7, 5, 3, 0, 9}
	if !reflect.DeepEqual(items, want) {
		t.Errorf("items = %v; want %v", items, want)
	}
}
