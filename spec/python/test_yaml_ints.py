import unittest

from yaml_ints import YamlInts

class TestYamlInts(unittest.TestCase):
    def test_yaml_ints(self):
        r = YamlInts.from_file("src/fixed_struct.bin")

        self.assertEqual(r.test_u4_dec, 0xffffffff)
        self.assertEqual(r.test_u4_hex, 0xffffffff)
        self.assertEqual(r.test_u8_dec, 0xffffffffffffffff)
        self.assertEqual(r.test_u8_hex, 0xffffffffffffffff)
