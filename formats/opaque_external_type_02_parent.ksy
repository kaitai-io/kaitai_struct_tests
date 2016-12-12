# https://github.com/kaitai-io/kaitai_struct_compiler/issues/44
meta:
  id:     opaque_external_type_02_parent
  endian: le

seq:
  - id:   parent
    type: parent

types:
  parent:
    seq:
      - id:   child
        type: opaque_external_type_02_child
