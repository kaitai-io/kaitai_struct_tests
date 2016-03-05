import unittest

from enum_0 import Enum0

class TestEnum0(unittest.TestCase):
    def test_enum_0(self):
        r = Enum0.from_file("src/enum_0.bin")

        self.assertEquals(r.pet_1, Enum0.Animal.cat)
        self.assertEquals(r.pet_2, Enum0.Animal.chicken)
