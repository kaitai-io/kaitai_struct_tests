# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.bytes_eos_pad_term import BytesEosPadTerm

class TestBytesEosPadTerm(unittest.TestCase):
    def test_bytes_eos_pad_term(self):
        with BytesEosPadTerm.from_file('src/str_pad_term.bin') as r:

            self.assertEqual(r.str_pad.value, b"\x73\x74\x72\x31")
            self.assertEqual(r.str_term.value, b"\x73\x74\x72\x32\x66\x6F\x6F")
            self.assertEqual(r.str_term_and_pad.value, b"\x73\x74\x72\x2B\x2B\x2B\x33\x62\x61\x72\x2B\x2B\x2B")
            self.assertEqual(r.str_term_include.value, b"\x73\x74\x72\x34\x62\x61\x7A\x40")