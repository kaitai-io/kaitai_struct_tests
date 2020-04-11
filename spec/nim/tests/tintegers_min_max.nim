# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils, unittest
import ../../../compiled/nim/integers_min_max
import ../test_utils

let r = IntegersMinMax.fromFile("src/integers_min_max.bin")

test "IntegersMinMax":

  check(r.unsignedMin.u1 == uint8(0))
  check(r.unsignedMin.u2le == uint16(0))
  check(r.unsignedMin.u4le == uint32(0))
  check(r.unsignedMin.u8le == uint64(0))
  check(r.unsignedMin.u2be == uint16(0))
  check(r.unsignedMin.u4be == uint32(0))
  check(r.unsignedMin.u8be == uint64(0))
  check(r.unsignedMax.u1 == uint8(255))
  check(r.unsignedMax.u2le == uint16(65535))
  check(r.unsignedMax.u4le == uint32(4294967295'u64))
  check(r.unsignedMax.u8le == uint64(18446744073709551615'u64))
  check(r.unsignedMax.u2be == uint16(65535))
  check(r.unsignedMax.u4be == uint32(4294967295'u64))
  check(r.unsignedMax.u8be == uint64(18446744073709551615'u64))
  check(r.signedMin.s1 == int8(-128))
  check(r.signedMin.s2le == int16(-32768))
  check(r.signedMin.s4le == int32(-2147483648'u64))
  check(r.signedMin.s8le == int64(-9223372036854775808'u64))
  check(r.signedMin.s2be == int16(-32768))
  check(r.signedMin.s4be == int32(-2147483648'u64))
  check(r.signedMin.s8be == int64(-9223372036854775808'u64))
  check(r.signedMax.s1 == int8(127))
  check(r.signedMax.s2le == int16(32767))
  check(r.signedMax.s4le == int32(2147483647))
  check(r.signedMax.s8le == int64(9223372036854775807'u64))
  check(r.signedMax.s2be == int16(32767))
  check(r.signedMax.s4be == int32(2147483647))
  check(r.signedMax.s8be == int64(9223372036854775807'u64))
  discard