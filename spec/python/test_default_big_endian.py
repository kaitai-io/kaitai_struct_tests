import unittest

from default_big_endian import DefaultBigEndian

class TestDefaultBigEndian(unittest.TestCase):
    def test_default_big_endian(self):
        r = DefaultBigEndian.from_file("src/enum_0.bin")

        self.assertEquals(r.one, 0x7000000)
