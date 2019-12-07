meta:
  id: nav_parent_switch_cast
seq:
  - id: foo
    type: foo
types:
  foo:
    seq:
      - id: buf_type
        type: u1
      - id: flag
        type: u1
      - id: buf
        size: 4
        type:
          switch-on: buf_type
          cases:
            0: zero
            1: one
    types:
      zero:
        seq:
        - id: bar
          type: bar
      one:
        seq:
        - id: bar
          type: bar
      bar:
        instances:
          flag:
            value: '_parent._parent.as<foo>.flag'
