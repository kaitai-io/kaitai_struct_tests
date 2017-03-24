import unittest

from bits_byte_aligned import BitsByteAligned

class TestBitsByteAligned(unittest.TestCase):
    def test_bits_byte_aligned(self):
        r = BitsByteAligned.from_file("src/fixed_struct.bin")

        # 50 (6 + 2) = 010100|00
        self.assertEqual(r.one, 0b010100)
        # 41
        self.assertEqual(r.byte_1, 0x41)
        # 43 (3 + 1 + 4) = 010|0|0011
        self.assertEqual(r.two, 0b010)
        self.assertEqual(r.three, False)
        # 4B
        self.assertEqual(r.byte_2, 0x4b)
        # 2D 31 (14 + 2) = 00101101 001100|01
        self.assertEqual(r.four, 0b00101101001100)
        # FF
        self.assertEqual(r.byte_3, bytearray([0xff]))
        # FF
        self.assertEqual(r.full_byte, 0xff)
        # 50
        self.assertEqual(r.byte_4, 0x50)
