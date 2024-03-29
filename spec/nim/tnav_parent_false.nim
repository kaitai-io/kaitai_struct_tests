# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/nav_parent_false
import auxiliary/test_utils

let r = NavParentFalse.fromFile("../../src/nav_parent_codes.bin")

assert r.childSize == 3
assert r.elementA.foo.code == 73
assert r.elementA.foo.more == @[49'u8, 50'u8, 51'u8]
assert r.elementA.bar.foo.code == 66
assert r.elementB.foo.code == 98
