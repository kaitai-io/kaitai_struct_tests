meta:
  id: test_parent
seq:
  - id: root_byte
    type: u1
  - id: child
    type: child_struct
types:
  child_struct:
    seq:
      - id: child_byte
        type: u1
      - id: child2
        type: child2_struct
        repeat: expr
        repeat-expr: _root.root_byte
    types:
      child2_struct:
        seq:
          - id: child2_byte
            type: u1
            repeat: expr
            repeat-expr: _parent.child_byte
          - id: child3_struct
            type: child3_struct
            repeat: expr
            repeat-expr: _parent._parent.root_byte
        types:
          child3_struct:
            seq:
              - id: child3_byte
                type: u1
                repeat: expr
                repeat-expr: _parent._parent._parent.root_byte


