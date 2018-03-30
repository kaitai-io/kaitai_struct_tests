package spec

import (
    "os"
    "testing"

    "github.com/stretchr/testify/assert"
    "github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"

    . "test_formats"
)

func TestEnum0(t *testing.T) {
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

    assert.Equal(t, Cat, h.Pet1)
    assert.Equal(t, Chicken, h.Pet2)
}
