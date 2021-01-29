# expr_type_cast_unknown.ksy: /instances/foo_bar/value: error: unable to find type 'bar', searching from expr_type_cast_unknown
meta:
  id: expr_type_cast_unknown
seq:
  - id: foo
    type: dummy
types:
  dummy: {}
instances:
  foo_bar:
    value: dummy.as<bar>
