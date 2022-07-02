# io_on_bytes.ksy: /instances/one_in_foo/io:
# 	error: don't know how to call method '_io' of object type 'BytesLimitType(IntNum(100),None,false,None,None)'
#
meta:
  id: io_on_bytes
seq:
  - id: foo
    size: 100
instances:
  one_in_foo:
    io: foo._io
    pos: 23
    type: u1
