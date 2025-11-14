module kaitai/spec

go 1.24.0

replace (
	github.com/kaitai-io/kaitai_struct_go_runtime => ../../../runtime/go
	test_formats => ../../compiled/go/src/test_formats
)

require (
	github.com/kaitai-io/kaitai_struct_go_runtime v0.0.0-00010101000000-000000000000
	github.com/stretchr/testify v1.11.1
	test_formats v0.0.0-00010101000000-000000000000
)

require (
	github.com/davecgh/go-spew v1.1.1 // indirect
	github.com/pmezard/go-difflib v1.0.0 // indirect
	golang.org/x/text v0.31.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)
