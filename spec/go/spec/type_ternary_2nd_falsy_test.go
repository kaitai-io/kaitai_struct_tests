package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
	"unicode/utf8"
)

func TestTypeTernary2ndFalsy(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/switch_integers.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r TypeTernary2ndFalsy
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	tmp1, err := r.VFalse()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, false, tmp1)
	tmp2, err := r.VIntZero()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 0, tmp2)
	tmp3, err := r.VIntNegZero()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, -0, tmp3)
	tmp4, err := r.VFloatZero()
	if err != nil {
		t.Fatal(err)
	}
	assert.InDelta(t, 0.0, tmp4, 1e-6)
	tmp5, err := r.VFloatNegZero()
	if err != nil {
		t.Fatal(err)
	}
	assert.InDelta(t, -0.0, tmp5, 1e-6)
	tmp6, err := r.VStrWZero()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "0", tmp6)
	tmp7, err := r.VStrWZero()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 1, utf8.RuneCountInString(tmp7))
	assert.EqualValues(t, 7, r.Ut.M)
	tmp8, err := r.VNullUt()
	if err != nil {
		t.Fatal(err)
	}
	assert.Nil(t, tmp8)
	tmp9, err := r.VStrEmpty()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "", tmp9)
	tmp10, err := r.VStrEmpty()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 0, utf8.RuneCountInString(tmp10))
	assert.EqualValues(t, 2, len(r.IntArray))
	tmp11, err := r.VIntArrayEmpty()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 0, len(tmp11))
}
