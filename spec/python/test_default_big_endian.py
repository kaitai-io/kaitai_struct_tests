import unittest

from default_big_endian import DefaultBigEndian

class TestDefaultBigEndian(unittest.TestCase):
    def test_default_big_endian(self):
        with DefaultBigEndian.from_file("src/enum_0.bin") as r:

            self.assertEqual(r.one, 0x7000000)
