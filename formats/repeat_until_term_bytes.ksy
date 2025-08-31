# see "repeat_n_term_bytes"
meta:
  id: repeat_until_term_bytes
seq:
  - id: records1
    terminator: 0xaa
    repeat: until
    repeat-until: _.length == 0
  - id: records2
    terminator: 0xaa
    include: true
    repeat: until
    repeat-until: _ != [0xaa]
  - id: records3
    terminator: 0x55
    consume: false
    repeat: until
    repeat-until: _ == records1.last
