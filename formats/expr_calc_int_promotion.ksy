# Built-in integer properties and variables such as `_io.size` and `_index`
# have the type `CalcIntType` in the compiler, because they don't have a
# specific integer type yet. To avoid truncation, arithmetic operations that
# combine them with an integer wider than 32 bits must have a 64-bit result
# type. If the other operand has at most 32 bits, the result type is
# `CalcIntType` (at least a signed 32-bit integer in all targets).
meta:
  id: expr_calc_int_promotion
  endian: le
seq:
  - id: u4_val
    type: u4
  - id: u8_val
    type: u8
  - id: s8_val
    type: s8
  - id: items
    type: str
    # Unlike `s4`, `CalcIntType` can be combined with `u4`
    size: _index + u4_val
    encoding: ASCII
    repeat: expr
    repeat-expr: 2
instances:
  io_size_minus_u4:
    # Unlike `s4`, `CalcIntType` can be combined with `u4`, and the result
    # type is `CalcIntType`. The result is small enough for a signed 32-bit
    # integer.
    value: _io.size - u4_val
  u8_plus_io_size:
    # Because of `u8_val`, the result type is `u8`. The result doesn't fit
    # into 32 bits.
    value: u8_val + _io.size
  io_size_minus_s8:
    # Because of `s8_val`, the result type is `s8`. The result doesn't fit
    # into 32 bits, because `s8_val` is `-0x1_0000_0000`. `_io.size` is the
    # left-hand operand here, so that both operand orders are covered.
    value: _io.size - s8_val
  io_size_and_u4_lit:
    # Unlike the other operands, this one is a literal. Its value doesn't fit
    # into `s4`, but its type `u4` has at most 32 bits, so the result type is
    # `CalcIntType` (see above). The mask only clears the lowest 4 bits.
    value: _io.size & 0xffff_fff0
