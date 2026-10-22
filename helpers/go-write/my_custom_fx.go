package test_formats_write

// MyCustomFx shifts each byte by the signed key, modulo 256. The third argument
// is intentionally unused, matching the Java/Python fixture constructors.
type MyCustomFx struct{ key byte }

// Generated custom processor arguments retain their Go integer widths (for
// example, a u1 field is uint8 and _io.pos is int64).
func NewMyCustomFx[T ~int | ~int8 | ~int16 | ~int32 | ~int64 | ~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64](key T, flag bool, someBytes []byte) *MyCustomFx {
	k := byte(key)
	if !flag {
		k = -k
	}
	return &MyCustomFx{key: k}
}

func (fx *MyCustomFx) Decode(src []byte) ([]byte, error) {
	dst := make([]byte, len(src))
	for i, b := range src {
		dst[i] = b + fx.key
	}
	return dst, nil
}

func (fx *MyCustomFx) Encode(src []byte) ([]byte, error) {
	dst := make([]byte, len(src))
	for i, b := range src {
		dst[i] = b - fx.key
	}
	return dst, nil
}
