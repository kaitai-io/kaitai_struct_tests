meta:
  id: repeat_until_bytes
seq:
  - id: records
    size: 5
    repeat: until
    repeat-until: _[0] == 0xaa
