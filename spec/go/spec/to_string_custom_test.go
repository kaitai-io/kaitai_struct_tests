package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
	"fmt"
)

func TestToStringCustom(t *testing.T) {
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
	var r ToStringCustom
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	// Implicitly calls String() method of the object
	repr := fmt.Sprint(r)

	assert.EqualValues(t, "s1 = foo, s2 = bar", repr)
}
