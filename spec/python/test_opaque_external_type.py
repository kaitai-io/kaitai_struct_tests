import unittest
from opaque_external_type import OpaqueExternalType

class TestOpaqueExternalType(unittest.TestCase):
    def test_opaque_external_type(self):
        with OpaqueExternalType.from_file('src/term_strz.bin') as r:
            self.assertEqual(r.one.s1, "foo")
            self.assertEqual(r.one.s2, "bar")
            self.assertEqual(r.one.s3, "|baz@")
