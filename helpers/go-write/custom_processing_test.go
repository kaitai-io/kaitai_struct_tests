package test_formats_write

import (
	"bytes"
	"testing"
)

func TestCustomFxNoArgs(t *testing.T) {
	fx := NewCustomFxNoArgs()
	for _, tc := range []struct{ raw, decoded []byte }{
		{nil, []byte("__")},
		{[]byte{}, []byte("__")},
		{[]byte("_"), []byte("___")},
		{[]byte{0, 0x80, 0xff}, []byte{'_', 0, 0x80, 0xff, '_'}},
	} {
		rawBefore, decodedBefore := bytes.Clone(tc.raw), bytes.Clone(tc.decoded)
		decoded, err := fx.Decode(tc.raw)
		if err != nil || !bytes.Equal(decoded, tc.decoded) {
			t.Fatalf("Decode(%x) = %x, %v; want %x", tc.raw, decoded, err, tc.decoded)
		}
		encoded, err := fx.Encode(tc.decoded)
		if err != nil || !bytes.Equal(encoded, tc.raw) {
			t.Fatalf("Encode(%x) = %x, %v; want %x", tc.decoded, encoded, err, tc.raw)
		}
		roundTrip, err := fx.Encode(decoded)
		if err != nil || !bytes.Equal(roundTrip, tc.raw) {
			t.Fatalf("round trip = %x, %v; want %x", roundTrip, err, tc.raw)
		}
		decoded[0] ^= 0xff
		if len(encoded) > 0 {
			encoded[0] ^= 0xff
		}
		if !bytes.Equal(tc.raw, rawBefore) || !bytes.Equal(tc.decoded, decodedBefore) {
			t.Fatal("codec mutates or aliases its input")
		}
	}
	for _, invalid := range []string{"", "_", "abc", "_abc", "abc_"} {
		result, err := fx.Encode([]byte(invalid))
		if err == nil || err.Error() != "CustomFxNoArgs can only encode data like '_(.*)_'" || result != nil {
			t.Fatalf("Encode(%q) = %x, %v; want reference validation error", invalid, result, err)
		}
	}
}

func TestMyCustomFx(t *testing.T) {
	for _, tc := range []struct {
		name         string
		fx           *MyCustomFx
		raw, decoded []byte
	}{
		{"positive", NewMyCustomFx(7, true, []byte{32, 48, 64}), []byte{9, 172, 141, 141, 237}, []byte{16, 179, 148, 148, 244}},
		{"negative", NewMyCustomFx(uint8(1), false, []byte{0}), []byte{42, 52, 178, 57, 178}, []byte{41, 51, 177, 56, 177}},
		{"wrap-positive", NewMyCustomFx(int64(257), true, nil), []byte{0, 127, 128, 255}, []byte{1, 128, 129, 0}},
		{"wrap-negative", NewMyCustomFx(-1, true, nil), []byte{0, 127, 128, 255}, []byte{255, 126, 127, 254}},
		{"negative-key-false", NewMyCustomFx(int16(-1), false, nil), []byte{0, 255}, []byte{1, 0}},
		{"zero", NewMyCustomFx(0, false, []byte{255}), []byte{0, 255}, []byte{0, 255}},
		{"empty", NewMyCustomFx(32, false, nil), nil, []byte{}},
	} {
		t.Run(tc.name, func(t *testing.T) {
			rawBefore, decodedBefore := bytes.Clone(tc.raw), bytes.Clone(tc.decoded)
			// Reuse the same object: Encode must not leave its key negated.
			for i := 0; i < 2; i++ {
				decoded, err := tc.fx.Decode(tc.raw)
				if err != nil || !bytes.Equal(decoded, tc.decoded) {
					t.Fatalf("Decode(%x) = %x, %v; want %x", tc.raw, decoded, err, tc.decoded)
				}
				encoded, err := tc.fx.Encode(tc.decoded)
				if err != nil || !bytes.Equal(encoded, tc.raw) {
					t.Fatalf("Encode(%x) = %x, %v; want %x", tc.decoded, encoded, err, tc.raw)
				}
				roundTrip, err := tc.fx.Encode(decoded)
				if err != nil || !bytes.Equal(roundTrip, tc.raw) {
					t.Fatalf("round trip = %x, %v; want %x", roundTrip, err, tc.raw)
				}
				if len(decoded) > 0 {
					decoded[0] ^= 0xff
					encoded[0] ^= 0xff
				}
				if !bytes.Equal(tc.raw, rawBefore) || !bytes.Equal(tc.decoded, decodedBefore) {
					t.Fatal("codec mutates or aliases its input")
				}
			}
		})
	}
}
