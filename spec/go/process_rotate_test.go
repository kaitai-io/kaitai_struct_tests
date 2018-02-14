package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestProcessRotate(t *testing.T) {
	f, err := os.Open("../../src/process_rotate.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r ProcessRotate
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, "Hello", r.Buf1)
	assert.EqualValues(t, "World", r.Buf2)
	assert.EqualValues(t, "There", r.Buf3)
}
