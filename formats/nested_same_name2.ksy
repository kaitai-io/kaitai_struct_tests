meta:
  id: nested_same_name2
  endian: le
seq:
  - id: version
    type: u4
  - id: main_data
    type: main
  - id: dummy
    type: dummy
types:
  main:
    seq:
      - id: main_size
        type: s4
      - id: foo
        type: foo
    types:
      foo:
        seq:
          - id: data1
            size: _parent.main_size * 2
  dummy:
    seq:
      - id: dummy_size
        type: s4
      - id: foo
        type: foo
    types:
      foo:
        seq:
          - id: data2
            size: _parent.dummy_size * 2
