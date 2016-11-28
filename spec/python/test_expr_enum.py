import unittest

from expr_enum import ExprEnum

class TestExprEnum(unittest.TestCase):
    def test_expr_enum(self):
        r = ExprEnum.from_file("src/term_strz.bin")

        self.assertEqual(r.const_dog, ExprEnum.Animal.dog)
        self.assertEqual(r.derived_boom, ExprEnum.Animal.boom)
        self.assertEqual(r.derived_dog, ExprEnum.Animal.dog)
