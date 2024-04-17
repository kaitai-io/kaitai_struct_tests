meta:
  id: eof_exception_sized
seq:
  - id: buf
    # only 12 bytes available, should fail with EOF exception
    size: 13
    type: foo
types:
  foo: {}
