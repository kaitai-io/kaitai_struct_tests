# Demonstrates problems in Java where you cannot naively compare boxed integer
# types (e.g. `Short`, `Integer`) using `==` (at best you get a reference
# equality where `Integer.valueOf(-1) != Integer.valueOf(-1)`, at worst the
# generated Java code will have a compile-time error like "Incompatible operand
# types Integer and Short")
meta:
  id: expr_if_int_eq
  endian: le
seq:
  - id: skip
    size: 2
  - id: seq
    type: s2
  - id: seq_if
    type: s2
    if: true
instances:
  calc:
    value: 0x4141
  calc_if:
    value: 0x4141
    if: true

  seq_eq_lit:
    value: seq == 0x4141
  seq_eq_calc:
    value: seq == calc
  seq_eq_calc_if:
    value: seq == calc_if
  seq_eq_seq_if:
    value: seq == seq_if

  calc_eq_lit:
    value: calc == 0x4141
  calc_eq_calc_if:
    value: calc == calc_if
  calc_eq_seq_if:
    value: calc == seq_if

  calc_if_eq_lit:
    value: calc_if == 0x4141
  calc_if_eq_seq_if:
    value: calc_if == seq_if

  seq_if_eq_lit:
    value: seq_if == 0x4141
