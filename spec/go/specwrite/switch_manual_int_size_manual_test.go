package specwrite

import (
	. "test_formats_write"
	"testing"
)

func TestSwitchManualIntSizeBodySizeMismatch(t *testing.T) {
	chunk := NewSwitchManualIntSize_Chunk()
	chunk.SetCode(0x33)
	chunk.SetSize(3)
	chunk.SetBody([]byte{0x10, 0x20})

	err := chunk.Check()
	consistency := requireConsistencyField(t, err, "body")
	if consistency.Expected() != uint32(3) || consistency.Actual() != 2 {
		t.Fatalf("body size mismatch = (%v, %v); want (3, 2)", consistency.Expected(), consistency.Actual())
	}
}
