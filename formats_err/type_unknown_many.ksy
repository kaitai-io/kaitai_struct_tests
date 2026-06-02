# type_unknown_many.ksy: /seq/0/type:
# 	error: unable to find type 'some_unknown_name', searching from 'type_unknown_many'
#
# type_unknown_many.ksy: /seq/1/type:
# 	error: unable to find type 'also_unknown_name', searching from 'type_unknown_many'
#
# type_unknown_many.ksy: /seq/2/enum:
# 	error: unable to find enum 'unknown_enum', searching from 'type_unknown_many'
#
# type_unknown_many.ksy: /seq/3/type:
# 	error: unable to find type 'unknown_type_in_array', searching from 'type_unknown_many'
#
# type_unknown_many.ksy: /seq/4/enum:
# 	error: unable to find enum 'unknown_enum_in_array', searching from 'type_unknown_many'
#
meta:
  id: type_unknown_many
  ks-opaque-types: false
seq:
  - id: foo
    type: some_unknown_name
  - id: bar
    type: also_unknown_name
  - id: baz
    type: u1
    enum: unknown_enum
  - id: type_in_array
    type: unknown_type_in_array
    repeat: eos
  - id: enum_in_array
    type: u1
    enum: unknown_enum_in_array
    repeat: eos
