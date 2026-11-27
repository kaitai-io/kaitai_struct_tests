package specwrite

import (
	"errors"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func newProcessRotate(buf2 string) *ProcessRotate {
	r := NewProcessRotate()
	r.SetRoot_(r)
	r.SetBuf1([]byte("Hello"))
	r.SetBuf2([]byte(buf2))
	r.SetKey(1)
	r.SetBuf3([]byte("There"))
	return r
}

func TestProcessRotateManual(t *testing.T) {
	t.Run("fresh_processor_recomputation", func(t *testing.T) {
		assertEqualToFullFile(t, newProcessRotate("World"), "../../../src/process_rotate.bin")
	})

	t.Run("size_mismatch_check", func(t *testing.T) {
		r := newProcessRotate("Way too long")
		var consistency kaitai.ConsistencyError
		if err := r.Check(); !errors.As(err, &consistency) {
			t.Fatalf("Check error = %v; want ConsistencyError", err)
		}
	})

	t.Run("size_mismatch_write", func(t *testing.T) {
		r := newProcessRotate("Way too long")
		var consistency kaitai.ConsistencyError
		if err := r.Check(); !errors.As(err, &consistency) {
			t.Fatalf("Check error = %v; want ConsistencyError", err)
		}
		buf := kaitai.NewFixedSeekableBuffer(16)
		var unchecked kaitai.ConsistencyNotCheckedError
		if err := r.Write(kaitai.NewWriter(buf)); !errors.As(err, &unchecked) {
			t.Fatalf("Write after failed Check = %v; want ConsistencyNotCheckedError", err)
		}
	})
}
