# imports/enum_two.ksy: /seq/0/enum:
# 	error: unable to find type 'enum_one', searching from 'enum_two'
#
# imports/enum_two.ksy: /instances/instance_one/enum:
# 	error: unable to find type 'enum_one', searching from 'enum_two'
#
meta:
  id: imports_enum_leaking
  imports:
    - imports/enum_one
    - imports/enum_two
seq:
  - id: another_one
    type: u1
    enum: enum_one::one
  - id: another_two
    type: u1
    enum: enum_two::two
instances:
  instance_one:
    value: 0
    enum: enum_one::one
  instance_two:
    value: 0
    enum: enum_two::two
