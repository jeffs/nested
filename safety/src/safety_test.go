package safety

import (
	"reflect"
	"testing"
)

func TestAppend(t *testing.T) {
	want := List{1, 2, 3, 4}
	if got := Append(List{1, 2}, List{3, 4}); !reflect.DeepEqual(got, want) {
		t.Errorf("Append(List{1, 2}, List{3, 4}) = %v; want %v", got, want)
	}
}

func TestAppendRepeatedly(t *testing.T) {
	var items List
	for _, suffix := range []List{List{8, 6}, List{7, 5}, List{3, 0, 9}} {
		items = Append(items, suffix)
	}
	want := List{8, 6, 7, 5, 3, 0, 9}
	if !reflect.DeepEqual(items, want) {
		t.Errorf("items = %v; want %v", items, want)
	}
}

func TestAppendReuse(t *testing.T) {
	items := List{1}
	want := List{1, 2}
	for i := 0; i < 2; i++ {
		if got := Append(items, List{2}); !reflect.DeepEqual(got, want) {
			t.Errorf("Append(items, List{2}) = %v; want %v", got, want) // OK
		}
	}
}
