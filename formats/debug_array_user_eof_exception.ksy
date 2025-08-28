# Similar to "debug_array_user", but with a shorter input file that causes the
# parsing of `array_of_cats[2].chirp` to fail due to an EOF exception. Despite
# this failure, `array_of_cats[2].meow` should be accessible because it was
# parsed successfully.
#
# This behavior enables visualizers to display as much partial output as
# possible despite a parsing error, see
# https://github.com/kaitai-io/kaitai_struct/issues/1105#issuecomment-2274496312
meta:
  id: debug_array_user_eof_exception
  ks-debug: true
seq:
  - id: one_cat
    type: cat
  - id: array_of_cats
    type: cat
    repeat: expr
    repeat-expr: 3
types:
  cat:
    seq:
      - id: meow
        type: u1
      - id: chirp
        type: u1
