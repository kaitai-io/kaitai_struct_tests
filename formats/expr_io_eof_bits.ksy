# Tests _io.eof operation when bit-sized integers are involved
meta:
  id: expr_io_eof_bits
  bit-endian: be
  ks-debug: true
seq:
  - id: foo
    type: b20
  - id: bar
    type: b4
    if: not _io.eof
  # `_io.eof` is expected to be true, so we "assert" it like this - if `_io.eof`
  # is false, it will attempt to consume 8 bytes, which would trigger a EOF
  # exception.
  - id: assert_io_eof_before_baz
    size: '_io.eof ? 0 : 8'

    # 0 bits available at this point. When parsing, this is basically guaranteed
    # to fail with EOF exception immediately  (because it would translate to
    # requesting more bytes, which are not available). But when writing, it
    # would succeed until an attempt to align the stream to a byte position is
    # done - which should happen at the latest when the stream is closed.
  - id: baz
    type: b3

  # `_io.eof` is expected to be true, so we "assert" it like this - if `_io.eof`
  # is false, it will attempt to consume 8 bytes, which would trigger a EOF
  # exception.
  - id: assert_io_eof_after_baz
    size: 8
    if: not _io.eof
