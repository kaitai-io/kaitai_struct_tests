package test_formats

// Nested_Deeply_CustomFx wraps decoded bytes with '_' on both sides.
type Nested_Deeply_CustomFx struct{}

func NewNested_Deeply_CustomFx(pKey uint8) *Nested_Deeply_CustomFx {
	return &Nested_Deeply_CustomFx{}
}

func (p *Nested_Deeply_CustomFx) Decode(src []byte) ([]byte, error) {
	res := make([]byte, len(src)+2)
	res[0] = '_'
	copy(res[1:], src)
	res[len(res)-1] = '_'
	return res, nil
}
