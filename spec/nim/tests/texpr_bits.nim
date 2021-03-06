# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils, unittest
import ../../../compiled/nim/expr_bits
import ../test_utils

let r = ExprBits.fromFile("src/switch_opcodes.bin")

test "ExprBits":

  check(r.a == uint64(2))
  check(r.enumSeq == expr_bits.foo)
  check(r.byteSize == @[102'u8, 111'u8])
  check(len(r.repeatExpr) == int(2))
  check(r.repeatExpr[0] == int8(111))
  check(r.repeatExpr[1] == int8(98))
  check(r.switchOnType == int8(97))
  check(r.switchOnEndian.foo == int16(29184))
  check(r.enumInst == expr_bits.bar)
  check(r.instPos == int8(111))
  discard
