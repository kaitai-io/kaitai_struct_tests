meta:
  id: valid_regex
seq:
  - id: foo1
    type: str
    encoding: ASCII
    size: 6
    valid:
      regex: P[A-Z]{2}K-\d
  - id: pad1
    size: 2
  - id: foo2
    type: str
    encoding: ASCII
    size: 10
    valid:
      regex: "[A-Z]{4}p*-+U-+DEF"
