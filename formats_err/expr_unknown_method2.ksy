# expr_unknown_method2.ksy: /instances/foo/value:
# 	error: don't know how to call method 'frobnicate' of object type 'CalcBytesType'
#
meta:
  id: expr_unknown_method2
instances:
  foo:
    value: '[1, 2, 3].frobnicate(2, true)'
