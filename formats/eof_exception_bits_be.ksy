meta:
  id: eof_exception_bits_be
  bit-endian: be
  ks-debug: true
seq:
  - id: pre_bits
    type: b7
  # only 2 full bytes (17 bits) available in the stream, should fail with EOF
  # exception
  - id: fail_bits
    type: b18
