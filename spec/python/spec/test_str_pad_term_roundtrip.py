# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.str_pad_term_roundtrip import StrPadTermRoundtrip

class TestStrPadTermRoundtrip(unittest.TestCase):
    def test_str_pad_term_roundtrip(self):
        with StrPadTermRoundtrip.from_file('src/str_pad_term.bin') as r:

            self.assertEqual(r.str_pad, u"str1")
            self.assertEqual(r.str_term, u"str2foo")
            self.assertEqual(r.str_term_and_pad, u"str+++3bar+++")
            self.assertEqual(r.str_term_include, u"str4baz@")