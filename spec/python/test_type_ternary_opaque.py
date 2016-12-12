import unittest

from type_ternary_opaque import TypeTernaryOpaque

class TestTypeTernaryOpaque(unittest.TestCase):
    def test_type_ternary_opaque(self):
        r = TypeTernaryOpaque.from_file("src/term_strz.bin")

        self.assertEqual(r.dif.s1, "foo")
        self.assertEqual(r.dif.s2, "bar")
        self.assertEqual(r.dif.s3, "|baz@")
