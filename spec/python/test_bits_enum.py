import unittest

from bits_enum import BitsEnum

class TestBitsEnum(unittest.TestCase):
    def test_bits_enum(self):
        r = BitsEnum.from_file("src/fixed_struct.bin")

        # 50 41 (4 + 8 + 1) = 0101|0000 0100|0|001
        self.assertEqual(r.one, BitsEnum.Animal.platypus)
        self.assertEqual(r.two, BitsEnum.Animal.horse)
        self.assertEqual(r.three, BitsEnum.Animal.cat)
