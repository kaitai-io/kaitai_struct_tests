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

func TestStrEncodingsUtf16(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/str_encodings_utf16.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r StrEncodingsUtf16
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, 12, r.LenBe)
	assert.EqualValues(t, 65279, r.BeBomRemoved.Bom)
	assert.EqualValues(t, "\u3053\u3093\u306b\u3061\u306f", r.BeBomRemoved.Str)
	assert.EqualValues(t, 12, r.LenLe)
	assert.EqualValues(t, 65279, r.LeBomRemoved.Bom)
	assert.EqualValues(t, "\u3053\u3093\u306b\u3061\u306f", r.LeBomRemoved.Str)
}
