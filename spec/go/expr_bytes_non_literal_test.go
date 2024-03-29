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

func TestExprBytesNonLiteral(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/enum_negative.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r ExprBytesNonLiteral
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	tmp1, err := r.CalcBytes()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 2, len(tmp1))
	tmp2, err := r.CalcBytes()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 255, tmp2[0])
	tmp3, err := r.CalcBytes()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 1, tmp3[1])
}
