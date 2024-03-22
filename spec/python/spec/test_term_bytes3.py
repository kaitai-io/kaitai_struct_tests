# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.term_bytes3 import TermBytes3

class TestTermBytes3(unittest.TestCase):
    def test_term_bytes3(self):
        with TermBytes3.from_file('src/term_strz.bin') as r:

            self.assertEqual(r.s1, b"\x66\x6F\x6F")
            self.assertEqual(r.s2, b"\x7C\x62\x61\x72\x7C\x62\x61\x7A")
            self.assertEqual(r.s3, b"")
