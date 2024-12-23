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

func TestDebugArrayUserEofException(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/nav_parent_codes.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r DebugArrayUserEofException
	err = r.Read(s, &r, &r)
	assert.Error(t, err)
	assert.ErrorIs(t, err, io.EOF)

	assert.EqualValues(t, 3, r.OneCat.Meow)
	assert.EqualValues(t, 73, r.OneCat.Chirp)
	assert.EqualValues(t, 3, len(r.ArrayOfCats))
	assert.EqualValues(t, 49, r.ArrayOfCats[0].Meow)
	assert.EqualValues(t, 50, r.ArrayOfCats[0].Chirp)
	assert.EqualValues(t, 51, r.ArrayOfCats[1].Meow)
	assert.EqualValues(t, 66, r.ArrayOfCats[1].Chirp)
	assert.EqualValues(t, 98, r.ArrayOfCats[2].Meow)
}
