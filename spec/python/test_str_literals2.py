import unittest

from str_literals2 import StrLiterals2

class TestStrLiterals2(unittest.TestCase):
    def test_str_literals2(self):
        r = StrLiterals2.from_file("src/fixed_struct.bin")

        self.assertEqual(r.dollar1, "$foo")
        self.assertEqual(r.dollar2, "${foo}")
        self.assertEqual(r.hash, "#{foo}")
        self.assertEqual(r.at_sign, "@foo")
