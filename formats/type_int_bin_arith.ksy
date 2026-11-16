meta:
  id: type_int_bin_arith
  endian: le
  bit-endian: le
seq:
  - id: s1_val
    type: s1
  - id: s2_val
    type: s2
  - id: s4_val
    type: s4
  - id: s8_val
    type: s8

  - id: u1_val
    type: u1
  - id: u2_val
    type: u2
  - id: u4_val_msb_set
    type: u4
  - id: u4_val_msb_clear
    type: u4
  - id: u8_val_msb_set
    type: u8
  - id: u8_val_msb_clear
    type: u8

  - id: b08_val
    type: b8
  - id: b16_val
    type: b16
  - id: b32_val
    type: b32
  - id: b64_val
    type: b64
  - id: b31_val
    type: b31
  - id: b63_val
    type: b63
instances:
  # Addition, subtraction and multiplication: the operations must be done
  # in the result type, not in a narrower operand type, and unsigned
  # results must stay positive even if their most significant bit is set
  add_u4_u4:
    # The operands fit into `s4`, but the result only fits into `u4`, which is
    # the result type
    value: u4_val_msb_clear + u4_val_msb_clear
  sub_u4_u1:
    # `u1_val` is promoted to `u4`, and the result has the most significant
    # bit set
    value: u4_val_msb_set - u1_val
  add_u8_u8:
    # The operands fit into `s8`, but the result only fits into `u8`, which is
    # the result type
    value: u8_val_msb_clear + u8_val_msb_clear
  add_u1_u1:
    # The result type is `s4`, so the result (506) doesn't have to fit into
    # `u1`
    value: u1_val + u1_val
  mul_s1_s2:
    # The result type is `s4`, so the result (4194304) doesn't have to fit
    # into `s2`
    value: s1_val * s2_val

  # Multiplication, division and modulo: the dividends of `/` and `%` are
  # deliberately not divisible by the divisors, because an exact division
  # could give the right result even with a wrong implementation (e.g. one
  # that divides via floats). Most unsigned dividends have the most
  # significant bit set, which would make them negative if they were
  # treated as signed.
  mul_u4_lit:
    # The operands fit into `s4`, but the result only fits into `u4`, which is
    # the result type
    value: u4_val_msb_clear * 2
  mul_u8_lit:
    # The operands fit into `s8`, but the result only fits into `u8`, which is
    # the result type
    value: u8_val_msb_clear * 2
  mul_s8_lit:
    # The result type is `s8`, and the result is negative
    value: s8_val * 3
  div_u4_lit:
    value: u4_val_msb_set / 7
  div_u8_msb_set_lit:
    value: u8_val_msb_set / 7
  div_u8_msb_clear_lit:
    # The dividend doesn't have the most significant bit set, so a signed
    # division gives the right result, but a division via floats doesn't:
    # the result is too large to be represented exactly as a `double`
    value: u8_val_msb_clear / 7
  mod_u4_lit:
    value: u4_val_msb_set % 10
  mod_u8_msb_set_lit:
    value: u8_val_msb_set % 7
  mod_u8_msb_clear_lit:
    # The dividend doesn't have the most significant bit set, so a signed
    # modulo gives the right result, but a modulo via floats doesn't: the
    # quotient is too large to be represented exactly as a `double`
    value: u8_val_msb_clear % 7
  mod_s8_lit:
    # The dividend is negative, but the result is non-negative (in `0..999`),
    # unlike `%` in C
    value: s8_val % 1000
  mod_s8_u1:
    # `u1_val` is promoted to `s8`. As in `mod_s8_lit`, the dividend is
    # negative, but the result is non-negative.
    value: s8_val % u1_val
  mod_s8_lit_u4:
    # The literal's own type is `u4`, but it takes the type of the other
    # operand (`s8`)
    value: s8_val % 3_000_000_000
