# imports/type_two.ksy: /seq/0/type:
# 	error: unable to find type 'type_one', searching from 'type_two'
#
meta:
  id: imports_type_leaking
  imports:
    - imports/type_one
    - imports/type_two
seq:
  - id: another_one
    type: type_one
  - id: another_two
    type: type_two
