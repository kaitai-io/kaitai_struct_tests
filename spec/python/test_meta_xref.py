import unittest

from meta_xref import MetaXref

class TestMetaXref(unittest.TestCase):
    def test_meta_xref(self):
        r = MetaXref.from_file("src/fixed_struct.bin")
