import os, streams, options, sequtils
import ../../compiled/nim/debug_0
import auxiliary/test_utils

let r = Debug0.fromFile("../../src/fixed_struct.bin")
assert r.one == 80
assert len(r.arrayOfInts) == 3
assert r.arrayOfInts[0] == 65
assert r.arrayOfInts[1] == 67
assert r.arrayOfInts[2] == 75
assert r.unnamed2 == 45

# FIXME: also test --read-pos once it is implemented
