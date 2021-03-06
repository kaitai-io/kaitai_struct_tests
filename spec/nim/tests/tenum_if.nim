# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils, unittest
import ../../../compiled/nim/enum_if
import ../test_utils

let r = EnumIf.fromFile("src/if_struct.bin")

test "EnumIf":

  check(r.op1.opcode == enum_if.a_string)
  check(r.op1.argStr.str == string("foo"))
  check(r.op2.opcode == enum_if.a_tuple)
  check(r.op2.argTuple.num1 == uint8(66))
  check(r.op2.argTuple.num2 == uint8(67))
  check(r.op3.opcode == enum_if.a_string)
  check(r.op3.argStr.str == string("bar"))
  discard
