import unittest

from bcd_user_type_be import BcdUserTypeBe

class TestBcdUserTypeBe(unittest.TestCase):
    def test_bcd_user_type_be(self):
        r = BcdUserTypeBe.from_file("src/bcd_user_type_be.bin")

        self.assertEqual(r.ltr.as_int, 12345678)
        self.assertEqual(r.ltr.as_str, "12345678")
        self.assertEqual(r.rtl.as_int, 87654321)
        self.assertEqual(r.rtl.as_str, "87654321")
        self.assertEqual(r.leading_zero_ltr.as_int, 123456)
        self.assertEqual(r.leading_zero_ltr.as_str, "00123456")
