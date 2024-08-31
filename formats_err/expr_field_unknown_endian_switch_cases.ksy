# expr_field_unknown_endian_switch_cases.ksy: /meta/endian/cases/Name(identifier(qux)):
# 	error: unable to access 'qux' in expr_field_unknown_endian_switch_cases context
#
meta:
  id: expr_field_unknown_endian_switch_cases
  endian:
    switch-on: 42
    cases:
      qux: be
      _: le
