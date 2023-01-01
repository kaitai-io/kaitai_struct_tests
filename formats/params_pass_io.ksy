meta:
  id: params_pass_io
seq:
  - id: first
    size: 1
    type: block
  - id: one
    type: 'param_type(first.foo == 0xff ? first._io : _root._io)'
types:
  block:
    seq:
      - id: foo
        type: u1
  param_type:
    params:
      - id: arg_stream
        type: io
    seq:
      - id: buf
        size: arg_stream.size
