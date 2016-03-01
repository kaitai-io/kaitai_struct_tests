meta:
  id: expr_0
  endian: le
seq:
  - id: len_of_1
    type: u2
  - id: str1
    type: str
    byte_size: len_of_1
    encoding: ASCII
instances:
  must_be_f7:
    value: 7 + 0xf0
  must_be_abc123:
    value: '"abc" + "123"'
