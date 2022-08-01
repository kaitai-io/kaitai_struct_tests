package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
)

func TestStrLiterals(t *testing.T) {
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
	var r StrLiterals
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	tmp1, err := r.ComplexStr()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, []rune{0, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 9787}, []rune(tmp1))
	tmp2, err := r.DoubleQuotes()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, []rune{34, 34, 34}, []rune(tmp2))
	tmp3, err := r.Backslashes()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, []rune{92, 92, 92}, []rune(tmp3))
	tmp4, err := r.OctalEatup()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, []rune{0, 50, 50}, []rune(tmp4))
	tmp5, err := r.OctalEatup2()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, []rune{2, 50}, []rune(tmp5))
}
