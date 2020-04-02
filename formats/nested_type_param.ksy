meta:
  id: test
  endian: be
seq:
  - id: seq_block
    type: test::my_type(2)
types:
  my_type:
    params:
      - id: repeat_count
        type: s4
    seq:
      - id: world
        type: s4
      - id: repeated_thing
        type: s4
        repeat: expr
        repeat-expr: repeat_count
