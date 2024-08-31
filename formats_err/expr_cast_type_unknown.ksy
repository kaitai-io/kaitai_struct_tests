# expr_cast_type_unknown.ksy: /instances/foo_bar/value:
# 	error: unable to find type 'bar', searching from expr_cast_type_unknown
#
meta:
  id: expr_cast_type_unknown
seq:
  - id: foo
    type: dummy
types:
  dummy: {}
instances:
  foo_bar:
    value: dummy.as<bar>
