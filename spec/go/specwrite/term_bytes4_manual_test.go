package specwrite

import (
	"io"
	. "test_formats_write"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
)

func TestTermBytes4MissingIncludedTerminator(t *testing.T) {
	r := NewTermBytes4()
	r.SetRoot_(r)

	s1 := NewTermBytes4_S1Type()
	s1.SetParent_(r)
	s1.SetRoot_(r)
	s1.SetValue([]byte("foo"))
	if err := s1.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetS1(s1)
	r.SetSkipTerm1('|')

	s2 := NewTermBytes4_S2Type()
	s2.SetParent_(r)
	s2.SetRoot_(r)
	s2.SetValue([]byte("bar"))
	if err := s2.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetS2(s2)
	r.SetSkipTerm2('|')

	s3 := NewTermBytes4_S3Type()
	s3.SetParent_(r)
	s3.SetRoot_(r)
	s3.SetValue([]byte("ba"))
	if err := s3.Check(); err != nil {
		t.Fatal(err)
	}
	r.SetS3(s3)
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}

	buf := kaitai.NewFixedSeekableBuffer(11)
	err := r.Write(kaitai.NewWriter(buf))
	consistency := requireConsistencyField(t, err, "value")
	if consistency.Expected() != true || consistency.Actual() != false {
		t.Fatalf("terminator status = (%v, %v); want (true, false)", consistency.Expected(), consistency.Actual())
	}
	if pos, seekErr := buf.Seek(0, io.SeekCurrent); seekErr != nil || pos != 11 {
		t.Fatalf("writer position = %d, %v; want 11", pos, seekErr)
	}
}
