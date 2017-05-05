import unittest

from switch_integers2 import SwitchIntegers2

class TestSwitchIntegers2(unittest.TestCase):
    def test_switch_integers2(self):
        r = SwitchIntegers2.from_file("src/switch_integers.bin")

        self.assertEqual(r.code, 1)
        self.assertEqual(r.len, 7)
        self.assertEqual(r.ham, bytearray([2, 64, 64, 4, 55, 19, 0]))
        self.assertEqual(r.padding, 0)
        self.assertEqual(r.len_mod_str, "13")
