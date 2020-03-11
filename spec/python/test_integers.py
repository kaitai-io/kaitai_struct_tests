# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from integers import Integers

class TestIntegers(unittest.TestCase):
    def test_integers(self):
        with Integers.from_file('src/integers.bin') as r:

            self.assertEqual(r.unsigned_min.u1, 0)
            self.assertEqual(r.unsigned_min.u2le, 0)
            self.assertEqual(r.unsigned_min.u4le, 0)
            self.assertEqual(r.unsigned_min.u8le, 0)
            self.assertEqual(r.unsigned_min.u2be, 0)
            self.assertEqual(r.unsigned_min.u4be, 0)
            self.assertEqual(r.unsigned_min.u8be, 0)
            self.assertEqual(r.unsigned_max.u1, 255)
            self.assertEqual(r.unsigned_max.u2le, 65535)
            self.assertEqual(r.unsigned_max.u4le, 4294967295)
            self.assertEqual(r.unsigned_max.u8le, 18446744073709551615)
            self.assertEqual(r.unsigned_max.u2be, 65535)
            self.assertEqual(r.unsigned_max.u4be, 4294967295)
            self.assertEqual(r.unsigned_max.u8be, 18446744073709551615)
            self.assertEqual(r.signed_min.s1, -128)
            self.assertEqual(r.signed_min.s2le, -32768)
            self.assertEqual(r.signed_min.s4le, -2147483648)
            self.assertEqual(r.signed_min.s8le, -9223372036854775808)
            self.assertEqual(r.signed_min.s2be, -32768)
            self.assertEqual(r.signed_min.s4be, -2147483648)
            self.assertEqual(r.signed_min.s8be, -9223372036854775808)
            self.assertEqual(r.signed_max.s1, 127)
            self.assertEqual(r.signed_max.s2le, 32767)
            self.assertEqual(r.signed_max.s4le, 2147483647)
            self.assertEqual(r.signed_max.s8le, 9223372036854775807)
            self.assertEqual(r.signed_max.s2be, 32767)
            self.assertEqual(r.signed_max.s4be, 2147483647)
            self.assertEqual(r.signed_max.s8be, 9223372036854775807)
