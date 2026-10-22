package specwrite

import (
	"bytes"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"os"
	. "test_formats_write"
	"testing"
)

// The lazy header has the same type at the same absolute position. Only one
// level is requested by the KST; recursively dumping all instances is unbounded.
func TestCastToTop(t *testing.T) {
	source, err := os.ReadFile("../../../src/fixed_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	parsed := NewCastToTop()
	if err := parsed.Read(kaitai.NewStream(bytes.NewReader(source)), nil, parsed); err != nil {
		t.Fatal(err)
	}
	if parsed.GetCode() != 0x50 {
		t.Fatal(parsed.GetCode())
	}
	parsedHeader, err := parsed.Header()
	if err != nil {
		t.Fatal(err)
	}
	castHeader, err := parsed.HeaderCasted()
	if err != nil {
		t.Fatal(err)
	}
	if parsedHeader.GetCode() != 0x41 || castHeader.GetCode() != 0x41 {
		t.Fatal("bounded KST header assertions failed")
	}

	r := NewCastToTop()
	r.SetRoot_(r)
	r.SetCode(0x77)
	header := NewCastToTop()
	header.SetRoot_(r)
	header.SetParent_(r)
	header.SetCode(0xa8)
	header.SetHeaderEnabled_(false)
	if err := header.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetHeader(header)
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewFixedSeekableBuffer(2)
	if err := r.Write(kaitai.NewWriter(buf)); err != nil {
		t.Fatal(err)
	}
	if want := []byte{0x77, 0xa8}; !bytes.Equal(buf.Bytes(), want) {
		t.Fatalf("bytes = %x; want %x", buf.Bytes(), want)
	}
	reparsed := NewCastToTop()
	if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(buf.Bytes())), nil, reparsed); err != nil {
		t.Fatal(err)
	}
	if reparsed.GetCode() != 0x77 {
		t.Fatal(reparsed.GetCode())
	}
	h, err := reparsed.HeaderCasted()
	if err != nil {
		t.Fatal(err)
	}
	if h.GetCode() != 0xa8 {
		t.Fatal(h.GetCode())
	}
}
