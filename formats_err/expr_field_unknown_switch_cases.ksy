# expr_field_unknown_switch_cases.ksy: /seq/1/type/cases/Name(identifier(ahem)):
# 	error: unable to access 'ahem' in expr_field_unknown_switch_cases context
#
meta:
  id: expr_field_unknown_switch_cases
seq:
  - id: code
    type: strz
    encoding: ASCII
  - id: foo
    type:
      switch-on: code
      cases:
        "ahem": really
types:
  really: {}
