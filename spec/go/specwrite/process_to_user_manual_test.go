package specwrite

import (
	"bytes"
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func TestProcessToUserManual(t *testing.T) {
	r := NewProcessToUser()
	r.SetRoot_(r)
	child := NewProcessToUser_JustStr()
	child.SetParent_(r)
	child.SetRoot_(r)
	child.SetStr("Hello")
	if err := child.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetBuf1(child)
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}

	source, err := os.ReadFile("../../../src/process_rotate.bin")
	if err != nil {
		t.Fatal(err)
	}
	want := source[:5]
	buf := kaitai.NewFixedSeekableBuffer(5)
	if err := r.Write(kaitai.NewWriter(buf)); err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(buf.Bytes(), want) {
		t.Fatalf("bytes = %x; want %x", buf.Bytes(), want)
	}
	reparsed := NewProcessToUser()
	if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(buf.Bytes())), nil, reparsed); err != nil {
		t.Fatal(err)
	}
	if reparsed.GetBuf1().GetStr() != "Hello" {
		t.Fatalf("reparsed = %q; want Hello", reparsed.GetBuf1().GetStr())
	}
}
