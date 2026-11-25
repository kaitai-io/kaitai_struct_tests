# Ternary operator (`?:`) with a branch of type `CalcIntType`, which
# built-in integer properties and variables such as `_io.size` have in the
# compiler (see ExprCalcIntPromotion). As in arithmetic operations, if the
# other branch is wider than 32 bits, the result type is the type of that
# branch. Otherwise, the result type is `CalcIntType`.
meta:
  id: expr_calc_int_ternary
  endian: le
seq:
  - id: u4_val
    type: u4
  - id: u8_val
    type: u8
  - id: s8_val
    type: s8
instances:
  u8_io_size:
    # Because of `u8_val`, the result type is `u8`. The condition is true, so
    # `u8_val` is selected. Its value doesn't fit into 32 bits.
    value: 'u4_val != 0 ? u8_val : _io.size'
  io_size_u8:
    # `_io.size` is the first branch here, so that both branch orders are
    # covered. The condition is negated, so that `u8_val` is selected again.
    value: 'u4_val == 0 ? _io.size : u8_val'
  div_s8_io_size:
    # Because of `s8_val`, the ternary is `s8`, so the division is signed.
    # `s8_val` is selected. Its value is negative, so the result is negative
    # too. The division is exact, so it gives the same result whether it
    # rounds toward zero or down.
    value: '(u4_val != 0 ? s8_val : _io.size) / 2'
  io_size_u4:
    # `u4_val` is `u4`, which has at most 32 bits, so the result type is
    # `CalcIntType`
    value: 'u4_val != 0 ? u4_val : _io.size'
