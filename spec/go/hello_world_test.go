package spec

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"

	. "test_formats"
)

func TestHelloWorld(t *testing.T) {
	f, err := os.Open("../../src/fixed_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var h HelloWorld
	err = h.Read(s, &h, &h)
	if err != nil {
		t.Fatal(err)
	}

	assert.Equal(t, byte(0x50), h.One)
}
