import unittest

from enum_of_value_inst import EnumOfValueInst

class TestEnumOfValueInst(unittest.TestCase):
    def test_enum_of_value_inst(self):
        r = EnumOfValueInst.from_file("src/enum_0.bin")

        self.assertEqual(r.pet_1, EnumOfValueInst.Animal.cat)
        self.assertEqual(r.pet_2, EnumOfValueInst.Animal.chicken)
        self.assertEqual(r.pet_3, EnumOfValueInst.Animal.dog)
        self.assertEqual(r.pet_4, EnumOfValueInst.Animal.dog)
