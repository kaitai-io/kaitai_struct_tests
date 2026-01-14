import unittest
from testformats.opaque_external_type_02_parent import OpaqueExternalType02Parent

class TestOpaqueExternalType02Parent(unittest.TestCase):
    def test_opaque_external_type_02_parent(self):
        with OpaqueExternalType02Parent.from_file('src/term_strz.bin') as r:
            self.assertEqual(r.parent.child.s1, "foo")
            self.assertEqual(r.parent.child.s2, "bar")
            self.assertEqual(r.parent.child.s3.s3, "|baz@")
