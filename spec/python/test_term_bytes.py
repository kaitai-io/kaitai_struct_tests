import unittest

from term_bytes import TermBytes

class TestTermBytes(unittest.TestCase):
    def test_term_bytes(self):
        r = TermBytes.from_file("src/term_strz.bin")

        self.assertEqual(r.s1, b"foo")
        self.assertEqual(r.s2, b"bar")
        self.assertEqual(r.s3, b"|baz@")
