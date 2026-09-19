package specwrite

import (
	"bytes"
	"errors"
	"strings"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

const bytesPadTermDefault = "12345678901234567890"

type bytesEosPadTermValues struct {
	strPad         []byte
	strTerm        []byte
	strTermAndPad  []byte
	strTermInclude []byte
}

func bytesEosValue(value []byte) []byte {
	if value == nil {
		return []byte(bytesPadTermDefault)
	}
	return value
}

func newBytesEosPadTerm(t *testing.T, values bytesEosPadTermValues) *BytesEosPadTerm {
	t.Helper()
	r := NewBytesEosPadTerm()
	r.SetRoot_(r)

	strPad := NewBytesEosPadTerm_StrPadType()
	strPad.SetParent_(r)
	strPad.SetRoot_(r)
	strPad.SetValue(bytesEosValue(values.strPad))
	if err := strPad.Check(); err != nil {
		t.Fatalf("check str_pad child: %v", err)
	}
	r.SetStrPad(strPad)

	strTerm := NewBytesEosPadTerm_StrTermType()
	strTerm.SetParent_(r)
	strTerm.SetRoot_(r)
	strTerm.SetValue(bytesEosValue(values.strTerm))
	if err := strTerm.Check(); err != nil {
		t.Fatalf("check str_term child: %v", err)
	}
	r.SetStrTerm(strTerm)

	strTermAndPad := NewBytesEosPadTerm_StrTermAndPadType()
	strTermAndPad.SetParent_(r)
	strTermAndPad.SetRoot_(r)
	strTermAndPad.SetValue(bytesEosValue(values.strTermAndPad))
	if err := strTermAndPad.Check(); err != nil {
		t.Fatalf("check str_term_and_pad child: %v", err)
	}
	r.SetStrTermAndPad(strTermAndPad)

	strTermInclude := NewBytesEosPadTerm_StrTermIncludeType()
	strTermInclude.SetParent_(r)
	strTermInclude.SetRoot_(r)
	strTermInclude.SetValue(bytesEosValue(values.strTermInclude))
	if err := strTermInclude.Check(); err != nil {
		t.Fatalf("check str_term_include child: %v", err)
	}
	r.SetStrTermInclude(strTermInclude)

	if err := r.Check(); err != nil {
		t.Fatalf("check root: %v", err)
	}
	return r
}

func TestBytesEosPadTermManual(t *testing.T) {
	digits := bytesPadTermDefault
	tests := []struct {
		name            string
		values          bytesEosPadTermValues
		wantSegments    [4]string
		wantConsistency bool
		wantCapacity    bool
	}{
		{name: "testCheckGoodMaxLengths", wantSegments: [4]string{digits, digits, digits, digits}},
		{
			name: "testCheckGoodMinLengths",
			values: bytesEosPadTermValues{
				strPad: []byte{}, strTerm: []byte{}, strTermAndPad: []byte{}, strTermInclude: []byte("@"),
			},
			wantSegments: [4]string{
				strings.Repeat("@", 20),
				"@" + strings.Repeat("\x00", 19),
				"@" + strings.Repeat("+", 19),
				"@" + strings.Repeat("\x00", 19),
			},
		},
		{name: "testCheckLongerStrPad", values: bytesEosPadTermValues{strPad: []byte("123456789012345678901")}, wantCapacity: true},
		{name: "testCheckGoodLastByteStrPad", values: bytesEosPadTermValues{strPad: []byte("@@@@@@@@@@@@@@@@@@@?")}, wantSegments: [4]string{"@@@@@@@@@@@@@@@@@@@?", digits, digits, digits}},
		{name: "testCheckBadLastByteStrPad", values: bytesEosPadTermValues{strPad: []byte("123456789012345678@")}, wantConsistency: true},
		{name: "testCheckLongerStrTerm", values: bytesEosPadTermValues{strTerm: []byte("123456789012345678901")}, wantCapacity: true},
		{name: "testCheckBadHasTerminator1StrTerm", values: bytesEosPadTermValues{strTerm: []byte("123456789012@4567890")}, wantConsistency: true},
		{name: "testCheckBadHasTerminator2StrTerm", values: bytesEosPadTermValues{strTerm: []byte("1234567890123456789@")}, wantConsistency: true},
		{name: "testCheckLongerStrTermAndPad", values: bytesEosPadTermValues{strTermAndPad: []byte("123456789012345678901")}, wantCapacity: true},
		{name: "testCheckBadHasTerminatorStrTermAndPad", values: bytesEosPadTermValues{strTermAndPad: []byte("@2345678901234567890")}, wantConsistency: true},
		{name: "testCheckGoodLastByte1StrTermAndPad", values: bytesEosPadTermValues{strTermAndPad: []byte("+++++++++++++++++++")}, wantSegments: [4]string{digits, digits, "+++++++++++++++++++@", digits}},
		{name: "testCheckGoodLastByte2StrTermAndPad", values: bytesEosPadTermValues{strTermAndPad: []byte("+++++++++++++++++++0")}, wantSegments: [4]string{digits, digits, "+++++++++++++++++++0", digits}},
		{name: "testCheckBadLastByteStrTermAndPad", values: bytesEosPadTermValues{strTermAndPad: []byte("1234567890123456789+")}, wantConsistency: true},
		{name: "testCheckLongerStrTermInclude", values: bytesEosPadTermValues{strTermInclude: []byte("123456789012345678901")}, wantCapacity: true},
		{name: "testCheckEmptyStrTermInclude", values: bytesEosPadTermValues{strTermInclude: []byte{}}, wantConsistency: true},
		{name: "testCheckBadNoTerminatorStrTermInclude", values: bytesEosPadTermValues{strTermInclude: []byte("123")}, wantConsistency: true},
		{name: "testCheckGoodTerminator1StrTermInclude", values: bytesEosPadTermValues{strTermInclude: []byte("12@")}, wantSegments: [4]string{digits, digits, digits, "12@" + strings.Repeat("\x00", 17)}},
		{name: "testCheckEarlyTerminator1StrTermInclude", values: bytesEosPadTermValues{strTermInclude: []byte("1@@")}, wantConsistency: true},
		{name: "testCheckGoodTerminator2StrTermInclude", values: bytesEosPadTermValues{strTermInclude: []byte("1234567890123456789@")}, wantSegments: [4]string{digits, digits, digits, "1234567890123456789@"}},
		{name: "testCheckEarlyTerminator2StrTermInclude", values: bytesEosPadTermValues{strTermInclude: []byte("123456789012345678@@")}, wantConsistency: true},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			r := newBytesEosPadTerm(t, tc.values)
			buf := kaitai.NewFixedSeekableBuffer(80)
			err := r.Write(kaitai.NewWriter(buf))
			switch {
			case tc.wantCapacity:
				if !errors.Is(err, kaitai.ErrBufferCapacity) {
					t.Fatalf("expected bounded capacity error, got %v", err)
				}
			case tc.wantConsistency:
				var consistencyErr kaitai.ConsistencyError
				if !errors.As(err, &consistencyErr) {
					t.Fatalf("expected consistency error, got %v", err)
				}
				if consistencyErr.ID() != "value" {
					t.Fatalf("consistency field = %q; want value", consistencyErr.ID())
				}
			default:
				if err != nil {
					t.Fatal(err)
				}
				want := []byte(strings.Join(tc.wantSegments[:], ""))
				if !bytes.Equal(buf.Bytes(), want) {
					t.Fatalf("bytes = %x; want %x", buf.Bytes(), want)
				}
			}
		})
	}
}
