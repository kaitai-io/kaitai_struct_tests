# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.bcd_user_type_le import BcdUserTypeLe

class TestBcdUserTypeLe(unittest.TestCase):
    def test_bcd_user_type_le(self):
        with BcdUserTypeLe.from_file('src/bcd_user_type_le.bin') as r:

            self.assertEqual(r.ltr.as_int, 12345678)
            self.assertEqual(r.ltr.as_str, u"12345678")
            self.assertEqual(r.rtl.as_int, 87654321)
            self.assertEqual(r.rtl.as_str, u"87654321")
            self.assertEqual(r.leading_zero_ltr.as_int, 123456)
            self.assertEqual(r.leading_zero_ltr.as_str, u"00123456")