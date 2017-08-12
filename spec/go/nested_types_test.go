package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestNestedTypes(t *testing.T) {
	f, err := os.Open("../../src/fixed_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r NestedTypes
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, r.One.TypedAtRoot.ValueB, 80)
	assert.EqualValues(t, r.One.TypedHere.ValueC, 65)
	assert.EqualValues(t, r.Two.ValueB, 67)
}
