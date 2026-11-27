package specwrite

import "testing"

func TestStrEncodingsEscapingToS(t *testing.T) {
	assertInvalidEncodingDiagnostics(t, "str_encodings_escaping_to_s", true)
}
