# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/process_coerce_switch
import auxiliary/test_utils

let r = ProcessCoerceSwitch.fromFile("../../src/process_coerce_switch.bin")

assert r.bufType == 0
assert r.flag == 0
assert (ProcessCoerceSwitch_Foo(r.buf)).bar == @[65'u8, 65'u8, 65'u8, 65'u8]