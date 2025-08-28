# attr_bad_valid_repeat_eq_short.ksy: /seq/0/valid:
# 	error: can't compare IntMultiType(true,Width4,Some(LittleEndian)) and ArrayTypeInStream(CalcIntType)
#
meta:
  id: attr_bad_valid_repeat_eq_short
seq:
  - id: foo
    type: s4le
    valid: '[1000, -2000, 3000]'
    repeat: eos
