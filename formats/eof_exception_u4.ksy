meta:
  id: eof_exception_u4
seq:
  - id: prebuf
    size: 9
  - id: fail_int
    # only 3 bytes available, should fail with EOF exception
    type: u4le
