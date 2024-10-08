# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from enum_long_range_s import EnumLongRangeS
import enum_long_range_s

class TestEnumLongRangeS(unittest.TestCase):
    def test_enum_long_range_s(self):
        with EnumLongRangeS.from_file('src/enum_long_range_s.bin') as r:
            self.assertEqual(r.f1, enum_long_range_s.EnumLongRangeS.Constants.long_min)
            self.assertEqual(r.f2, enum_long_range_s.EnumLongRangeS.Constants.int_below_min)
            self.assertEqual(r.f3, enum_long_range_s.EnumLongRangeS.Constants.int_min)
            self.assertEqual(r.f4, enum_long_range_s.EnumLongRangeS.Constants.zero)
            self.assertEqual(r.f5, enum_long_range_s.EnumLongRangeS.Constants.int_max)
            self.assertEqual(r.f6, enum_long_range_s.EnumLongRangeS.Constants.int_over_max)
            self.assertEqual(r.f7, enum_long_range_s.EnumLongRangeS.Constants.long_max)
