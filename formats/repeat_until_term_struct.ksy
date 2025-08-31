# Like "repeat_until_term_bytes", but the byte array is wrapped in a user-defined type
meta:
  id: repeat_until_term_struct
seq:
  - id: records1
    terminator: 0xaa
    type: bytes_wrapper
    repeat: until
    repeat-until: _.value.length == 0
  - id: records2
    terminator: 0xaa
    include: true
    type: bytes_wrapper
    repeat: until
    repeat-until: _.value != [0xaa]
  - id: records3
    terminator: 0x55
    consume: false
    type: bytes_wrapper
    repeat: until
    repeat-until: _.value == records1.last.value
types:
  bytes_wrapper:
    seq:
      - id: value
        size-eos: true
