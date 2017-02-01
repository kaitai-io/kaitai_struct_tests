import unittest

from bcd_user_type_le import BcdUserTypeLe

class TestBcdUserTypeLe(unittest.TestCase):
    def test_bcd_user_type_le(self):
        r = BcdUserTypeLe.from_file("src/bcd_user_type_le.bin")

        self.assertEqual(r.ltr.as_int, 12345678)
        self.assertEqual(r.ltr.as_str, "12345678")
        self.assertEqual(r.rtl.as_int, 87654321)
        self.assertEqual(r.rtl.as_str, "87654321")
        self.assertEqual(r.leading_zero_ltr.as_int, 123456)
        self.assertEqual(r.leading_zero_ltr.as_str, "00123456")
