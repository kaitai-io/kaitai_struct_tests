import unittest

from str_pad_term_empty import StrPadTermEmpty

class TestStrPadTermEmpty(unittest.TestCase):
    def test_str_pad_term_empty(self):
        r = StrPadTermEmpty.from_file("src/str_pad_term_empty.bin")

        self.assertEqual(r.str_pad, "")
        self.assertEqual(r.str_term, "")
        self.assertEqual(r.str_term_and_pad, "")
        self.assertEqual(r.str_term_include, "@")
