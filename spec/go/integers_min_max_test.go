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

func TestIntegersMinMax(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/integers_min_max.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r IntegersMinMax
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, 0, r.UnsignedMin.U1)
	assert.EqualValues(t, 0, r.UnsignedMin.U2le)
	assert.EqualValues(t, 0, r.UnsignedMin.U4le)
	assert.EqualValues(t, 0, r.UnsignedMin.U8le)
	assert.EqualValues(t, 0, r.UnsignedMin.U2be)
	assert.EqualValues(t, 0, r.UnsignedMin.U4be)
	assert.EqualValues(t, 0, r.UnsignedMin.U8be)
	assert.EqualValues(t, 255, r.UnsignedMax.U1)
	assert.EqualValues(t, 65535, r.UnsignedMax.U2le)
	assert.EqualValues(t, uint32(4294967295), r.UnsignedMax.U4le)
	assert.EqualValues(t, uint64(18446744073709551615), r.UnsignedMax.U8le)
	assert.EqualValues(t, 65535, r.UnsignedMax.U2be)
	assert.EqualValues(t, uint32(4294967295), r.UnsignedMax.U4be)
	assert.EqualValues(t, uint64(18446744073709551615), r.UnsignedMax.U8be)
	assert.EqualValues(t, -128, r.SignedMin.S1)
	assert.EqualValues(t, -32768, r.SignedMin.S2le)
	assert.EqualValues(t, -2147483648, r.SignedMin.S4le)
	assert.EqualValues(t, int64(-9223372036854775808), r.SignedMin.S8le)
	assert.EqualValues(t, -32768, r.SignedMin.S2be)
	assert.EqualValues(t, -2147483648, r.SignedMin.S4be)
	assert.EqualValues(t, int64(-9223372036854775808), r.SignedMin.S8be)
	assert.EqualValues(t, 127, r.SignedMax.S1)
	assert.EqualValues(t, 32767, r.SignedMax.S2le)
	assert.EqualValues(t, 2147483647, r.SignedMax.S4le)
	assert.EqualValues(t, int64(9223372036854775807), r.SignedMax.S8le)
	assert.EqualValues(t, 32767, r.SignedMax.S2be)
	assert.EqualValues(t, 2147483647, r.SignedMax.S4be)
	assert.EqualValues(t, int64(9223372036854775807), r.SignedMax.S8be)
}
