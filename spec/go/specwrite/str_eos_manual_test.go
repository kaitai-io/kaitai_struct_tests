package specwrite

import (
	"io"
	. "test_formats_write"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
)

func TestStrEosLongerOutput(t *testing.T) {
	r := NewStrEos()
	r.SetRoot_(r)
	r.SetStr("Hello")
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}

	buf := kaitai.NewFixedSeekableBuffer(7)
	err := r.Write(kaitai.NewWriter(buf))
	requireConsistencyField(t, err, "str")
	if pos, seekErr := buf.Seek(0, io.SeekCurrent); seekErr != nil || pos != 5 {
		t.Fatalf("writer position = %d, %v; want 5", pos, seekErr)
	}
}
