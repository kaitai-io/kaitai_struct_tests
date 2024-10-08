# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from yaml_ints import YamlInts

class TestYamlInts(unittest.TestCase):
    def test_yaml_ints(self):
        with YamlInts.from_file('src/fixed_struct.bin') as r:
            self.assertEqual(r.test_u4_dec, 4294967295)
            self.assertEqual(r.test_u4_hex, 4294967295)
            self.assertEqual(r.test_u8_dec, 18446744073709551615)
            self.assertEqual(r.test_u8_hex, 18446744073709551615)
