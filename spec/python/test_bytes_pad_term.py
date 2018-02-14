import unittest

from bytes_pad_term import BytesPadTerm

class TestBytesPadTerm(unittest.TestCase):
    def test_bytes_pad_term(self):
        r = BytesPadTerm.from_file("src/str_pad_term.bin")

        self.assertEqual(r.str_pad, b"str1")
        self.assertEqual(r.str_term, b"str2foo")
        self.assertEqual(r.str_term_and_pad, b"str+++3bar+++")
        self.assertEqual(r.str_term_include, b"str4baz@")
