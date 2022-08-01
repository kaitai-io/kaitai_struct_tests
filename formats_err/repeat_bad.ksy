# repeat_bad.ksy: /seq/0/repeat:
# 	error: expected eos / expr / until, got 'bad'
#
meta:
  id: repeat_bad
seq:
  - id: foo
    type: u1
    repeat: bad
