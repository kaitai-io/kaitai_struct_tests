package specwrite

import (
	"bytes"
	"errors"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"strconv"
	. "test_formats_write"
	"testing"
)

func TestExprToITrailing(t *testing.T) {
	check := func(r *ExprToITrailing) {
		t.Helper()
		for _, accessor := range []func() (int, error){r.ToIR10, r.ToIGarbage} {
			_, err := accessor()
			var numError *strconv.NumError
			if !errors.As(err, &numError) || !errors.Is(err, strconv.ErrSyntax) {
				t.Fatalf("expected conversion error, got %v", err)
			}
		}
		if v, err := r.ToIR16(); err != nil || v != 0x9173abc {
			t.Fatalf("hex value = %x, error = %v", v, err)
		}
	}
	r := NewExprToITrailing()
	check(r)
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewSeekableBuffer(nil)
	if err := r.Write(kaitai.NewWriter(buf)); err != nil {
		t.Fatal(err)
	}
	if len(buf.Bytes()) != 0 {
		t.Fatalf("value-only schema wrote %x", buf.Bytes())
	}
	reparsed := NewExprToITrailing()
	if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(buf.Bytes())), nil, reparsed); err != nil {
		t.Fatal(err)
	}
	check(reparsed)
}
