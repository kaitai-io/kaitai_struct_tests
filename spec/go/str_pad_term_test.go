package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestStrPadTerm(t *testing.T) {
	f, err := os.Open("../../src/str_pad_term.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r StrPadTerm
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, r.StrPad, "str1")
	assert.EqualValues(t, r.StrTerm, "str2foo")
	assert.EqualValues(t, r.StrTermAndPad, "str+++3bar+++")
	assert.EqualValues(t, r.StrTermInclude, "str4baz@")
}
