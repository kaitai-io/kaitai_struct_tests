import os, streams, options, sequtils, unittest
import ../../../compiled/nim/optional_id
import ../test_utils

let r = OptionalId.fromFile("src/fixed_struct.bin")

test "OptionalId":

  check(r.unnamed0 == uint8(80))
  check(r.unnamed1 == uint8(65))
  check(r.unnamed2 == string(@[67'i8, 75, 45, 49, -1].toString))
  discard
