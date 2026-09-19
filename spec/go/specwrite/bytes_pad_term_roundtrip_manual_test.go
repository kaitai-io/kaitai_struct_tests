package specwrite

import (
	"testing"

	. "test_formats_write"
)

func TestBytesPadTermRoundtripExact(t *testing.T) {
	r := NewBytesPadTermRoundtrip()
	r.SetRoot_(r)
	r.SetStrPad([]byte("str1"))
	r.SetStrTerm([]byte("str2foo"))
	r.SetStrTermAndPad([]byte("str+++3bar+++"))
	r.SetStrTermInclude([]byte("str4baz@"))

	assertEqualToFullFile(t, r, "../../../src/str_pad_term.bin")
}
