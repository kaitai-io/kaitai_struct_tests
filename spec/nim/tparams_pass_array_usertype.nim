# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/params_pass_array_usertype
import auxiliary/test_utils

let r = ParamsPassArrayUsertype.fromFile("../../src/position_to_end.bin")

assert len(r.passBlocks.bar) == 2
assert r.passBlocks.bar[0].foo == 1
assert r.passBlocks.bar[1].foo == 2
assert r.passBlocks.one == @[3'u8]
assert r.passBlocks.two == @[4'u8, 5'u8]
