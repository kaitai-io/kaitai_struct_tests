package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
)

func TestTypeTernaryOpaque(t *testing.T) {
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
	var r TypeTernaryOpaque
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	tmp1, err := r.Dif()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "foo", tmp1.S1)
	tmp2, err := r.Dif()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "bar", tmp2.S2)
	tmp3, err := r.Dif()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "|baz@", tmp3.S3)
}
