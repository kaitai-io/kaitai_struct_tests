package deeply

import (
	"bytes"
	"errors"
)

// CustomFx matches the namespaced Java/Python fixture. Its constructor argument
// is deliberately ignored by those reference implementations.
type CustomFx struct{}

func NewCustomFx(x int) *CustomFx { return &CustomFx{} }

func (fx *CustomFx) Decode(src []byte) ([]byte, error) {
	dst := make([]byte, len(src)+2)
	dst[0], dst[len(dst)-1] = '_', '_'
	copy(dst[1:], src)
	return dst, nil
}

func (fx *CustomFx) Encode(src []byte) ([]byte, error) {
	if len(src) < 2 || src[0] != '_' || src[len(src)-1] != '_' {
		return nil, errors.New("CustomFx can only encode data like '_(.*)_'")
	}
	return bytes.Clone(src[1 : len(src)-1]), nil
}
