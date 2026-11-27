package specwrite

import (
	. "test_formats_write"
	"testing"
)

func TestStrPadTermRoundtripExact(t *testing.T) {
	r := NewStrPadTermRoundtrip()
	r.SetRoot_(r)
	r.SetStrPad("str1")
	r.SetStrTerm("str2foo")
	r.SetStrTermAndPad("str+++3bar+++")
	r.SetStrTermInclude("str4baz@")

	assertEqualToFullFile(t, r, "../../../src/str_pad_term.bin")
}
