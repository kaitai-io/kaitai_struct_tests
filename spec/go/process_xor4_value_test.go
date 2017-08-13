package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestProcessXor4Value(t *testing.T) {
	f, err := os.Open("../../src/process_xor_4.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r ProcessXor4Value
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, []byte{0xec, 0xbb, 0xa3, 0x14}, r.Key)
	assert.EqualValues(t, "foo bar", r.Buf)
}
