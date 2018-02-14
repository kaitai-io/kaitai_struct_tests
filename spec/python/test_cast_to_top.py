import unittest

from cast_to_top import CastToTop

class TestCastToTop(unittest.TestCase):
    def test_cast_to_top(self):
        r = CastToTop.from_file("src/fixed_struct.bin")

        self.assertEqual(r.code, 0x50)
        self.assertEqual(r.header.code, 0x41)
        self.assertEqual(r.header_casted.code, 0x41)
