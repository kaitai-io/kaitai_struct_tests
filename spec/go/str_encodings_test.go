package spec

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"

	. "test_formats"
)

func TestStrEncodings(t *testing.T) {
	f, err := os.Open("../../src/str_encodings.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var h StrEncodings
	err = h.Read(s, &h, &h)
	if err != nil {
		t.Fatal(err)
	}

	assert.Equal(t, "Some ASCII", h.Str1)
	assert.Equal(t, "こんにちは", h.Str2)
	assert.Equal(t, "こんにちは", h.Str3)
	assert.Equal(t, "░▒▓", h.Str4)
}
