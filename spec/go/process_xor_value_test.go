package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestProcessXorValue(t *testing.T) {
	f, err := os.Open("../../src/process_xor_1.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r ProcessXorValue
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, 0xff, r.Key)
	assert.EqualValues(t, "foo bar", r.Buf)
}
