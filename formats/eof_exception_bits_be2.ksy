meta:
  id: eof_exception_bits_be2
  bit-endian: be
  ks-debug: true
seq:
  - id: pre_bits
    type: b8
  # only 2 full bytes (16 bits) available in the stream, should fail with EOF
  # exception
  - id: fail_bits
    type: b17
