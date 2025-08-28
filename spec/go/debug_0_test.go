package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
)

func TestDebug0(t *testing.T) {
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
	var r Debug0
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 80, r.One)
	assert.EqualValues(t, 3, len(r.ArrayOfInts))
	assert.EqualValues(t, 65, r.ArrayOfInts[0])
	assert.EqualValues(t, 67, r.ArrayOfInts[1])
	assert.EqualValues(t, 75, r.ArrayOfInts[2])
	// // FIXME: `_unnamed*` fields do not start with an uppercase letter and thus are not exported
	// assert.EqualValues(t, 45, r._unnamed2)

	// FIXME: also test --read-pos once it is implemented
}
