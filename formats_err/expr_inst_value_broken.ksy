# expr_inst_value_broken.ksy: /instances/foo/value:
# 	error: parsing expression '1 *' failed on "*" at position 1:3, expected end-of-input
#
meta:
  id: expr_inst_value_broken
instances:
  foo:
    value: '1 *'
