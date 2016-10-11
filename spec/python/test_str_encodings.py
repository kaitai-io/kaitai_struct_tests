# coding: utf-8
import unittest

from str_encodings import StrEncodings

class TestStrEncodings(unittest.TestCase):
    def test_str_encodings(self):
        with StrEncodings.from_file("src/str_encodings.bin") as r:

            self.assertEqual(r.str1, "Some ASCII")
            self.assertEqual(r.str2, u"こんにちは")
            self.assertEqual(r.str3, u"こんにちは")
            self.assertEqual(r.str4, u"░▒▓")
