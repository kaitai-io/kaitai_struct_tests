package specwrite

import (
	. "test_formats_write"
	"testing"
)

func TestRepeatNStrzCountMismatch(t *testing.T) {
	r := NewRepeatNStrz()
	r.SetRoot_(r)
	r.SetQty(7)
	r.SetLines([]string{"foo", "bar"})

	err := r.Check()
	consistency := requireConsistencyField(t, err, "lines")
	if consistency.Expected() != uint32(7) || consistency.Actual() != 2 {
		t.Fatalf("count mismatch = (%v, %v); want (7, 2)", consistency.Expected(), consistency.Actual())
	}
}
