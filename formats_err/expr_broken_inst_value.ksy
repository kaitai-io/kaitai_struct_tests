# expr_broken_inst_value.ksy: /instances/foo/value:
# 	error: parsing expression '1 *' failed on "*" at position 1:3, expected end-of-input
#
meta:
  id: expr_broken_inst_value
instances:
  foo:
    value: '1 *'
