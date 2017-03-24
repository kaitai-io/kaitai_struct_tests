import unittest

from str_pad_term import StrPadTerm

class TestStrPadTerm(unittest.TestCase):
    def test_str_pad_term(self):
        r = StrPadTerm.from_file("src/str_pad_term.bin")

        self.assertEqual(r.str_pad, "str1")
        self.assertEqual(r.str_term, "str2foo")
        self.assertEqual(r.str_term_and_pad, "str+++3bar+++")
        self.assertEqual(r.str_term_include, "str4baz@")
