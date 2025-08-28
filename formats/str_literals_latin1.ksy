meta:
  id: str_literals_latin1
seq:
  - id: len_parsed
    type: u2le
  - id: parsed
    size: len_parsed
    type: str
    encoding: UTF-8
instances:
  parsed_eq_literal:
    value: parsed == "\u00a3"
