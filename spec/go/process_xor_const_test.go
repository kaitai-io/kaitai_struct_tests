package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestProcessXorConst(t *testing.T) {
	f, err := os.Open("../../src/process_xor_1.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r ProcessXorConst
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, r.Key, 0xff)
	assert.EqualValues(t, r.Buf, "foo bar")
}
