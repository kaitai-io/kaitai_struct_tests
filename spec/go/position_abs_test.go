package spec

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"

	. "test_formats"
)

func TestPositionAbs(t *testing.T) {
	f, err := os.Open("../../src/position_abs.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var h PositionAbs
	err = h.Read(s, &h, &h)
	if err != nil {
		t.Fatal(err)
	}

	assert.Equal(t, uint32(0x20), h.IndexOffset)

	index, err := h.Index()
	if err != nil {
		t.Fatal(err)
	}
	assert.Equal(t, "foo", index.Entry)
}
