import unittest

from enum_negative import EnumNegative

class TestEnumNegative(unittest.TestCase):
    def test_enum_negative(self):
        r = EnumNegative.from_file("src/enum_negative.bin")

        self.assertEqual(r.f1, EnumNegative.Constants.negative_one)
        self.assertEqual(r.f2, EnumNegative.Constants.positive_one)
