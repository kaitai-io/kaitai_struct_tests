// Autogenerated from KST: please remove this line if doing any edits by hand!

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
	assert.EqualValues(t, 80, r.OneCat.Meow)
	assert.EqualValues(t, 3, len(r.ArrayOfCats))
	assert.EqualValues(t, 65, r.ArrayOfCats[0].Meow)
	assert.EqualValues(t, 67, r.ArrayOfCats[1].Meow)
	assert.EqualValues(t, 75, r.ArrayOfCats[2].Meow)
}
