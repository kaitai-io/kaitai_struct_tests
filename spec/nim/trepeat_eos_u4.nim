# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/repeat_eos_u4
import auxiliary/test_utils

let r = RepeatEosU4.fromFile("../../src/repeat_eos_struct.bin")

assert r.numbers == @[uint32(0), 66, 66, 2069]
