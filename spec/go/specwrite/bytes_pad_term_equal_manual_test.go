package specwrite

import (
	"bytes"
	"errors"
	"strings"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats_write"
)

type bytesPadTermEqualValues struct {
	s1 []byte
	s2 []byte
	s3 []byte
	s4 []byte
}

func newBytesPadTermEqual(values bytesPadTermEqualValues) *BytesPadTermEqual {
	r := NewBytesPadTermEqual()
	r.SetRoot_(r)
	r.SetS1(bytesEosValue(values.s1))
	r.SetS2(bytesEosValue(values.s2))
	r.SetS3(bytesEosValue(values.s3))
	r.SetS4(bytesEosValue(values.s4))
	return r
}

func TestBytesPadTermEqualManual(t *testing.T) {
	digits := bytesPadTermDefault
	tests := []struct {
		name         string
		values       bytesPadTermEqualValues
		wantSegments [4]string
		wantErrorID  string
	}{
		{name: "testCheckGoodMaxLengths", wantSegments: [4]string{digits, digits, digits, digits}},
		{
			name: "testCheckGoodMinLengths",
			values: bytesPadTermEqualValues{
				s1: []byte{}, s2: []byte{}, s3: []byte{}, s4: []byte("."),
			},
			wantSegments: [4]string{
				strings.Repeat("@", 20),
				strings.Repeat("+", 20),
				strings.Repeat("+", 20),
				strings.Repeat(".", 20),
			},
		},
		{name: "testCheckLongerS1", values: bytesPadTermEqualValues{s1: []byte("123456789012345678901")}, wantErrorID: "s1"},
		{name: "testCheckBadHasTerminator1S1", values: bytesPadTermEqualValues{s1: []byte("123456789012@4567890")}, wantErrorID: "s1"},
		{name: "testCheckBadHasTerminator2S1", values: bytesPadTermEqualValues{s1: []byte("1234567890123456789@")}, wantErrorID: "s1"},
		{name: "testCheckLongerS2", values: bytesPadTermEqualValues{s2: []byte("123456789012345678901")}, wantErrorID: "s2"},
		{name: "testCheckGoodTerminator1S2", values: bytesPadTermEqualValues{s2: []byte("123456789012345678@")}, wantSegments: [4]string{digits, "123456789012345678@+", digits, digits}},
		{name: "testCheckEarlyTerminator1S2", values: bytesPadTermEqualValues{s2: []byte("1234567890123456@8@")}, wantErrorID: "s2"},
		{name: "testCheckGoodLastByteS2", values: bytesPadTermEqualValues{s2: []byte("+++++++++++++++++++9")}, wantSegments: [4]string{digits, "+++++++++++++++++++9", digits, digits}},
		{name: "testCheckBadLastByteS2", values: bytesPadTermEqualValues{s2: []byte("1234567890123456789+")}, wantErrorID: "s2"},
		{name: "testCheckLongerS3", values: bytesPadTermEqualValues{s3: []byte("123456789012345678901")}, wantErrorID: "s3"},
		{name: "testCheckBadHasTerminator1S3", values: bytesPadTermEqualValues{s3: []byte("1234567890123456789+")}, wantErrorID: "s3"},
		{name: "testCheckBadHasTerminator2S3", values: bytesPadTermEqualValues{s3: []byte("12345678901234567+9")}, wantErrorID: "s3"},
		{name: "testCheckLongerS4", values: bytesPadTermEqualValues{s4: []byte("123456789012345678901")}, wantErrorID: "s4"},
		{name: "testCheckEmptyS4", values: bytesPadTermEqualValues{s4: []byte{}}, wantErrorID: "s4"},
		{name: "testCheckBadNoTerminatorS4", values: bytesPadTermEqualValues{s4: []byte("1234567890123456789")}, wantErrorID: "s4"},
		{name: "testCheckGoodTerminator1S4", values: bytesPadTermEqualValues{s4: []byte("123456789012345678.")}, wantSegments: [4]string{digits, digits, digits, "123456789012345678.."}},
		{name: "testCheckEarlyTerminator1S4", values: bytesPadTermEqualValues{s4: []byte(".23456789012345678.")}, wantErrorID: "s4"},
		{name: "testCheckGoodTerminator2S4", values: bytesPadTermEqualValues{s4: []byte("1234567890123456789.")}, wantSegments: [4]string{digits, digits, digits, "1234567890123456789."}},
		{name: "testCheckEarlyTerminator2S4", values: bytesPadTermEqualValues{s4: []byte("123456789012345678..")}, wantErrorID: "s4"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			r := newBytesPadTermEqual(tc.values)
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
