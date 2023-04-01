# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.bits_unaligned_b64_le import BitsUnalignedB64Le

class TestBitsUnalignedB64Le(unittest.TestCase):
    def test_bits_unaligned_b64_le(self):
        with BitsUnalignedB64Le.from_file('src/process_xor_4.bin') as r:

            self.assertEqual(r.a, False)
            self.assertEqual(r.b, 1902324737369038326)
            self.assertEqual(r.c, 71)