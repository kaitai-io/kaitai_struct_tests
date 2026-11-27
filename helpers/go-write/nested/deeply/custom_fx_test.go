package deeply

import (
	"bytes"
	"testing"
)

func TestCustomFx(t *testing.T) {
	for _, arg := range []int{7, 0, -1} {
		fx := NewCustomFx(arg)
		for _, tc := range []struct{ raw, decoded []byte }{
			{nil, []byte("__")},
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
			if err == nil || err.Error() != "CustomFx can only encode data like '_(.*)_'" || result != nil {
				t.Fatalf("Encode(%q) = %x, %v; want reference validation error", invalid, result, err)
			}
		}
	}
}
