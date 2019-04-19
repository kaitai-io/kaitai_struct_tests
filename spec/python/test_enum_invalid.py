import unittest

from enum_invalid import EnumInvalid

class TestEnumInvalid(unittest.TestCase):
    def test_enum_invalid(self):
        with EnumInvalid.from_file("src/term_strz.bin") as r:
            self.assertEqual(r.pet_1, EnumInvalid.Animal.dog)
            self.assertEqual(r.pet_2, 0x6f)
