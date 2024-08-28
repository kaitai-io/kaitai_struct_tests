# expr_field_unknown_endian_switch_on.ksy: /meta/endian/switch-on:
# 	error: unable to access 'qux' in expr_field_unknown_endian_switch_on context
#
meta:
  id: expr_field_unknown_endian_switch_on
  endian:
    switch-on: qux
    cases:
      0: be
      _: le
