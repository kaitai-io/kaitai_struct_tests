meta:
  id: valid_fail_regex
seq:
  - id: foo1
    type: str
    encoding: ASCII
    size: 6
    valid:
      regex: P[a-z]{2}K-\d
#expecting lower case, where there are upper case letters in the file
