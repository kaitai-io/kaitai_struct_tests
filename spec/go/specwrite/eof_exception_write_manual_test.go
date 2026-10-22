package specwrite

import (
	"errors"
	"io"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func assertFixedWriteCapacity(t *testing.T, size int, wantPos int64, obj writeStruct) {
	t.Helper()
	if err := obj.Check(); err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewFixedSeekableBuffer(size)
	err := obj.Write(kaitai.NewWriter(buf))
	if !errors.Is(err, kaitai.ErrBufferCapacity) {
		t.Fatalf("Write error = %v; want ErrBufferCapacity", err)
	}
	pos, posErr := buf.Seek(0, io.SeekCurrent)
	if posErr != nil {
		t.Fatal(posErr)
	}
	if pos != wantPos {
		t.Fatalf("writer position = %d; want %d", pos, wantPos)
	}
}

func TestEofExceptionBitsBeWriteCapacity(t *testing.T) {
	r := NewEofExceptionBitsBe()
	r.SetRoot_(r)
	r.SetPreBits(0)
	r.SetFailBits(0b1_01000010_11111111_0)
	assertFixedWriteCapacity(t, 3, 1, r)
}

func TestEofExceptionBitsBe2WriteCapacity(t *testing.T) {
	r := NewEofExceptionBitsBe2()
	r.SetRoot_(r)
	r.SetPreBits(1)
	r.SetFailBits(0b01000010_11111111_0)
	assertFixedWriteCapacity(t, 3, 1, r)
}

func TestEofExceptionBitsLeWriteCapacity(t *testing.T) {
	r := NewEofExceptionBitsLe()
	r.SetRoot_(r)
	r.SetPreBits(1)
	r.SetFailBits(0b0_11111111_01000010_0)
	assertFixedWriteCapacity(t, 3, 1, r)
}

func TestEofExceptionBitsLe2WriteCapacity(t *testing.T) {
	r := NewEofExceptionBitsLe2()
	r.SetRoot_(r)
	r.SetPreBits(1)
	r.SetFailBits(0b0_11111111_01000010)
	assertFixedWriteCapacity(t, 3, 1, r)
}

func TestEofExceptionBytesWriteCapacity(t *testing.T) {
	r := NewEofExceptionBytes()
	r.SetRoot_(r)
	r.SetBuf([]byte{0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0xff, 0xfe, 0xfd, 0xfc, 0xfb})
	assertFixedWriteCapacity(t, 12, 0, r)
}

func TestEofExceptionU4WriteCapacity(t *testing.T) {
	r := NewEofExceptionU4()
	r.SetRoot_(r)
	r.SetPrebuf([]byte{0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80})
	r.SetFailInt(3000500200)
	assertFixedWriteCapacity(t, 12, 9, r)
}
