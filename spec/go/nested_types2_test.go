package spec

import (
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	"github.com/stretchr/testify/assert"

	. "test_formats"
)

func TestNestedTypes2(t *testing.T) {
	f, err := os.Open("../../src/fixed_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)

	var r NestedTypes2
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, r.One.TypedAtRoot.ValueB, 80)

	assert.EqualValues(t, r.One.TypedHere1.ValueC, 65)

	assert.EqualValues(t, r.One.TypedHere1.TypedHere.ValueD, 67)
	assert.EqualValues(t, r.One.TypedHere1.TypedParent.ValueCc, 75)
	assert.EqualValues(t, r.One.TypedHere1.TypedRoot.ValueB, 45)

	assert.EqualValues(t, r.One.TypedHere2.ValueCc, 49)

	assert.EqualValues(t, r.Two.ValueB, -1)
}
