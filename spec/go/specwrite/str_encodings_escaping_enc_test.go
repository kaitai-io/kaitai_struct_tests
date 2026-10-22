package specwrite

import "testing"

func TestStrEncodingsEscapingEnc(t *testing.T) {
	assertInvalidEncodingDiagnostics(t, "str_encodings_escaping_enc", false)
}
