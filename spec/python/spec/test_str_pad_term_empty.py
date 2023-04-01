# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.str_pad_term_empty import StrPadTermEmpty

class TestStrPadTermEmpty(unittest.TestCase):
    def test_str_pad_term_empty(self):
        with StrPadTermEmpty.from_file('src/str_pad_term_empty.bin') as r:

            self.assertEqual(r.str_pad, u"")
            self.assertEqual(r.str_term, u"")
            self.assertEqual(r.str_term_and_pad, u"")
            self.assertEqual(r.str_term_include, u"@")