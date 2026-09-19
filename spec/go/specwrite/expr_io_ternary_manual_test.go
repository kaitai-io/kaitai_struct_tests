package specwrite

import (
	"bytes"
	"os"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func TestExprIoTernaryBoundary(t *testing.T) {
	source, err := os.ReadFile("../../../src/if_struct.bin")
	if err != nil {
		t.Fatal(err)
	}
	parsed := NewExprIoTernary()
	if err := parsed.Read(kaitai.NewStream(bytes.NewReader(source)), nil, parsed); err != nil {
		t.Fatal(err)
	}
	size1, err := parsed.OneOrTwoIoSize1()
	if err != nil {
		t.Fatal(err)
	}
	size2, err := parsed.OneOrTwoIoSize2()
	if err != nil {
		t.Fatal(err)
	}
	sizeAdd3, err := parsed.OneOrTwoIoSizeAdd3()
	if err != nil {
		t.Fatal(err)
	}
	if size1 != 8 || size2 != 8 || sizeAdd3 != 11 {
		t.Fatalf("original selected sizes = %d, %d, %d; want 8, 8, 11", size1, size2, sizeAdd3)
	}
	parsedOut := kaitai.NewFixedSeekableBuffer(13)
	if err := parsed.Write(kaitai.NewWriter(parsedOut)); err != nil {
		t.Fatal(err)
	}
	wantOriginalSelected := []byte{0x54, 0x42, 0x43, 0x53, 0x03, 0x62, 0x61, 0x72}
	wantCanonicalSelected := []byte{0x54, 0, 0, 0, 0, 0, 0, 0}
	if !bytes.Equal(source[5:], wantOriginalSelected) || !bytes.Equal(parsedOut.Bytes()[5:], wantCanonicalSelected) {
		t.Fatalf("selected stream original=%x rewritten=%x", source[5:], parsedOut.Bytes()[5:])
	}

	fresh := NewExprIoTernary()
	fresh.SetRoot_(fresh)
	fresh.SetFlag(0)
	one := NewExprIoTernary_One()
	one.SetParent_(fresh)
	one.SetRoot_(fresh)
	one.SetOne(0)
	if err := one.Check(); err != nil {
		t.Fatal(err)
	}
	two := NewExprIoTernary_Two()
	two.SetParent_(fresh)
	two.SetRoot_(fresh)
	two.SetTwo(0)
	if err := two.Check(); err != nil {
		t.Fatal(err)
	}
	fresh.SetObj1(one)
	fresh.SetObj2(two)
	if err := fresh.Check(); err != nil {
		t.Fatal(err)
	}
	freshOut := kaitai.NewFixedSeekableBuffer(13)
	if err := fresh.Write(kaitai.NewWriter(freshOut)); err != nil {
		t.Fatal(err)
	}
	wantFresh := make([]byte, 13)
	if !bytes.Equal(freshOut.Bytes(), wantFresh) {
		t.Fatalf("fresh bytes = %x; want %x", freshOut.Bytes(), wantFresh)
	}
	reparsed := NewExprIoTernary()
	if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(freshOut.Bytes())), nil, reparsed); err != nil {
		t.Fatal(err)
	}
	if reparsed.GetFlag() != 0 || reparsed.GetObj1().GetOne() != 0 || reparsed.GetObj2().GetTwo() != 0 {
		t.Fatal("fresh canonical output did not reparse")
	}
}
