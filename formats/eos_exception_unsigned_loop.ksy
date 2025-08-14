meta:
  id: eos_exception_unsigned_loop
  endian: le

doc: |
  This spec if for a test of when an unsigned integer is read as a loop
  variable, it is not cast to a negative, but rather correctly interpreted as a
  (large) positive number (which leads to an EOS exception if the file is not
  that long).

  See: https://github.com/kaitai-io/kaitai_struct/issues/1220

seq:
  - id: num_elems
    type: u4
  - id: elems
    type: u1
    repeat: expr
    repeat-expr: num_elems
  - id: next
    type: u1
    doc: |
      If the loop is (wrongly) skipped, this will be the next field,
      and contain what should have been the first loop byte.