package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
)

func TestDebugArrayUser(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/fixed_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r DebugArrayUser
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, 0x50, r.OneCat.Meow)
	assert.EqualValues(t, 0x41, r.ArrayOfCats[0].Meow)
	assert.EqualValues(t, 0x43, r.ArrayOfCats[1].Meow)
	assert.EqualValues(t, 0x4b, r.ArrayOfCats[2].Meow)
}
