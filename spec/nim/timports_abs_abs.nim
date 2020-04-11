import os, streams, options, sequtils, unittest
import ../../../compiled/nim/imports_abs_abs
import ../test_utils

let r = ImportsAbsAbs.fromFile("src/fixed_struct.bin")

test "ImportsAbsAbs":

  check(r.one == uint8(80))
  check(r.two.one == uint8(65))
  check(r.two.two.one == uint8(67))
  discard
