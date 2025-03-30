# meta_bad_endian_cases.ksy: /meta/endian/cases/1:
# 	error: expected be / le, got 'bad'
#
meta:
  id: meta_bad_endian_cases
  endian:
    switch-on: 1
    cases:
      1: bad
      _: le
