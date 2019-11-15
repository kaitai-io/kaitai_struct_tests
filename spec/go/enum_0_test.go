package spec

import (
	"runtime/debug"
    "os"
    "testing"

    "github.com/stretchr/testify/assert"
    "github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"

    . "test_formats"
)

func TestEnum0(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
    f, err := os.Open("../../src/enum_0.bin")
    if err != nil {
        t.Fatal(err)
    }
    s := kaitai.NewStream(f)

    var h Enum0
    err = h.Read(s, &h, &h)
    if err != nil {
        t.Fatal(err)
    }

    assert.Equal(t, Enum0_Animal__Cat, h.Pet1)
    assert.Equal(t, Enum0_Animal__Chicken, h.Pet2)
}
