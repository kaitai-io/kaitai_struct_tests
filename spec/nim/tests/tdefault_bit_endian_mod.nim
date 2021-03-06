# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils, unittest
import ../../../compiled/nim/default_bit_endian_mod
import ../test_utils

let r = DefaultBitEndianMod.fromFile("src/fixed_struct.bin")

test "DefaultBitEndianMod":

  check(r.main.one == uint64(336))
  check(r.main.two == uint64(8608))
  check(r.main.nest.two == uint64(11595))
  check(r.main.nestBe.two == uint64(12799))
  discard
