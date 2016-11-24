import unittest

from default_endian_mod import DefaultEndianMod

class TestDefaultEndianMod(unittest.TestCase):
    def test_default_endian_mod(self):
        r = DefaultEndianMod.from_file("src/fixed_struct.bin")

        self.assertEqual(r.main.one, 0x4b434150)
        self.assertEqual(r.main.nest.two, -52947)
        self.assertEqual(r.main.nest_be.two, 0x5041434b)
