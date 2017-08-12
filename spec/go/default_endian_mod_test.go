package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestDefaultEndianMod(t *testing.T) {
	f, err := os.Open("../../src/fixed_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r DefaultEndianMod
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, r.Main.One, 0x4b434150)
	assert.EqualValues(t, r.Main.Nest.Two, -52947)
	assert.EqualValues(t, r.Main.NestBe.Two, 0x5041434b)
}
