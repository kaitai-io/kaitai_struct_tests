meta:
  id: bits_byte_aligned_eof_be
  bit-endian: be
seq:
  - id: prebuf
    size: 8
  - id: bits
    type: b31
  # 1 bit left until the end of file. This means that during serialization, when
  # the stream is closed / `seek()` is called by the test code, the stream
  # position must be "aligned" to a byte boundary so that the buffered bits are
  # written into the final byte.
  #
  # We're specifically testing the alignment right before the EOF here to ensure
  # that the EOF check in the implicitly called "write_align_to_byte()" method
  # works properly, i.e. that it doesn't wrongly throw an EOF error here.
