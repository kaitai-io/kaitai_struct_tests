# Demonstrates https://github.com/kaitai-io/kaitai_struct/issues/1223 in C++98
meta:
  id: eof_exception_switch_user
  endian: le
seq:
  - id: code
    type: u2
  - id: data
    type:
      switch-on: code
      cases:
        0x1ff: one
        2: two
types:
  one:
    seq:
      # only 0 bytes available, should fail with EOF exception
      - id: val
        type: s2
  two:
    seq:
      - id: val
        type: u2
