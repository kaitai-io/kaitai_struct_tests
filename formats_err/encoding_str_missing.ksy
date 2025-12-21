# encoding_str_missing.ksy: /seq/0:
# 	error: string type, but no encoding found
#
# encoding_str_missing.ksy: /seq/1:
# 	error: string type, but no encoding found
#
# encoding_str_missing.ksy: /instances/baz:
# 	error: string type, but no encoding found
#
meta:
  id: encoding_str_missing
seq:
  - id: foo
    type: strz
  - id: bar
    type: strz
instances:
  baz:
    pos: 1
    type: strz
