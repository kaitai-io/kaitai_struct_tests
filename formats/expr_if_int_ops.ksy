# Crafted with Java in mind, where you need to take extra care with type
# conversions to make the unboxing + downcasting required here work
meta:
  id: expr_if_int_ops
  endian: le
seq:
  - id: key
    type: u8
    if: true
  - id: skip
    size: 8
  - id: bytes
    size: 8
    process: xor(key)
  - id: items
    type: s1
    repeat: expr
    repeat-expr: 4
instances:
  bytes_sub_key:
    value: bytes[key]
  items_sub_key:
    value: items[key]
