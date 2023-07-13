# encoding_str_bad.ksy: /seq/1/encoding:
# 	warning: unrecognized encoding name 'invalid1'
#
# encoding_str_bad.ksy: /instances/baz/encoding:
# 	warning: unrecognized encoding name 'invalid2'
#
meta:
  id: encoding_str_bad
seq:
  - id: foo
    type: strz
    encoding: UTF-8
  - id: bar
    type: strz
    encoding: invalid1
instances:
  baz:
    pos: 1
    type: str
    size: 3
    encoding: invalid2
