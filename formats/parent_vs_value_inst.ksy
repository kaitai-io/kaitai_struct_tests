# https://github.com/kaitai-io/kaitai_struct_compiler/issues/38#issuecomment-265525999
meta:
  id: parent_vs_value_inst
  endian: le

seq:
  - id: s1
    type: strz
    encoding: UTF-8
    terminator: 0x7C
  - id: child
    type: child

types:
  child:
    instances:
      do_something:
        value: "_parent.s1 == 'foo' ? true : false"
