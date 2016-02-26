import unittest

from term_strz import TermStrz

class TestTermStrz(unittest.TestCase):
    def test_term_strz(self):
        r = TermStrz.from_file("src/term_strz.bin")

        self.assertEquals(r.s1, "foo")
        self.assertEquals(r.s2, "bar")
        self.assertEquals(r.s3, "|baz@")
