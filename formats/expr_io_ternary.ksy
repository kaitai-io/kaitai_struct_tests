# Tests usage of ._io on common KaitaiStruct interface
meta:
  id: expr_io_ternary
seq:
  - id: flag
    type: u1
  - id: obj1
    size: 4
    type: one
  - id: obj2
    size: 8
    type: two
types:
  one:
    seq:
      - id: one
        type: u1
  two:
    seq:
      - id: two
        type: u1
instances:
  one_or_two_obj:
    value: |
      flag == 0x40 ? obj1 : obj2
  one_or_two_io:
    value: |
      (flag == 0x40 ? obj1 : obj2)._io
  one_or_two_io_size1:
    value: |
      (flag == 0x40 ? obj1 : obj2)._io.size
  one_or_two_io_size2:
    value: one_or_two_io.size
  one_or_two_io_size_add_3:
    value: |
      (flag == 0x40 ? obj1 : obj2)._io.size + 3
