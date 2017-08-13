package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestRepeatNStruct(t *testing.T) {
	f, err := os.Open("../../src/repeat_n_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r RepeatNStruct
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, 2, len(r.Chunks))
	assert.EqualValues(t, 0x10, r.Chunks[0].Offset)
	assert.EqualValues(t, 0x2078, r.Chunks[0].Len)
	assert.EqualValues(t, 0x2088, r.Chunks[1].Offset)
	assert.EqualValues(t, 0xf, r.Chunks[1].Len)
}
