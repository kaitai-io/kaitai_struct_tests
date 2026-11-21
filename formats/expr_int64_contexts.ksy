# 64-bit integers in `pos`, `size`, `repeat-expr`, array subscripts,
# parameters, string methods and arithmetic operations with floats. In
# JavaScript, KSC uses `bigint` for them, and `bigint` can't be mixed with
# `number` in some of these places (see
# https://github.com/kaitai-io/kaitai_struct/issues/183 and
# https://github.com/kaitai-io/kaitai_struct/issues/1333).
meta:
  id: expr_int64_contexts
  endian: le
  encoding: ASCII
seq:
  - id: ofs_end
    type: u8
  - id: ofs_start
    type: u8
  - id: len_str
    type: u8
  - id: num_items
    type: u8
  - id: idx
    type: u8
  - id: items
    type: u1
    repeat: expr
    repeat-expr: num_items
  - id: str_val
    type: str
    size: len_str
  - id: child
    # A `u1` value is passed as a `u8` parameter
    type: with_u8_param(items[0])
instances:
  byte_at_ofs:
    # The offsets don't fit into 32 bits, but their difference does
    pos: ofs_end - ofs_start + 16
    type: u1
  item_at_idx:
    value: items[idx]
  substr:
    value: str_val.substring(idx, idx + 2)
  mul_float:
    value: len_str * 1.5
  mul_float_cast:
    # The cast makes the left-hand operand a float
    value: len_str.as<f8> * 0.5
  mul_float_ternary:
    # One branch is a float, so the ternary is a float too
    value: '(idx == 1 ? len_str : 1.5) * 2'
types:
  with_u8_param:
    params:
      - id: val
        type: u8
    instances:
      val_plus_one:
        value: val + 1
