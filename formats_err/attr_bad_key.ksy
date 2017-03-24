# /seq/1/blah: unknown key found, expected: consume, contents, doc, eos-error, id, if, include, parent, process, repeat, size, size-eos, terminator, type
meta:
  id: attr_bad_key
  endian: le
seq:
  - id: foo
    type: u2
  - id: bar
    type: kazam
    blah: 1.234
