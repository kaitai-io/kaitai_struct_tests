# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/enum_deep
import auxiliary/test_utils

let r = EnumDeep.fromFile("../../src/enum_0.bin")

assert r.pet1 == enum_deep.cat
assert r.pet2 == enum_deep.hare
