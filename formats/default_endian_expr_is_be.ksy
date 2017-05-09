meta:
  id: default_endian_expr_is_be
seq:
  - id: docs
    repeat: eos
    type: doc
types:
  doc:
    seq:
      - id: indicator
        size: 2
      - id: main
        type: main_obj
    types:
      main_obj:
        meta:
          endian: expr
          endian-is-be: _parent.indicator == [0x4d, 0x4d]
        seq:
          - id: some_int
            type: u4
          - id: some_int_be
            type: u2be
          - id: some_int_le
            type: u2le
