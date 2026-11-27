package specwrite

import (
	. "test_formats_write"
	"testing"
	"unicode/utf8"
)

func TestStrEncodingsEncodedLengthMismatch(t *testing.T) {
	const japanese = "こんにちは"
	if len([]byte(japanese)) != 15 || utf8.RuneCountInString(japanese) != 5 {
		t.Fatal("test fixture must distinguish encoded byte length from rune count")
	}

	r := NewStrEncodings()
	r.SetRoot_(r)
	r.SetStr1("Some ASCII")
	r.SetStr2(japanese)
	r.SetStr3(japanese)
	r.SetStr4("░▒▓")
	r.SetLenOf1(10)
	r.SetLenOf2(12)
	r.SetLenOf3(10)
	r.SetLenOf4(3)

	err := r.Check()
	consistency := requireConsistencyField(t, err, "str2")
	if consistency.Expected() != uint16(12) || consistency.Actual() != 15 {
		t.Fatalf("encoded length mismatch = (%v, %v); want (12, 15)", consistency.Expected(), consistency.Actual())
	}
}
