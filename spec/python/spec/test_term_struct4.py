# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.term_struct4 import TermStruct4

class TestTermStruct4(unittest.TestCase):
    def test_term_struct4(self):
        with TermStruct4.from_file('src/term_strz.bin') as r:

            self.assertEqual(r.s1.value.value, b"\x66\x6F\x6F")
            self.assertEqual(r.s2.value.value, b"\x62\x61\x72")
            self.assertEqual(r.s3.value.value, b"\x62\x61\x7A")