# type_unknown_switch.ksy: /seq/0/type/cases/IntNum(42):
# 	error: unable to find type 'some_unknown_name', searching from type_unknown_switch
#
meta:
  id: type_unknown_switch
seq:
  - id: foo
    type:
      switch-on: 42
      cases:
        42: some_unknown_name
