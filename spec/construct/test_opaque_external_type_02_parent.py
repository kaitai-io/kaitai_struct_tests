import unittest

from opaque_external_type_02_parent import _schema

class TestOpaqueExternalType02Parent(unittest.TestCase):
    def test_opaque_external_type_02_parent(self):
        r = _schema.parse_file('src/term_strz.bin')

        self.assertEqual(r.parent.child.s1, u"foo")
        self.assertEqual(r.parent.child.s2, u"bar")
        self.assertEqual(r.parent.child.s3.s3, u"|baz@")
