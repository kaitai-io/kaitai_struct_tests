# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/process_xor_const
import auxiliary/test_utils

let r = ProcessXorConst.fromFile("../../src/process_xor_1.bin")

assert r.key == 255
assert r.buf == @[102'u8, 111'u8, 111'u8, 32'u8, 98'u8, 97'u8, 114'u8]
