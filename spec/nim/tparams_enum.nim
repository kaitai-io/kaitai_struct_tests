# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/params_enum
import auxiliary/test_utils

let r = ParamsEnum.fromFile("../../src/enum_0.bin")

assert r.one == params_enum.cat
assert r.invokeWithParam.isCat == true
