module test_formats_write

go 1.24.0

replace github.com/kaitai-io/kaitai_struct_go_runtime => ../../../../../runtime/go

replace nested/deeply => ./nested/deeply

require (
	github.com/kaitai-io/kaitai_struct_go_runtime v0.0.0-00010101000000-000000000000
	golang.org/x/text v0.31.0
	nested/deeply v0.0.0
)
