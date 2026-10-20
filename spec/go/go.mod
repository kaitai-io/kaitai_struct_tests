module kaitai/spec

go 1.26.0

replace (
	github.com/kaitai-io/kaitai_struct_go_runtime => ../../../runtime/go
	test_formats => ../../compiled/go/src/test_formats
)

require (
	github.com/kaitai-io/kaitai_struct_go_runtime v0.0.0-00010101000000-000000000000
	github.com/stretchr/testify v1.12.1
	test_formats v0.0.0-00010101000000-000000000000
)

require (
	go.yaml.in/yaml/v3 v3.0.5 // indirect
	golang.org/x/text v0.43.0 // indirect
)
