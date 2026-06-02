# repeat_eos_repeat-until.ksy: /seq/0/repeat:
# 	error: `repeat-until` requires either a `repeat: until` or absence of a `repeat` key
#
meta:
  id: repeat_eos_repeat_until
seq:
  - id: foo
    type: u1
    repeat: eos
    repeat-until: 'true'
