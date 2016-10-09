import unittest

from term_strz import TermStrz

class TestTermStrz(unittest.TestCase):
    def test_term_strz(self):
        r = TermStrz.from_file("src/term_strz.bin")

        self.assertEqual(r.s1, "foo")
        self.assertEqual(r.s2, "bar")
        self.assertEqual(r.s3, "|baz@")
