# io_on_bytes: /instances/one_in_foo/io: called invalid attribute '_io' on expression of type BytesLimitType(IntNum(100),None,false,None,None)
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
