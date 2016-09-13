meta:
  id: nested_same_name
  endian: le
seq:
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
          - id: data
            size: '_parent.main_size * 2'
  dummy:
    types:
      foo: {}
