# Autogenerated from KST: please remove this line if doing any edits by hand!

import ../../compiled/nim/hello_world
import os, streams, options, sequtils
import ../../compiled/nim/cast_to_imported
import auxiliary/test_utils

let r = CastToImported.fromFile("../../src/fixed_struct.bin")

assert r.one.one == 80
assert r.oneCasted.one == 80
