package specwrite

import (
	"bytes"
	"errors"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"io"
	"os"
	. "test_formats_write"
	"testing"
)

func TestDebugArrayUserEofException(t *testing.T) {
	source, err := os.ReadFile("../../../src/nav_parent_codes.bin")
	if err != nil {
		t.Fatal(err)
	}
	r := NewDebugArrayUserEofException()
	err = r.Read(kaitai.NewStream(bytes.NewReader(source)), nil, r)
	if err == io.EOF || !errors.Is(err, io.EOF) {
		t.Fatalf("expected wrapped EOF, got %v", err)
	}
	cats := r.GetArrayOfCats()
	if len(cats) != 3 {
		t.Fatalf("retained %d cats; want 3", len(cats))
	}
	if r.GetOneCat().GetMeow() != 3 || r.GetOneCat().GetChirp() != 73 || cats[0].GetMeow() != 49 || cats[0].GetChirp() != 50 || cats[1].GetMeow() != 51 || cats[1].GetChirp() != 66 || cats[2].GetMeow() != 98 {
		t.Fatal("partial fields differ from KST")
	}
	buf := kaitai.NewSeekableBuffer(nil)
	var unchecked kaitai.ConsistencyNotCheckedError
	if err := r.Write(kaitai.NewWriter(buf)); !errors.As(err, &unchecked) {
		t.Fatalf("incomplete object Write = %v", err)
	}
	if len(buf.Bytes()) != 0 {
		t.Fatalf("incomplete object emitted %x", buf.Bytes())
	}
	// Complete the existing fourth child and validate the graph bottom-up.
	cats[2].SetChirp(0x63)
	for _, cat := range append([]*DebugArrayUserEofException_Cat{r.GetOneCat()}, cats...) {
		if err := cat.Check(); err != nil {
			t.Fatal(err)
		}
	}
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}
	if err := r.Write(kaitai.NewWriter(buf)); err != nil {
		t.Fatal(err)
	}
	want := []byte{3, 73, 49, 50, 51, 66, 98, 99}
	if !bytes.Equal(buf.Bytes(), want) {
		t.Fatalf("bytes = %x; want %x", buf.Bytes(), want)
	}
	reparsed := NewDebugArrayUserEofException()
	if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(buf.Bytes())), nil, reparsed); err != nil {
		t.Fatal(err)
	}
	if len(reparsed.GetArrayOfCats()) != 3 || reparsed.GetArrayOfCats()[2].GetChirp() != 99 {
		t.Fatal("completed graph did not reparse")
	}
}
