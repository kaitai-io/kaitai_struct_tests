# Like "term_struct", but the parsed bytes go through `process` before they are passed to
# the user type `bytes_wrapper`.
#
# We use a custom process `my_custom_fx` (not usual built-in ones like `process:
# xor(key)`), for which the compiler doesn't know whether it preserves the byte size when
# decoding/encoding. This is important, we want this test to demonstrate a case when both
# the *outer size* (the length of the byte array read directly from the file, which will
# be the input to the `process` decoder) and the *inner size* (exact length of the
# substream created from the decoded bytes) are unknown at serialization and must be both
# specified by the user. We cannot use the built-in processes `xor`, `rol` or `ror`
# because the compiler knows that these preserve the length of the input, and therefore
# once the *outer size* is known, *inner size* is also known because it's the same.
meta:
  id: process_term_struct
seq:
  - id: s1
    terminator: 0x7c
    process: my_custom_fx(0x20, false, [0])
    type: bytes_wrapper
  - id: s2
    terminator: 0x7c
    consume: false
    process: my_custom_fx(0x20, false, [0])
    type: bytes_wrapper
  - id: s3
    terminator: 0x40
    include: true
    process: my_custom_fx(0x20, false, [0])
    type: bytes_wrapper
types:
  bytes_wrapper:
    seq:
      - id: value
        size-eos: true
