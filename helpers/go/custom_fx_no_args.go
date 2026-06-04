package test_formats

// CustomFxNoArgs wraps decoded bytes with '_' on both sides.
type CustomFxNoArgs struct{}

func NewCustomFxNoArgs() *CustomFxNoArgs {
	return &CustomFxNoArgs{}
}

func (p *CustomFxNoArgs) Decode(src []byte) ([]byte, error) {
	res := make([]byte, len(src)+2)
	res[0] = '_'
	copy(res[1:], src)
	res[len(res)-1] = '_'
	return res, nil
}
