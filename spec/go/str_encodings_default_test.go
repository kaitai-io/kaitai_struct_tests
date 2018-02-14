package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestStrEncodingsDefault(t *testing.T) {
	f, err := os.Open("../../src/str_encodings.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r StrEncodingsDefault
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, "Some ASCII", r.Str1)
	assert.EqualValues(t, "こんにちは", r.Rest.Str2)
	assert.EqualValues(t, "こんにちは", r.Rest.Str3)
	assert.EqualValues(t, "░▒▓", r.Rest.Str4)
}
