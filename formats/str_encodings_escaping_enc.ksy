# Like "str_encodings_escaping_to_s", but the `encoding` key is used instead of
# the `.to_s(encoding)` expression language method.
meta:
  id: str_encodings_escaping_enc
  endian: le
seq:
  - id: len_of_1
    type: u2
  - id: str1
    size: len_of_1
    type: str1_wrapper
  - id: len_of_2
    type: u2
  - id: str2
    size: len_of_2
    type: str2_wrapper
  - id: len_of_3
    type: u2
  - id: str3
    size: len_of_3
    type: str3_wrapper
  - id: len_of_4
    type: u2
  - id: str4
    size: len_of_4
    type: str4_wrapper
types:
  # The single-quoted strings in YAML are treated similarly as in the KS
  # expression language ('\\' are two literal backslashes, "\\" is one
  # backslash) - every character in '(...)' is treated literally, except that
  # it's possible to get a literal single quote by doubling it (i.e. `foo:
  # 'here '' is a single quote'` stands for `{ "foo": "here ' is a single
  # quote" }`).
  str1_wrapper:
    instances:
      v:
        pos: 0
        size-eos: true
        type: str
        encoding: 'ASCII\\x'
  str2_wrapper:
    instances:
      v:
        pos: 0
        size-eos: true
        type: str
        encoding: 'UTF-8\''x'
  str3_wrapper:
    instances:
      v:
        pos: 0
        size-eos: true
        type: str
        encoding: 'SJIS\"x'
  str4_wrapper:
    instances:
      v:
        pos: 0
        size-eos: true
        type: str
        encoding: 'IBM437\nx'
