# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/multiple_use
import auxiliary/test_utils

let r = MultipleUse.fromFile("../../src/position_abs.bin")

assert r.t1.firstUse.value == 32
assert r.t2.secondUse.value == 32
