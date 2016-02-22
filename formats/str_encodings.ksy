meta:
  id: str_encodings
  endian: le
seq:
  - id: len1
    type: u2
  - id: str1
    type: str
    byte_size: len1
    encoding: ASCII
  - id: len2
    type: u2
  - id: str2
    type: str
    byte_size: len2
    encoding: UTF-8
  - id: len3
    type: u2
  - id: str3
    type: str
    byte_size: len3
    encoding: SJIS
  - id: len4
    type: u2
  - id: str4
    type: str
    byte_size: len4
    encoding: CP437
