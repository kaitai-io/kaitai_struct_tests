# Tests whether the currently parsed element is excluded from the array (i.e.
# not yet added to the array) and is added only after it is fully parsed. This
# is natural in the `autoRead = true` mode (non-debug, default): the `_read()`
# method is called right in the constructor, so parsing is tightly coupled with
# object creation, and it is not even possible to get a reference to the created
# object from the outside before it is parsed. However, in the `autoRead = false`
# (--debug or actually --no-auto-read) mode, it is of course possible to have a
# reference to an object that is not parsed yet (that's the point).
#
# So the implementation of --no-auto-read must be careful to only append the
# object after `_read()` has been called. But at the same time, if `_read()`
# fails, the partial object should be pushed anyway, as required by
# "debug_array_user_eof_exception". See
# https://github.com/kaitai-io/kaitai_struct/issues/1105#issuecomment-2274496312
# for how this can be done.
meta:
  id: debug_array_user_current_excluded
  ks-debug: true
seq:
  - id: array_of_cats
    type: cat
    repeat: expr
    repeat-expr: 3
types:
  cat:
    seq:
      - id: meow
        # should be 3, 2, 1 (if it's 2, 1, 0 instead, then it's wrong and the test should fail)
        size: 3 - _parent.array_of_cats.size
