module kaitai/specwrite

go 1.24.0

replace (
	github.com/kaitai-io/kaitai_struct_go_runtime => ../../../../runtime/go
	nested/deeply => ../../../compiled/go/src/test_formats_write/nested/deeply
	test_formats_write => ../../../compiled/go/src/test_formats_write
)

require (
	github.com/kaitai-io/kaitai_struct_go_runtime v0.0.0-00010101000000-000000000000
	test_formats_write v0.0.0-00010101000000-000000000000
)

require (
	golang.org/x/text v0.31.0 // indirect
	nested/deeply v0.0.0 // indirect
)
