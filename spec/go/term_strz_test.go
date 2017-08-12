package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestTermStrz(t *testing.T) {
	f, err := os.Open("../../src/term_strz.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r TermStrz
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, r.S1, "foo")
	assert.EqualValues(t, r.S2, "bar")
	assert.EqualValues(t, r.S3, "|baz@")
}
