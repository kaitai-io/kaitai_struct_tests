# Like "bits_byte_aligned_eof_be", but for little-endian order.
meta:
  id: bits_byte_aligned_eof_le
  bit-endian: le
seq:
  - id: prebuf
    size: 8
  - id: bits
    type: b31
    # 1 bit left until the end of file. (... - see "bits_byte_aligned_eof_be"
    # for more details)
