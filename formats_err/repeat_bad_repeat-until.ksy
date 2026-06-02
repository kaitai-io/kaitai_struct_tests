# repeat_bad_repeat-until.ksy: /seq/0/repeat:
# 	error: expected eos / expr / until, got 'bad'
#
meta:
  id: repeat_bad_repeat_until
seq:
  - id: foo
    type: u1
    repeat: bad
    repeat-until: 'true'
