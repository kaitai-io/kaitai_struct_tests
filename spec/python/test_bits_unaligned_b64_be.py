# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from bits_unaligned_b64_be import BitsUnalignedB64Be

class TestBitsUnalignedB64Be(unittest.TestCase):
    def test_bits_unaligned_b64_be(self):
        with BitsUnalignedB64Be.from_file('src/process_xor_4.bin') as r:
            self.assertEqual(r.a, True)
            self.assertEqual(r.b, 15670070570729969769)
            self.assertEqual(r.c, 14)
