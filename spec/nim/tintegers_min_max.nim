import os, streams, options, sequtils
import ../../compiled/nim/integers_min_max
import auxiliary/test_utils

let r = IntegersMinMax.fromFile("../../src/integers_min_max.bin")

assert r.unsignedMin.u1 == 0
assert r.unsignedMin.u2le == 0
assert r.unsignedMin.u4le == 0
assert r.unsignedMin.u8le == 0
assert r.unsignedMin.u2be == 0
assert r.unsignedMin.u4be == 0
assert r.unsignedMin.u8be == 0
assert r.unsignedMax.u1 == 255
assert r.unsignedMax.u2le == 65535
assert r.unsignedMax.u4le == 4294967295'i64
assert r.unsignedMax.u8le == 18446744073709551615'u64
assert r.unsignedMax.u2be == 65535
assert r.unsignedMax.u4be == 4294967295'i64
assert r.unsignedMax.u8be == 18446744073709551615'u64
assert r.signedMin.s1 == -128
assert r.signedMin.s2le == -32768
assert r.signedMin.s4le == -2147483647 - 1
assert r.signedMin.s8le == -9223372036854775807 - 1
assert r.signedMin.s2be == -32768
assert r.signedMin.s4be == -2147483647 - 1
assert r.signedMin.s8be == -9223372036854775807 - 1
assert r.signedMax.s1 == 127
assert r.signedMax.s2le == 32767
assert r.signedMax.s4le == 2147483647
assert r.signedMax.s8le == 9223372036854775807'i64
assert r.signedMax.s2be == 32767
assert r.signedMax.s4be == 2147483647
assert r.signedMax.s8be == 9223372036854775807'i64
