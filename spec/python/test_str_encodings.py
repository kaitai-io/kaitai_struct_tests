# coding: utf-8
import unittest

from str_encodings import StrEncodings

class TestStrEncodings(unittest.TestCase):
    def test_str_encodings(self):
        r = StrEncodings.from_file("src/str_encodings.bin")

        self.assertEquals(r.str1, "Some ASCII")
        self.assertEquals(r.str2, u"こんにちは")
        self.assertEquals(r.str3, u"こんにちは")
        self.assertEquals(r.str4, u"░▒▓")
