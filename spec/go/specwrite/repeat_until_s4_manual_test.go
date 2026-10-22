package specwrite

import (
	. "test_formats_write"
	"testing"
)

func TestRepeatUntilS4Manual(t *testing.T) {
	tests := []struct {
		name    string
		entries []int32
		valid   bool
	}{
		{name: "testCheckBadNoEntries", entries: []int32{}},
		{name: "testCheckGoodOneEntry", entries: []int32{-1}, valid: true},
		{name: "testCheckBadEarlyUntilEntry", entries: []int32{-3, 275000, -1, 0, -1}},
		{name: "testCheckBadNoUntilEntry", entries: []int32{-3, 275000, -2, 0}},
		{name: "testCheckGoodUntilEntry", entries: []int32{-3, 275000, -2, 0, -1}, valid: true},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			r := NewRepeatUntilS4()
			r.SetRoot_(r)
			r.SetEntries(tc.entries)
			if tc.valid {
				r.SetAfterall("")
				if err := r.Check(); err != nil {
					t.Fatal(err)
				}
				return
			}
			requireConsistencyField(t, r.Check(), "entries")
		})
	}
}
