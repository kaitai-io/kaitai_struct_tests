# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.str_eos_pad_term_equal import StrEosPadTermEqual

class TestStrEosPadTermEqual(unittest.TestCase):
    def test_str_eos_pad_term_equal(self):
        with StrEosPadTermEqual.from_file('src/str_pad_term.bin') as r:

            self.assertEqual(r.s1.value, u"str1")
            self.assertEqual(r.s2.value, u"str2foo@")
            self.assertEqual(r.s3.value, u"str")
            self.assertEqual(r.s4.value, u"str4baz@.")
