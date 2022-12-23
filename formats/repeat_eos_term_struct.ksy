# Like "repeat_eos_term_bytes", but the byte array is wrapped in a user-defined type
meta:
  id: repeat_eos_term_struct
seq:
  - id: records
    terminator: 0xb2
    include: true
    type: bytes_wrapper
    repeat: eos
types:
  bytes_wrapper:
    seq:
      - id: value
        size-eos: true
