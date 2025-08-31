# Demonstrates that if an instance is used in `repeat-expr`, it's necessary that
# the expression is actually evaluated because the "num_chunks" instance must be
# invoked, even when writing (where you already know the array length).
#
# If the `repeat-expr` isn't evaluated and we would for example use a for-each
# loop over "chunks" when writing instead, the "num_chunks" instance would not
# be invoked at the right moment and wouldn't be written. But writing the
# "num_chunks" _after_ the seq has been written is incorrect, because the `pos`
# expression of "num_chunks" will evaluate differently (`_io.pos` will be `16`,
# since that's where the `seq` left off, not `0` as it should be) and
# "num_chunks" will be written to a totally different position.
meta:
  id: instance_in_repeat_expr
  endian: le
seq:
  - id: chunks
    type: chunk
    repeat: expr
    repeat-expr: num_chunks
instances:
  num_chunks:
    pos: _io.pos + 16 # _io.pos is 0 when first invoked, so this should be at byte 16
    type: u4
types:
  chunk:
    seq:
      - id: offset
        type: u4
      - id: len
        type: u4
