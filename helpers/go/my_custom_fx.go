package test_formats

// MyCustomFx shifts each byte of the input by key.
// key = +pKey if pFlag is true, -pKey otherwise.
type MyCustomFx struct {
	key int
}

func NewMyCustomFx(pKey uint8, pFlag bool, pSomeBytes []uint8) *MyCustomFx {
	k := int(pKey)
	if !pFlag {
		k = -k
	}
	return &MyCustomFx{key: k}
}

func (p *MyCustomFx) Decode(src []byte) ([]byte, error) {
	res := make([]byte, len(src))
	for i, b := range src {
		res[i] = byte(int(b) + p.key)
	}
	return res, nil
}
