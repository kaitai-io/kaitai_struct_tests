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

func TestEnumToI(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/enum_0.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r EnumToI
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, EnumToI_Animal__Cat, r.Pet1)
	assert.EqualValues(t, EnumToI_Animal__Chicken, r.Pet2)
	tmp1, err := r.Pet1I()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 7, tmp1)
	tmp2, err := r.Pet1IToS()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "7", tmp2)
	tmp3, err := r.Pet1Mod()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 32775, tmp3)
	tmp4, err := r.OneLtTwo()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, true, tmp4)
	tmp5, err := r.Pet1EqInt()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, true, tmp5)
	tmp6, err := r.Pet2EqInt()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, false, tmp6)
}
