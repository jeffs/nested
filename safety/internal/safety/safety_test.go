package safety

import (
	"reflect"
	"testing"
)

func TestConcatenate(t *testing.T) {
	want := []int{1, 2, 3, 4}
	got := Concatenate([]int{1, 2}, []int{3, 4})
	if !reflect.DeepEqual(got, want) {
		t.Errorf("Concatenate([]int{1, 2}, []int{3, 4}) = %v; want %v", got, want)
	}
}
