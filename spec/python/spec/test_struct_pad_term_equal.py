# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.struct_pad_term_equal import StructPadTermEqual

class TestStructPadTermEqual(unittest.TestCase):
    def test_struct_pad_term_equal(self):
        with StructPadTermEqual.from_file('src/str_pad_term.bin') as r:

            self.assertEqual(r.s1.value, b"\x73\x74\x72\x31")
            self.assertEqual(r.s2.value, b"\x73\x74\x72\x32\x66\x6F\x6F\x40")
            self.assertEqual(r.s3.value, b"\x73\x74\x72")
            self.assertEqual(r.s4.value, b"\x73\x74\x72\x34\x62\x61\x7A\x40\x2E")