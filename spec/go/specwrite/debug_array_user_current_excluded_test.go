package specwrite

import (
	"bytes"
	"errors"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
	"testing"
)

// Negative semantic coverage, not a successful serialization or expected skip.
// During Read the collection grows; Check sees its final length of three.
func TestDebugArrayUserCurrentExcluded(t *testing.T) {
	r := NewDebugArrayUserCurrentExcluded()
	if err := r.Read(kaitai.NewStream(bytes.NewReader([]byte("foo|ba"))), nil, r); err != nil {
		t.Fatal(err)
	}
	cats := r.GetArrayOfCats()
	if len(cats) != 3 {
		t.Fatal(len(cats))
	}
	for i, want := range [][]byte{[]byte("foo"), []byte("|b"), []byte("a")} {
		if !bytes.Equal(cats[i].GetMeow(), want) {
			t.Fatalf("cat %d = %x; want %x", i, cats[i].GetMeow(), want)
		}
		var mismatch kaitai.ConsistencyError
		if err := cats[i].Check(); !errors.As(err, &mismatch) || mismatch.ID() != "meow" || mismatch.Expected() != 0 || mismatch.Actual() != len(want) {
			t.Fatalf("cat %d: expected prefix-sensitive size rejection, got %v", i, err)
		}
	}
}
