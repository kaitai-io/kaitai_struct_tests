package specwrite

import (
	"bytes"
	"io"
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func TestIoLocalVarManual(t *testing.T) {
	source, err := os.ReadFile("../../../src/full256.bin")
	if err != nil {
		t.Fatal(err)
	}

	parse := func(t *testing.T) *IoLocalVar {
		t.Helper()
		r := NewIoLocalVar()
		if err := r.Read(kaitai.NewStream(bytes.NewReader(source)), r, r); err != nil {
			t.Fatal(err)
		}
		if _, err := r.MessUp(); err != nil {
			t.Fatal(err)
		}
		return r
	}

	writeInto := func(t *testing.T, r *IoLocalVar, initial []byte) []byte {
		t.Helper()
		if err := r.Check(); err != nil {
			t.Fatal(err)
		}
		output := kaitai.NewSeekableBuffer(bytes.Clone(initial))
		if err := r.Write(kaitai.NewWriter(output)); err != nil {
			t.Fatal(err)
		}
		return bytes.Clone(output.Bytes())
	}

	t.Run("parsed empty positioned child preserves the complete file", func(t *testing.T) {
		r := parse(t)
		actual := writeInto(t, r, source)
		if !bytes.Equal(actual, source) {
			t.Fatalf("serialized bytes differ\nexpected: %x\nactual:   %x", source, actual)
		}
	})

	t.Run("parent mutation wins over parsed raw child bytes", func(t *testing.T) {
		r := parse(t)
		r.GetSkip()[8], r.GetSkip()[9] = 0xe8, 0xe9
		expected := bytes.Clone(source)
		expected[8], expected[9] = 0xe8, 0xe9
		actual := writeInto(t, r, source)
		if !bytes.Equal(actual, expected) {
			t.Fatalf("serialized bytes differ\nexpected: %x\nactual:   %x", expected, actual)
		}
	})

	t.Run("fresh empty positioned child preserves parent-owned bytes", func(t *testing.T) {
		r := NewIoLocalVar()
		r.SetRoot_(r)
		r.SetParent_(r)
		r.SetSkip(bytes.Clone(source[:20]))
		r.SetFollowup(source[20])
		child := NewIoLocalVar_Dummy()
		child.SetParent_(r)
		child.SetRoot_(r)
		if err := child.Check(); err != nil {
			t.Fatal(err)
		}
		r.SetMessUp(child)

		expected := make([]byte, len(source))
		copy(expected, source[:21])
		actual := writeInto(t, r, make([]byte, len(source)))
		if !bytes.Equal(actual, expected) {
			t.Fatalf("serialized bytes differ\nexpected: %x\nactual:   %x", expected, actual)
		}
	})

	t.Run("partial positioned child preserves holes in the complete file", func(t *testing.T) {
		output := kaitai.NewSeekableBuffer(bytes.Clone(source))
		writer := kaitai.NewWriter(output)
		if _, err := writer.Seek(8, io.SeekStart); err != nil {
			t.Fatal(err)
		}
		child, err := writer.NewOverlayChild(4)
		if err != nil {
			t.Fatal(err)
		}
		if err := child.WriteU1(0xe8); err != nil {
			t.Fatal(err)
		}
		if _, err := child.Seek(3, io.SeekStart); err != nil {
			t.Fatal(err)
		}
		if err := child.WriteU1(0xeb); err != nil {
			t.Fatal(err)
		}
		if err := child.WriteBack(); err != nil {
			t.Fatal(err)
		}

		expected := bytes.Clone(source)
		expected[8], expected[11] = 0xe8, 0xeb
		if actual := output.Bytes(); !bytes.Equal(actual, expected) {
			t.Fatalf("serialized bytes differ\nexpected: %x\nactual:   %x", expected, actual)
		}
	})
}
