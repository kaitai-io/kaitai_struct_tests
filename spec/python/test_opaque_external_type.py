import unittest

from opaque_external_type import OpaqueExternalType

class TestOpaqueExternalType(unittest.TestCase):
    def test_opaque_external_type(self):
        r = OpaqueExternalType.from_file("src/term_strz.bin")

        self.assertEquals(r.one.s1, "foo")
        self.assertEquals(r.one.s2, "bar")
        self.assertEquals(r.one.s3, "|baz@")
