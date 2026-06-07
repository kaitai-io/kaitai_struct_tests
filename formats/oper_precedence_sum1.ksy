meta:
  id: oper_precedence_sum1
doc: |
  This test checks the result of chain of two operators: `+` and any other possible,
  without use of parenthesis.

  This test ensures, that results of the KS expression in each supported language will be identical.
instances:
  sum:
    value: 1 + 2 + 3
  sub:
    value: 1 + 2 - 3
  mul:
    value: 1 + 2 * 3
  div_int:
    value: 1 + 2 / 3
  div_float1:
    value: 1 + 2.0 / 3
  div_float2:
    value: 1 + 2 / 3.0
  rem:
    value: 1 + 2 % 3

  shl:
    value: 1 + 2 << 3
  shr:
    value: 1 + 2 >> 3

  bit_and:
    # 1 + 2 & 3 gives the same result regardless of the precedence, so swap some operands
    value: 1 + 3 & 2
  bit_or:
    value: 1 + 2 | 3
  bit_xor:
    value: 1 + 2 ^ 3

  eq:
    value: 1 + 2 == 3
  ne:
    value: 1 + 2 != 3
  le:
    value: 1 + 2 <= 3
  ge:
    value: 1 + 2 >= 3
  lt:
    value: 1 + 2 < 3
  gt:
    value: 1 + 2 > 3

  # not possible: KS compilation errors
  # and:
  #   value: 1 + 2 and 3
  # or:
  #   value: 1 + 2 or 3