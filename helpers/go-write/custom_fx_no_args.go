package test_formats_write

import (
	"bytes"
	"errors"
)

// CustomFxNoArgs matches the Java/Python fixture codec, including its unusual
// restriction on encoding rather than decoding.
type CustomFxNoArgs struct{}

func NewCustomFxNoArgs() *CustomFxNoArgs { return &CustomFxNoArgs{} }

func (fx *CustomFxNoArgs) Decode(src []byte) ([]byte, error) {
	dst := make([]byte, len(src)+2)
	dst[0], dst[len(dst)-1] = '_', '_'
	copy(dst[1:], src)
	return dst, nil
}

func (fx *CustomFxNoArgs) Encode(src []byte) ([]byte, error) {
	if len(src) < 2 || src[0] != '_' || src[len(src)-1] != '_' {
		return nil, errors.New("CustomFxNoArgs can only encode data like '_(.*)_'")
	}
	return bytes.Clone(src[1 : len(src)-1]), nil
}
