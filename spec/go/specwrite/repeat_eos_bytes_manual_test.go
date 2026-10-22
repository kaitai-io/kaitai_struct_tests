package specwrite

import (
	"io"
	. "test_formats_write"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
)

func TestRepeatEosBytesLongerOutput(t *testing.T) {
	r := NewRepeatEosBytes()
	r.SetRoot_(r)
	r.SetRecords([][]byte{
		{0xe8, 0xba, 0xaa, 0xaa, 0xaa},
		{0xfa, 0x9e, 0xb8, 0xaa, 0xaa},
		{0xaa, 0x55, 0x55, 0x55, 0x55},
	})
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}

	buf := kaitai.NewFixedSeekableBuffer(18)
	err := r.Write(kaitai.NewWriter(buf))
	requireConsistencyField(t, err, "records")
	if pos, seekErr := buf.Seek(0, io.SeekCurrent); seekErr != nil || pos != 15 {
		t.Fatalf("writer position = %d, %v; want 15", pos, seekErr)
	}
}
