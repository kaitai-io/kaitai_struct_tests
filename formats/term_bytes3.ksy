# Another variation of "term_bytes" - this time with two `consume: false` fields, which makes sure
# that multiple `consume: false` fields in the same `seq` do not cause redeclaration of the local
# `_pos` variable (which is illegal in some targets), which is used to remember the stream position
# before the `terminator` byte is written so that the generated code can seek back there after
# writing it.
#
# Other case tested here is that `s3` will be a 0-length field (the `terminator` will occur as the
# first byte and thus the field value will be cut off), which is probably not common but perfectly
# legal, so this test makes sure that we don't fail in this case.
meta:
  id: term_bytes3
seq:
  - id: s1
    terminator: 0x7c
    consume: false
  - id: s2
    terminator: 0x40
    consume: false
  - id: s3
    terminator: 0x40
