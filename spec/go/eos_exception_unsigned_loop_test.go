// Autogenerated from KST: please remove this line if doing any edits by hand!

package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
	"io"
)

func TestEosExceptionUnsignedLoop(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/high_bit_u4_loop_count.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r EosExceptionUnsignedLoop
	err = r.Read(s, &r, &r)
	assert.Error(t, err)
	assert.ErrorIs(t, err, io.ErrUnexpectedEOF)
}
