package specwrite

import "testing"

// The selected stream includes bytes that the schema does not model. Those
// bytes canonicalize to zero on write, so generic whole-dump equality is not a
// valid oracle for this fixture. Keep the exact source/canonical rewrite
// contract in TestExprIoTernaryBoundary authoritative.
func TestExprIoTernary(t *testing.T) {
	TestExprIoTernaryBoundary(t)
}
