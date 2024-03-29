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

func TestEnumToIInvalid(t *testing.T) {
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
	var r EnumToIInvalid
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, EnumToIInvalid_Animal__Dog, r.Pet1)
	assert.EqualValues(t, 111, r.Pet2)
	tmp1, err := r.Pet2I()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 111, tmp1)
	tmp2, err := r.Pet2IToS()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "111", tmp2)
	tmp3, err := r.Pet2Mod()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 32879, tmp3)
	tmp4, err := r.OneLtTwo()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, true, tmp4)
	tmp5, err := r.Pet2EqIntT()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, true, tmp5)
	tmp6, err := r.Pet2EqIntF()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, false, tmp6)
}
