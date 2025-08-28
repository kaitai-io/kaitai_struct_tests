meta:
  id: eos_exception_u4
seq:
  - id: envelope
    size: 6
    type: data
types:
  data:
    seq:
      - id: prebuf
        size: 3
      - id: fail_int
        type: u4le
