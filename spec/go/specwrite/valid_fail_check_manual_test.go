package specwrite

import (
	"bytes"
	"errors"
	"io"
	"os"
	"reflect"
	"strings"
	. "test_formats_write"
	"testing"

	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
)

type validationCheckSubject interface {
	Check() error
}

type validationCheckCase struct {
	name        string
	source      string
	path        string
	field       string
	fresh       func() validationCheckSubject
	retained    func(*kaitai.Stream) (validationCheckSubject, error)
	assertParse func(*testing.T, error)
}

func TestValidFailCheckManual(t *testing.T) {
	tests := []validationCheckCase{
		{
			name: "valid_fail_anyof_int", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailAnyofInt()
				r.SetRoot_(r)
				r.SetFoo(0x50)
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailAnyofInt()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationNotAnyOf,
		},
		{
			name: "valid_fail_contents", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailContents()
				r.SetRoot_(r)
				r.SetFoo([]byte{0x50, 0x41})
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailContents()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationNotEqual,
		},
		{
			name: "valid_fail_eq_bytes", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailEqBytes()
				r.SetRoot_(r)
				r.SetFoo([]byte{0x50, 0x41})
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailEqBytes()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationNotEqual,
		},
		{
			name: "valid_fail_eq_int", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailEqInt()
				r.SetRoot_(r)
				r.SetFoo(0x50)
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailEqInt()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationNotEqual,
		},
		{
			name: "valid_fail_eq_str", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailEqStr()
				r.SetRoot_(r)
				r.SetFoo("PACK")
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailEqStr()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationNotEqual,
		},
		{
			name: "valid_fail_expr", source: "nav_parent_switch.bin", path: "/seq/1", field: "bar",
			fresh: func() validationCheckSubject {
				r := NewValidFailExpr()
				r.SetRoot_(r)
				r.SetFoo(1)
				r.SetBar(-190)
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailExpr()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationExpr,
		},
		{
			name: "valid_fail_inst", source: "fixed_struct.bin", path: "/instances/inst", field: "inst",
			fresh: func() validationCheckSubject {
				r := NewValidFailInst()
				r.SetRoot_(r)
				r.SetA(0x50)
				r.SetInst(0x31)
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailInst()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationNotEqual,
		},
		{
			name: "valid_fail_max_int", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailMaxInt()
				r.SetRoot_(r)
				r.SetFoo(0x50)
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailMaxInt()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationGreaterThan,
		},
		{
			name: "valid_fail_min_int", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailMinInt()
				r.SetRoot_(r)
				r.SetFoo(0x50)
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailMinInt()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationLessThan,
		},
		{
			name: "valid_fail_range_bytes", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailRangeBytes()
				r.SetRoot_(r)
				r.SetFoo([]byte{0x50, 0x41})
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailRangeBytes()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationGreaterThan,
		},
		{
			name: "valid_fail_range_float", source: "floating_points.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailRangeFloat()
				r.SetRoot_(r)
				r.SetFoo(0.5)
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailRangeFloat()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationGreaterThan,
		},
		{
			name: "valid_fail_range_int", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailRangeInt()
				r.SetRoot_(r)
				r.SetFoo(80)
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailRangeInt()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationGreaterThan,
		},
		{
			name: "valid_fail_range_str", source: "fixed_struct.bin", path: "/seq/0", field: "foo",
			fresh: func() validationCheckSubject {
				r := NewValidFailRangeStr()
				r.SetRoot_(r)
				r.SetFoo("PA")
				return r
			},
			retained: func(stream *kaitai.Stream) (validationCheckSubject, error) {
				r := NewValidFailRangeStr()
				return r, r.Read(stream, nil, r)
			},
			assertParse: assertValidationGreaterThan,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			t.Run("fresh", func(t *testing.T) {
				assertValidationCheck(t, tc.fresh().Check(), tc.path, tc.field)
			})

			t.Run("retained_after_failed_read", func(t *testing.T) {
				source, err := os.ReadFile("../../../src/" + tc.source)
				if err != nil {
					t.Fatal(err)
				}
				stream := kaitai.NewStream(bytes.NewReader(source))
				retained, readErr := tc.retained(stream)
				tc.assertParse(t, readErr)

				first := assertValidationCheck(t, retained.Check(), tc.path, tc.field)
				if _, err := stream.Seek(0, io.SeekStart); err != nil {
					t.Fatal(err)
				}
				second := assertValidationCheck(t, retained.Check(), tc.path, tc.field)
				if first.ID() != second.ID() || !reflect.DeepEqual(first.Expected(), second.Expected()) || !reflect.DeepEqual(first.Actual(), second.Actual()) {
					t.Fatalf("Check result changed with prior stream position: first=%v second=%v", first, second)
				}
			})
		})
	}
}

func requireConsistencyField(t *testing.T, err error, field string) kaitai.ConsistencyError {
	t.Helper()
	if err == nil {
		t.Fatalf("expected consistency error for %q", field)
	}
	var consistency kaitai.ConsistencyError
	if !errors.As(err, &consistency) {
		t.Fatalf("error type = %T; want kaitai.ConsistencyError: %v", err, err)
	}
	if consistency.ID() != field {
		t.Fatalf("consistency field = %q; want %q: %v", consistency.ID(), field, err)
	}
	return consistency
}

func assertValidationCheck(t *testing.T, err error, path, field string) kaitai.ConsistencyError {
	t.Helper()
	consistency := requireConsistencyField(t, err, field)
	if !strings.HasPrefix(err.Error(), path+": ") {
		t.Fatalf("Check context = %q; want prefix %q", err, path+": ")
	}
	if strings.Contains(err.Error(), "at pos") {
		t.Fatalf("Check unexpectedly depends on the old input position: %v", err)
	}
	return consistency
}

func assertValidationNotAnyOf(t *testing.T, err error) {
	t.Helper()
	var typed kaitai.ValidationNotAnyOfError
	if !errors.As(err, &typed) {
		t.Fatalf("parse error type = %T; want kaitai.ValidationNotAnyOfError: %v", err, err)
	}
}

func assertValidationNotEqual(t *testing.T, err error) {
	t.Helper()
	var typed kaitai.ValidationNotEqualError
	if !errors.As(err, &typed) {
		t.Fatalf("parse error type = %T; want kaitai.ValidationNotEqualError: %v", err, err)
	}
}

func assertValidationExpr(t *testing.T, err error) {
	t.Helper()
	var typed kaitai.ValidationExprError
	if !errors.As(err, &typed) {
		t.Fatalf("parse error type = %T; want kaitai.ValidationExprError: %v", err, err)
	}
}

func assertValidationGreaterThan(t *testing.T, err error) {
	t.Helper()
	var typed kaitai.ValidationGreaterThanError
	if !errors.As(err, &typed) {
		t.Fatalf("parse error type = %T; want kaitai.ValidationGreaterThanError: %v", err, err)
	}
}

func assertValidationLessThan(t *testing.T, err error) {
	t.Helper()
	var typed kaitai.ValidationLessThanError
	if !errors.As(err, &typed) {
		t.Fatalf("parse error type = %T; want kaitai.ValidationLessThanError: %v", err, err)
	}
}
