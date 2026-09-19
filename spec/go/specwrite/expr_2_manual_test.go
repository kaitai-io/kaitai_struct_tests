package specwrite

import (
	"bytes"
	"os"
	"testing"
	"unicode/utf8"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

func TestExpr2Edit(t *testing.T) {
	source, err := os.ReadFile("../../../src/str_encodings.bin")
	if err != nil {
		t.Fatal(err)
	}
	r := NewExpr2()
	if err := r.Read(kaitai.NewStream(bytes.NewReader(source)), nil, r); err != nil {
		t.Fatal(err)
	}

	str2 := r.GetStr2()
	oldLenMod, err := str2.LenMod()
	if err != nil {
		t.Fatal(err)
	}
	oldAvg, err := str2.GetRest().Avg()
	if err != nil {
		t.Fatal(err)
	}

	const replacement = "Kaitai Struct カ"
	str2.SetStr(replacement)
	str2.InvalidateLenMod()
	byteLen := len([]byte(replacement))
	str2.SetLenOrig(uint16(byteLen + 3))
	lenMod, err := str2.LenMod()
	if err != nil {
		t.Fatal(err)
	}
	if lenMod != byteLen || lenMod == oldLenMod || utf8.RuneCountInString(replacement) == byteLen {
		t.Fatalf("len_mod = %d, old = %d, byte length = %d", lenMod, oldLenMod, byteLen)
	}

	rest := str2.GetRest()
	rest.SetByte0(0xfa)
	rest.InvalidateAvg()
	rest.SetByte1(0x44)
	rest.SetByte2(0x88)
	avg, err := rest.Avg()
	if err != nil {
		t.Fatal(err)
	}
	if avg != 102 || avg == oldAvg {
		t.Fatalf("avg = %d, old = %d; want 102 and changed", avg, oldAvg)
	}

	if err := rest.Check(); err != nil {
		t.Fatal(err)
	}
	if err := str2.Check(); err != nil {
		t.Fatal(err)
	}
	if err := r.GetStr1().GetRest().Check(); err != nil {
		t.Fatal(err)
	}
	if err := r.GetStr1().Check(); err != nil {
		t.Fatal(err)
	}
	if err := r.Check(); err != nil {
		t.Fatal(err)
	}
	str1Len, err := r.GetStr1().LenMod()
	if err != nil {
		t.Fatal(err)
	}
	buf := kaitai.NewFixedSeekableBuffer(2 + str1Len + 3 + 2 + lenMod + 3)
	if err := r.Write(kaitai.NewWriter(buf)); err != nil {
		t.Fatal(err)
	}

	reparsed := NewExpr2()
	if err := reparsed.Read(kaitai.NewStream(bytes.NewReader(buf.Bytes())), nil, reparsed); err != nil {
		t.Fatal(err)
	}
	got := reparsed.GetStr2()
	gotLen, err := got.LenMod()
	if err != nil {
		t.Fatal(err)
	}
	gotAvg, err := got.GetRest().Avg()
	if err != nil {
		t.Fatal(err)
	}
	if got.GetStr() != replacement || got.GetLenOrig() != uint16(byteLen+3) || gotLen != byteLen || gotAvg != 102 {
		t.Fatalf("reparsed str2 = %q len_orig=%d len_mod=%d avg=%d", got.GetStr(), got.GetLenOrig(), gotLen, gotAvg)
	}
	if got.GetRest().GetByte0() != 0xfa || got.GetRest().GetByte1() != 0x44 || got.GetRest().GetByte2() != 0x88 {
		t.Fatalf("reparsed rest = %x %x %x", got.GetRest().GetByte0(), got.GetRest().GetByte1(), got.GetRest().GetByte2())
	}
}
