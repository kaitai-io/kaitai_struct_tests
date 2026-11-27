package specwrite

import (
	"errors"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func TestEosExceptionBytesWriteOverflow(t *testing.T) {
	r := NewEosExceptionBytes()
	r.SetRoot_(r)
	data := NewEosExceptionBytes_Data()
	data.SetParent_(r)
	data.SetRoot_(r)
	data.SetBuf([]byte{0, 0xff, 0xfe, 0xfd, 0xfc, 0xfb, 0xfa})
	if err := data.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetEnvelope(data)
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewFixedSeekableBuffer(12)
	if err := r.Write(kaitai.NewWriter(buf)); !errors.Is(err, kaitai.ErrBufferCapacity) {
		t.Fatalf("Write error = %v; want ErrBufferCapacity", err)
	}
}

func TestEosExceptionU4WriteOverflow(t *testing.T) {
	r := NewEosExceptionU4()
	r.SetRoot_(r)
	data := NewEosExceptionU4_Data()
	data.SetParent_(r)
	data.SetRoot_(r)
	data.SetPrebuf([]byte{0, 0xff, 0xfe})
	data.SetFailInt(3000500200)
	if err := data.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetEnvelope(data)
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewFixedSeekableBuffer(12)
	if err := r.Write(kaitai.NewWriter(buf)); !errors.Is(err, kaitai.ErrBufferCapacity) {
		t.Fatalf("Write error = %v; want ErrBufferCapacity", err)
	}
}
