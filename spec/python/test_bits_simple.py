import unittest

from bits_simple import BitsSimple

class TestBitsSimple(unittest.TestCase):
    def test_bits_simple(self):
        r = BitsSimple.from_file("src/fixed_struct.bin")

        # 50 41
        self.assertEqual(r.byte_1, 0x50)
        self.assertEqual(r.byte_2, 0x41)

        # 43 (1 + 3 + 4) = 0|100|0011
        self.assertEqual(r.bits_a, False)
        self.assertEqual(r.bits_b, 0b100)
        self.assertEqual(r.bits_c, 0b0011)

        # 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001
        self.assertEqual(r.large_bits_1, 0b0100101100)
        self.assertEqual(r.spacer, 0b101)
        self.assertEqual(r.large_bits_2, 0b10100110001)

        # FF FF
        self.assertEqual(r.normal_s2, -1)

        # 50 41 43
        self.assertEqual(r.byte_8_9_10, 0x504143)

        # 4B 2D 55 2D
        self.assertEqual(r.byte_11_to_14, 0x4B2D552D)

        # 44 45 46 FF FF
        self.assertEqual(r.byte_15_to_19, 0x444546FFFF)

        # FF FF FF FF FF FF FF FF
        self.assertEqual(r.byte_20_to_27, 0xFFFFFFFFFFFFFFFF)

        self.assertEqual(r.test_if_b1, 123)
