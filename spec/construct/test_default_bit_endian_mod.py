# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from default_bit_endian_mod import _schema

class TestDefaultBitEndianMod(unittest.TestCase):
    def test_default_bit_endian_mod(self):
        r = _schema.parse_file('src/fixed_struct.bin')

        self.assertEqual(r.main.one, 336)
        self.assertEqual(r.main.two, 8608)
        self.assertEqual(r.main.nest.two, 11595)
        self.assertEqual(r.main.nest_be.two, 12799)
