package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestRepeatEosStruct(t *testing.T) {
	f, err := os.Open("../../src/repeat_eos_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r RepeatEosStruct
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, len(r.Chunks), 2)
	assert.EqualValues(t, r.Chunks[0].Offset, 0)
	assert.EqualValues(t, r.Chunks[0].Len, 0x42)
	assert.EqualValues(t, r.Chunks[1].Offset, 0x42)
	assert.EqualValues(t, r.Chunks[1].Len, 0x815)
}
