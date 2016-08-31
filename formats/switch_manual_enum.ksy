meta:
  id: switch_manual_enum
seq:
  - id: opcodes
    type: opcode
    repeat: eos
types:
  opcode:
    seq:
      - id: code
        type: u1
        enum: code
      - id: body
        type:
          switch-on: code
          cases:
            code::intval: intval
            code::strval: strval
    enums:
      code:
        73: intval # 'I'
        83: strval # 'S'
    types:
      intval:
        seq:
          - id: value
            type: u1
      strval:
        seq:
          - id: value
            type: strz
            encoding: ASCII
