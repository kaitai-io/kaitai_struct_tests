# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from enum_to_i_invalid import _schema

class TestEnumToIInvalid(unittest.TestCase):
    def test_enum_to_i_invalid(self):
        r = _schema.parse_file('src/term_strz.bin')

        self.assertEqual(r.pet_1, 'dog')
        self.assertEqual(r.pet_2, 111)
        self.assertEqual(r.pet_2_i, 111)
        self.assertEqual(r.pet_2_i_to_s, u"111")
        self.assertEqual(r.pet_2_mod, 32879)
        self.assertEqual(r.one_lt_two, True)
        self.assertEqual(r.pet_2_eq_int_t, True)
        self.assertEqual(r.pet_2_eq_int_f, False)
