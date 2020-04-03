meta:
  id: nested_type_param
seq:
  - id: main_seq
    type: nested_type_param::my_type(5)
types:
  my_type:
    params:
      - id: my_len
        type: u4
    seq:
      - id: body
        type: str
        size: my_len
        encoding: UTF-8