# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from bits_byte_aligned import BitsByteAligned

class TestBitsByteAligned(unittest.TestCase):
    def test_bits_byte_aligned(self):
        with BitsByteAligned.from_file('src/fixed_struct.bin') as r:
            self.assertEqual(r.one, 20)
            self.assertEqual(r.byte_1, 65)
            self.assertEqual(r.two, 2)
            self.assertEqual(r.three, False)
            self.assertEqual(r.byte_2, 75)
            self.assertEqual(r.four, 2892)
            self.assertEqual(r.byte_3, b"\xFF")
            self.assertEqual(r.full_byte, 255)
            self.assertEqual(r.byte_4, 80)
