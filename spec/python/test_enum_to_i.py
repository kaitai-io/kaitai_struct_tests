import unittest

from enum_to_i import EnumToI

class TestEnumToI(unittest.TestCase):
    def test_enum_to_i(self):
        r = EnumToI.from_file("src/enum_0.bin")

        self.assertEqual(r.pet_1, EnumToI.Animal.cat)
        self.assertEqual(r.pet_2, EnumToI.Animal.chicken)

        self.assertEqual(r.pet_1_i, 7)
        self.assertEqual(r.one_lt_two, True)
