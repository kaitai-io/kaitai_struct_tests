# coding: utf-8
import unittest

from str_encodings_default import StrEncodingsDefault

class TestStrEncodingsDefault(unittest.TestCase):
    def test_str_encodings_default(self):
        r = StrEncodingsDefault.from_file("src/str_encodings.bin")

        self.assertEqual(r.str1, "Some ASCII")
        self.assertEqual(r.rest.str2, u"こんにちは")
        self.assertEqual(r.rest.str3, u"こんにちは")
        self.assertEqual(r.rest.str4, u"░▒▓")
