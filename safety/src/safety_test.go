package safety

import (
	"reflect"
	"testing"
)

func TestConcatenate(t *testing.T) {
	want := List{1, 2, 3, 4}
	if got := Concatenate(List{1, 2}, List{3, 4}); !reflect.DeepEqual(got, want) {
		t.Errorf("Concatenate(List{1, 2}, List{3, 4}) = %v; want %v", got, want)
	}
}

func TestConcatenateRepeatedly(t *testing.T) {
	var items List
	for _, suffix := range []List{List{8, 6}, List{7, 5}, List{3, 0, 9}} {
		items = Concatenate(items, suffix)
	}
	want := List{8, 6, 7, 5, 3, 0, 9}
	if !reflect.DeepEqual(items, want) {
		t.Errorf("items = %v; want %v", items, want)
	}
}

func TestConcatenateReuse(t *testing.T) {
	items := List{1}
	want := List{1, 2}
	for i := 0; i < 2; i++ {
		if got := Concatenate(items, List{2}); !reflect.DeepEqual(got, want) {
			t.Errorf("Concatenate(items, List{2}) = %v; want %v", got, want) // OK
		}
	}
}
