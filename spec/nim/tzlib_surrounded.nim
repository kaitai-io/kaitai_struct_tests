# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/zlib_surrounded
import auxiliary/test_utils

let r = ZlibSurrounded.fromFile("../../src/zlib_surrounded.bin")

assert r.zlib.num == -1
