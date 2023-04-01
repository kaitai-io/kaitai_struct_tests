# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.bytes_pad_term_empty import BytesPadTermEmpty

class TestBytesPadTermEmpty(unittest.TestCase):
    def test_bytes_pad_term_empty(self):
        with BytesPadTermEmpty.from_file('src/str_pad_term_empty.bin') as r:

            self.assertEqual(r.str_pad, b"")
            self.assertEqual(r.str_term, b"")
            self.assertEqual(r.str_term_and_pad, b"")
            self.assertEqual(r.str_term_include, b"\x40")