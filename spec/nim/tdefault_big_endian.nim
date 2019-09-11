import unittest
import ../../../runtime/nim/kaitai
import ../../compiled/nim/default_big_endian

suite "Default big endian":
  test "Value read":
    let value = DefaultBigEndian.read("../../src/enum_0.bin")
    check(value.one == 117440512)
