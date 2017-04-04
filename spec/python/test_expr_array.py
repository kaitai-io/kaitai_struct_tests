import unittest

from expr_array import ExprArray

class TestExprArray(unittest.TestCase):
    def test_expr_array(self):
        r = ExprArray.from_file("src/expr_array.bin")

        self.assertEqual(r.aint_size, 4)
        self.assertEqual(r.aint_first, 7657765)
        self.assertEqual(r.aint_last, 16272640)
        self.assertEqual(r.aint_min, 49185)
        self.assertEqual(r.aint_max, 1123362332)

        self.assertEqual(r.afloat_size, 3)
        self.assertEqual(r.afloat_first, -2.6839530254859364e-121)
        self.assertEqual(r.afloat_last, -1.1103359815095273e-175)
        self.assertEqual(r.afloat_min, -8.754689149998834e+288)
        self.assertEqual(r.afloat_max, -1.1103359815095273e-175)

        self.assertEqual(r.astr_size, 3)
        self.assertEqual(r.astr_first, "foo")
        self.assertEqual(r.astr_last, "baz")
        self.assertEqual(r.astr_min, "bar")
        self.assertEqual(r.astr_max, "foo")
