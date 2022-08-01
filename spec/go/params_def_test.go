package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
)

func TestParamsDef(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/term_strz.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r = NewParamsDef(5, true)
	err = r.Read(s, r, r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, "foo|b", r.Buf)
	assert.EqualValues(t, 97, r.Trailer)
}
