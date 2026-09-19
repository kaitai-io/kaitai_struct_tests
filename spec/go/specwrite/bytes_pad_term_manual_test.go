package specwrite

import (
	"bytes"
	"errors"
	"strings"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

type bytesPadTermValues struct {
	strPad         []byte
	strTerm        []byte
	strTermAndPad  []byte
	strTermInclude []byte
}

func newBytesPadTerm(values bytesPadTermValues) *BytesPadTerm {
	r := NewBytesPadTerm()
	r.SetRoot_(r)
	r.SetStrPad(bytesEosValue(values.strPad))
	r.SetStrTerm(bytesEosValue(values.strTerm))
	r.SetStrTermAndPad(bytesEosValue(values.strTermAndPad))
	r.SetStrTermInclude(bytesEosValue(values.strTermInclude))
	return r
}

func TestBytesPadTermManual(t *testing.T) {
	digits := bytesPadTermDefault
	tests := []struct {
		name         string
		values       bytesPadTermValues
		wantSegments [4]string
		wantErrorID  string
	}{
		{name: "testCheckGoodMaxLengths", wantSegments: [4]string{digits, digits, digits, digits}},
		{
			name: "testCheckGoodMinLengths",
			values: bytesPadTermValues{
				strPad: []byte{}, strTerm: []byte{}, strTermAndPad: []byte{}, strTermInclude: []byte("@"),
			},
			wantSegments: [4]string{
				strings.Repeat("@", 20),
				"@" + strings.Repeat("\x00", 19),
				"@" + strings.Repeat("+", 19),
				"@" + strings.Repeat("\x00", 19),
			},
		},
		{name: "testCheckLongerStrPad", values: bytesPadTermValues{strPad: []byte("123456789012345678901")}, wantErrorID: "str_pad"},
		{name: "testCheckGoodLastByteStrPad", values: bytesPadTermValues{strPad: []byte("@@@@@@@@@@@@@@@@@@@?")}, wantSegments: [4]string{"@@@@@@@@@@@@@@@@@@@?", digits, digits, digits}},
		{name: "testCheckBadLastByteStrPad", values: bytesPadTermValues{strPad: []byte("1234567890123456789@")}, wantErrorID: "str_pad"},
		{name: "testCheckLongerStrTerm", values: bytesPadTermValues{strTerm: []byte("123456789012345678901")}, wantErrorID: "str_term"},
		{name: "testCheckBadHasTerminator1StrTerm", values: bytesPadTermValues{strTerm: []byte("123456789012@4567890")}, wantErrorID: "str_term"},
		{name: "testCheckBadHasTerminator2StrTerm", values: bytesPadTermValues{strTerm: []byte("1234567890123456789@")}, wantErrorID: "str_term"},
		{name: "testCheckLongerStrTermAndPad", values: bytesPadTermValues{strTermAndPad: []byte("123456789012345678901")}, wantErrorID: "str_term_and_pad"},
		{name: "testCheckBadHasTerminatorStrTermAndPad", values: bytesPadTermValues{strTermAndPad: []byte("@2345678901234567890")}, wantErrorID: "str_term_and_pad"},
		{name: "testCheckGoodLastByte1StrTermAndPad", values: bytesPadTermValues{strTermAndPad: []byte("+++++++++++++++++++")}, wantSegments: [4]string{digits, digits, "+++++++++++++++++++@", digits}},
		{name: "testCheckGoodLastByte2StrTermAndPad", values: bytesPadTermValues{strTermAndPad: []byte("+++++++++++++++++++0")}, wantSegments: [4]string{digits, digits, "+++++++++++++++++++0", digits}},
		{name: "testCheckBadLastByteStrTermAndPad", values: bytesPadTermValues{strTermAndPad: []byte("1234567890123456789+")}, wantErrorID: "str_term_and_pad"},
		{name: "testCheckLongerStrTermInclude", values: bytesPadTermValues{strTermInclude: []byte("123456789012345678901")}, wantErrorID: "str_term_include"},
		{name: "testCheckEmptyStrTermInclude", values: bytesPadTermValues{strTermInclude: []byte{}}, wantErrorID: "str_term_include"},
		{name: "testCheckBadNoTerminatorStrTermInclude", values: bytesPadTermValues{strTermInclude: []byte("123")}, wantErrorID: "str_term_include"},
		{name: "testCheckGoodTerminator1StrTermInclude", values: bytesPadTermValues{strTermInclude: []byte("12@")}, wantSegments: [4]string{digits, digits, digits, "12@" + strings.Repeat("\x00", 17)}},
		{name: "testCheckEarlyTerminator1StrTermInclude", values: bytesPadTermValues{strTermInclude: []byte("1@@")}, wantErrorID: "str_term_include"},
		{name: "testCheckGoodTerminator2StrTermInclude", values: bytesPadTermValues{strTermInclude: []byte("1234567890123456789@")}, wantSegments: [4]string{digits, digits, digits, "1234567890123456789@"}},
		{name: "testCheckEarlyTerminator2StrTermInclude", values: bytesPadTermValues{strTermInclude: []byte("123456789012345678@@")}, wantErrorID: "str_term_include"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			r := newBytesPadTerm(tc.values)
			err := r.Check()
			if tc.wantErrorID != "" {
				var consistencyErr kaitai.ConsistencyError
				if !errors.As(err, &consistencyErr) {
					t.Fatalf("expected consistency error, got %v", err)
				}
				if consistencyErr.ID() != tc.wantErrorID {
					t.Fatalf("consistency field = %q; want %q", consistencyErr.ID(), tc.wantErrorID)
				}
				return
			}
			if err != nil {
				t.Fatal(err)
			}

			buf := kaitai.NewFixedSeekableBuffer(80)
			if err := r.Write(kaitai.NewWriter(buf)); err != nil {
				t.Fatal(err)
			}
			want := []byte(strings.Join(tc.wantSegments[:], ""))
			if !bytes.Equal(buf.Bytes(), want) {
				t.Fatalf("bytes = %x; want %x", buf.Bytes(), want)
			}
		})
	}
}
