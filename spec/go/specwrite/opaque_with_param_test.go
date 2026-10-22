package specwrite

import (
	"bytes"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func TestOpaqueWithParam(t *testing.T) {
	want := []byte("foo|ba")
	r := NewOpaqueWithParam()
	r.SetRoot_(r)
	one := NewParamsDef(5, true)
	one.SetRoot_(one)
	one.SetParent_(nil)
	one.SetBuf("foo|b")
	one.SetTrailer('a')
	if one.Len != 5 || !one.HasTrailer {
		t.Fatalf("parameters = (%d, %v); want (5, true)", one.Len, one.HasTrailer)
	}
	if err := one.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetOne(one)
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewFixedSeekableBuffer(len(want))
	if err := r.Write(kaitai.NewWriter(buf)); err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(buf.Bytes(), want) {
		t.Fatalf("bytes = %x; want %x", buf.Bytes(), want)
	}
	reparsed := NewOpaqueWithParam()
	if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(buf.Bytes())), nil, reparsed); err != nil {
		t.Fatal(err)
	}
	got := reparsed.GetOne()
	if got.Len != 5 || !got.HasTrailer || got.GetBuf() != "foo|b" || got.GetTrailer() != 'a' || got.Parent_() != nil {
		t.Fatalf("reparsed opaque value = (%d, %v, %q, %q)", got.Len, got.HasTrailer, got.GetBuf(), got.GetTrailer())
	}
}
