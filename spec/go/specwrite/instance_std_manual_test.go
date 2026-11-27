package specwrite

import (
	"bytes"
	"errors"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func TestInstanceStdManual(t *testing.T) {
	for _, tc := range []struct {
		name   string
		header string
		dump   bool
	}{
		{name: "short", header: "1234"},
		{name: "long", header: "123456"},
		{name: "empty_via_dump", header: "", dump: true},
	} {
		t.Run(tc.name, func(t *testing.T) {
			r := NewInstanceStd()
			r.SetRoot_(r)
			r.SetHeader(tc.header)
			var err error
			if tc.dump {
				_, err = dumpStruct(r)
			} else {
				err = r.Check()
			}
			var consistency kaitai.ConsistencyError
			if !errors.As(err, &consistency) {
				t.Fatalf("error = %v; want ConsistencyError", err)
			}
		})
	}

	t.Run("write", func(t *testing.T) {
		r := NewInstanceStd()
		r.SetRoot_(r)
		r.SetHeader("Hello")
		if err := r.Check(); err != nil {
			t.Fatal(err)
		}
		buf := kaitai.NewFixedSeekableBuffer(7)
		if err := r.Write(kaitai.NewWriter(buf)); err != nil {
			t.Fatal(err)
		}
		want := []byte{0, 0, 'H', 'e', 'l', 'l', 'o'}
		if !bytes.Equal(buf.Bytes(), want) {
			t.Fatalf("bytes = %x; want %x", buf.Bytes(), want)
		}
		reparsed := NewInstanceStd()
		if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(buf.Bytes())), nil, reparsed); err != nil {
			t.Fatal(err)
		}
		header, err := reparsed.Header()
		if err != nil {
			t.Fatal(err)
		}
		if header != "Hello" {
			t.Fatalf("header = %q; want Hello", header)
		}
	})
}
